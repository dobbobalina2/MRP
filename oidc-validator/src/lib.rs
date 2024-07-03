mod certs;
use alloy_primitives::{Uint, U256};
use certs::{GOOGLE_PUB_JWK, TEST_PUB_JWK};
use jwt_compact::{
    alg::{Rsa, RsaPublicKey},
    jwk::{JsonWebKey, KeyType},
    AlgorithmExt, UntrustedToken,
};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use thiserror::Error;

lazy_static! {
    static ref GOOGLE_KEYS: JwkKeys =
        serde_json::from_str(GOOGLE_PUB_JWK).expect("Failed to parse JWK");
    static ref TEST_KEYS: JwkKeys =
        serde_json::from_str(TEST_PUB_JWK).expect("Failed to parse JWK");
}

#[derive(Deserialize, Serialize)]
struct JwkKeys {
    keys: Vec<ExtendedJsonWebKey<'static, Extra>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ExtendedJsonWebKey<'a, T> {
    #[serde(flatten)]
    base: JsonWebKey<'a>,
    #[serde(flatten)]
    extra: T,
}

#[derive(Debug, Deserialize, Serialize)]
struct Extra {
    #[serde(rename = "kid")]
    key_id: String,
}

#[derive(Deserialize, Serialize)]
pub enum IdentityProvider {
    Google,
    Test,
}

impl IdentityProvider {
    pub fn validate(&self, token: &str) -> Result<(String, String), OidcErr> {
        match self {
            Self::Google => {
                let decoded = decode_token::<GoogleClaims>(token, &GOOGLE_KEYS).unwrap();
                Ok((decoded.email.to_string(), decoded.nonce))
            }
            Self::Test => {
                let decoded = decode_token::<TestClaims>(token, &TEST_KEYS).unwrap();
                Ok((decoded.email.to_string(), decoded.nonce))
            }
        }
    }
}

impl From<Uint<256, 4>> for IdentityProvider {
    fn from(value: Uint<256, 4>) -> Self {
        match value {
            U256::ZERO => Self::Google,
            _ => Self::Test,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GoogleClaims {
    pub aud: String,
    pub iss: String,
    pub sub: String,
    pub nonce: String, // I require this one.
    pub email: String, // And this one too.
    pub exp: Option<u64>,
    pub iat: Option<u64>,
    pub at_hash: Option<String>,
    pub azp: Option<String>,
    pub email_verified: Option<bool>,
    pub family_name: Option<String>,
    pub given_name: Option<String>,
    pub hd: Option<String>,
    pub locale: Option<String>,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub nbf: Option<u64>,
    pub jti: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TestClaims {
    pub email: String,
    pub nonce: String,
}

#[derive(Error, Debug)]
pub enum OidcErr {
    #[error("Failed to parse certificate")]
    CertificateParseError,
    #[error("Failed to decode token")]
    TokenDecodeError,
    #[error("Algorithm not found")]
    AlgorithmNotFoundError,
    #[error("Failed to generate token")]
    TokenGenerationError,
    #[error("Failed to validate token")]
    TokenValidationError,
    #[error("Certificate not found")]
    CertificateNotFoundError,
    #[error("Key id missing")]
    KeyIdMissingError,
}

fn decode_token<T>(token: &str, keys: &JwkKeys) -> Result<T, OidcErr>
where
    T: for<'de> Deserialize<'de> + Serialize + Clone,
{
    let token = UntrustedToken::new(token).map_err(|_| OidcErr::TokenDecodeError)?;

    let key_id = token
        .header()
        .key_id
        .as_deref()
        .ok_or(OidcErr::KeyIdMissingError)?;
    println!("kid: {:?} ", key_id);
    let key = keys
        .keys
        .iter()
        .find(|k| k.extra.key_id == key_id)
        .ok_or(OidcErr::CertificateNotFoundError)?;

    let (alg, vkey) = match key.base.key_type() {
        KeyType::Rsa => RsaPublicKey::try_from(&key.base)
            .map(|vkey| (Rsa::rs256(), vkey))
            .map_err(|_| OidcErr::CertificateParseError)?,
        _ => return Err(OidcErr::AlgorithmNotFoundError),
    };

    // Validate the token integrity.
    // NOTE: This does not verify the `exp` field.
    let res = alg
        .validate_integrity::<T>(&token, &vkey)
        .map_err(|_| OidcErr::TokenValidationError);

    Ok(res.unwrap().claims().custom.clone())
}

#[cfg(test)]
pub mod test_oidc_validator {

    use crate::GOOGLE_KEYS;
    use std::env;

    use super::{decode_token, GoogleClaims};

    #[ignore] // Ignoring this test because it requires a valid jwt token with env var.
    #[test]
    fn test_validate_google_jwt_valid_token() {
        let jwt = env::var("jwt").expect("jwt not set");
        let decoded = decode_token::<GoogleClaims>(&jwt, &GOOGLE_KEYS).unwrap();

        assert_eq!(&decoded.email, "hans@risczero.com");
        assert_eq!(&decoded.nonce, "0xefdF9861F3eDc2404643B588378FE242FCadE658");
    }

    #[test]
    fn test_validate_test_jwt_valid_token() {
        let jwt = "eyJhbGciOiJSUzI1NiIsImtpZCI6Ijg3YmJlMDgxNWIwNjRlNmQ0NDljYWM5OTlmMGU1MGU3MmEzZTQzNzQiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyODAzNzI3MzkzNjgtcXY0YnJ2YTBlaXEwdjFvbzFqdHNxZGFwaDZtdjdvbW8uYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyODAzNzI3MzkzNjgtcXY0YnJ2YTBlaXEwdjFvbzFqdHNxZGFwaDZtdjdvbW8uYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTc3MzYzNTE4MjIzNTY1NTc3NDkiLCJlbWFpbCI6ImpvaG5rZW5ueTY3OTlAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5vbmNlIjoiMHgyM0Q0YThkMjZCNzc3YzFGRGNCQjc0YWZhNzlDQWRBMWNhRjc3MkY4IiwibmJmIjoxNzE5OTQwNDE5LCJuYW1lIjoiSm9obiBLZW5ueSIsInBpY3R1cmUiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BQ2c4b2NKdHczTGFqNXdUNUN4QjV2ZzJySjJkSnlHWWpTX29MaXliMEkzTDIwTmJFeHBBdXc9czk2LWMiLCJnaXZlbl9uYW1lIjoiSm9obiIsImZhbWlseV9uYW1lIjoiS2VubnkiLCJpYXQiOjE3MTk5NDA3MTksImV4cCI6MTcxOTk0NDMxOSwianRpIjoiOTk5ZmM4YjNlZjc4ZmIwYzEyODMzMGZkNGUyOWI0YTZmZmU4OGNiNyJ9.gFthXoI5nj-e59qEdiZwxCJO9WDZBVRvKqYNufrQIlRTnSRH6pjSyHJp0b_eGalM38zMZu4q8CdOAaNj-VwrrkKb-iGKBY_A7JngwSp3s_0F20lR-uMZQcrLK1iilAen1wRlj2NEbX6lZ3rmsrNRDwHBaUnJ_eZRjlOcKQjruqkGy5_aVEz6FzFglUzmQuHIlkLZIr3G8W56J5sLoj78oq_DgssfrdI-YDusr9N7FtsuGcoMtZ5AZNH19xkbx2-mQkcN_hXqHgliEh_OAmy99AyxhygEHg961jruj9vxcKQnQjDIcXCZEH1iVMOEPwoX8mLlGQXycqlS1OUD-rIoEQ";
        let decoded = decode_token::<super::TestClaims>(&jwt, &super::TEST_KEYS).unwrap();

        assert_eq!(&decoded.email, "johnkenny6799@gmail.com");
        assert_eq!(&decoded.nonce, "0x23D4a8d26B777c1FDcBB74afa79CAdA1caF772F8");
    }

    #[test]
    #[should_panic]
    fn test_fail_invalid_test_token() {
        let jwt = "xxxxxxxxxxxxxxxxxxxxxxxxxxxxx.xxxxxxxxxxxxxxxxxxxx.xxxxxxxxxxxxxxxxxxxxx";
        decode_token::<super::TestClaims>(&jwt, &super::TEST_KEYS).unwrap();
    }
}
