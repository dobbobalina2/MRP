import React, { useState, useCallback, useEffect, useContext } from "react";
import Account from "./Account";
import { AaContext } from "../AaContext";
import { useBalance } from "wagmi";
import { toast } from "react-toastify";
import { isAddress } from 'viem'


interface SendProps {
  currentStep: number,
  setCurrentStep: React.Dispatch<React.SetStateAction<number>>,
}

const Send: React.FC<SendProps> = ({currentStep,setCurrentStep}) => {
  const [isLoading, setIsLoading] = useState(false);
  const { address,debounceDest,setDest } = useContext(AaContext); 


  const [isNonZeroBalance, setIsNonZeroBalance] = useState(false);



  const { data:balance } = useBalance({
    address: address,
    watch:true,
  });

 

  useEffect(() => { 
    setIsNonZeroBalance(balance?.value !== 0n);
  }, [balance]);


  const { VITE_API_HOST } = import.meta.env;

  const handleConfirmation= ()=>{
    setCurrentStep(4);
  }

  const handleSendTX = useCallback(async () => {
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
      const response = await fetch(`${VITE_API_HOST}/execute`, {
        method: "GET",
        headers: {
          "X-Auth-Token": jwt,
          "X-DEST": debounceDest ,
        },
      });

      if (response.ok) {
        await response.body;
        toast.success("Your tx has been sent")
      } else {
        throw new Error("Response not OK");
      }
    } catch (error) {
      console.error("Error fetching data:", error);
    } finally {
      setIsLoading(false);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <>
      <Account  disabled={false} hideClaim={true} />
      <div className="send-input">
      <input
        type="text"
        onChange={(e) => setDest(e.target.value)}
        value= {debounceDest}
        disabled ={currentStep == 5}
        placeholder="Destination Address"
        style={{margin: 'auto', marginBottom:'1em'}}
      />
      <button
       style={{margin: 'auto'}}
       onClick={currentStep ==3 ? handleConfirmation : handleSendTX} 
       disabled={!isNonZeroBalance || isLoading || !debounceDest ||  !isAddress(debounceDest) }>
       {currentStep ==3 ? "Confirm Destination": isLoading ? "Sending..." : "Send Payment™"}
      </button>
      </div>
      {isLoading ? <p>This will take a few moments...</p> : <p></p>} 
    </>
  );
};

export default Send;