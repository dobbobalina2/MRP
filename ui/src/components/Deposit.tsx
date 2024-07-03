import React, { useState, useEffect, useCallback, useContext } from "react";
import { AaContext } from "../AaContext";
import { toast } from "react-toastify";
import { SignInWithGoogle } from "./SignInWithGoogle";
import Send from "./Send";

interface DepositProps {}

const Deposit: React.FC<DepositProps> = () => {
  const [isLoading, setIsLoading] = useState(false);
  const {address, debounceDest,  setAddress, jwtExists, walletExists,email,nonceExists } = useContext(AaContext); 
   const [currentStep, setCurrentStep] = useState<number>(1);


  const { VITE_API_HOST } = import.meta.env;

  

  const handleClick = useCallback(async () => {
    setIsLoading(true);

    const jwtCookie = document.cookie
      .split("; ")
      .find((row) => row.startsWith("jwt="));
    const jwt = jwtCookie?.split("=")[1];

    if (!jwt) {
      console.error("JWT not found");
      setIsLoading(false);
      return;
    }
    console.log(jwt);

    try {
      const response = await fetch(`${VITE_API_HOST}/deploy`, {
        method: "GET",
        headers: {
          "X-Auth-Token": jwt,
        },
      });

      if (response.ok) {
        const json = await response.json();
        document.cookie = `aaAddress=${json}; SameSite=None; Secure`;
        setAddress(json);
        toast.success(`Your wallet has been created at: ${json}`)

      } else {
        throw new Error("Response not OK");
      }
    } catch (error) {
      toast.error("Deployment failed to go through. Please try again")
      console.error("Error fetching data:", error);
    } finally {
      setIsLoading(false);
    }
  }, [VITE_API_HOST, setAddress]);

  const stepDescriptions = ["Sign In", "Create", "Send"];

  const renderStepIndicator = () => {
    return (
      <div className="step-indicator">
        {stepDescriptions.map((description, index) => (
          <div
            key={index}
            className={`step ${index + 1 === currentStep ? "current" : ""}`}
          >
            {index + 1}: {description}
          </div>
        ))}
      </div>
    );
  };

  

  useEffect(() => {
    if (!jwtExists) {
      setCurrentStep(1);
    } else if (!walletExists && jwtExists) {
      setCurrentStep(2);
    }else if ( nonceExists){
      console.log('here5')

      setCurrentStep(5);
    }else if (walletExists && jwtExists) {
      console.log('here3')
      setCurrentStep(3);
    }
  }, [walletExists, jwtExists,nonceExists]);

  const renderCurrentStep = () => {
    switch (currentStep) {
      case 1:
        return (
          <>
            <h4>Sign in to your account</h4>
            <SignInWithGoogle disabled={jwtExists} />
          </>
        );
      case 2:
        return (
          <>
            <h4>Create Your New Account</h4>
            {email && <h5>{`Welcome, ${email}`}</h5>}
            <button type="submit" disabled={isLoading} onClick={handleClick}>
              {isLoading ? "Creating..." : "Create"}
            </button>
            <h6>
              Creating will automatically give you <br />
              some Holesky ETH.
            </h6>
          </>
        );
      case 3:
        return (
          <>
           <Send currentStep= {currentStep} setCurrentStep= {setCurrentStep} />
          </>
        );

      case 4: return(
        <>
        <h4>Confirm the Destination address by signing in</h4>
        <h5>Destination Address: <br/>  {debounceDest}</h5>
        <SignInWithGoogle disabled={jwtExists} />
      </>
      )
      case 5: return(
        <>
        <Send  currentStep= {currentStep} setCurrentStep= {setCurrentStep} />
       </>
      )
      default:
        return <p>Unknown step</p>;
    }
  };

  return (
    <div className="deposit-container">
      {renderStepIndicator()}
      <div className="step-content">{renderCurrentStep()}</div>
      <h6>Only Google accounts are supported.</h6>
      <h5>This process may take awhile please be patient</h5>
    </div>
  );
};

export default Deposit;