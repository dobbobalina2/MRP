import React, { useContext } from "react";
import { formatUnits } from "viem";
import { Token } from "../libs/types";
import { useBalance } from 'wagmi'
import { AaContext } from "../AaContext";

interface BalanceProps {
  token: Token;
  disabled: boolean;
  hideClaim?: boolean;
}

export const Balance: React.FC<BalanceProps> = ({
  token,
  disabled,
  hideClaim,
}) => {

  const { address } = useContext(AaContext); 

  const { data:balance } = useBalance({
    address: address,
    watch:true,
  });

  // refresh balance every 5 seconds
 
  return (
    <>
    <div>
      <h5>Wallet: <br/> {address}</h5>
      <p>Availiable Balance:</p>
    </div>
      <div className="balance">
        <img src={token.icon} alt={token.name} />
        <p>
          {` ${formatUnits(balance?.value || 0n, token.decimals)} ${token.name}
        `}
        </p>
      </div>
      <button
        disabled={disabled}
        hidden={hideClaim ?? false}
      >
        {`Claim ${token.name}`}
      </button>
    </>
  );
};
