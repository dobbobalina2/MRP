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

import {console2} from "forge-std/console2.sol";
import {Test} from "forge-std/Test.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";

import {AADemo} from "../contracts/AADemo.sol";
import "forge-std/console.sol";
import {Elf} from "./Elf.sol"; // auto-generated contract after running `cargo build`.

contract AADemoTest is RiscZeroCheats, Test {
    AADemo public aaDemo;
    address public alice = makeAddr("alice");
    address public bob = makeAddr("bob");
    address public charlie = makeAddr("charlie");

    struct Input {
        uint256 id_provider;
        string jwt;
    }

    function setUp() public {
        IRiscZeroVerifier verifier = deployRiscZeroVerifier();
        aaDemo = new AADemo(verifier);

        // fund alice and bob and charlie
        vm.deal(alice, 5 ether);
        vm.deal(bob, 5 ether);
        vm.deal(charlie, 5 ether);
        vm.deal(0x23D4a8d26B777c1FDcBB74afa79CAdA1caF772F8, 5 ether);
        vm.deal(payable(address(aaDemo)),5 ether);
    }

    function test_SetOwner() public payable {
        bytes32 claimId = sha256(abi.encodePacked("bob@email.com"));
        vm.prank(alice);
        aaDemo.setOwner(claimId);
        assertEq(aaDemo.owner() , claimId);
    }

    function test_Execute() public {
        // deposit as alice
        bytes32 claimId = sha256(abi.encodePacked("johnkenny6799@gmail.com"));
        vm.prank(alice);

        // claim as bob

        string memory jwt ="eyJhbGciOiJSUzI1NiIsImtpZCI6Ijg3YmJlMDgxNWIwNjRlNmQ0NDljYWM5OTlmMGU1MGU3MmEzZTQzNzQiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIyODAzNzI3MzkzNjgtcXY0YnJ2YTBlaXEwdjFvbzFqdHNxZGFwaDZtdjdvbW8uYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJhdWQiOiIyODAzNzI3MzkzNjgtcXY0YnJ2YTBlaXEwdjFvbzFqdHNxZGFwaDZtdjdvbW8uYXBwcy5nb29nbGV1c2VyY29udGVudC5jb20iLCJzdWIiOiIxMTc3MzYzNTE4MjIzNTY1NTc3NDkiLCJlbWFpbCI6ImpvaG5rZW5ueTY3OTlAZ21haWwuY29tIiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5vbmNlIjoiMHgyM0Q0YThkMjZCNzc3YzFGRGNCQjc0YWZhNzlDQWRBMWNhRjc3MkY4IiwibmJmIjoxNzIwMDM1MzUzLCJuYW1lIjoiSm9obiBLZW5ueSIsInBpY3R1cmUiOiJodHRwczovL2xoMy5nb29nbGV1c2VyY29udGVudC5jb20vYS9BQ2c4b2NKdHczTGFqNXdUNUN4QjV2ZzJySjJkSnlHWWpTX29MaXliMEkzTDIwTmJFeHBBdXc9czk2LWMiLCJnaXZlbl9uYW1lIjoiSm9obiIsImZhbWlseV9uYW1lIjoiS2VubnkiLCJpYXQiOjE3MjAwMzU2NTMsImV4cCI6MTcyMDAzOTI1MywianRpIjoiYmY2ZWM3NmE5MDY2YmRlODY1NDc1ZGM2NDgxNDk2MWY2M2YxNTNjZiJ9.eEhzAmDcE-nilHL8v64Agd499hQkEgmmWUZFRa0pZyYVtK_p5kIpAjuo0dbX_QcjRYjhBzO3rzUrpvLnmQ2rfLFktCdnz-vfH1gk7XHCQaDvgW3hDMUa55RHpnp5Feaqs8mXcGf9ZVEBf7yMJ_-i-404qXzollG2EYPvmdjyfPCAYQhugKmjjeqYJiCGtQjwpNEkSanrBQ4UQOCf6Vi0hbN60bGBv6bXFbl820hKhfl4SaqX70S5DTFuccPsZXsU9eIJslWtIGcbqJurNY93OK2KAL7BCUZYXRmBihxoNVqgqWR9XmWtD1O3AXJDN54zq6yuWYeQajPq3X0h2HfWcA";
         uint256 id_provider = 1;

        Input memory input = Input({id_provider: id_provider, jwt: jwt});
        console.log("here");      

        (bytes memory journal, bytes memory seal) = prove(Elf.JWT_VALIDATOR_PATH, abi.encode(input));
        console.log("here2");      

        bytes memory fake_seal = hex"00000000000000000000000000000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000000104310fe5982cc637e458c64bfe07775b2176f091e33068b7ab286fcedf8ef7a5e7724c805e1541f4b54c800f271eeb27d6b7df2d7b6c2b1f06c290ab1f95a532fb7d881f7727f9cdf176bf977dca711457156a3ad5836e4cf80a7db10c643a4646263c0fb52c59ed606d4c336d09cf9566ab7de23abde16f45fe6a2b66cf9b01a14320b3f32f01d31798dd1fe21cb033795d156026106707e4be5527a3fdfcf2084d86b0532370f8a6fb589744a06a7d6fe01f947bb10dd8c74d1b8ccb7c1a92a192703b1026bc76398819f6d7de69128e12c0673df77ed1232f456620187a78216d14b12e14e9337fe3a4080480a4eff8953197f2a228621730b0053fbbb52f0fa4d64f4e00000000000000000000000000000000000000000000000000000000";
        vm.prank(0x23D4a8d26B777c1FDcBB74afa79CAdA1caF772F8);
        aaDemo.setOwner(claimId);  
        console.log("here3");      
        console.logBytes32(aaDemo.owner());

        vm.prank(alice);
        aaDemo.execute(payable(0x23D4a8d26B777c1FDcBB74afa79CAdA1caF772F8), 1 ether , "", claimId, seal);
        assertEq(payable(address(aaDemo)).balance, 4 ether);
        assertEq(address(0x23D4a8d26B777c1FDcBB74afa79CAdA1caF772F8).balance, 6 ether);        
    }
    




}
