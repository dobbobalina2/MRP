import React from "react";
import tokens from "../assets/tokens.json";
import { Balance } from "./Balance";
import { Token } from "../libs/types";

interface AccountProps {
  disabled: boolean;
  hideClaim: boolean;
}

const Account: React.FC<AccountProps> = (props) => {
  const { disabled, hideClaim } = props;


  return (
    <>
      <div className="balance-container">
        <Balance
          
          token={tokens["sepolia"][0] as Token}
          disabled={disabled}
          hideClaim={hideClaim}
        />
      </div>
    </>
  );
};

export default Account;
