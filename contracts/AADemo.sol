
// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.20;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol";

contract AADemo {
    IRiscZeroVerifier public immutable verifier;
    bytes32 public constant imageId = ImageID.JWT_VALIDATOR_ID;
    bytes32  public  owner;

    event DebugSeal(bytes32 claimId,  bytes seal);
    event Debugfunc(address dest, uint256 value, bytes  func);

    error InvalidClaim(string message);
    error TransferFailed();
    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }


    /**
     * execute a transaction (called directly from owner, or by entryPoint)
     * @param _owner email of owner
 
     */

    function setOwner(bytes32 _owner) public {
    if (_owner == bytes32(0)) revert InvalidClaim("Empty owner");

    owner = _owner;

    }
    
    receive() external payable {}
    
    fallback() external payable {}


     /**
     * execute a transaction (called directly from owner, or by entryPoint)
     * @param dest destination address to call
     * @param value the value to pass in this call
     * @param func the calldata to pass in this call
     */

    function execute(address dest, uint256 value, bytes calldata func,  bytes32 claimId, bytes calldata seal) external {
        require(claimId == owner, "You are not the owner");
        emit DebugSeal(claimId, seal);
       verifier.verify(seal, imageId , sha256(abi.encode(dest, claimId)));
        emit Debugfunc(dest,value,func);
        _call(dest, value, func);
    }

     function _call(address target, uint256 value, bytes memory data) internal {
        (bool success, bytes memory result) = target.call{value: value}(data);
        if (!success) {
            assembly {
                revert(add(result, 32), mload(result))
            }
        }
    }

}