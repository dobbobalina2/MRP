import React, { createContext, useEffect, useState } from 'react';
import { GoogleTokenPayload } from './libs/types';
import { useDebounce } from "use-debounce";

// Create the context
export const AaContext = createContext(null);

// Create a provider component
export const AaProvider = ({ children }) => {
    const [address, setAddress] = useState("");
    const [jwtExists, setJwtExists] = useState<boolean>(false);
    const [walletExists, setWalletExists] = useState<boolean>(false);
    const [email, setEmail] = useState<string | null>(null);
    const [dest ,setDest] = useState<string > ("");
    const [debounceDest] = useDebounce (dest,800);
    const [nonceExists, setNonceExists] = useState<boolean > (false);



    const checkCookies = () => {
        const aaAddress = document.cookie
            .split("; ")
            .find((row) => row.startsWith("aaAddress="));
        const addressValue = aaAddress && aaAddress.split("=")[1];

        if (addressValue && addressValue !== "") {
            setWalletExists(true);
            setAddress(addressValue);
        }

        const jwt = document.cookie
            .split("; ")
            .find((row) => row.startsWith("jwt="));
        const jwtValue = jwt && jwt.split("=")[1];
       

        if (jwtValue && jwtValue !== "") {
            const payload: GoogleTokenPayload = JSON.parse(
              atob(jwtValue.split(".")[1])
            );
            setEmail(payload.email);
            setJwtExists(Boolean(jwtValue));
            setNonceExists(payload.nonce && payload.nonce !=="");
            console.log(nonceExists);

        }
        
    };

    useEffect(() => {
        checkCookies();
        const intervalId = setInterval(checkCookies, 2000);
        return () => clearInterval(intervalId);
    }, []);


    return (
        <AaContext.Provider value={{ address, setAddress,jwtExists,walletExists,email,debounceDest,setDest,dest,nonceExists }}>
            {children}
        </AaContext.Provider>
    );
};