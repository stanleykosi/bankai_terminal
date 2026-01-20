Directory structure:
└── polymarket-ctf-exchange/
    ├── README.md
    ├── foundry.lock
    ├── foundry.toml
    ├── LICENSE.md
    ├── package.json
    ├── remappings.txt
    ├── .env.example
    ├── .prettierrc
    ├── artifacts/
    │   └── CTFExchange.json
    ├── broadcast/
    │   └── ExchangeDeployment.s.sol/
    │       ├── 137/
    │       │   ├── deployExchange-1663954950.json
    │       │   ├── deployExchange-1663955866.json
    │       │   ├── deployExchange-1663958824.json
    │       │   ├── deployExchange-1663958850.json
    │       │   ├── deployExchange-1663958971.json
    │       │   ├── deployExchange-1663958977.json
    │       │   └── deployExchange-1664228337.json
    │       └── 80001/
    │           ├── deployExchange-1663792323.json
    │           ├── deployExchange-1663792337.json
    │           ├── deployExchange-1663954744.json
    │           ├── deployExchange-1663954757.json
    │           ├── deployExchange-1663955818.json
    │           ├── deployExchange-1663955831.json
    │           ├── deployExchange-1664228099.json
    │           ├── deployExchange-1664228139.json
    │           └── deployExchange-latest.json
    ├── deploy/
    │   └── scripts/
    │       └── deploy_exchange.sh
    ├── docs/
    │   ├── CTFExchange.md
    │   ├── Overview.md
    │   └── mixins/
    │       ├── AssetOperations.md
    │       ├── Assets.md
    │       ├── Auth.md
    │       ├── Fees.md
    │       ├── Hashing.md
    │       ├── NonceManager.md
    │       ├── Pausable.md
    │       ├── ProxyFactoryHelper.md
    │       ├── Registry.md
    │       ├── Signatures.md
    │       └── Trading.md
    ├── src/
    │   ├── common/
    │   │   ├── ERC20.sol
    │   │   ├── ReentrancyGuard.sol
    │   │   ├── auth/
    │   │   │   ├── Authorized.sol
    │   │   │   ├── Ownable.sol
    │   │   │   ├── Owned.sol
    │   │   │   └── interfaces/
    │   │   │       ├── IAuthorized.sol
    │   │   │       └── IOwned.sol
    │   │   ├── interfaces/
    │   │   │   └── IERC20.sol
    │   │   └── libraries/
    │   │       └── SafeTransferLib.sol
    │   ├── dev/
    │   │   ├── TestHelper.sol
    │   │   ├── libraries/
    │   │   │   └── TestMath.sol
    │   │   ├── mocks/
    │   │   │   ├── ERC1271Mock.sol
    │   │   │   ├── ERC20.sol
    │   │   │   └── USDC.sol
    │   │   ├── script/
    │   │   │   ├── callTest.sol
    │   │   │   ├── ffi.sol
    │   │   │   ├── poolBytecodeHash.s.sol
    │   │   │   ├── useSolcVersion.s.sol
    │   │   │   └── ZeroTx.s.sol
    │   │   └── util/
    │   │       ├── Ascii.sol
    │   │       ├── Deployer.sol
    │   │       ├── Io.sol
    │   │       ├── Json.sol
    │   │       ├── Log.sol
    │   │       ├── Predictor.sol
    │   │       ├── Reader.sol
    │   │       ├── vm.sol
    │   │       └── script/
    │   │           └── prepareTempFolder.s.sol
    │   └── exchange/
    │       ├── BaseExchange.sol
    │       ├── CTFExchange.sol
    │       ├── interfaces/
    │       │   ├── IAssetOperations.sol
    │       │   ├── IAssets.sol
    │       │   ├── IAuth.sol
    │       │   ├── IConditionalTokens.sol
    │       │   ├── IFees.sol
    │       │   ├── IHashing.sol
    │       │   ├── INonceManager.sol
    │       │   ├── IPausable.sol
    │       │   ├── IRegistry.sol
    │       │   ├── ISignatures.sol
    │       │   └── ITrading.sol
    │       ├── libraries/
    │       │   ├── CalculatorHelper.sol
    │       │   ├── OrderStructs.sol
    │       │   ├── PolyProxyLib.sol
    │       │   ├── PolySafeLib.sol
    │       │   └── TransferHelper.sol
    │       ├── mixins/
    │       │   ├── AssetOperations.sol
    │       │   ├── Assets.sol
    │       │   ├── Auth.sol
    │       │   ├── Fees.sol
    │       │   ├── Hashing.sol
    │       │   ├── NonceManager.sol
    │       │   ├── Pausable.sol
    │       │   ├── PolyFactoryHelper.sol
    │       │   ├── Registry.sol
    │       │   ├── Signatures.sol
    │       │   └── Trading.sol
    │       ├── scripts/
    │       │   └── ExchangeDeployment.s.sol
    │       └── test/
    │           ├── BaseExchangeTest.sol
    │           ├── CTFExchange.t.sol
    │           ├── ERC1271Signature.t.sol
    │           ├── MatchOrders.t.sol
    │           └── libraries/
    │               └── CalculatorHelper.t.sol
    └── .github/
        └── workflows/
            └── Tests.yml


Files Content:

================================================
FILE: README.md
================================================
# Polymarket CTF Exchange

[![Version][version-badge]][version-link]
[![License][license-badge]][license-link]
[![Test][ci-badge]][ci-link]

[version-badge]: https://img.shields.io/github/v/release/polymarket/ctf-exchange.svg?label=version
[version-link]: https://github.com/Polymarket/ctf-exchange/releases
[license-badge]: https://img.shields.io/github/license/polymarket/ctf-exchange
[license-link]: https://github.com/Polymarket/ctf-exchange/blob/main/LICENSE.md
[ci-badge]: https://github.com/Polymarket/ctf-exchange/actions/workflows/Tests.yml/badge.svg
[ci-link]: https://github.com/Polymarket/ctf-exchange/actions/workflows/Tests.yml

## Background

The Polymarket CTF Exchange is an exchange protocol that facilitates atomic swaps between [Conditional Tokens Framework(CTF)](https://docs.gnosis.io/conditionaltokens/) ERC1155 assets and an ERC20 collateral asset.

It is intended to be used in a hybrid-decentralized exchange model wherein there is an operator that provides offchain matching services while settlement happens on-chain, non-custodially.


## Documentation

Docs for the CTF Exchange are available in this repo [here](./docs/Overview.md).

## Audit

These contracts have been audited by Chainsecurity and the report is available [here](./audit/ChainSecurity_Polymarket_Exchange_audit.pdf).


## Deployments

| Network          | Address                                                                           |
| ---------------- | --------------------------------------------------------------------------------- |
| Polygon          | [0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E](https://polygonscan.com/address/0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E)|
| Amoy           | [0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40](https://amoy.polygonscan.com/address/0xdfe02eb6733538f8ea35d585af8de5958ad99e40)|


## Development

Install [Foundry](https://github.com/foundry-rs/foundry/).

Foundry has daily updates, run `foundryup` to update `forge` and `cast`.

---

## Testing

To run all tests: `forge test`

To run test functions matching a regex pattern `forge test -m PATTERN`

To run tests in contracts matching a regex pattern `forge test --mc PATTERN`

Set `-vvv` to see a stack trace for a failed test.

---

To install new foundry submodules: `forge install UserName/RepoName@CommitHash`

To remove: `forge remove UserName/RepoName`




================================================
FILE: foundry.lock
================================================
{
  "lib/forge-std": {
    "rev": "d26946aeef956d9d11238ce02c94b7a22ac23ca8"
  },
  "lib/openzeppelin-contracts": {
    "rev": "8769b19860863ed14e82ac78eb0d09449a49290b"
  },
  "lib/solady": {
    "tag": {
      "name": "v0.1.26",
      "rev": "acd959aa4bd04720d640bf4e6a5c71037510cc4b"
    }
  },
  "lib/solmate": {
    "rev": "bff24e835192470ed38bf15dbed6084c2d723ace"
  }
}


================================================
FILE: foundry.toml
================================================
[profile.default]
solc = "0.8.15"
ffi = true
gas_reports = ["*"]
out = "out"
optimizer_runs = 1000000

# fuzz settings
[profile.default.fuzz]
runs = 256
[profile.intense.fuzz]
runs = 10_000

[fmt]
line_length = 120
tab_width = 4
bracket_spacing = true
wrap_comments = true
single_line_statement_blocks = "single"


================================================
FILE: LICENSE.md
================================================
# MIT License

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.



================================================
FILE: package.json
================================================
{
    "name": "polymarket-ctf-exchange",
    "scripts": {
        "test": "forge test"
    },
    "devDependencies": {
        "prettier": "^2.5.1",
        "prettier-plugin-solidity": "^1.0.0-beta.19"
    }
}



================================================
FILE: remappings.txt
================================================
openzeppelin/=lib/openzeppelin-contracts/contracts/
openzeppelin-contracts/=lib/openzeppelin-contracts/contracts/

solmate/=lib/solmate/src/
solady/=lib/solady/src/

common/=src/common/
creator/=src/creator/
dev/=src/dev/
exchange/=src/exchange/


================================================
FILE: .env.example
================================================
PK=
ADMIN=
RPC_URL=
COLLATERAL=
CTF=
PROXY_FACTORY=
SAFE_FACTORY=



================================================
FILE: .prettierrc
================================================
{
    "overrides": [
        {
            "files": "*.md",
            "options": {
                "tabWidth": 1
            }
        }
    ],
    "arrowParens": "avoid",
    "bracketSpacing": true,
    "endOfLine": "auto",
    "printWidth": 120,
    "singleQuote": false,
    "tabWidth": 4,
    "trailingComma": "all"
}



================================================
FILE: artifacts/CTFExchange.json
================================================
{
    "abi": [
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "_collateral",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "_ctf",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "_proxyFactory",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "_safeFactory",
            "type": "address"
          }
        ],
        "stateMutability": "nonpayable",
        "type": "constructor"
      },
      {
        "inputs": [],
        "name": "AlreadyRegistered",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "FeeTooHigh",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "InvalidComplement",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "InvalidNonce",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "InvalidSignature",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "InvalidTokenId",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "MakingGtRemaining",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "MismatchedTokenIds",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "NotAdmin",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "NotCrossing",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "NotOperator",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "NotOwner",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "NotTaker",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "OrderExpired",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "OrderFilledOrCancelled",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "Paused",
        "type": "error"
      },
      {
        "inputs": [],
        "name": "TooLittleTokensReceived",
        "type": "error"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "receiver",
            "type": "address"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "tokenId",
            "type": "uint256"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "amount",
            "type": "uint256"
          }
        ],
        "name": "FeeCharged",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "newAdminAddress",
            "type": "address"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "admin",
            "type": "address"
          }
        ],
        "name": "NewAdmin",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "newOperatorAddress",
            "type": "address"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "admin",
            "type": "address"
          }
        ],
        "name": "NewOperator",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "bytes32",
            "name": "orderHash",
            "type": "bytes32"
          }
        ],
        "name": "OrderCancelled",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "bytes32",
            "name": "orderHash",
            "type": "bytes32"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "maker",
            "type": "address"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "taker",
            "type": "address"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "makerAssetId",
            "type": "uint256"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "takerAssetId",
            "type": "uint256"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "makerAmountFilled",
            "type": "uint256"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "takerAmountFilled",
            "type": "uint256"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "fee",
            "type": "uint256"
          }
        ],
        "name": "OrderFilled",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "bytes32",
            "name": "takerOrderHash",
            "type": "bytes32"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "takerOrderMaker",
            "type": "address"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "makerAssetId",
            "type": "uint256"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "takerAssetId",
            "type": "uint256"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "makerAmountFilled",
            "type": "uint256"
          },
          {
            "indexed": false,
            "internalType": "uint256",
            "name": "takerAmountFilled",
            "type": "uint256"
          }
        ],
        "name": "OrdersMatched",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "oldProxyFactory",
            "type": "address"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "newProxyFactory",
            "type": "address"
          }
        ],
        "name": "ProxyFactoryUpdated",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "removedAdmin",
            "type": "address"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "admin",
            "type": "address"
          }
        ],
        "name": "RemovedAdmin",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "removedOperator",
            "type": "address"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "admin",
            "type": "address"
          }
        ],
        "name": "RemovedOperator",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "oldSafeFactory",
            "type": "address"
          },
          {
            "indexed": true,
            "internalType": "address",
            "name": "newSafeFactory",
            "type": "address"
          }
        ],
        "name": "SafeFactoryUpdated",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "uint256",
            "name": "token0",
            "type": "uint256"
          },
          {
            "indexed": true,
            "internalType": "uint256",
            "name": "token1",
            "type": "uint256"
          },
          {
            "indexed": true,
            "internalType": "bytes32",
            "name": "conditionId",
            "type": "bytes32"
          }
        ],
        "name": "TokenRegistered",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "pauser",
            "type": "address"
          }
        ],
        "name": "TradingPaused",
        "type": "event"
      },
      {
        "anonymous": false,
        "inputs": [
          {
            "indexed": true,
            "internalType": "address",
            "name": "pauser",
            "type": "address"
          }
        ],
        "name": "TradingUnpaused",
        "type": "event"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "admin_",
            "type": "address"
          }
        ],
        "name": "addAdmin",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "operator_",
            "type": "address"
          }
        ],
        "name": "addOperator",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "name": "admins",
        "outputs": [
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "address",
                "name": "maker",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "signer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "taker",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "makerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "takerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "expiration",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "nonce",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "feeRateBps",
                "type": "uint256"
              },
              {
                "internalType": "enum Side",
                "name": "side",
                "type": "uint8"
              },
              {
                "internalType": "enum SignatureType",
                "name": "signatureType",
                "type": "uint8"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "internalType": "struct Order",
            "name": "order",
            "type": "tuple"
          }
        ],
        "name": "cancelOrder",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "address",
                "name": "maker",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "signer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "taker",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "makerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "takerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "expiration",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "nonce",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "feeRateBps",
                "type": "uint256"
              },
              {
                "internalType": "enum Side",
                "name": "side",
                "type": "uint8"
              },
              {
                "internalType": "enum SignatureType",
                "name": "signatureType",
                "type": "uint8"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "internalType": "struct Order[]",
            "name": "orders",
            "type": "tuple[]"
          }
        ],
        "name": "cancelOrders",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "domainSeparator",
        "outputs": [
          {
            "internalType": "bytes32",
            "name": "",
            "type": "bytes32"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "address",
                "name": "maker",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "signer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "taker",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "makerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "takerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "expiration",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "nonce",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "feeRateBps",
                "type": "uint256"
              },
              {
                "internalType": "enum Side",
                "name": "side",
                "type": "uint8"
              },
              {
                "internalType": "enum SignatureType",
                "name": "signatureType",
                "type": "uint8"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "internalType": "struct Order",
            "name": "order",
            "type": "tuple"
          },
          {
            "internalType": "uint256",
            "name": "fillAmount",
            "type": "uint256"
          }
        ],
        "name": "fillOrder",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "address",
                "name": "maker",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "signer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "taker",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "makerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "takerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "expiration",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "nonce",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "feeRateBps",
                "type": "uint256"
              },
              {
                "internalType": "enum Side",
                "name": "side",
                "type": "uint8"
              },
              {
                "internalType": "enum SignatureType",
                "name": "signatureType",
                "type": "uint8"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "internalType": "struct Order[]",
            "name": "orders",
            "type": "tuple[]"
          },
          {
            "internalType": "uint256[]",
            "name": "fillAmounts",
            "type": "uint256[]"
          }
        ],
        "name": "fillOrders",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "getCollateral",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "uint256",
            "name": "token",
            "type": "uint256"
          }
        ],
        "name": "getComplement",
        "outputs": [
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "uint256",
            "name": "token",
            "type": "uint256"
          }
        ],
        "name": "getConditionId",
        "outputs": [
          {
            "internalType": "bytes32",
            "name": "",
            "type": "bytes32"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "getCtf",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "getMaxFeeRate",
        "outputs": [
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          }
        ],
        "stateMutability": "pure",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "bytes32",
            "name": "orderHash",
            "type": "bytes32"
          }
        ],
        "name": "getOrderStatus",
        "outputs": [
          {
            "components": [
              {
                "internalType": "bool",
                "name": "isFilledOrCancelled",
                "type": "bool"
              },
              {
                "internalType": "uint256",
                "name": "remaining",
                "type": "uint256"
              }
            ],
            "internalType": "struct OrderStatus",
            "name": "",
            "type": "tuple"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "getPolyProxyFactoryImplementation",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "_addr",
            "type": "address"
          }
        ],
        "name": "getPolyProxyWalletAddress",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "getProxyFactory",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "_addr",
            "type": "address"
          }
        ],
        "name": "getSafeAddress",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "getSafeFactory",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "getSafeFactoryImplementation",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "address",
                "name": "maker",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "signer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "taker",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "makerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "takerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "expiration",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "nonce",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "feeRateBps",
                "type": "uint256"
              },
              {
                "internalType": "enum Side",
                "name": "side",
                "type": "uint8"
              },
              {
                "internalType": "enum SignatureType",
                "name": "signatureType",
                "type": "uint8"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "internalType": "struct Order",
            "name": "order",
            "type": "tuple"
          }
        ],
        "name": "hashOrder",
        "outputs": [
          {
            "internalType": "bytes32",
            "name": "",
            "type": "bytes32"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "incrementNonce",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "usr",
            "type": "address"
          }
        ],
        "name": "isAdmin",
        "outputs": [
          {
            "internalType": "bool",
            "name": "",
            "type": "bool"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "usr",
            "type": "address"
          }
        ],
        "name": "isOperator",
        "outputs": [
          {
            "internalType": "bool",
            "name": "",
            "type": "bool"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "usr",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "nonce",
            "type": "uint256"
          }
        ],
        "name": "isValidNonce",
        "outputs": [
          {
            "internalType": "bool",
            "name": "",
            "type": "bool"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "address",
                "name": "maker",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "signer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "taker",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "makerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "takerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "expiration",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "nonce",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "feeRateBps",
                "type": "uint256"
              },
              {
                "internalType": "enum Side",
                "name": "side",
                "type": "uint8"
              },
              {
                "internalType": "enum SignatureType",
                "name": "signatureType",
                "type": "uint8"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "internalType": "struct Order",
            "name": "takerOrder",
            "type": "tuple"
          },
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "address",
                "name": "maker",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "signer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "taker",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "makerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "takerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "expiration",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "nonce",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "feeRateBps",
                "type": "uint256"
              },
              {
                "internalType": "enum Side",
                "name": "side",
                "type": "uint8"
              },
              {
                "internalType": "enum SignatureType",
                "name": "signatureType",
                "type": "uint8"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "internalType": "struct Order[]",
            "name": "makerOrders",
            "type": "tuple[]"
          },
          {
            "internalType": "uint256",
            "name": "takerFillAmount",
            "type": "uint256"
          },
          {
            "internalType": "uint256[]",
            "name": "makerFillAmounts",
            "type": "uint256[]"
          }
        ],
        "name": "matchOrders",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "name": "nonces",
        "outputs": [
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          },
          {
            "internalType": "uint256[]",
            "name": "",
            "type": "uint256[]"
          },
          {
            "internalType": "uint256[]",
            "name": "",
            "type": "uint256[]"
          },
          {
            "internalType": "bytes",
            "name": "",
            "type": "bytes"
          }
        ],
        "name": "onERC1155BatchReceived",
        "outputs": [
          {
            "internalType": "bytes4",
            "name": "",
            "type": "bytes4"
          }
        ],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          },
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          },
          {
            "internalType": "bytes",
            "name": "",
            "type": "bytes"
          }
        ],
        "name": "onERC1155Received",
        "outputs": [
          {
            "internalType": "bytes4",
            "name": "",
            "type": "bytes4"
          }
        ],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "name": "operators",
        "outputs": [
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "bytes32",
            "name": "",
            "type": "bytes32"
          }
        ],
        "name": "orderStatus",
        "outputs": [
          {
            "internalType": "bool",
            "name": "isFilledOrCancelled",
            "type": "bool"
          },
          {
            "internalType": "uint256",
            "name": "remaining",
            "type": "uint256"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "parentCollectionId",
        "outputs": [
          {
            "internalType": "bytes32",
            "name": "",
            "type": "bytes32"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "pauseTrading",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "paused",
        "outputs": [
          {
            "internalType": "bool",
            "name": "",
            "type": "bool"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "proxyFactory",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "uint256",
            "name": "token",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "complement",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "conditionId",
            "type": "bytes32"
          }
        ],
        "name": "registerToken",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "uint256",
            "name": "",
            "type": "uint256"
          }
        ],
        "name": "registry",
        "outputs": [
          {
            "internalType": "uint256",
            "name": "complement",
            "type": "uint256"
          },
          {
            "internalType": "bytes32",
            "name": "conditionId",
            "type": "bytes32"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "admin",
            "type": "address"
          }
        ],
        "name": "removeAdmin",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "operator",
            "type": "address"
          }
        ],
        "name": "removeOperator",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "renounceAdminRole",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "renounceOperatorRole",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "safeFactory",
        "outputs": [
          {
            "internalType": "address",
            "name": "",
            "type": "address"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "_newProxyFactory",
            "type": "address"
          }
        ],
        "name": "setProxyFactory",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "address",
            "name": "_newSafeFactory",
            "type": "address"
          }
        ],
        "name": "setSafeFactory",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "bytes4",
            "name": "interfaceId",
            "type": "bytes4"
          }
        ],
        "name": "supportsInterface",
        "outputs": [
          {
            "internalType": "bool",
            "name": "",
            "type": "bool"
          }
        ],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [],
        "name": "unpauseTrading",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "uint256",
            "name": "token",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "complement",
            "type": "uint256"
          }
        ],
        "name": "validateComplement",
        "outputs": [],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "address",
                "name": "maker",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "signer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "taker",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "makerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "takerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "expiration",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "nonce",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "feeRateBps",
                "type": "uint256"
              },
              {
                "internalType": "enum Side",
                "name": "side",
                "type": "uint8"
              },
              {
                "internalType": "enum SignatureType",
                "name": "signatureType",
                "type": "uint8"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "internalType": "struct Order",
            "name": "order",
            "type": "tuple"
          }
        ],
        "name": "validateOrder",
        "outputs": [],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "bytes32",
            "name": "orderHash",
            "type": "bytes32"
          },
          {
            "components": [
              {
                "internalType": "uint256",
                "name": "salt",
                "type": "uint256"
              },
              {
                "internalType": "address",
                "name": "maker",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "signer",
                "type": "address"
              },
              {
                "internalType": "address",
                "name": "taker",
                "type": "address"
              },
              {
                "internalType": "uint256",
                "name": "tokenId",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "makerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "takerAmount",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "expiration",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "nonce",
                "type": "uint256"
              },
              {
                "internalType": "uint256",
                "name": "feeRateBps",
                "type": "uint256"
              },
              {
                "internalType": "enum Side",
                "name": "side",
                "type": "uint8"
              },
              {
                "internalType": "enum SignatureType",
                "name": "signatureType",
                "type": "uint8"
              },
              {
                "internalType": "bytes",
                "name": "signature",
                "type": "bytes"
              }
            ],
            "internalType": "struct Order",
            "name": "order",
            "type": "tuple"
          }
        ],
        "name": "validateOrderSignature",
        "outputs": [],
        "stateMutability": "view",
        "type": "function"
      },
      {
        "inputs": [
          {
            "internalType": "uint256",
            "name": "tokenId",
            "type": "uint256"
          }
        ],
        "name": "validateTokenId",
        "outputs": [],
        "stateMutability": "view",
        "type": "function"
      }
    ]
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/137/deployExchange-1663954950.json
================================================
{
  "transactions": [
    {
      "hash": null,
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": null,
      "arguments": [
        "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
        "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "gas": "0x41072e",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa841740000000000000000000000004d97dcd97ec945f40cf65f87097ace5ea0476045000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/137/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db"
    }
  },
  "timestamp": 1663954950,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/137/deployExchange-1663955866.json
================================================
{
  "transactions": [
    {
      "hash": null,
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": null,
      "arguments": [
        "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
        "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "gas": "0x41072e",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa841740000000000000000000000004d97dcd97ec945f40cf65f87097ace5ea0476045000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/137/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f"
    }
  },
  "timestamp": 1663955866,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/137/deployExchange-1663958824.json
================================================
{
  "transactions": [
    {
      "hash": null,
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": null,
      "arguments": [
        "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
        "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "gas": "0x41072e",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa841740000000000000000000000004d97dcd97ec945f40cf65f87097ace5ea0476045000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/137/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f"
    }
  },
  "timestamp": 1663958824,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/137/deployExchange-1663958850.json
================================================
{
  "transactions": [
    {
      "hash": null,
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": null,
      "arguments": [
        "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
        "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "gas": "0x41072e",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa841740000000000000000000000004d97dcd97ec945f40cf65f87097ace5ea0476045000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/137/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f"
    }
  },
  "timestamp": 1663958850,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/137/deployExchange-1663958971.json
================================================
{
  "transactions": [
    {
      "hash": "0xf7f61cb1ce8e09f9652e85c6ef1196f7225a40221a473d89c117108101f31b8e",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": null,
      "arguments": [
        "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
        "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "gas": "0x41072e",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa841740000000000000000000000004d97dcd97ec945f40cf65f87097ace5ea0476045000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x9084668eccf2c9fbf05d49323b5fd6de7a2578a01a7c817de9ac34d128d081ba",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x022d727a4c8ecdaa9e0aee5e5ef1a6ade286ea48052db6a2d76d949f3122273d",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xa035304622733ecdf80c9226c65241b683d839ffb278a25bd3ca1cdc2ab24ecb",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x212ec4c121617c2cf0ce56ebca9bc28da53ebcb6f8c8cb6fb2e810fd8e7a4e26",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [
    "0xf7f61cb1ce8e09f9652e85c6ef1196f7225a40221a473d89c117108101f31b8e",
    "0x9084668eccf2c9fbf05d49323b5fd6de7a2578a01a7c817de9ac34d128d081ba",
    "0x022d727a4c8ecdaa9e0aee5e5ef1a6ade286ea48052db6a2d76d949f3122273d",
    "0xa035304622733ecdf80c9226c65241b683d839ffb278a25bd3ca1cdc2ab24ecb",
    "0x212ec4c121617c2cf0ce56ebca9bc28da53ebcb6f8c8cb6fb2e810fd8e7a4e26"
  ],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/137/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f"
    }
  },
  "timestamp": 1663958971,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/137/deployExchange-1663958977.json
================================================
{
  "transactions": [
    {
      "hash": "0xf7f61cb1ce8e09f9652e85c6ef1196f7225a40221a473d89c117108101f31b8e",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": null,
      "arguments": [
        "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
        "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "gas": "0x41072e",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa841740000000000000000000000004d97dcd97ec945f40cf65f87097ace5ea0476045000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x9084668eccf2c9fbf05d49323b5fd6de7a2578a01a7c817de9ac34d128d081ba",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x022d727a4c8ecdaa9e0aee5e5ef1a6ade286ea48052db6a2d76d949f3122273d",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xa035304622733ecdf80c9226c65241b683d839ffb278a25bd3ca1cdc2ab24ecb",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x212ec4c121617c2cf0ce56ebca9bc28da53ebcb6f8c8cb6fb2e810fd8e7a4e26",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [
    {
      "transactionHash": "0xf7f61cb1ce8e09f9652e85c6ef1196f7225a40221a473d89c117108101f31b8e",
      "transactionIndex": "0x0",
      "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
      "blockNumber": "0x1fed106",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": null,
      "cumulativeGasUsed": "0x320586",
      "gasUsed": "0x320586",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "logs": [
        {
          "address": "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
          "topics": [
            "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
            "0x000000000000000000000000fffd6f0db1ec30a58884b23546b4f1bb333f818f",
            "0x0000000000000000000000004d97dcd97ec945f40cf65f87097ace5ea0476045"
          ],
          "data": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0xf7f61cb1ce8e09f9652e85c6ef1196f7225a40221a473d89c117108101f31b8e",
          "transactionIndex": "0x0",
          "logIndex": "0x0",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000ef46d5fe753c988606e6f703260d816af53b03eb"
          ],
          "data": "0x00000000000000000000000000000000000000000000000009194fa71f0098320000000000000000000000000000000000000000000000004139c1192c5600000000000000000000000000000000000000000000000028b1c70d7fdb56bd10e1000000000000000000000000000000000000000000000000382071720d5567ce0000000000000000000000000000000000000000000028b1d026cf8275bda913",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0xf7f61cb1ce8e09f9652e85c6ef1196f7225a40221a473d89c117108101f31b8e",
          "transactionIndex": "0x0",
          "logIndex": "0x1",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000040000000000000000000000004000000000008000008000000000000000000000600000000000000000000000000000000200800000000000000000000100000000000000000000000000000000008000000000000000000000000180000000000000000001000000000000000001000000000000000000000000000000000000000000220000000400000000000000000001000000000000000000000000200000004000000000000000000041000000000000000000000000000000100000000000000010008000000000000000000000000002000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0x2e90edd000"
    },
    {
      "transactionHash": "0x9084668eccf2c9fbf05d49323b5fd6de7a2578a01a7c817de9ac34d128d081ba",
      "transactionIndex": "0x1",
      "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
      "blockNumber": "0x1fed106",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "cumulativeGasUsed": "0x32bfd8",
      "gasUsed": "0xba52",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
          "topics": [
            "0xf9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc",
            "0x000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4"
          ],
          "data": "0x",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0x9084668eccf2c9fbf05d49323b5fd6de7a2578a01a7c817de9ac34d128d081ba",
          "transactionIndex": "0x1",
          "logIndex": "0x2",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000ef46d5fe753c988606e6f703260d816af53b03eb"
          ],
          "data": "0x0000000000000000000000000000000000000000000000000021e437354329d6000000000000000000000000000000000000000000000000382071720acb20000000000000000000000000000000000000000000000028b1d026cf8275bda91300000000000000000000000000000000000000000000000037fe8d3ad587f62a0000000000000000000000000000000000000000000028b1d048b3b9ab00d2e9",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0x9084668eccf2c9fbf05d49323b5fd6de7a2578a01a7c817de9ac34d128d081ba",
          "transactionIndex": "0x1",
          "logIndex": "0x3",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000008000008000000000010000000000400000010000000000000000000000000000800000000000000000000100000000000000000080000000000000000000200000000000000000000080000000000000000000000000000000000000000001000000000000000000000000000000000000200000000000000000000000000000000000080000000000000000200000004000000000000028000041000000000000000000000000000000100000000000000000000000000000000000000000000102000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0x2e90edd000"
    },
    {
      "transactionHash": "0x022d727a4c8ecdaa9e0aee5e5ef1a6ade286ea48052db6a2d76d949f3122273d",
      "transactionIndex": "0x2",
      "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
      "blockNumber": "0x1fed106",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "cumulativeGasUsed": "0x337a79",
      "gasUsed": "0xbaa1",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
          "topics": [
            "0xf1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c",
            "0x000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4"
          ],
          "data": "0x",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0x022d727a4c8ecdaa9e0aee5e5ef1a6ade286ea48052db6a2d76d949f3122273d",
          "transactionIndex": "0x2",
          "logIndex": "0x4",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000ef46d5fe753c988606e6f703260d816af53b03eb"
          ],
          "data": "0x0000000000000000000000000000000000000000000000000021f295eea655d300000000000000000000000000000000000000000000000037fe8d3ad57e80000000000000000000000000000000000000000000000028b1d048b3b9ab00d2e900000000000000000000000000000000000000000000000037dc9aa4e6d82a2d0000000000000000000000000000000000000000000028b1d06aa64f99a728bc",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0x022d727a4c8ecdaa9e0aee5e5ef1a6ade286ea48052db6a2d76d949f3122273d",
          "transactionIndex": "0x2",
          "logIndex": "0x5",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000008000008000000000010000000000400000010000000000000000000000000000800000000000000000000100000000000000000080000000000000000000000000000000000000000080000000000000000000000000000000000000000001000000000000000000000000000000002000200000000000000000100000000000000000000000000000000000200000004000000000000028000041000000000000000000000000000000100000000000000000000000000000000000000000000002800000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0x2e90edd000"
    },
    {
      "transactionHash": "0xa035304622733ecdf80c9226c65241b683d839ffb278a25bd3ca1cdc2ab24ecb",
      "transactionIndex": "0x3",
      "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
      "blockNumber": "0x1fed106",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "cumulativeGasUsed": "0x33d525",
      "gasUsed": "0x5aac",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
          "topics": [
            "0x787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4"
          ],
          "data": "0x",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0xa035304622733ecdf80c9226c65241b683d839ffb278a25bd3ca1cdc2ab24ecb",
          "transactionIndex": "0x3",
          "logIndex": "0x6",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000ef46d5fe753c988606e6f703260d816af53b03eb"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000107e3cfae3254400000000000000000000000000000000000000000000000037dc9aa4e6ceb0000000000000000000000000000000000000000000000028b1d06aa64f99a728bc00000000000000000000000000000000000000000000000037cc1c67ebeb8abc0000000000000000000000000000000000000000000028b1d07b248c948a4e00",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0xa035304622733ecdf80c9226c65241b683d839ffb278a25bd3ca1cdc2ab24ecb",
          "transactionIndex": "0x3",
          "logIndex": "0x7",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000002000000000000000000000000040000000000000000000000000000000000008000008000000000010000000000400000000000000000000000000000000000800000000000000000000100000000000000020080000000000000000000000000000000000000002080000000000000000000000000000000000000000001000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000200000004000000000000000000041000000000000000000000000000000100000000000000000000000000000000000000000000002000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0x2e90edd000"
    },
    {
      "transactionHash": "0x212ec4c121617c2cf0ce56ebca9bc28da53ebcb6f8c8cb6fb2e810fd8e7a4e26",
      "transactionIndex": "0x4",
      "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
      "blockNumber": "0x1fed106",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "cumulativeGasUsed": "0x342ff7",
      "gasUsed": "0x5ad2",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
          "topics": [
            "0xf7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4"
          ],
          "data": "0x",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0x212ec4c121617c2cf0ce56ebca9bc28da53ebcb6f8c8cb6fb2e810fd8e7a4e26",
          "transactionIndex": "0x4",
          "logIndex": "0x8",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000ef46d5fe753c988606e6f703260d816af53b03eb"
          ],
          "data": "0x000000000000000000000000000000000000000000000000001085267e30035600000000000000000000000000000000000000000000000037cc1c67ebe6f0000000000000000000000000000000000000000000000028b1d07b248c948a4e0000000000000000000000000000000000000000000000000037bb97416db6ecaa0000000000000000000000000000000000000000000028b1d08ba9b312ba5156",
          "blockHash": "0x174f5a07ba31e49438fa23b48c2b1181fab0003a63ff6907eed7a4108f576614",
          "blockNumber": "0x1fed106",
          "transactionHash": "0x212ec4c121617c2cf0ce56ebca9bc28da53ebcb6f8c8cb6fb2e810fd8e7a4e26",
          "transactionIndex": "0x4",
          "logIndex": "0x9",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000008000008000000000010000000000400000000000000000000000000000000000800000000000000000000100000000000000000080000000000000100000000000000000000000000080000000000000000000000000000000000000000001000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000200000004004000000000000800041000000000000000000000000000000100000000000000000000000000000000000000000000002000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0x2e90edd000"
    }
  ],
  "libraries": [],
  "pending": [],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/137/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f"
    }
  },
  "timestamp": 1663958977,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/137/deployExchange-1664228337.json
================================================
{
  "transactions": [
    {
      "hash": "0x35423c49cb07c9ccecad9af20df52cccdeff0d46f833d438de8b02f2504aed22",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": null,
      "arguments": [
        "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174",
        "0x4D97DCd97eC945f40cF65F87097ACe5EA0476045",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "gas": "0x4d88f0",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b506040516200473f3803806200473f8339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e05161010051610120516101405161016051610180516143386200040760003960006108970152600081816104c801528181612698015281816129450152818161355201526136820152600081816105eb015281816125e3015281816128ed0152818161358e01526136be01526000612258015260006122a701526000612282015260006121db015260006122050152600061222f01526143386000f3fe608060405234801561001057600080fd5b50600436106103365760003560e01c806370480275116101b2578063d798eff6116100f9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610892578063fa950b48146108b9578063fbddd751146108cc578063fe729aaf146108df57600080fd5b8063e60f0c0514610834578063edef7d8e14610847578063f23a6e611461085a57600080fd5b8063e03ac3d0116100d3578063e03ac3d014610806578063e2eec4051461080e578063e50e4f971461082157600080fd5b8063d798eff6146107bd578063d7fb272f146107d0578063d82da838146107f357600080fd5b8063a287bdf11161015b578063b28c51c011610135578063b28c51c01461073b578063bc197c8114610759578063c10f1a751461079d57600080fd5b8063a287bdf114610702578063a6dfcf8614610715578063ac8a584a1461072857600080fd5b806383b8a5ae1161018c57806383b8a5ae146106d45780639870d7fe146106dc578063a10f3dce146106ef57600080fd5b8063704802751461068357806375d7370a146106965780637ecebe00146106b457600080fd5b8063429b62e5116102815780635893253c1161022a578063627cdcb911610204578063627cdcb91461061c578063654f0ce41461062457806368c7450f146106375780636d70f7ae1461064a57600080fd5b80635893253c146105ad5780635c1548fb146105e95780635c975abb1461060f57600080fd5b8063456068d21161025b578063456068d21461052f57806346423aa7146105375780634a2a11f5146105a557600080fd5b8063429b62e5146104f457806344bea37e146105145780634544f0551461051c57600080fd5b80631785f53c116102e357806334600901116102bd57806334600901146104b35780633b521d78146104c65780633d6d3598146104ec57600080fd5b80631785f53c1461042257806324d7806c146104355780632dff692d1461046f57600080fd5b80631031e36e116103145780631031e36e146103ca578063131e7e1c146103d457806313e7c9d8146103f457600080fd5b806301ffc9a71461033b5780630647ee201461036357806306b9d6911461039d575b600080fd5b61034e6103493660046136e2565b6108f2565b60405190151581526020015b60405180910390f35b61034e610371366004613756565b73ffffffffffffffffffffffffffffffffffffffff919091166000908152600460205260409020541490565b6103a561098b565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200161035a565b6103d2610a24565b005b6007546103a59073ffffffffffffffffffffffffffffffffffffffff1681565b610414610402366004613782565b60026020526000908152604090205481565b60405190815260200161035a565b6103d2610430366004613782565b610a78565b61034e610443366004613782565b73ffffffffffffffffffffffffffffffffffffffff166000908152600160208190526040909120541490565b61049c61047d36600461379f565b6008602052600090815260409020805460019091015460ff9091169082565b60408051921515835260208301919091520161035a565b6103d26104c136600461379f565b610b15565b7f00000000000000000000000000000000000000000000000000000000000000006103a5565b6103d2610b5f565b610414610502366004613782565b60016020526000908152604090205481565b610414600081565b6103d261052a366004613782565b610be3565b6103d2610c36565b61058861054536600461379f565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b60408051825115158152602092830151928101929092520161035a565b6103e8610414565b6105d46105bb36600461379f565b6005602052600090815260409020805460019091015482565b6040805192835260208301919091520161035a565b7f00000000000000000000000000000000000000000000000000000000000000006103a5565b60035461034e9060ff1681565b6103d2610c88565b6103d26106323660046139f8565b610c92565b6103d2610645366004613a2d565b610cad565b61034e610658366004613782565b73ffffffffffffffffffffffffffffffffffffffff1660009081526002602052604090205460011490565b6103d2610691366004613782565b610d07565b60075473ffffffffffffffffffffffffffffffffffffffff166103a5565b6104146106c2366004613782565b60046020526000908152604090205481565b6103d2610da7565b6103d26106ea366004613782565b610e2c565b6104146106fd36600461379f565b610eca565b6103a5610710366004613782565b610ee8565b6103d26107233660046139f8565b610f14565b6103d2610736366004613782565b610f1d565b60065473ffffffffffffffffffffffffffffffffffffffff166103a5565b61076c610767366004613ae8565b610fba565b6040517fffffffff00000000000000000000000000000000000000000000000000000000909116815260200161035a565b6006546103a59073ffffffffffffffffffffffffffffffffffffffff1681565b6103d26107cb366004613c16565b610fe5565b6104146107de36600461379f565b60009081526005602052604090206001015490565b6103d2610801366004613c7a565b6110f5565b6103a5611136565b6103d261081c366004613c9c565b6111a6565b61041461082f3660046139f8565b6111fb565b6103d2610842366004613cd9565b611298565b6103a5610855366004613782565b6113a6565b61076c610868366004613d6b565b7ff23a6e610000000000000000000000000000000000000000000000000000000095945050505050565b6104147f000000000000000000000000000000000000000000000000000000000000000081565b6103d26108c7366004613dd4565b6113d2565b6103d26108da366004613782565b611409565b6103d26108ed366004613e09565b61145c565b60007fffffffff0000000000000000000000000000000000000000000000000000000082167f4e2312e000000000000000000000000000000000000000000000000000000000148061098557507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b600654604080517faaf10f42000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163aaf10f429160048083019260209291908290030181865afa1580156109fb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a1f9190613e4e565b905090565b3360009081526001602081905260409091205414610a6e576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a7661155e565b565b3360009081526001602081905260409091205414610ac2576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610b5c576040517f3f6cc76800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50565b33600090815260026020526040902054600114610ba8576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610c2d576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610b5c816115b6565b3360009081526001602081905260409091205414610c80576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a76611644565b610a766001611699565b6000610c9d826111fb565b9050610ca981836116c7565b5050565b3360009081526001602081905260409091205414610cf7576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d02838383611826565b505050565b3360009081526001602081905260409091205414610d51576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610df1576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610e76576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610ed582610b15565b5060009081526005602052604090205490565b600061098582610ef6611136565b60075473ffffffffffffffffffffffffffffffffffffffff16611982565b610b5c81611a80565b3360009081526001602081905260409091205414610f67576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b7fbc197c81000000000000000000000000000000000000000000000000000000005b95945050505050565b600054600203611056576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e43590000000000000000000000000000000000000000000060448201526064015b60405180910390fd5b6002600081815533815260209190915260409020546001146110a4576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff16156110e1576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110ec828233611b85565b50506001600055565b806110ff83610eca565b14610ca9576040517f66f8620a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600754604080517fa619486e000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163a619486e9160048083019260209291908290030181865afa1580156109fb573d6000803e3d6000fd5b6111c58160400151826020015184846101800151856101600151611bde565b610ca9576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006109857fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e610160015160405160200161127d9d9c9b9a99989796959493929190613eae565b60405160208183030381529060405280519060200120611c3c565b600054600203611304576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e435900000000000000000000000000000000000000000000604482015260640161104d565b600260008181553381526020919091526040902054600114611352576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff161561138f576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61139b84848484611ca5565b505060016000555050565b6000610985826113b461098b565b60065473ffffffffffffffffffffffffffffffffffffffff16611e5c565b805160005b81811015610d02576114018382815181106113f4576113f4613f4c565b6020026020010151611a80565b6001016113d7565b3360009081526001602081905260409091205414611453576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610b5c81611ebe565b6000546002036114c8576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e435900000000000000000000000000000000000000000000604482015260640161104d565b600260008181553381526020919091526040902054600114611516576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff1615611553576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110ec828233611f4c565b600380547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b60075460405173ffffffffffffffffffffffffffffffffffffffff8084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b600380547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b336000908152600460205260409020546116b4908290613faa565b3360009081526004602052604090205550565b60008160e001511180156116de5750428160e00151105b15611715576040517fc56873ba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61171f82826111a6565b6103e88161012001511115611760576040517fcd4e616700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61176d8160800151610b15565b60008281526008602052604090205460ff16156117b6576040517f7b38b76e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117f0816020015182610100015173ffffffffffffffffffffffffffffffffffffffff919091166000908152600460205260409020541490565b610ca9576040517f756688fe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8183148061183a575082158061183a575081155b15611871576040517f3f6cc76800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008381526005602052604090205415158061189a575060008281526005602052604090205415155b156118d1576040517f3a81d6fc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b60008061198e8461205a565b8051906020012090506000856040516020016119c6919073ffffffffffffffffffffffffffffffffffffffff91909116815260200190565b604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815282825280516020918201207fff000000000000000000000000000000000000000000000000000000000000008285015260609790971b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000166021840152603583019690965260558083019490945280518083039094018452607590910190525080519201919091209392505050565b602081015173ffffffffffffffffffffffffffffffffffffffff163314611ad3576040517f30cd747100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611ade826111fb565b600081815260086020526040902080549192509060ff1615611b2c576040517f7b38b76e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b81811015611bd757611bcf858281518110611ba757611ba7613f4c565b6020026020010151858381518110611bc157611bc1613f4c565b602002602001015185611f4c565b600101611b8a565b5050505050565b600080826002811115611bf357611bf3613e6b565b03611c0b57611c04868686866120eb565b9050610fdc565b6002826002811115611c1f57611c1f613e6b565b03611c3057611c0486868686612139565b611c048686868661218d565b6000610985611c496121c1565b836040517f19010000000000000000000000000000000000000000000000000000000000006020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080611cb387846122f5565b91509150600080611cc389612342565b91509150611cd78960200151308488612379565b611ce28989886123a3565b611cec84826123f5565b6101208a0151909450600090611d2e90828c61014001516001811115611d1457611d14613e6b565b14611d1f5787611d21565b865b88888e610140015161243d565b9050611d4b308b60200151848489611d469190613fc2565b612379565b611d573033848461252d565b6000611d6284612596565b90508015611d7a57611d7a308c602001518684612379565b60208b8101516040805187815292830186905282018990526060820188905260808201849052309173ffffffffffffffffffffffffffffffffffffffff9091169087907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c01516040805187815292830186905282018990526060820188905273ffffffffffffffffffffffffffffffffffffffff169086907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a35050505050505050505050565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606085901b166020820152600090611eb49083908590603401604051602081830303815290604052805190602001206126c6565b90505b9392505050565b60065460405173ffffffffffffffffffffffffffffffffffffffff8084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b81600080611f5a86846122f5565b6101208801519193509150600090611fa790825b8961014001516001811115611f8557611f85613e6b565b14611f905785611f92565b845b8960a001518a60c001518b610140015161243d565b9050600080611fb589612342565b91509150611fcf338a60200151838689611d469190613fc2565b611fdf8960200151888489612379565b6020898101516040805185815292830184905282018890526060820187905260808201859052339173ffffffffffffffffffffffffffffffffffffffff9091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a4505050505050505050565b6060604051806101a00160405280610171815260200161419261017191396040805173ffffffffffffffffffffffffffffffffffffffff8516602082015201604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526120d59291602001614005565b6040516020818303038152906040529050919050565b60008373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff1614801561212e575061212e858484612763565b90505b949350505050565b6000612146858484612763565b801561212e57508373ffffffffffffffffffffffffffffffffffffffff1661216d86610ee8565b73ffffffffffffffffffffffffffffffffffffffff161495945050505050565b600061219a858484612763565b801561212e57508373ffffffffffffffffffffffffffffffffffffffff1661216d866113a6565b60003073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614801561222757507f000000000000000000000000000000000000000000000000000000000000000046145b1561225157507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b60008061230584606001516127a5565b61230e846111fb565b905061231a81856116c7565b61232d838560a001518660c00151612817565b915061233a81858561283e565b509250929050565b60008080836101400151600181111561235d5761235d613e6b565b0361236d57505060800151600091565b50506080015190600090565b816000036123915761238c8484836128eb565b61239d565b61239d84848484612940565b50505050565b815160005b81811015611bd7576123ed858583815181106123c6576123c6613f4c565b60200260200101518584815181106123e0576123e0613f4c565b602002602001015161296d565b6001016123a8565b60008061240183612596565b905083811015611eb7576040517fdf4d808000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008515610fdc576000612452858585612a52565b905060008111801561246c5750670de0b6b3a76400008111155b1561252357600083600181111561248557612485613e6b565b036124d75761249661271082614034565b866124b2836124ad81670de0b6b3a7640000613fc2565b612ac1565b6124bc908a614034565b6124c69190614034565b6124d09190614071565b9150612523565b6124eb670de0b6b3a7640000612710614034565b86612502836124ad81670de0b6b3a7640000613fc2565b61250c908a614034565b6125169190614034565b6125209190614071565b91505b5095945050505050565b801561239d5761253f84848484612379565b604080518381526020810183905273ffffffffffffffffffffffffffffffffffffffff8516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b60008160000361264f576040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa15801561262b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061098591906140ac565b6040517efdd58e0000000000000000000000000000000000000000000000000000000081523060048201526024810183905273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e9060440161260e565b6000806126d38585612ad7565b8051602091820120604080517fff000000000000000000000000000000000000000000000000000000000000008185015260609890981b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000166021890152603588019590955260558088019190915284518088039091018152607590960190935250508251920191909120919050565b60008373ffffffffffffffffffffffffffffffffffffffff166127868484612c5a565b73ffffffffffffffffffffffffffffffffffffffff1614949350505050565b73ffffffffffffffffffffffffffffffffffffffff8116158015906127e0575073ffffffffffffffffffffffffffffffffffffffff81163314155b15610b5c576040517f5211a07900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008260000361282957506000611eb7565b826128348386614034565b611eb49190614071565b6000838152600860205260409020600181015490811561285e5781612864565b8360a001515b9150818311156128a0576040517fe2cc6ad600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6128aa8383613fc2565b9150816000036128de5780547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011781555b6001018190559392505050565b7f00000000000000000000000000000000000000000000000000000000000000003073ffffffffffffffffffffffffffffffffffffffff8516036129345761238c818484612c7e565b61239d81858585612c89565b61239d7f000000000000000000000000000000000000000000000000000000000000000085858585612c95565b60006129798484612d41565b9050612986848483612ddd565b8160008061299486846122f5565b61012088015191935091506000906129ac9082611f6e565b90506000806129ba89612342565b915091506129d186868b6020015185858c89612e89565b6020808b01518a820151604080518681529384018590528301899052606083018890526080830186905273ffffffffffffffffffffffffffffffffffffffff9182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b600080826001811115612a6757612a67613e6b565b03612a9f5782600003612a7b576000612a98565b82612a8e670de0b6b3a764000086614034565b612a989190614071565b9050611eb7565b83600003612aae576000611eb4565b83612834670de0b6b3a764000085614034565b6000818310612ad05781611eb7565b5090919050565b6040805160008082526020820190925260609190612af890604481016140c5565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152918152602080830180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f52e831dd000000000000000000000000000000000000000000000000000000001790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe60208201526c010000000000000000000000008502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be60418201526c01000000000000000000000000840260608201527f5af43d82803e903d91602b57fd5bf3000000000000000000000000000000000060748201528082604051602001612c41929190614005565b6040516020818303038152906040529250505092915050565b6000806000612c698585612f09565b91509150612c7681612f4e565b509392505050565b610d02838383613101565b61239d848484846131ba565b6040517ff242432a00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b158015612d2257600080fd5b505af1158015612d36573d6000803e3d6000fd5b505050505050505050565b6000808361014001516001811115612d5b57612d5b613e6b565b148015612d7e575060008261014001516001811115612d7c57612d7c613e6b565b145b15612d8b57506001610985565b60018361014001516001811115612da457612da4613e6b565b148015612dc7575060018261014001516001811115612dc557612dc5613e6b565b145b15612dd457506002610985565b50600092915050565b612de78383613279565b612e1d576040517f7f9a6f4600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000816002811115612e3157612e31613e6b565b03612e77578160800151836080015114610d02576040517fa0b9446500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d02836080015183608001516110f5565b612e958530868a612379565b612ea287878686866132c3565b85612eac84612596565b1015612ee4576040517fdf4d808000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612ef4308685611d46858b613fc2565b612f003033858461252d565b50505050505050565b6000808251604103612f3f5760208301516040840151606085015160001a612f338782858561334b565b94509450505050612f47565b506000905060025b9250929050565b6000816004811115612f6257612f62613e6b565b03612f6a5750565b6001816004811115612f7e57612f7e613e6b565b03612fe5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f45434453413a20696e76616c6964207369676e61747572650000000000000000604482015260640161104d565b6002816004811115612ff957612ff9613e6b565b03613060576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e67746800604482015260640161104d565b600381600481111561307457613074613e6b565b03610b5c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c60448201527f7565000000000000000000000000000000000000000000000000000000000000606482015260840161104d565b60006040517fa9059cbb000000000000000000000000000000000000000000000000000000008152836004820152826024820152602060006044836000895af13d15601f3d116001600051141617169150508061239d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600f60248201527f5452414e534645525f4641494c45440000000000000000000000000000000000604482015260640161104d565b60006040517f23b872dd0000000000000000000000000000000000000000000000000000000081528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080611bd7576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f5452414e534645525f46524f4d5f4641494c4544000000000000000000000000604482015260640161104d565b60008260c0015160001480613290575060c0820151155b1561329d57506001610985565b611eb76132a98461343a565b6132b28461343a565b856101400151856101400151613454565b60008160028111156132d7576132d7613e6b565b14611bd75760018160028111156132f0576132f0613e6b565b036133165760008281526005602052604090206001015461331190856134ee565b611bd7565b600281600281111561332a5761332a613e6b565b03611bd757600083815260056020526040902060010154613311908661361e565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156133825750600090506003613431565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156133d6573d6000803e3d6000fd5b50506040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0015191505073ffffffffffffffffffffffffffffffffffffffff811661342a57600060019250925050613431565b9150600090505b94509492505050565b60006109858260a001518360c00151846101400151612a52565b60008083600181111561346957613469613e6b565b036134ad57600082600181111561348257613482613e6b565b036134a357670de0b6b3a764000061349a8587613faa565b10159050612131565b5082841015612131565b60008260018111156134c1576134c1613e6b565b036134d0575083831015612131565b670de0b6b3a76400006134e38587613faa565b111595945050505050565b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061352457613524613f4c565b60200260200101818152505060028160018151811061354557613545613f4c565b60209081029190910101527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526135f09190600090889087908990600401614116565b600060405180830381600087803b15801561360a57600080fd5b505af1158015612f00573d6000803e3d6000fd5b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061365457613654613f4c565b60200260200101818152505060028160018151811061367557613675613f4c565b60209081029190910101527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16639e7212ad7f00000000000000000000000000000000000000000000000000000000000000006135ae565b6000602082840312156136f457600080fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611eb757600080fd5b73ffffffffffffffffffffffffffffffffffffffff81168114610b5c57600080fd5b803561375181613724565b919050565b6000806040838503121561376957600080fd5b823561377481613724565b946020939093013593505050565b60006020828403121561379457600080fd5b8135611eb781613724565b6000602082840312156137b157600080fd5b5035919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040516101a0810167ffffffffffffffff8111828210171561380b5761380b6137b8565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715613858576138586137b8565b604052919050565b80356002811061375157600080fd5b80356003811061375157600080fd5b600082601f83011261388f57600080fd5b813567ffffffffffffffff8111156138a9576138a96137b8565b6138da60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601613811565b8181528460208386010111156138ef57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a0828403121561391f57600080fd5b6139276137e7565b90508135815261393960208301613746565b602082015261394a60408301613746565b604082015261395b60608301613746565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e08201526101008083013581830152506101208083013581830152506101406139ae818401613860565b908201526101606139c083820161386f565b908201526101808281013567ffffffffffffffff8111156139e057600080fd5b6139ec8582860161387e565b82840152505092915050565b600060208284031215613a0a57600080fd5b813567ffffffffffffffff811115613a2157600080fd5b6121318482850161390c565b600080600060608486031215613a4257600080fd5b505081359360208301359350604090920135919050565b600067ffffffffffffffff821115613a7357613a736137b8565b5060051b60200190565b600082601f830112613a8e57600080fd5b81356020613aa3613a9e83613a59565b613811565b82815260059290921b84018101918181019086841115613ac257600080fd5b8286015b84811015613add5780358352918301918301613ac6565b509695505050505050565b600080600080600060a08688031215613b0057600080fd5b8535613b0b81613724565b94506020860135613b1b81613724565b9350604086013567ffffffffffffffff80821115613b3857600080fd5b613b4489838a01613a7d565b94506060880135915080821115613b5a57600080fd5b613b6689838a01613a7d565b93506080880135915080821115613b7c57600080fd5b50613b898882890161387e565b9150509295509295909350565b600082601f830112613ba757600080fd5b81356020613bb7613a9e83613a59565b82815260059290921b84018101918181019086841115613bd657600080fd5b8286015b84811015613add57803567ffffffffffffffff811115613bfa5760008081fd5b613c088986838b010161390c565b845250918301918301613bda565b60008060408385031215613c2957600080fd5b823567ffffffffffffffff80821115613c4157600080fd5b613c4d86838701613b96565b93506020850135915080821115613c6357600080fd5b50613c7085828601613a7d565b9150509250929050565b60008060408385031215613c8d57600080fd5b50508035926020909101359150565b60008060408385031215613caf57600080fd5b82359150602083013567ffffffffffffffff811115613ccd57600080fd5b613c708582860161390c565b60008060008060808587031215613cef57600080fd5b843567ffffffffffffffff80821115613d0757600080fd5b613d138883890161390c565b95506020870135915080821115613d2957600080fd5b613d3588838901613b96565b9450604087013593506060870135915080821115613d5257600080fd5b50613d5f87828801613a7d565b91505092959194509250565b600080600080600060a08688031215613d8357600080fd5b8535613d8e81613724565b94506020860135613d9e81613724565b93506040860135925060608601359150608086013567ffffffffffffffff811115613dc857600080fd5b613b898882890161387e565b600060208284031215613de657600080fd5b813567ffffffffffffffff811115613dfd57600080fd5b61213184828501613b96565b60008060408385031215613e1c57600080fd5b823567ffffffffffffffff811115613e3357600080fd5b613e3f8582860161390c565b95602094909401359450505050565b600060208284031215613e6057600080fd5b8151611eb781613724565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b60038110613eaa57613eaa613e6b565b9052565b60006101a0820190508e82528d602083015273ffffffffffffffffffffffffffffffffffffffff808e166040840152808d166060840152808c166080840152508960a08301528860c08301528760e083015286610100830152856101208301528461014083015260028410613f2557613f25613e6b565b83610160830152613f3a610180830184613e9a565b9e9d5050505050505050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60008219821115613fbd57613fbd613f7b565b500190565b600082821015613fd457613fd4613f7b565b500390565b60005b83811015613ff4578181015183820152602001613fdc565b8381111561239d5750506000910152565b60008351614017818460208801613fd9565b83519083019061402b818360208801613fd9565b01949350505050565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff048311821515161561406c5761406c613f7b565b500290565b6000826140a7577f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b500490565b6000602082840312156140be57600080fd5b5051919050565b60208152600082518060208401526140e4816040850160208701613fd9565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169190910160400192915050565b600060a0820173ffffffffffffffffffffffffffffffffffffffff881683526020878185015286604085015260a0606085015281865180845260c086019150828801935060005b818110156141795784518352938301939183019160010161415d565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a264697066735822122056df26e165b5957191bd0ff149c07ae13f5a6b4252973fb3c07a4653cce0f3b164736f6c634300080f00330000000000000000000000002791bca1f2de4661ed88a30c99a7a9449aa841740000000000000000000000004d97dcd97ec945f40cf65f87097ace5ea0476045000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x8dedda12c2ee8a8893c436e726bb0112d542455e3db12bddb9a5cd097d8a6d16",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x1107c",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x2f62b1db98a1173317d27d8ea06fc8c657b456b4c7c16b0c4955a06d70d9ee3a",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x10169",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xde6e10751f2a3679109c25a571f210858da0812d6635e2702f61da15d5c5a71a",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x7d00",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x0e0391adbe52ad44a30ee682c4cf270896190b02931b5c421ddc0aafbec0590a",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x7d34",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [
    "0x35423c49cb07c9ccecad9af20df52cccdeff0d46f833d438de8b02f2504aed22",
    "0x8dedda12c2ee8a8893c436e726bb0112d542455e3db12bddb9a5cd097d8a6d16",
    "0x2f62b1db98a1173317d27d8ea06fc8c657b456b4c7c16b0c4955a06d70d9ee3a",
    "0xde6e10751f2a3679109c25a571f210858da0812d6635e2702f61da15d5c5a71a",
    "0x0e0391adbe52ad44a30ee682c4cf270896190b02931b5c421ddc0aafbec0590a"
  ],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/137/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E"
    }
  },
  "timestamp": 1664228337,
  "commit": "af3ba7f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/80001/deployExchange-1663792323.json
================================================
{
  "transactions": [
    {
      "hash": "0xee31daedd823a5bc2134ddf35fa4fb17704349e4892837aa8913e737dd1c0dfb",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": null,
      "arguments": [
        "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
        "0x7D8610E9567d2a6C9FBf66a5A13E9Ba8bb120d43",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "gas": "0x4088ed",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b2938038062003b298339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161372262000407600039600061079e01526000818161043401528181611e670152818161203b01528181612a520152612b5d01526000818161055701528181611dd801528181611ff001528181612a810152612b8c01526000611a9601526000611ae501526000611ac001526000611a1901526000611a4301526000611a6d01526137226000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bb0565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612bff565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c2b565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c2b565b6108de565b6102ee6103bc366004612c2b565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c48565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c48565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c2b565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c2b565b6109f1565b610358610a2b565b6104f46104b1366004612c48565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c48565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e48565b610a6e565b6103586105b1366004612e7c565b610a89565b6102ee6105c4366004612c2b565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c2b565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c2b565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c2b565b610bb0565b61038d61064f366004612c48565b610c28565b610338610662366004612c2b565b610c46565b610358610675366004612e48565b610c65565b610358610688366004612c2b565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f36565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb366004613062565b610cf7565b61038d6106fe366004612c48565b60009081526005602052604090206001015490565b6103586107213660046130c5565b610d8f565b610338610db7565b61035861073c3660046130e7565b610e01565b61038d61074f366004612e48565b610e3d565b610358610762366004613123565b610eda565b610338610775366004612c2b565b610f6c565b6106b16107883660046131b4565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce36600461321c565b610f8b565b6103586107e1366004612c2b565b610fc2565b6103586107f4366004613250565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e9190613294565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132b1565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a999897969594939291906132ff565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132b1565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611724565b805160005b81811015610ac557610fba838281518110610fad57610fad613391565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611773565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132b1565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117cf565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133bd565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a1846118d2565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef848383611938565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c3613391565b60200260200101518583815181106114dd576114dd613391565b6020026020010151856117cf565b6001016114a6565b5050505050565b60008082600281111561150f5761150f6132d5565b036115275761152086868686611977565b9050610cee565b600282600281111561153b5761153b6132d5565b0361154c57611520868686866119ab565b611520868686866119e5565b600061082a611565611a0c565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b33565b9250925092506000806115c78a611b83565b915091506115db8a60200151308489611bba565b6115e68a8a89611be4565b6115f08582611c36565b6101208b015190955060009061163290828d61014001516001811115611618576116186132d5565b146116235788611625565b875b89898f6101400151611c65565b905061164f308c6020015184848a61164a91906133d5565b611bba565b61165b30338484611d55565b60408051848152602081018490529081018890526060810186905260808101829052309085907f6cda7c3afcd28346af42a5c662af2fbf6678f0af621dabb4b6fa9ee1c3b3c2e99060a00160405180910390a38183857fe914d2271d0909cb9f124ce60596eaa1e20ffc58a6a906ad7d5f9d096cc77fa28a8a6040516116eb929190918252602082015260400190565b60405180910390a460006116fe84611db1565b9050801561171657611716308d602001518684611bba565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b166020820152600090611769908390859060340160405160208183030381529060405280519060200120611e95565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806117de8785611b33565b925092509250600061183b88610120015160006001811115611802576118026132d5565b8a61014001516001811115611819576118196132d5565b146118245786611826565b855b8a60a001518b60c001518c6101400151611c65565b90506000806118498a611b83565b91509150611863338b6020015183868a61164a91906133d5565b6118738a6020015189848a611bba565b60408051838152602081018390529081018890526060810186905260808101849052339085907f6cda7c3afcd28346af42a5c662af2fbf6678f0af621dabb4b6fa9ee1c3b3c2e99060a00160405180910390a350505050505050505050565b6060604051806101a00160405280610171815260200161357c6101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119229291602001613418565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119569493929190613447565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119a057506119a0858484611eea565b90505b949350505050565b60006119b8858484611eea565b80156119a05750836001600160a01b03166119d286610c46565b6001600160a01b03161495945050505050565b60006119f2858484611eea565b80156119a05750836001600160a01b03166119d286610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a6557507f000000000000000000000000000000000000000000000000000000000000000046145b15611a8f57507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b458560600151611f12565b611b4e85610e3d565b9050611b5a818661117d565b611b6d848660a001518760c00151611f51565b9250611b7a818686611f78565b91509250925092565b600080808361014001516001811115611b9e57611b9e6132d5565b03611bae57505060800151600091565b50506080015190600090565b81600003611bd257611bcd848483611fee565b611bde565b611bde84848484612036565b50505050565b815160005b818110156114f357611c2e85858381518110611c0757611c07613391565b6020026020010151858481518110611c2157611c21613391565b6020026020010151612063565b600101611be9565b600080611c4283611db1565b90508381101561176c576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611c7a858585612140565b9050600081118015611c945750670de0b6b3a76400008111155b15611d4b576000836001811115611cad57611cad6132d5565b03611cff57611cbe61271082613480565b86611cda83611cd581670de0b6b3a76400006133d5565b6121af565b611ce4908a613480565b611cee9190613480565b611cf8919061349f565b9150611d4b565b611d13670de0b6b3a7640000612710613480565b86611d2a83611cd581670de0b6b3a76400006133d5565b611d34908a613480565b611d3e9190613480565b611d48919061349f565b91505b5095945050505050565b8015611bde57611d6784848484611bba565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e44576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e20573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134c1565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e03565b600080611ea285856121c5565b805190602001209050600060ff60f81b868584604051602001611ec89493929190613447565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f0084846122dc565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f3357506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f635750600061176c565b82611f6e8386613480565b611769919061349f565b60008381526008602052604090206001810154908115611f985781611f9e565b8360a001515b915081831115611fc157604051637166356b60e11b815260040160405180910390fd5b611fcb83836133d5565b915081600003611fe157805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361202a57611bcd818484612300565b611bde8185858561230b565b611bde7f000000000000000000000000000000000000000000000000000000000000000085858585612317565b600061206f848461239d565b905061207c848483612439565b816000808061208b8785611b33565b92509250925060006120af88610120015160006001811115611802576118026132d5565b90506000806120bd8a611b83565b915091506120d487878c6020015185858d896124b3565b6020808c015160408051858152928301849052820189905260608201879052608082018590526001600160a01b03169085907f6cda7c3afcd28346af42a5c662af2fbf6678f0af621dabb4b6fa9ee1c3b3c2e99060a00160405180910390a35050505050505050505050565b600080826001811115612155576121556132d5565b0361218d5782600003612169576000612186565b8261217c670de0b6b3a764000086613480565b612186919061349f565b905061176c565b8360000361219c576000611769565b83611f6e670de0b6b3a764000085613480565b60008183106121be578161176c565b5090919050565b60408051600080825260208201909252606091906121e690604481016134da565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122c3929190613418565b6040516020818303038152906040529250505092915050565b60008060006122eb858561251a565b915091506122f88161255f565b509392505050565b610ac58383836126a9565b611bde84848484612721565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b15801561237e57600080fd5b505af1158015612392573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123b7576123b76132d5565b1480156123da5750600082610140015160018111156123d8576123d86132d5565b145b156123e75750600161082a565b60018361014001516001811115612400576124006132d5565b148015612423575060018261014001516001811115612421576124216132d5565b145b156124305750600261082a565b50600092915050565b61244383836127a4565b61246057604051633fcd37a360e11b815260040160405180910390fd5b6000816002811115612474576124746132d5565b036124a1578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124bf8530868a611bba565b6124cc87878686866127ee565b856124d684611db1565b10156124f5576040516301be9b0160e71b815260040160405180910390fd5b61250530868561164a858b6133d5565b61251130338584611d55565b50505050505050565b60008082516041036125505760208301516040840151606085015160001a61254487828585612876565b94509450505050612558565b506000905060025b9250929050565b6000816004811115612573576125736132d5565b0361257b5750565b600181600481111561258f5761258f6132d5565b036125dc5760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b60028160048111156125f0576125f06132d5565b0361263d5760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b6003816004811115612651576126516132d5565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611bde5760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127bb575060c0820151155b156127c85750600161082a565b61176c6127d48461293a565b6127dd8461293a565b856101400151856101400151612954565b6000816002811115612802576128026132d5565b146114f357600181600281111561281b5761281b6132d5565b036128415760008281526005602052604090206001015461283c90856129ee565b6114f3565b6002816002811115612855576128556132d5565b036114f35760008381526005602052604090206001015461283c9086612af9565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128ad5750600090506003612931565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015612901573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b03811661292a57600060019250925050612931565b9150600090505b94509492505050565b600061082a8260a001518360c00151846101400151612140565b600080836001811115612969576129696132d5565b036129ad576000826001811115612982576129826132d5565b036129a357670de0b6b3a764000061299a85876133bd565b101590506119a3565b50828410156119a3565b60008260018111156129c1576129c16132d5565b036129d05750838310156119a3565b670de0b6b3a76400006129e385876133bd565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a2457612a24613391565b602002602001018181525050600281600181518110612a4557612a45613391565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612acb919060009088908790899060040161350d565b600060405180830381600087803b158015612ae557600080fd5b505af1158015612511573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b2f57612b2f613391565b602002602001018181525050600281600181518110612b5057612b50613391565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612aa1565b600060208284031215612bc257600080fd5b81356001600160e01b03198116811461176c57600080fd5b6001600160a01b038116811461098357600080fd5b8035612bfa81612bda565b919050565b60008060408385031215612c1257600080fd5b8235612c1d81612bda565b946020939093013593505050565b600060208284031215612c3d57600080fd5b813561176c81612bda565b600060208284031215612c5a57600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612c9a57612c9a612c61565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612cc857612cc8612c61565b604052919050565b803560028110612bfa57600080fd5b803560038110612bfa57600080fd5b600082601f830112612cff57600080fd5b81356001600160401b03811115612d1857612d18612c61565b612d2b601f8201601f1916602001612ca0565b818152846020838601011115612d4057600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612d7057600080fd5b612d78612c77565b905081358152612d8a60208301612bef565b6020820152612d9b60408301612bef565b6040820152612dac60608301612bef565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612dff818401612cd0565b90820152610160612e11838201612cdf565b90820152610180828101356001600160401b03811115612e3057600080fd5b612e3c85828601612cee565b82840152505092915050565b600060208284031215612e5a57600080fd5b81356001600160401b03811115612e7057600080fd5b6119a384828501612d5d565b600080600060608486031215612e9157600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612ec157612ec1612c61565b5060051b60200190565b600082601f830112612edc57600080fd5b81356020612ef1612eec83612ea8565b612ca0565b82815260059290921b84018101918181019086841115612f1057600080fd5b8286015b84811015612f2b5780358352918301918301612f14565b509695505050505050565b600080600080600060a08688031215612f4e57600080fd5b8535612f5981612bda565b94506020860135612f6981612bda565b935060408601356001600160401b0380821115612f8557600080fd5b612f9189838a01612ecb565b94506060880135915080821115612fa757600080fd5b612fb389838a01612ecb565b93506080880135915080821115612fc957600080fd5b50612fd688828901612cee565b9150509295509295909350565b600082601f830112612ff457600080fd5b81356020613004612eec83612ea8565b82815260059290921b8401810191818101908684111561302357600080fd5b8286015b84811015612f2b5780356001600160401b038111156130465760008081fd5b6130548986838b0101612d5d565b845250918301918301613027565b6000806040838503121561307557600080fd5b82356001600160401b038082111561308c57600080fd5b61309886838701612fe3565b935060208501359150808211156130ae57600080fd5b506130bb85828601612ecb565b9150509250929050565b600080604083850312156130d857600080fd5b50508035926020909101359150565b600080604083850312156130fa57600080fd5b8235915060208301356001600160401b0381111561311757600080fd5b6130bb85828601612d5d565b6000806000806080858703121561313957600080fd5b84356001600160401b038082111561315057600080fd5b61315c88838901612d5d565b9550602087013591508082111561317257600080fd5b61317e88838901612fe3565b945060408701359350606087013591508082111561319b57600080fd5b506131a887828801612ecb565b91505092959194509250565b600080600080600060a086880312156131cc57600080fd5b85356131d781612bda565b945060208601356131e781612bda565b9350604086013592506060860135915060808601356001600160401b0381111561321057600080fd5b612fd688828901612cee565b60006020828403121561322e57600080fd5b81356001600160401b0381111561324457600080fd5b6119a384828501612fe3565b6000806040838503121561326357600080fd5b82356001600160401b0381111561327957600080fd5b61328585828601612d5d565b95602094909401359450505050565b6000602082840312156132a657600080fd5b815161176c81612bda565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b600381106132fb576132fb6132d5565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a081016002841061336a5761336a6132d5565b8361016083015261337f6101808301846132eb565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600082198211156133d0576133d06133a7565b500190565b6000828210156133e7576133e76133a7565b500390565b60005b838110156134075781810151838201526020016133ef565b83811115611bde5750506000910152565b6000835161342a8184602088016133ec565b83519083019061343e8183602088016133ec565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b600081600019048311821515161561349a5761349a6133a7565b500290565b6000826134bc57634e487b7160e01b600052601260045260246000fd5b500490565b6000602082840312156134d357600080fd5b5051919050565b60208152600082518060208401526134f98160408501602087016133ec565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561356357845183529383019391830191600101613547565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a26469706673582212203f57711e1c3b3c9d2d81cac71a38da37ceb66c66b55131f273e99a283ad06a5664736f6c634300080f00330000000000000000000000002e8dcfe708d44ae2e406a1c02dfe2fa13012f9610000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x4a8b6a6a0b3604df860d88eaef3e2f6617ab98d701ecbf8b27318b6382b74dfe",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "to": "0xdfe02eb6733538f8ea35d585af8de5958ad99e40",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xe747cb68590bd99b3f3db2a2e610606b1f3b08161df677fcc0f1451dff445292",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "to": "0xdfe02eb6733538f8ea35d585af8de5958ad99e40",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xc5096781de50cfe0d1c5157a2e17dfcf0f5412b609fe364fd8a81022c6c09ffe",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "to": "0xdfe02eb6733538f8ea35d585af8de5958ad99e40",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x5c47c047ee5a1e05c246bcee2ef2fd69a68ce00dd9839a18a48220ca8989ed9f",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "to": "0xdfe02eb6733538f8ea35d585af8de5958ad99e40",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [
    "0xee31daedd823a5bc2134ddf35fa4fb17704349e4892837aa8913e737dd1c0dfb",
    "0x4a8b6a6a0b3604df860d88eaef3e2f6617ab98d701ecbf8b27318b6382b74dfe",
    "0xe747cb68590bd99b3f3db2a2e610606b1f3b08161df677fcc0f1451dff445292",
    "0xc5096781de50cfe0d1c5157a2e17dfcf0f5412b609fe364fd8a81022c6c09ffe",
    "0x5c47c047ee5a1e05c246bcee2ef2fd69a68ce00dd9839a18a48220ca8989ed9f"
  ],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40"
    }
  },
  "timestamp": 1663792323,
  "commit": "99d3728"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/80001/deployExchange-1663792337.json
================================================
{
  "transactions": [
    {
      "hash": "0xee31daedd823a5bc2134ddf35fa4fb17704349e4892837aa8913e737dd1c0dfb",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": null,
      "arguments": [
        "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
        "0x7D8610E9567d2a6C9FBf66a5A13E9Ba8bb120d43",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "gas": "0x4088ed",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b2938038062003b298339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161372262000407600039600061079e01526000818161043401528181611e670152818161203b01528181612a520152612b5d01526000818161055701528181611dd801528181611ff001528181612a810152612b8c01526000611a9601526000611ae501526000611ac001526000611a1901526000611a4301526000611a6d01526137226000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bb0565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612bff565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c2b565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c2b565b6108de565b6102ee6103bc366004612c2b565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c48565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c48565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c2b565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c2b565b6109f1565b610358610a2b565b6104f46104b1366004612c48565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c48565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e48565b610a6e565b6103586105b1366004612e7c565b610a89565b6102ee6105c4366004612c2b565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c2b565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c2b565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c2b565b610bb0565b61038d61064f366004612c48565b610c28565b610338610662366004612c2b565b610c46565b610358610675366004612e48565b610c65565b610358610688366004612c2b565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f36565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb366004613062565b610cf7565b61038d6106fe366004612c48565b60009081526005602052604090206001015490565b6103586107213660046130c5565b610d8f565b610338610db7565b61035861073c3660046130e7565b610e01565b61038d61074f366004612e48565b610e3d565b610358610762366004613123565b610eda565b610338610775366004612c2b565b610f6c565b6106b16107883660046131b4565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce36600461321c565b610f8b565b6103586107e1366004612c2b565b610fc2565b6103586107f4366004613250565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e9190613294565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132b1565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a999897969594939291906132ff565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132b1565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611724565b805160005b81811015610ac557610fba838281518110610fad57610fad613391565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611773565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132b1565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117cf565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133bd565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a1846118d2565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef848383611938565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c3613391565b60200260200101518583815181106114dd576114dd613391565b6020026020010151856117cf565b6001016114a6565b5050505050565b60008082600281111561150f5761150f6132d5565b036115275761152086868686611977565b9050610cee565b600282600281111561153b5761153b6132d5565b0361154c57611520868686866119ab565b611520868686866119e5565b600061082a611565611a0c565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b33565b9250925092506000806115c78a611b83565b915091506115db8a60200151308489611bba565b6115e68a8a89611be4565b6115f08582611c36565b6101208b015190955060009061163290828d61014001516001811115611618576116186132d5565b146116235788611625565b875b89898f6101400151611c65565b905061164f308c6020015184848a61164a91906133d5565b611bba565b61165b30338484611d55565b60408051848152602081018490529081018890526060810186905260808101829052309085907f6cda7c3afcd28346af42a5c662af2fbf6678f0af621dabb4b6fa9ee1c3b3c2e99060a00160405180910390a38183857fe914d2271d0909cb9f124ce60596eaa1e20ffc58a6a906ad7d5f9d096cc77fa28a8a6040516116eb929190918252602082015260400190565b60405180910390a460006116fe84611db1565b9050801561171657611716308d602001518684611bba565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b166020820152600090611769908390859060340160405160208183030381529060405280519060200120611e95565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806117de8785611b33565b925092509250600061183b88610120015160006001811115611802576118026132d5565b8a61014001516001811115611819576118196132d5565b146118245786611826565b855b8a60a001518b60c001518c6101400151611c65565b90506000806118498a611b83565b91509150611863338b6020015183868a61164a91906133d5565b6118738a6020015189848a611bba565b60408051838152602081018390529081018890526060810186905260808101849052339085907f6cda7c3afcd28346af42a5c662af2fbf6678f0af621dabb4b6fa9ee1c3b3c2e99060a00160405180910390a350505050505050505050565b6060604051806101a00160405280610171815260200161357c6101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119229291602001613418565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119569493929190613447565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119a057506119a0858484611eea565b90505b949350505050565b60006119b8858484611eea565b80156119a05750836001600160a01b03166119d286610c46565b6001600160a01b03161495945050505050565b60006119f2858484611eea565b80156119a05750836001600160a01b03166119d286610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a6557507f000000000000000000000000000000000000000000000000000000000000000046145b15611a8f57507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b458560600151611f12565b611b4e85610e3d565b9050611b5a818661117d565b611b6d848660a001518760c00151611f51565b9250611b7a818686611f78565b91509250925092565b600080808361014001516001811115611b9e57611b9e6132d5565b03611bae57505060800151600091565b50506080015190600090565b81600003611bd257611bcd848483611fee565b611bde565b611bde84848484612036565b50505050565b815160005b818110156114f357611c2e85858381518110611c0757611c07613391565b6020026020010151858481518110611c2157611c21613391565b6020026020010151612063565b600101611be9565b600080611c4283611db1565b90508381101561176c576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611c7a858585612140565b9050600081118015611c945750670de0b6b3a76400008111155b15611d4b576000836001811115611cad57611cad6132d5565b03611cff57611cbe61271082613480565b86611cda83611cd581670de0b6b3a76400006133d5565b6121af565b611ce4908a613480565b611cee9190613480565b611cf8919061349f565b9150611d4b565b611d13670de0b6b3a7640000612710613480565b86611d2a83611cd581670de0b6b3a76400006133d5565b611d34908a613480565b611d3e9190613480565b611d48919061349f565b91505b5095945050505050565b8015611bde57611d6784848484611bba565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e44576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e20573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134c1565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e03565b600080611ea285856121c5565b805190602001209050600060ff60f81b868584604051602001611ec89493929190613447565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f0084846122dc565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f3357506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f635750600061176c565b82611f6e8386613480565b611769919061349f565b60008381526008602052604090206001810154908115611f985781611f9e565b8360a001515b915081831115611fc157604051637166356b60e11b815260040160405180910390fd5b611fcb83836133d5565b915081600003611fe157805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361202a57611bcd818484612300565b611bde8185858561230b565b611bde7f000000000000000000000000000000000000000000000000000000000000000085858585612317565b600061206f848461239d565b905061207c848483612439565b816000808061208b8785611b33565b92509250925060006120af88610120015160006001811115611802576118026132d5565b90506000806120bd8a611b83565b915091506120d487878c6020015185858d896124b3565b6020808c015160408051858152928301849052820189905260608201879052608082018590526001600160a01b03169085907f6cda7c3afcd28346af42a5c662af2fbf6678f0af621dabb4b6fa9ee1c3b3c2e99060a00160405180910390a35050505050505050505050565b600080826001811115612155576121556132d5565b0361218d5782600003612169576000612186565b8261217c670de0b6b3a764000086613480565b612186919061349f565b905061176c565b8360000361219c576000611769565b83611f6e670de0b6b3a764000085613480565b60008183106121be578161176c565b5090919050565b60408051600080825260208201909252606091906121e690604481016134da565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122c3929190613418565b6040516020818303038152906040529250505092915050565b60008060006122eb858561251a565b915091506122f88161255f565b509392505050565b610ac58383836126a9565b611bde84848484612721565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b15801561237e57600080fd5b505af1158015612392573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123b7576123b76132d5565b1480156123da5750600082610140015160018111156123d8576123d86132d5565b145b156123e75750600161082a565b60018361014001516001811115612400576124006132d5565b148015612423575060018261014001516001811115612421576124216132d5565b145b156124305750600261082a565b50600092915050565b61244383836127a4565b61246057604051633fcd37a360e11b815260040160405180910390fd5b6000816002811115612474576124746132d5565b036124a1578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124bf8530868a611bba565b6124cc87878686866127ee565b856124d684611db1565b10156124f5576040516301be9b0160e71b815260040160405180910390fd5b61250530868561164a858b6133d5565b61251130338584611d55565b50505050505050565b60008082516041036125505760208301516040840151606085015160001a61254487828585612876565b94509450505050612558565b506000905060025b9250929050565b6000816004811115612573576125736132d5565b0361257b5750565b600181600481111561258f5761258f6132d5565b036125dc5760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b60028160048111156125f0576125f06132d5565b0361263d5760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b6003816004811115612651576126516132d5565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611bde5760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127bb575060c0820151155b156127c85750600161082a565b61176c6127d48461293a565b6127dd8461293a565b856101400151856101400151612954565b6000816002811115612802576128026132d5565b146114f357600181600281111561281b5761281b6132d5565b036128415760008281526005602052604090206001015461283c90856129ee565b6114f3565b6002816002811115612855576128556132d5565b036114f35760008381526005602052604090206001015461283c9086612af9565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128ad5750600090506003612931565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015612901573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b03811661292a57600060019250925050612931565b9150600090505b94509492505050565b600061082a8260a001518360c00151846101400151612140565b600080836001811115612969576129696132d5565b036129ad576000826001811115612982576129826132d5565b036129a357670de0b6b3a764000061299a85876133bd565b101590506119a3565b50828410156119a3565b60008260018111156129c1576129c16132d5565b036129d05750838310156119a3565b670de0b6b3a76400006129e385876133bd565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a2457612a24613391565b602002602001018181525050600281600181518110612a4557612a45613391565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612acb919060009088908790899060040161350d565b600060405180830381600087803b158015612ae557600080fd5b505af1158015612511573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b2f57612b2f613391565b602002602001018181525050600281600181518110612b5057612b50613391565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612aa1565b600060208284031215612bc257600080fd5b81356001600160e01b03198116811461176c57600080fd5b6001600160a01b038116811461098357600080fd5b8035612bfa81612bda565b919050565b60008060408385031215612c1257600080fd5b8235612c1d81612bda565b946020939093013593505050565b600060208284031215612c3d57600080fd5b813561176c81612bda565b600060208284031215612c5a57600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612c9a57612c9a612c61565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612cc857612cc8612c61565b604052919050565b803560028110612bfa57600080fd5b803560038110612bfa57600080fd5b600082601f830112612cff57600080fd5b81356001600160401b03811115612d1857612d18612c61565b612d2b601f8201601f1916602001612ca0565b818152846020838601011115612d4057600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612d7057600080fd5b612d78612c77565b905081358152612d8a60208301612bef565b6020820152612d9b60408301612bef565b6040820152612dac60608301612bef565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612dff818401612cd0565b90820152610160612e11838201612cdf565b90820152610180828101356001600160401b03811115612e3057600080fd5b612e3c85828601612cee565b82840152505092915050565b600060208284031215612e5a57600080fd5b81356001600160401b03811115612e7057600080fd5b6119a384828501612d5d565b600080600060608486031215612e9157600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612ec157612ec1612c61565b5060051b60200190565b600082601f830112612edc57600080fd5b81356020612ef1612eec83612ea8565b612ca0565b82815260059290921b84018101918181019086841115612f1057600080fd5b8286015b84811015612f2b5780358352918301918301612f14565b509695505050505050565b600080600080600060a08688031215612f4e57600080fd5b8535612f5981612bda565b94506020860135612f6981612bda565b935060408601356001600160401b0380821115612f8557600080fd5b612f9189838a01612ecb565b94506060880135915080821115612fa757600080fd5b612fb389838a01612ecb565b93506080880135915080821115612fc957600080fd5b50612fd688828901612cee565b9150509295509295909350565b600082601f830112612ff457600080fd5b81356020613004612eec83612ea8565b82815260059290921b8401810191818101908684111561302357600080fd5b8286015b84811015612f2b5780356001600160401b038111156130465760008081fd5b6130548986838b0101612d5d565b845250918301918301613027565b6000806040838503121561307557600080fd5b82356001600160401b038082111561308c57600080fd5b61309886838701612fe3565b935060208501359150808211156130ae57600080fd5b506130bb85828601612ecb565b9150509250929050565b600080604083850312156130d857600080fd5b50508035926020909101359150565b600080604083850312156130fa57600080fd5b8235915060208301356001600160401b0381111561311757600080fd5b6130bb85828601612d5d565b6000806000806080858703121561313957600080fd5b84356001600160401b038082111561315057600080fd5b61315c88838901612d5d565b9550602087013591508082111561317257600080fd5b61317e88838901612fe3565b945060408701359350606087013591508082111561319b57600080fd5b506131a887828801612ecb565b91505092959194509250565b600080600080600060a086880312156131cc57600080fd5b85356131d781612bda565b945060208601356131e781612bda565b9350604086013592506060860135915060808601356001600160401b0381111561321057600080fd5b612fd688828901612cee565b60006020828403121561322e57600080fd5b81356001600160401b0381111561324457600080fd5b6119a384828501612fe3565b6000806040838503121561326357600080fd5b82356001600160401b0381111561327957600080fd5b61328585828601612d5d565b95602094909401359450505050565b6000602082840312156132a657600080fd5b815161176c81612bda565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b600381106132fb576132fb6132d5565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a081016002841061336a5761336a6132d5565b8361016083015261337f6101808301846132eb565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b600082198211156133d0576133d06133a7565b500190565b6000828210156133e7576133e76133a7565b500390565b60005b838110156134075781810151838201526020016133ef565b83811115611bde5750506000910152565b6000835161342a8184602088016133ec565b83519083019061343e8183602088016133ec565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b600081600019048311821515161561349a5761349a6133a7565b500290565b6000826134bc57634e487b7160e01b600052601260045260246000fd5b500490565b6000602082840312156134d357600080fd5b5051919050565b60208152600082518060208401526134f98160408501602087016133ec565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561356357845183529383019391830191600101613547565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a26469706673582212203f57711e1c3b3c9d2d81cac71a38da37ceb66c66b55131f273e99a283ad06a5664736f6c634300080f00330000000000000000000000002e8dcfe708d44ae2e406a1c02dfe2fa13012f9610000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x4a8b6a6a0b3604df860d88eaef3e2f6617ab98d701ecbf8b27318b6382b74dfe",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "to": "0xdfe02eb6733538f8ea35d585af8de5958ad99e40",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xe747cb68590bd99b3f3db2a2e610606b1f3b08161df677fcc0f1451dff445292",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "to": "0xdfe02eb6733538f8ea35d585af8de5958ad99e40",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xc5096781de50cfe0d1c5157a2e17dfcf0f5412b609fe364fd8a81022c6c09ffe",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "to": "0xdfe02eb6733538f8ea35d585af8de5958ad99e40",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x5c47c047ee5a1e05c246bcee2ef2fd69a68ce00dd9839a18a48220ca8989ed9f",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x404b2bc72675776b85d9ba64c39af4c0ad18304b",
        "to": "0xdfe02eb6733538f8ea35d585af8de5958ad99e40",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [
    {
      "transactionHash": "0xee31daedd823a5bc2134ddf35fa4fb17704349e4892837aa8913e737dd1c0dfb",
      "transactionIndex": "0x2",
      "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
      "blockNumber": "0x1aead92",
      "from": "0x404B2bC72675776B85d9BA64c39af4C0AD18304b",
      "to": null,
      "cumulativeGasUsed": "0x348867",
      "gasUsed": "0x31a468",
      "contractAddress": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "logs": [
        {
          "address": "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
          "topics": [
            "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
            "0x000000000000000000000000dfe02eb6733538f8ea35d585af8de5958ad99e40",
            "0x0000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43"
          ],
          "data": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0xee31daedd823a5bc2134ddf35fa4fb17704349e4892837aa8913e737dd1c0dfb",
          "transactionIndex": "0x2",
          "logIndex": "0x4",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x0000000000000000000000000000000000000000000000000022acb81ede3000000000000000000000000000000000000000000000000000058d15e1762800000000000000000000000000000000000000000000000025853de7eff343f48bf9000000000000000000000000000000000000000000000000056a69295749d0000000000000000000000000000000000000000000000025853e0a9cab62d2bbf9",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0xee31daedd823a5bc2134ddf35fa4fb17704349e4892837aa8913e737dd1c0dfb",
          "transactionIndex": "0x2",
          "logIndex": "0x5",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00008000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000080008000000000000000000000200000001000000000000000000000000000800000000000000000000100000000004000000000000020000000000000000008000000000000000080040000000000000000000000000000000000000000000000000000000080000000000000000000220000000000002000000000000000000000000000000000000000020000004000000000000000000001000400000000000000000000000000100040000000000010000000000000000000000000000000000000000000001000000001100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    },
    {
      "transactionHash": "0x4a8b6a6a0b3604df860d88eaef3e2f6617ab98d701ecbf8b27318b6382b74dfe",
      "transactionIndex": "0x3",
      "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
      "blockNumber": "0x1aead92",
      "from": "0x404B2bC72675776B85d9BA64c39af4C0AD18304b",
      "to": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "cumulativeGasUsed": "0x3542b9",
      "gasUsed": "0xba52",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
          "topics": [
            "0xf9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc",
            "0x000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b"
          ],
          "data": "0x",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0x4a8b6a6a0b3604df860d88eaef3e2f6617ab98d701ecbf8b27318b6382b74dfe",
          "transactionIndex": "0x3",
          "logIndex": "0x6",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000008224ab0a1c00000000000000000000000000000000000000000000000000056a69295492d2500000000000000000000000000000000000000000000025853e0a9cab62d2bbf90000000000000000000000000000000000000000000000000569e704a988b6500000000000000000000000000000000000000000000025853e0b1ed00ddcd7f9",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0x4a8b6a6a0b3604df860d88eaef3e2f6617ab98d701ecbf8b27318b6382b74dfe",
          "transactionIndex": "0x3",
          "logIndex": "0x7",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000010000000000000000000000000000800000000000000000000100002000004000000000000000000000000000200008000000000000000080040000000000000000000000000000000000000000000000080000000080000000000000000000200000000000002000000000000000000000080000000000000000000000004000000000020028000001000000000000000000000000000000100040000000000000000000000000000000000000000100000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    },
    {
      "transactionHash": "0xe747cb68590bd99b3f3db2a2e610606b1f3b08161df677fcc0f1451dff445292",
      "transactionIndex": "0x4",
      "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
      "blockNumber": "0x1aead92",
      "from": "0x404B2bC72675776B85d9BA64c39af4C0AD18304b",
      "to": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "cumulativeGasUsed": "0x35fd5a",
      "gasUsed": "0xbaa1",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
          "topics": [
            "0xf1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c",
            "0x000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b"
          ],
          "data": "0x",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0xe747cb68590bd99b3f3db2a2e610606b1f3b08161df677fcc0f1451dff445292",
          "transactionIndex": "0x4",
          "logIndex": "0x8",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x0000000000000000000000000000000000000000000000000000825bd9571e000000000000000000000000000000000000000000000000000569e704a97e85d40000000000000000000000000000000000000000000025853e0b1ed00ddcd7f9000000000000000000000000000000000000000000000000056964a8d02767d40000000000000000000000000000000000000000000025853e0ba12be733f5f9",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0xe747cb68590bd99b3f3db2a2e610606b1f3b08161df677fcc0f1451dff445292",
          "transactionIndex": "0x4",
          "logIndex": "0x9",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000010000000000000000000000000000800000000000000000000100002000004000000000000000000000000000000008000000000000000080040000000000000000000000000000000000000000000000080000000080000000000000002000200000000000002000100000000000000000000000000000000000000000004000000000020028000001000000000000000000000000000000100040000000000000000000000000000000000000000000800000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    },
    {
      "transactionHash": "0xc5096781de50cfe0d1c5157a2e17dfcf0f5412b609fe364fd8a81022c6c09ffe",
      "transactionIndex": "0x5",
      "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
      "blockNumber": "0x1aead92",
      "from": "0x404B2bC72675776B85d9BA64c39af4C0AD18304b",
      "to": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "cumulativeGasUsed": "0x365806",
      "gasUsed": "0x5aac",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
          "topics": [
            "0x787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b"
          ],
          "data": "0x",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0xc5096781de50cfe0d1c5157a2e17dfcf0f5412b609fe364fd8a81022c6c09ffe",
          "transactionIndex": "0x5",
          "logIndex": "0xa",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000003f55650b2800000000000000000000000000000000000000000000000000056964a8d01d33060000000000000000000000000000000000000000000025853e0ba12be733f5f9000000000000000000000000000000000000000000000000056925536b120b060000000000000000000000000000000000000000000025853e0be0814c3f1df9",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0xc5096781de50cfe0d1c5157a2e17dfcf0f5412b609fe364fd8a81022c6c09ffe",
          "transactionIndex": "0x5",
          "logIndex": "0xb",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000800000000000000000000100002000004000020000000000000000000000000008000000000000002080040000000000000000000000000000000000000000000000080000000080000000000000000000200000000000002000000000000000000000000000000000000000000000004000000000020000000001000000000000000000000000000000100040000000000000000000000000000000000000000000000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    },
    {
      "transactionHash": "0x5c47c047ee5a1e05c246bcee2ef2fd69a68ce00dd9839a18a48220ca8989ed9f",
      "transactionIndex": "0x6",
      "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
      "blockNumber": "0x1aead92",
      "from": "0x404B2bC72675776B85d9BA64c39af4C0AD18304b",
      "to": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
      "cumulativeGasUsed": "0x36b2d8",
      "gasUsed": "0x5ad2",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40",
          "topics": [
            "0xf7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b"
          ],
          "data": "0x",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0x5c47c047ee5a1e05c246bcee2ef2fd69a68ce00dd9839a18a48220ca8989ed9f",
          "transactionIndex": "0x6",
          "logIndex": "0xc",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000404b2bc72675776b85d9ba64c39af4c0ad18304b",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000003f6feff91c00000000000000000000000000000000000000000000000000056925536b0d159e0000000000000000000000000000000000000000000025853e0be0814c3f1df90000000000000000000000000000000000000000000000000568e5e37b13f99e0000000000000000000000000000000000000000000025853e0c1ff13c3839f9",
          "blockHash": "0xce7917e37585ad043a3da7b54488c7e0789dd3c0af184bebb289762819bc0b9d",
          "blockNumber": "0x1aead92",
          "transactionHash": "0x5c47c047ee5a1e05c246bcee2ef2fd69a68ce00dd9839a18a48220ca8989ed9f",
          "transactionIndex": "0x6",
          "logIndex": "0xd",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000000800000000000000000000100002000004000000000000000000000100000000008000000000000000080040000000000000000000000000000000000000000000000080000000080000000000000000000200000000000002000000000000000000000000000000000000000000000004004000000020000800001000000000000000000000000000000100040000000000000000000000000000000000000000000000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    }
  ],
  "libraries": [],
  "pending": [],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xdFE02Eb6733538f8Ea35D585af8DE5958AD99E40"
    }
  },
  "timestamp": 1663792337,
  "commit": "99d3728"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/80001/deployExchange-1663954744.json
================================================
{
  "transactions": [
    {
      "hash": "0x284159990e3c13c5133009aedfaae79a8c50b35b2134093c32b5ba905b187780",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": null,
      "arguments": [
        "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
        "0x7D8610E9567d2a6C9FBf66a5A13E9Ba8bb120d43",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "gas": "0x40cad7",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002e8dcfe708d44ae2e406a1c02dfe2fa13012f9610000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xf1133522574a9acd7b25cf90f070a1a1ad9805dfee1924b2f9cf140f19ebb888",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x8bce3d412b37bd7c1499d7eb5cf1f6f7f9c150272b8b0cb29aaeebe9e2ac5ae3",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x14e545af6c5eda791bbe40b810fab40edcb9fb891f809425d845d5282f429002",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xa189ba785b05fe6e6a0ac4b543f8f4dd5ae7090adae8a6ba2697817b36bbbe38",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [
    "0x284159990e3c13c5133009aedfaae79a8c50b35b2134093c32b5ba905b187780",
    "0xf1133522574a9acd7b25cf90f070a1a1ad9805dfee1924b2f9cf140f19ebb888",
    "0x8bce3d412b37bd7c1499d7eb5cf1f6f7f9c150272b8b0cb29aaeebe9e2ac5ae3",
    "0x14e545af6c5eda791bbe40b810fab40edcb9fb891f809425d845d5282f429002",
    "0xa189ba785b05fe6e6a0ac4b543f8f4dd5ae7090adae8a6ba2697817b36bbbe38"
  ],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db"
    }
  },
  "timestamp": 1663954744,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/80001/deployExchange-1663954757.json
================================================
{
  "transactions": [
    {
      "hash": "0x284159990e3c13c5133009aedfaae79a8c50b35b2134093c32b5ba905b187780",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": null,
      "arguments": [
        "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
        "0x7D8610E9567d2a6C9FBf66a5A13E9Ba8bb120d43",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "gas": "0x40cad7",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002e8dcfe708d44ae2e406a1c02dfe2fa13012f9610000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xf1133522574a9acd7b25cf90f070a1a1ad9805dfee1924b2f9cf140f19ebb888",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x8bce3d412b37bd7c1499d7eb5cf1f6f7f9c150272b8b0cb29aaeebe9e2ac5ae3",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x14e545af6c5eda791bbe40b810fab40edcb9fb891f809425d845d5282f429002",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xa189ba785b05fe6e6a0ac4b543f8f4dd5ae7090adae8a6ba2697817b36bbbe38",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x769bc17a26fd41ce24f934403c8492bdfac6c548",
        "to": "0xbe9f464bc8673dc26ae4f8ed91156c75677762db",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [
    {
      "transactionHash": "0x284159990e3c13c5133009aedfaae79a8c50b35b2134093c32b5ba905b187780",
      "transactionIndex": "0x1",
      "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
      "blockNumber": "0x1af2c10",
      "from": "0x769bC17a26FD41cE24F934403c8492bDfAC6C548",
      "to": null,
      "cumulativeGasUsed": "0x32c1b1",
      "gasUsed": "0x31d71c",
      "contractAddress": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "logs": [
        {
          "address": "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
          "topics": [
            "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
            "0x000000000000000000000000be9f464bc8673dc26ae4f8ed91156c75677762db",
            "0x0000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43"
          ],
          "data": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0x284159990e3c13c5133009aedfaae79a8c50b35b2134093c32b5ba905b187780",
          "transactionIndex": "0x1",
          "logIndex": "0x2",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x0000000000000000000000000000000000000000000000000022d0228bbc4800000000000000000000000000000000000000000000000000058d15e176280000000000000000000000000000000000000000000000002593ec000bb80767a15f000000000000000000000000000000000000000000000000056a45beea6bb800000000000000000000000000000000000000000000002593ec22dbda9323e95f",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0x284159990e3c13c5133009aedfaae79a8c50b35b2134093c32b5ba905b187780",
          "transactionIndex": "0x1",
          "logIndex": "0x3",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00008000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000080008000000000000000000000200000000000000000000000040000000000800010000000000000000100000000004000000000000020000000000000000000000000000000000080000000000000000020000000000000000000000000000000000000000080000000000000000000320000000000000000000000080000000000000000000000000000000000004000000000000000000001000400000000000000000000000400100040000000000010000000000000000000000000000000000000000000001000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    },
    {
      "transactionHash": "0xf1133522574a9acd7b25cf90f070a1a1ad9805dfee1924b2f9cf140f19ebb888",
      "transactionIndex": "0x2",
      "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
      "blockNumber": "0x1af2c10",
      "from": "0x769bC17a26FD41cE24F934403c8492bDfAC6C548",
      "to": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "cumulativeGasUsed": "0x337c03",
      "gasUsed": "0xba52",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
          "topics": [
            "0xf9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc",
            "0x000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548"
          ],
          "data": "0x",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0xf1133522574a9acd7b25cf90f070a1a1ad9805dfee1924b2f9cf140f19ebb888",
          "transactionIndex": "0x2",
          "logIndex": "0x4",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000008224ab0a1c00000000000000000000000000000000000000000000000000056a45bee7b1f478000000000000000000000000000000000000000000002593ec22dbda9323e95f0000000000000000000000000000000000000000000000000569c39a3ca7d878000000000000000000000000000000000000000000002593ec235dff3e2e055f",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0xf1133522574a9acd7b25cf90f070a1a1ad9805dfee1924b2f9cf140f19ebb888",
          "transactionIndex": "0x2",
          "logIndex": "0x5",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000010000000000000000000000000200800000000000000000000100000000004000000000000000000000000000200000000000000000000080000000000000000020000080000000000000000000000000000000000080000000000000000000300000000000000000000000000000000000080000000000000000000000004000008000000028000001000000000000000000000000000400100040000000000000000000000000000000000000000100000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    },
    {
      "transactionHash": "0x8bce3d412b37bd7c1499d7eb5cf1f6f7f9c150272b8b0cb29aaeebe9e2ac5ae3",
      "transactionIndex": "0x3",
      "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
      "blockNumber": "0x1af2c10",
      "from": "0x769bC17a26FD41cE24F934403c8492bDfAC6C548",
      "to": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "cumulativeGasUsed": "0x3436a4",
      "gasUsed": "0xbaa1",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
          "topics": [
            "0xf1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c",
            "0x000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548"
          ],
          "data": "0x",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0x8bce3d412b37bd7c1499d7eb5cf1f6f7f9c150272b8b0cb29aaeebe9e2ac5ae3",
          "transactionIndex": "0x3",
          "logIndex": "0x6",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x0000000000000000000000000000000000000000000000000000825bd9571e000000000000000000000000000000000000000000000000000569c39a3c9da7fc000000000000000000000000000000000000000000002593ec235dff3e2e055f0000000000000000000000000000000000000000000000000569413e634689fc000000000000000000000000000000000000000000002593ec23e05b1785235f",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0x8bce3d412b37bd7c1499d7eb5cf1f6f7f9c150272b8b0cb29aaeebe9e2ac5ae3",
          "transactionIndex": "0x3",
          "logIndex": "0x7",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000010000000000000000000000000200800000000000000000000100000000004000000000000000000000000000000000000000000000000080000000000000000020000080000000000000000000000000000000000080000000000000002000300000000000000000100000000000000000000000000000000000000000004000008000000028000001000000000000000000000000000400100040000000000000000000000000000000000000000000800000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    },
    {
      "transactionHash": "0x14e545af6c5eda791bbe40b810fab40edcb9fb891f809425d845d5282f429002",
      "transactionIndex": "0x4",
      "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
      "blockNumber": "0x1af2c10",
      "from": "0x769bC17a26FD41cE24F934403c8492bDfAC6C548",
      "to": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "cumulativeGasUsed": "0x349150",
      "gasUsed": "0x5aac",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
          "topics": [
            "0x787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548"
          ],
          "data": "0x",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0x14e545af6c5eda791bbe40b810fab40edcb9fb891f809425d845d5282f429002",
          "transactionIndex": "0x4",
          "logIndex": "0x8",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000003f55650b28000000000000000000000000000000000000000000000000000569413e633c552e000000000000000000000000000000000000000000002593ec23e05b1785235f000000000000000000000000000000000000000000000000056901e8fe312d2e000000000000000000000000000000000000000000002593ec241fb07c904b5f",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0x14e545af6c5eda791bbe40b810fab40edcb9fb891f809425d845d5282f429002",
          "transactionIndex": "0x4",
          "logIndex": "0x9",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000200800000000000000000000100000000004000020000000000000000000000000000000000000000002080000000000000000020000080000000000000000000000000000000000080000000000000000000300000000000000000000000000000000000000000000000000000000000004000008000000000000001000000000000000000000000000400100040000000000000000000000000000000000000000000000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    },
    {
      "transactionHash": "0xa189ba785b05fe6e6a0ac4b543f8f4dd5ae7090adae8a6ba2697817b36bbbe38",
      "transactionIndex": "0x5",
      "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
      "blockNumber": "0x1af2c10",
      "from": "0x769bC17a26FD41cE24F934403c8492bDfAC6C548",
      "to": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
      "cumulativeGasUsed": "0x34ec22",
      "gasUsed": "0x5ad2",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db",
          "topics": [
            "0xf7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548"
          ],
          "data": "0x",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0xa189ba785b05fe6e6a0ac4b543f8f4dd5ae7090adae8a6ba2697817b36bbbe38",
          "transactionIndex": "0x5",
          "logIndex": "0xa",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x000000000000000000000000769bc17a26fd41ce24f934403c8492bdfac6c548",
            "0x000000000000000000000000be188d6641e8b680743a4815dfa0f6208038960f"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000003f6feff91c00000000000000000000000000000000000000000000000000056901e8fe2c37c6000000000000000000000000000000000000000000002593ec241fb07c904b5f0000000000000000000000000000000000000000000000000568c2790e331bc6000000000000000000000000000000000000000000002593ec245f206c89675f",
          "blockHash": "0x20a1ebf99c45724a09b89d7b7a88097ab95f988da9ed60d1125d60aa802b7899",
          "blockNumber": "0x1af2c10",
          "transactionHash": "0xa189ba785b05fe6e6a0ac4b543f8f4dd5ae7090adae8a6ba2697817b36bbbe38",
          "transactionIndex": "0x5",
          "logIndex": "0xb",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000000000000000200800000000000000000000100000000004000000000000000000000100000000000000000000000000080000000000000000020000080000000000000000000000000000000000080000000000000000000300000000000000000000000000000000000000000000000000000000000004004008000000000800001000000000000000000000000000400100040000000000000000000000000000000000000000000000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e0e"
    }
  ],
  "libraries": [],
  "pending": [],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xBe9F464Bc8673Dc26AE4f8ED91156c75677762Db"
    }
  },
  "timestamp": 1663954757,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/80001/deployExchange-1663955818.json
================================================
{
  "transactions": [
    {
      "hash": "0x999e01a7d4c213c0fa844f13f0c4d951d4337c1f33391b86b38f1fe55f74969d",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": null,
      "arguments": [
        "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
        "0x7D8610E9567d2a6C9FBf66a5A13E9Ba8bb120d43",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "gas": "0x40cad7",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002e8dcfe708d44ae2e406a1c02dfe2fa13012f9610000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x9aaa83506c9b5ddb21f0eebb9c4f328e821c04aacef8b01a0d61c275c16ec50b",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xdc92ff12528e1297d512077ef17014d7d11530801898a96c59bde64a959b5c0c",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x59508bd1f8541283b91f60a9f719223f95475648bc24139a53f37fcb667a7fad",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xc923d61fbb2d7351d26e7045cb468ec6e0529d9aa7a8622668762bb4b0334b73",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [
    "0x999e01a7d4c213c0fa844f13f0c4d951d4337c1f33391b86b38f1fe55f74969d",
    "0x9aaa83506c9b5ddb21f0eebb9c4f328e821c04aacef8b01a0d61c275c16ec50b",
    "0xdc92ff12528e1297d512077ef17014d7d11530801898a96c59bde64a959b5c0c",
    "0x59508bd1f8541283b91f60a9f719223f95475648bc24139a53f37fcb667a7fad",
    "0xc923d61fbb2d7351d26e7045cb468ec6e0529d9aa7a8622668762bb4b0334b73"
  ],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f"
    }
  },
  "timestamp": 1663955818,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/80001/deployExchange-1663955831.json
================================================
{
  "transactions": [
    {
      "hash": "0x999e01a7d4c213c0fa844f13f0c4d951d4337c1f33391b86b38f1fe55f74969d",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": null,
      "arguments": [
        "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
        "0x7D8610E9567d2a6C9FBf66a5A13E9Ba8bb120d43",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "gas": "0x40cad7",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b5060405162003b6538038062003b658339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e051610100516101205161014051610160516101805161375e62000407600039600061079e01526000818161043401528181611e9a0152818161206e01528181612a8e0152612b9901526000818161055701528181611e0b0152818161202301528181612abd0152612bc801526000611ac901526000611b1801526000611af301526000611a4c01526000611a7601526000611aa0015261375e6000f3fe608060405234801561001057600080fd5b50600436106102d65760003560e01c80637048027511610182578063d798eff6116100e9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610799578063fa950b48146107c0578063fbddd751146107d3578063fe729aaf146107e657600080fd5b8063e60f0c0514610754578063edef7d8e14610767578063f23a6e611461077a57600080fd5b8063d798eff6146106dd578063d7fb272f146106f0578063d82da83814610713578063e03ac3d014610726578063e2eec4051461072e578063e50e4f971461074157600080fd5b8063a287bdf11161013b578063a287bdf114610654578063a6dfcf8614610667578063ac8a584a1461067a578063b28c51c01461068d578063bc197c811461069e578063c10f1a75146106ca57600080fd5b806370480275146105e257806375d7370a146105f55780637ecebe001461060657806383b8a5ae146106265780639870d7fe1461062e578063a10f3dce1461064157600080fd5b8063429b62e5116102415780635893253c116101fa578063627cdcb9116101d4578063627cdcb914610588578063654f0ce41461059057806368c7450f146105a35780636d70f7ae146105b657600080fd5b80635893253c146105195780635c1548fb146105555780635c975abb1461057b57600080fd5b8063429b62e51461046057806344bea37e146104805780634544f05514610488578063456068d21461049b57806346423aa7146104a35780634a2a11f51461051157600080fd5b80631785f53c116102935780631785f53c1461039b57806324d7806c146103ae5780632dff692d146103db578063346009011461041f5780633b521d78146104325780633d6d35981461045857600080fd5b806301ffc9a7146102db5780630647ee201461030357806306b9d691146103305780631031e36e14610350578063131e7e1c1461035a57806313e7c9d81461036d575b600080fd5b6102ee6102e9366004612bec565b6107f9565b60405190151581526020015b60405180910390f35b6102ee610311366004612c3b565b6001600160a01b03919091166000908152600460205260409020541490565b610338610830565b6040516001600160a01b0390911681526020016102fa565b6103586108a3565b005b600754610338906001600160a01b031681565b61038d61037b366004612c67565b60026020526000908152604090205481565b6040519081526020016102fa565b6103586103a9366004612c67565b6108de565b6102ee6103bc366004612c67565b6001600160a01b03166000908152600160208190526040909120541490565b6104086103e9366004612c84565b6008602052600090815260409020805460019091015460ff9091169082565b6040805192151583526020830191909152016102fa565b61035861042d366004612c84565b610955565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b610358610986565b61038d61046e366004612c67565b60016020526000908152604090205481565b61038d600081565b610358610496366004612c67565b6109f1565b610358610a2b565b6104f46104b1366004612c84565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b6040805182511515815260209283015192810192909252016102fa565b6103e861038d565b610540610527366004612c84565b6005602052600090815260409020805460019091015482565b604080519283526020830191909152016102fa565b7f0000000000000000000000000000000000000000000000000000000000000000610338565b6003546102ee9060ff1681565b610358610a64565b61035861059e366004612e84565b610a6e565b6103586105b1366004612eb8565b610a89565b6102ee6105c4366004612c67565b6001600160a01b031660009081526002602052604090205460011490565b6103586105f0366004612c67565b610aca565b6007546001600160a01b0316610338565b61038d610614366004612c67565b60046020526000908152604090205481565b610358610b44565b61035861063c366004612c67565b610bb0565b61038d61064f366004612c84565b610c28565b610338610662366004612c67565b610c46565b610358610675366004612e84565b610c65565b610358610688366004612c67565b610c6e565b6006546001600160a01b0316610338565b6106b16106ac366004612f72565b610ce5565b6040516001600160e01b031990911681526020016102fa565b600654610338906001600160a01b031681565b6103586106eb36600461309e565b610cf7565b61038d6106fe366004612c84565b60009081526005602052604090206001015490565b610358610721366004613101565b610d8f565b610338610db7565b61035861073c366004613123565b610e01565b61038d61074f366004612e84565b610e3d565b61035861076236600461315f565b610eda565b610338610775366004612c67565b610f6c565b6106b16107883660046131f0565b63f23a6e6160e01b95945050505050565b61038d7f000000000000000000000000000000000000000000000000000000000000000081565b6103586107ce366004613258565b610f8b565b6103586107e1366004612c67565b610fc2565b6103586107f436600461328c565b610ffc565b60006001600160e01b03198216630271189760e51b148061082a57506301ffc9a760e01b6001600160e01b03198316145b92915050565b6006546040805163557887a160e11b815290516000926001600160a01b03169163aaf10f429160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061089e91906132d0565b905090565b33600090815260016020819052604090912054146108d457604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611082565b565b336000908152600160208190526040909120541461090f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610983576040516307ed98ed60e31b815260040160405180910390fd5b50565b336000908152600260205260409020546001146109b657604051631f0853c160e21b815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610a2257604051637bfa4b9f60e01b815260040160405180910390fd5b610983816110bc565b3360009081526001602081905260409091205414610a5c57604051637bfa4b9f60e01b815260040160405180910390fd5b6108dc611118565b6108dc600161114f565b6000610a7982610e3d565b9050610a85818361117d565b5050565b3360009081526001602081905260409091205414610aba57604051637bfa4b9f60e01b815260040160405180910390fd5b610ac583838361126b565b505050565b3360009081526001602081905260409091205414610afb57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610b7557604051637bfa4b9f60e01b815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610be157604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b03811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610c3382610955565b5060009081526005602052604090205490565b600061082a82610c54610db7565b6007546001600160a01b0316611395565b610983816113f9565b3360009081526001602081905260409091205414610c9f57604051637bfa4b9f60e01b815260040160405180910390fd5b6001600160a01b038116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b63bc197c8160e01b5b95945050505050565b600054600203610d225760405162461bcd60e51b8152600401610d19906132ed565b60405180910390fd5b600260008181553381526020919091526040902054600114610d5757604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610d7b576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336114a1565b50506001600055565b80610d9983610c28565b14610a855760405163337c310560e11b815260040160405180910390fd5b6007546040805163530ca43760e11b815290516000926001600160a01b03169163a619486e9160048083019260209291908290030181865afa15801561087a573d6000803e3d6000fd5b610e2081604001518260200151848461018001518561016001516114fa565b610a8557604051638baa579f60e01b815260040160405180910390fd5b600061082a7fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e6101600151604051602001610ebf9d9c9b9a9998979695949392919061333b565b60405160208183030381529060405280519060200120611558565b600054600203610efc5760405162461bcd60e51b8152600401610d19906132ed565b600260008181553381526020919091526040902054600114610f3157604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615610f55576040516313d0ff5960e31b815260040160405180910390fd5b610f61848484846115a6565b505060016000555050565b600061082a82610f7a610830565b6006546001600160a01b0316611747565b805160005b81811015610ac557610fba838281518110610fad57610fad6133cd565b60200260200101516113f9565b600101610f90565b3360009081526001602081905260409091205414610ff357604051637bfa4b9f60e01b815260040160405180910390fd5b61098381611796565b60005460020361101e5760405162461bcd60e51b8152600401610d19906132ed565b60026000818155338152602091909152604090205460011461105357604051631f0853c160e21b815260040160405180910390fd5b60035460ff1615611077576040516313d0ff5960e31b815260040160405180910390fd5b610d868282336117f2565b6003805460ff1916600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b6007546040516001600160a01b038084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780546001600160a01b0319166001600160a01b0392909216919091179055565b6003805460ff1916905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b3360009081526004602052604090205461116a9082906133f9565b3360009081526004602052604090205550565b60008160e001511180156111945750428160e00151105b156111b2576040516362b439dd60e11b815260040160405180910390fd5b6111bc8282610e01565b6103e881610120015111156111e45760405163cd4e616760e01b815260040160405180910390fd5b6111f18160800151610955565b60008281526008602052604090205460ff161561122157604051633d9c5bb760e11b815260040160405180910390fd5b61124e81602001518261010001516001600160a01b03919091166000908152600460205260409020541490565b610a8557604051633ab3447f60e11b815260040160405180910390fd5b8183148061127f575082158061127f575081155b1561129d576040516307ed98ed60e31b815260040160405180910390fd5b6000838152600560205260409020541515806112c6575060008281526005602052604090205415155b156112e457604051630ea075bf60e21b815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b6000806113a184611905565b8051906020012090506000856040516020016113cc91906001600160a01b0391909116815260200190565b6040516020818303038152906040528051906020012090506113ef84838361196b565b9695505050505050565b60208101516001600160a01b03163314611426576040516330cd747160e01b815260040160405180910390fd5b600061143182610e3d565b600081815260086020526040902080549192509060ff161561146657604051633d9c5bb760e11b815260040160405180910390fd5b805460ff1916600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b818110156114f3576114eb8582815181106114c3576114c36133cd565b60200260200101518583815181106114dd576114dd6133cd565b6020026020010151856117f2565b6001016114a6565b5050505050565b60008082600281111561150f5761150f613311565b0361152757611520868686866119aa565b9050610cee565b600282600281111561153b5761153b613311565b0361154c57611520868686866119de565b61152086868686611a18565b600061082a611565611a3f565b8360405161190160f01b6020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080806115b58885611b66565b9250925092506000806115c78a611bb6565b915091506115db8a60200151308489611bed565b6115e68a8a89611c17565b6115f08582611c69565b6101208b015190955060009061163290828d6101400151600181111561161857611618613311565b146116235788611625565b875b89898f6101400151611c98565b905061164f308c6020015184848a61164a9190613411565b611bed565b61165b30338484611d88565b60208b810151604080518681529283018590528201899052606082018790526080820183905230916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c0151604080518681529283018590528201899052606082018890526001600160a01b03169085907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a3600061172184611de4565b9050801561173957611739308d602001518684611bed565b505050505050505050505050565b6040516bffffffffffffffffffffffff19606085901b16602082015260009061178c908390859060340160405160208183030381529060405280519060200120611ec8565b90505b9392505050565b6006546040516001600160a01b038084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680546001600160a01b0319166001600160a01b0392909216919091179055565b81600080806118018785611b66565b925092509250600061185e8861012001516000600181111561182557611825613311565b8a6101400151600181111561183c5761183c613311565b146118475786611849565b855b8a60a001518b60c001518c6101400151611c98565b905060008061186c8a611bb6565b91509150611886338b6020015183868a61164a9190613411565b6118968a6020015189848a611bed565b60208a810151604080518581529283018490528201899052606082018790526080820185905233916001600160a01b039091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b6060604051806101a0016040528061017181526020016135b86101719139604080516001600160a01b03851660208201520160408051601f19818403018152908290526119559291602001613454565b6040516020818303038152906040529050919050565b60008060ff60f81b8584866040516020016119899493929190613483565b60408051808303601f19018152919052805160209091012095945050505050565b6000836001600160a01b0316856001600160a01b03161480156119d357506119d3858484611f1d565b90505b949350505050565b60006119eb858484611f1d565b80156119d35750836001600160a01b0316611a0586610c46565b6001600160a01b03161495945050505050565b6000611a25858484611f1d565b80156119d35750836001600160a01b0316611a0586610f6c565b6000306001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016148015611a9857507f000000000000000000000000000000000000000000000000000000000000000046145b15611ac257507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b6000806000611b788560600151611f45565b611b8185610e3d565b9050611b8d818661117d565b611ba0848660a001518760c00151611f84565b9250611bad818686611fab565b91509250925092565b600080808361014001516001811115611bd157611bd1613311565b03611be157505060800151600091565b50506080015190600090565b81600003611c0557611c00848483612021565b611c11565b611c1184848484612069565b50505050565b815160005b818110156114f357611c6185858381518110611c3a57611c3a6133cd565b6020026020010151858481518110611c5457611c546133cd565b6020026020010151612096565b600101611c1c565b600080611c7583611de4565b90508381101561178f576040516301be9b0160e71b815260040160405180910390fd5b60008515610cee576000611cad85858561217c565b9050600081118015611cc75750670de0b6b3a76400008111155b15611d7e576000836001811115611ce057611ce0613311565b03611d3257611cf1612710826134bc565b86611d0d83611d0881670de0b6b3a7640000613411565b6121eb565b611d17908a6134bc565b611d2191906134bc565b611d2b91906134db565b9150611d7e565b611d46670de0b6b3a76400006127106134bc565b86611d5d83611d0881670de0b6b3a7640000613411565b611d67908a6134bc565b611d7191906134bc565b611d7b91906134db565b91505b5095945050505050565b8015611c1157611d9a84848484611bed565b60408051838152602081018390526001600160a01b038516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b600081600003611e77576040516370a0823160e01b81523060048201526001600160a01b037f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa158015611e53573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061082a91906134fd565b604051627eeac760e11b8152306004820152602481018390526001600160a01b037f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e90604401611e36565b600080611ed58585612201565b805190602001209050600060ff60f81b868584604051602001611efb9493929190613483565b60408051808303601f1901815291905280516020909101209695505050505050565b6000836001600160a01b0316611f338484612318565b6001600160a01b031614949350505050565b6001600160a01b03811615801590611f6657506001600160a01b0381163314155b1561098357604051635211a07960e01b815260040160405180910390fd5b600082600003611f965750600061178f565b82611fa183866134bc565b61178c91906134db565b60008381526008602052604090206001810154908115611fcb5781611fd1565b8360a001515b915081831115611ff457604051637166356b60e11b815260040160405180910390fd5b611ffe8383613411565b91508160000361201457805460ff191660011781555b6001018190559392505050565b7f0000000000000000000000000000000000000000000000000000000000000000306001600160a01b0385160361205d57611c0081848461233c565b611c1181858585612347565b611c117f000000000000000000000000000000000000000000000000000000000000000085858585612353565b60006120a284846123d9565b90506120af848483612475565b81600080806120be8785611b66565b92509250925060006120e28861012001516000600181111561182557611825613311565b90506000806120f08a611bb6565b9150915061210787878c6020015185858d896124ef565b6020808c01518b8201516040805186815293840185905283018a905260608301889052608083018690526001600160a01b039182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a45050505050505050505050565b60008082600181111561219157612191613311565b036121c957826000036121a55760006121c2565b826121b8670de0b6b3a7640000866134bc565b6121c291906134db565b905061178f565b836000036121d857600061178c565b83611fa1670de0b6b3a7640000856134bc565b60008183106121fa578161178f565b5090919050565b60408051600080825260208201909252606091906122229060448101613516565b60408051601f19818403018152918152602080830180516001600160e01b03166352e831dd60e01b1790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe6020820152600160601b8502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be6041820152600160601b840260608201526e5af43d82803e903d91602b57fd5bf360881b607482015280826040516020016122ff929190613454565b6040516020818303038152906040529250505092915050565b60008060006123278585612556565b915091506123348161259b565b509392505050565b610ac58383836126e5565b611c118484848461275d565b604051637921219560e11b81526001600160a01b0385811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b1580156123ba57600080fd5b505af11580156123ce573d6000803e3d6000fd5b505050505050505050565b60008083610140015160018111156123f3576123f3613311565b14801561241657506000826101400151600181111561241457612414613311565b145b156124235750600161082a565b6001836101400151600181111561243c5761243c613311565b14801561245f57506001826101400151600181111561245d5761245d613311565b145b1561246c5750600261082a565b50600092915050565b61247f83836127e0565b61249c57604051633fcd37a360e11b815260040160405180910390fd5b60008160028111156124b0576124b0613311565b036124dd578160800151836080015114610ac55760405163a0b9446560e01b815260040160405180910390fd5b610ac583608001518360800151610d8f565b6124fb8530868a611bed565b612508878786868661282a565b8561251284611de4565b1015612531576040516301be9b0160e71b815260040160405180910390fd5b61254130868561164a858b613411565b61254d30338584611d88565b50505050505050565b600080825160410361258c5760208301516040840151606085015160001a612580878285856128b2565b94509450505050612594565b506000905060025b9250929050565b60008160048111156125af576125af613311565b036125b75750565b60018160048111156125cb576125cb613311565b036126185760405162461bcd60e51b815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610d19565b600281600481111561262c5761262c613311565b036126795760405162461bcd60e51b815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610d19565b600381600481111561268d5761268d613311565b036109835760405162461bcd60e51b815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c604482015261756560f01b6064820152608401610d19565b600060405163a9059cbb60e01b8152836004820152826024820152602060006044836000895af13d15601f3d1160016000511416171691505080611c115760405162461bcd60e51b815260206004820152600f60248201526e1514905394d1915497d19052531151608a1b6044820152606401610d19565b60006040516323b872dd60e01b81528460048201528360248201528260448201526020600060648360008a5af13d15601f3d11600160005114161716915050806114f35760405162461bcd60e51b81526020600482015260146024820152731514905394d1915497d19493d357d1905253115160621b6044820152606401610d19565b60008260c00151600014806127f7575060c0820151155b156128045750600161082a565b61178f61281084612976565b61281984612976565b856101400151856101400151612990565b600081600281111561283e5761283e613311565b146114f357600181600281111561285757612857613311565b0361287d576000828152600560205260409020600101546128789085612a2a565b6114f3565b600281600281111561289157612891613311565b036114f3576000838152600560205260409020600101546128789086612b35565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156128e9575060009050600361296d565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa15801561293d573d6000803e3d6000fd5b5050604051601f1901519150506001600160a01b0381166129665760006001925092505061296d565b9150600090505b94509492505050565b600061082a8260a001518360c0015184610140015161217c565b6000808360018111156129a5576129a5613311565b036129e95760008260018111156129be576129be613311565b036129df57670de0b6b3a76400006129d685876133f9565b101590506119d6565b50828410156119d6565b60008260018111156129fd576129fd613311565b03612a0c5750838310156119d6565b670de0b6b3a7640000612a1f85876133f9565b111595945050505050565b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612a6057612a606133cd565b602002602001018181525050600281600181518110612a8157612a816133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b03166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040516001600160e01b031960e084901b168152612b079190600090889087908990600401613549565b600060405180830381600087803b158015612b2157600080fd5b505af115801561254d573d6000803e3d6000fd5b604080516002808252606082018352600092602083019080368337019050509050600181600081518110612b6b57612b6b6133cd565b602002602001018181525050600281600181518110612b8c57612b8c6133cd565b60209081029190910101527f00000000000000000000000000000000000000000000000000000000000000006001600160a01b0316639e7212ad7f0000000000000000000000000000000000000000000000000000000000000000612add565b600060208284031215612bfe57600080fd5b81356001600160e01b03198116811461178f57600080fd5b6001600160a01b038116811461098357600080fd5b8035612c3681612c16565b919050565b60008060408385031215612c4e57600080fd5b8235612c5981612c16565b946020939093013593505050565b600060208284031215612c7957600080fd5b813561178f81612c16565b600060208284031215612c9657600080fd5b5035919050565b634e487b7160e01b600052604160045260246000fd5b6040516101a081016001600160401b0381118282101715612cd657612cd6612c9d565b60405290565b604051601f8201601f191681016001600160401b0381118282101715612d0457612d04612c9d565b604052919050565b803560028110612c3657600080fd5b803560038110612c3657600080fd5b600082601f830112612d3b57600080fd5b81356001600160401b03811115612d5457612d54612c9d565b612d67601f8201601f1916602001612cdc565b818152846020838601011115612d7c57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a08284031215612dac57600080fd5b612db4612cb3565b905081358152612dc660208301612c2b565b6020820152612dd760408301612c2b565b6040820152612de860608301612c2b565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e0820152610100808301358183015250610120808301358183015250610140612e3b818401612d0c565b90820152610160612e4d838201612d1b565b90820152610180828101356001600160401b03811115612e6c57600080fd5b612e7885828601612d2a565b82840152505092915050565b600060208284031215612e9657600080fd5b81356001600160401b03811115612eac57600080fd5b6119d684828501612d99565b600080600060608486031215612ecd57600080fd5b505081359360208301359350604090920135919050565b60006001600160401b03821115612efd57612efd612c9d565b5060051b60200190565b600082601f830112612f1857600080fd5b81356020612f2d612f2883612ee4565b612cdc565b82815260059290921b84018101918181019086841115612f4c57600080fd5b8286015b84811015612f675780358352918301918301612f50565b509695505050505050565b600080600080600060a08688031215612f8a57600080fd5b8535612f9581612c16565b94506020860135612fa581612c16565b935060408601356001600160401b0380821115612fc157600080fd5b612fcd89838a01612f07565b94506060880135915080821115612fe357600080fd5b612fef89838a01612f07565b9350608088013591508082111561300557600080fd5b5061301288828901612d2a565b9150509295509295909350565b600082601f83011261303057600080fd5b81356020613040612f2883612ee4565b82815260059290921b8401810191818101908684111561305f57600080fd5b8286015b84811015612f675780356001600160401b038111156130825760008081fd5b6130908986838b0101612d99565b845250918301918301613063565b600080604083850312156130b157600080fd5b82356001600160401b03808211156130c857600080fd5b6130d48683870161301f565b935060208501359150808211156130ea57600080fd5b506130f785828601612f07565b9150509250929050565b6000806040838503121561311457600080fd5b50508035926020909101359150565b6000806040838503121561313657600080fd5b8235915060208301356001600160401b0381111561315357600080fd5b6130f785828601612d99565b6000806000806080858703121561317557600080fd5b84356001600160401b038082111561318c57600080fd5b61319888838901612d99565b955060208701359150808211156131ae57600080fd5b6131ba8883890161301f565b94506040870135935060608701359150808211156131d757600080fd5b506131e487828801612f07565b91505092959194509250565b600080600080600060a0868803121561320857600080fd5b853561321381612c16565b9450602086013561322381612c16565b9350604086013592506060860135915060808601356001600160401b0381111561324c57600080fd5b61301288828901612d2a565b60006020828403121561326a57600080fd5b81356001600160401b0381111561328057600080fd5b6119d68482850161301f565b6000806040838503121561329f57600080fd5b82356001600160401b038111156132b557600080fd5b6132c185828601612d99565b95602094909401359450505050565b6000602082840312156132e257600080fd5b815161178f81612c16565b6020808252600a90820152695245454e5452414e435960b01b604082015260600190565b634e487b7160e01b600052602160045260246000fd5b6003811061333757613337613311565b9052565b8d8152602081018d90526001600160a01b038c811660408301528b811660608301528a16608082015260a0810189905260c0810188905260e081018790526101008101869052610120810185905261014081018490526101a08101600284106133a6576133a6613311565b836101608301526133bb610180830184613327565b9e9d5050505050505050505050505050565b634e487b7160e01b600052603260045260246000fd5b634e487b7160e01b600052601160045260246000fd5b6000821982111561340c5761340c6133e3565b500190565b600082821015613423576134236133e3565b500390565b60005b8381101561344357818101518382015260200161342b565b83811115611c115750506000910152565b60008351613466818460208801613428565b83519083019061347a818360208801613428565b01949350505050565b6001600160f81b031994909416845260609290921b6bffffffffffffffffffffffff191660018401526015830152603582015260550190565b60008160001904831182151516156134d6576134d66133e3565b500290565b6000826134f857634e487b7160e01b600052601260045260246000fd5b500490565b60006020828403121561350f57600080fd5b5051919050565b6020815260008251806020840152613535816040850160208701613428565b601f01601f19169190910160400192915050565b6001600160a01b038616815260208082018690526040820185905260a06060830181905284519083018190526000918581019160c0850190845b8181101561359f57845183529383019391830191600101613583565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a2646970667358221220d93139e32bae530b273044d07d00326d19debeb5b49b08f172b04a7bc677797964736f6c634300080f00330000000000000000000000002e8dcfe708d44ae2e406a1c02dfe2fa13012f9610000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x9aaa83506c9b5ddb21f0eebb9c4f328e821c04aacef8b01a0d61c275c16ec50b",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x1107e",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xdc92ff12528e1297d512077ef17014d7d11530801898a96c59bde64a959b5c0c",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x110f1",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0x59508bd1f8541283b91f60a9f719223f95475648bc24139a53f37fcb667a7fad",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x7d3c",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": "0xc923d61fbb2d7351d26e7045cb468ec6e0529d9aa7a8622668762bb4b0334b73",
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x09b39caad32c6c3999aa3f9248c6dfb01f7806d4",
        "to": "0xfffd6f0db1ec30a58884b23546b4f1bb333f818f",
        "gas": "0x84d2",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [
    {
      "transactionHash": "0x999e01a7d4c213c0fa844f13f0c4d951d4337c1f33391b86b38f1fe55f74969d",
      "transactionIndex": "0x3",
      "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
      "blockNumber": "0x1af2ce6",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": null,
      "cumulativeGasUsed": "0x3c19b7",
      "gasUsed": "0x31d71c",
      "contractAddress": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "logs": [
        {
          "address": "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
          "topics": [
            "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925",
            "0x000000000000000000000000fffd6f0db1ec30a58884b23546b4f1bb333f818f",
            "0x0000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43"
          ],
          "data": "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0x999e01a7d4c213c0fa844f13f0c4d951d4337c1f33391b86b38f1fe55f74969d",
          "transactionIndex": "0x3",
          "logIndex": "0x14",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000f903ba9e006193c1527bfbe65fe2123704ea3f99"
          ],
          "data": "0x0000000000000000000000000000000000000000000000000022d0228bbc4800000000000000000000000000000000000000000000000000058d15e176280000000000000000000000000000000000000000000000000732edd19007eaca5164000000000000000000000000000000000000000000000000056a45beea6bb800000000000000000000000000000000000000000000000732edf4602a76869964",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0x999e01a7d4c213c0fa844f13f0c4d951d4337c1f33391b86b38f1fe55f74969d",
          "transactionIndex": "0x3",
          "logIndex": "0x15",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00008000000000000000000000000000000000000000000000000010000000000000000000000060004000000000008080008000000000000000000000600000000000000000000000000000000000800000000000000000000100000000000000000000000020000000008000000000000000000000000080000000000000000000000000000000000001000000000000000000000000000000000000000000220000000000000000000000000000000000000000000000000000000000004000000000000000000041000400000000000000000000000000100000001000000010000000000000000000000000000000000000000000001000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e12"
    },
    {
      "transactionHash": "0x9aaa83506c9b5ddb21f0eebb9c4f328e821c04aacef8b01a0d61c275c16ec50b",
      "transactionIndex": "0x4",
      "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
      "blockNumber": "0x1af2ce6",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "cumulativeGasUsed": "0x3cd409",
      "gasUsed": "0xba52",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
          "topics": [
            "0xf9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc",
            "0x000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4"
          ],
          "data": "0x",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0x9aaa83506c9b5ddb21f0eebb9c4f328e821c04aacef8b01a0d61c275c16ec50b",
          "transactionIndex": "0x4",
          "logIndex": "0x16",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000f903ba9e006193c1527bfbe65fe2123704ea3f99"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000008224ab0a1c00000000000000000000000000000000000000000000000000056a45bee6ea9808000000000000000000000000000000000000000000000732edf4602a768699640000000000000000000000000000000000000000000000000569c39a3be07c08000000000000000000000000000000000000000000000732edf4e24f2190b564",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0x9aaa83506c9b5ddb21f0eebb9c4f328e821c04aacef8b01a0d61c275c16ec50b",
          "transactionIndex": "0x4",
          "logIndex": "0x17",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000010000000000000000000000020000000000000008000008000000000010000000000400000010000000000000000000000000000800000000000000000000100000000000000000080000000000000000000200000000000000000000080000000000000000000000000000000000000000001000000000000000000000000000000000000200000000000000000000000000000000000080000000000000000000000004000000000000028000041000000000000000000000000000000100000001000000000000000000000000000000000000100000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e12"
    },
    {
      "transactionHash": "0xdc92ff12528e1297d512077ef17014d7d11530801898a96c59bde64a959b5c0c",
      "transactionIndex": "0x5",
      "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
      "blockNumber": "0x1af2ce6",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "cumulativeGasUsed": "0x3d8eaa",
      "gasUsed": "0xbaa1",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
          "topics": [
            "0xf1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c",
            "0x000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4"
          ],
          "data": "0x",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0xdc92ff12528e1297d512077ef17014d7d11530801898a96c59bde64a959b5c0c",
          "transactionIndex": "0x5",
          "logIndex": "0x18",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000f903ba9e006193c1527bfbe65fe2123704ea3f99"
          ],
          "data": "0x0000000000000000000000000000000000000000000000000000825bd9571e000000000000000000000000000000000000000000000000000569c39a3bd36244000000000000000000000000000000000000000000000732edf4e24f2190b5640000000000000000000000000000000000000000000000000569413e627c4444000000000000000000000000000000000000000000000732edf564aafae7d364",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0xdc92ff12528e1297d512077ef17014d7d11530801898a96c59bde64a959b5c0c",
          "transactionIndex": "0x5",
          "logIndex": "0x19",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000010000000000000000000000020000000000000008000008000000000010000000000400000010000000000000000000000000000800000000000000000000100000000000000000080000000000000000000000000000000000000000080000000000000000000000000000000000000000001000000000000000000000000000000002000200000000000000000100000000000000000000000000000000000000000004000000000000028000041000000000000000000000000000000100000001000000000000000000000000000000000000000800000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e12"
    },
    {
      "transactionHash": "0x59508bd1f8541283b91f60a9f719223f95475648bc24139a53f37fcb667a7fad",
      "transactionIndex": "0x6",
      "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
      "blockNumber": "0x1af2ce6",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "cumulativeGasUsed": "0x3de956",
      "gasUsed": "0x5aac",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
          "topics": [
            "0x787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4"
          ],
          "data": "0x",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0x59508bd1f8541283b91f60a9f719223f95475648bc24139a53f37fcb667a7fad",
          "transactionIndex": "0x6",
          "logIndex": "0x1a",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000f903ba9e006193c1527bfbe65fe2123704ea3f99"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000003f55650b28000000000000000000000000000000000000000000000000000569413e626f24f2000000000000000000000000000000000000000000000732edf564aafae7d364000000000000000000000000000000000000000000000000056901e8fd63fcf2000000000000000000000000000000000000000000000732edf5a4005ff2fb64",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0x59508bd1f8541283b91f60a9f719223f95475648bc24139a53f37fcb667a7fad",
          "transactionIndex": "0x6",
          "logIndex": "0x1b",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000002000000000000000000000010000000000000000000000020000000000000008000008000000000010000000000400000000000000000000000000000000000800000000000000000000100000000000000020080000000000000000000000000000000000000002080000000000000000000000000000000000000000001000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000004000000000000000000041000000000000000000000000000000100000001000000000000000000000000000000000000000000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e12"
    },
    {
      "transactionHash": "0xc923d61fbb2d7351d26e7045cb468ec6e0529d9aa7a8622668762bb4b0334b73",
      "transactionIndex": "0x7",
      "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
      "blockNumber": "0x1af2ce6",
      "from": "0x09b39caAd32c6C3999aA3f9248C6dfb01f7806d4",
      "to": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
      "cumulativeGasUsed": "0x3e4428",
      "gasUsed": "0x5ad2",
      "contractAddress": null,
      "logs": [
        {
          "address": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f",
          "topics": [
            "0xf7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4"
          ],
          "data": "0x",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0xc923d61fbb2d7351d26e7045cb468ec6e0529d9aa7a8622668762bb4b0334b73",
          "transactionIndex": "0x7",
          "logIndex": "0x1c",
          "removed": false
        },
        {
          "address": "0x0000000000000000000000000000000000001010",
          "topics": [
            "0x4dfe1bbbcf077ddc3e01291eea2d5c70c2b422b415d95645b9adcfd678cb1d63",
            "0x0000000000000000000000000000000000000000000000000000000000001010",
            "0x00000000000000000000000009b39caad32c6c3999aa3f9248c6dfb01f7806d4",
            "0x000000000000000000000000f903ba9e006193c1527bfbe65fe2123704ea3f99"
          ],
          "data": "0x00000000000000000000000000000000000000000000000000003f6feff91c00000000000000000000000000000000000000000000000000056901e8fd5d9cda000000000000000000000000000000000000000000000732edf5a4005ff2fb640000000000000000000000000000000000000000000000000568c2790d6480da000000000000000000000000000000000000000000000732edf5e3704fec1764",
          "blockHash": "0x5ba3fe70b82958f797ca9ecb042b5aab59dfa823e21f90afedc26de3d22d9d57",
          "blockNumber": "0x1af2ce6",
          "transactionHash": "0xc923d61fbb2d7351d26e7045cb468ec6e0529d9aa7a8622668762bb4b0334b73",
          "transactionIndex": "0x7",
          "logIndex": "0x1d",
          "removed": false
        }
      ],
      "status": "0x1",
      "logsBloom": "0x00000000000000000000000000000000000000000000000000000010000000000000000000000020000000000000008000008000000000010000000000400000000000000000000000000000000000800000000000000000000100000000000000000080000000000000100000000000000000000000000080000000000000000000000000000000000000000001000000000000000000000000000000000000200000000000000000000000000000000000000000000000000000000000004004000000000000800041000000000000000000000000000000100000001000000000000000000000000000000000000000000000000000000000000000100000",
      "type": "0x2",
      "effectiveGasPrice": "0xb2d05e12"
    }
  ],
  "libraries": [],
  "pending": [],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0xfffd6f0dB1ec30A58884B23546B4F1bB333f818f"
    }
  },
  "timestamp": 1663955831,
  "commit": "ec7c23f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/80001/deployExchange-1664228099.json
================================================
{
  "transactions": [
    {
      "hash": null,
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": null,
      "arguments": [
        "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
        "0x7D8610E9567d2a6C9FBf66a5A13E9Ba8bb120d43",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "gas": "0x4d4c99",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b506040516200473f3803806200473f8339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e05161010051610120516101405161016051610180516143386200040760003960006108970152600081816104c801528181612698015281816129450152818161355201526136820152600081816105eb015281816125e3015281816128ed0152818161358e01526136be01526000612258015260006122a701526000612282015260006121db015260006122050152600061222f01526143386000f3fe608060405234801561001057600080fd5b50600436106103365760003560e01c806370480275116101b2578063d798eff6116100f9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610892578063fa950b48146108b9578063fbddd751146108cc578063fe729aaf146108df57600080fd5b8063e60f0c0514610834578063edef7d8e14610847578063f23a6e611461085a57600080fd5b8063e03ac3d0116100d3578063e03ac3d014610806578063e2eec4051461080e578063e50e4f971461082157600080fd5b8063d798eff6146107bd578063d7fb272f146107d0578063d82da838146107f357600080fd5b8063a287bdf11161015b578063b28c51c011610135578063b28c51c01461073b578063bc197c8114610759578063c10f1a751461079d57600080fd5b8063a287bdf114610702578063a6dfcf8614610715578063ac8a584a1461072857600080fd5b806383b8a5ae1161018c57806383b8a5ae146106d45780639870d7fe146106dc578063a10f3dce146106ef57600080fd5b8063704802751461068357806375d7370a146106965780637ecebe00146106b457600080fd5b8063429b62e5116102815780635893253c1161022a578063627cdcb911610204578063627cdcb91461061c578063654f0ce41461062457806368c7450f146106375780636d70f7ae1461064a57600080fd5b80635893253c146105ad5780635c1548fb146105e95780635c975abb1461060f57600080fd5b8063456068d21161025b578063456068d21461052f57806346423aa7146105375780634a2a11f5146105a557600080fd5b8063429b62e5146104f457806344bea37e146105145780634544f0551461051c57600080fd5b80631785f53c116102e357806334600901116102bd57806334600901146104b35780633b521d78146104c65780633d6d3598146104ec57600080fd5b80631785f53c1461042257806324d7806c146104355780632dff692d1461046f57600080fd5b80631031e36e116103145780631031e36e146103ca578063131e7e1c146103d457806313e7c9d8146103f457600080fd5b806301ffc9a71461033b5780630647ee201461036357806306b9d6911461039d575b600080fd5b61034e6103493660046136e2565b6108f2565b60405190151581526020015b60405180910390f35b61034e610371366004613756565b73ffffffffffffffffffffffffffffffffffffffff919091166000908152600460205260409020541490565b6103a561098b565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200161035a565b6103d2610a24565b005b6007546103a59073ffffffffffffffffffffffffffffffffffffffff1681565b610414610402366004613782565b60026020526000908152604090205481565b60405190815260200161035a565b6103d2610430366004613782565b610a78565b61034e610443366004613782565b73ffffffffffffffffffffffffffffffffffffffff166000908152600160208190526040909120541490565b61049c61047d36600461379f565b6008602052600090815260409020805460019091015460ff9091169082565b60408051921515835260208301919091520161035a565b6103d26104c136600461379f565b610b15565b7f00000000000000000000000000000000000000000000000000000000000000006103a5565b6103d2610b5f565b610414610502366004613782565b60016020526000908152604090205481565b610414600081565b6103d261052a366004613782565b610be3565b6103d2610c36565b61058861054536600461379f565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b60408051825115158152602092830151928101929092520161035a565b6103e8610414565b6105d46105bb36600461379f565b6005602052600090815260409020805460019091015482565b6040805192835260208301919091520161035a565b7f00000000000000000000000000000000000000000000000000000000000000006103a5565b60035461034e9060ff1681565b6103d2610c88565b6103d26106323660046139f8565b610c92565b6103d2610645366004613a2d565b610cad565b61034e610658366004613782565b73ffffffffffffffffffffffffffffffffffffffff1660009081526002602052604090205460011490565b6103d2610691366004613782565b610d07565b60075473ffffffffffffffffffffffffffffffffffffffff166103a5565b6104146106c2366004613782565b60046020526000908152604090205481565b6103d2610da7565b6103d26106ea366004613782565b610e2c565b6104146106fd36600461379f565b610eca565b6103a5610710366004613782565b610ee8565b6103d26107233660046139f8565b610f14565b6103d2610736366004613782565b610f1d565b60065473ffffffffffffffffffffffffffffffffffffffff166103a5565b61076c610767366004613ae8565b610fba565b6040517fffffffff00000000000000000000000000000000000000000000000000000000909116815260200161035a565b6006546103a59073ffffffffffffffffffffffffffffffffffffffff1681565b6103d26107cb366004613c16565b610fe5565b6104146107de36600461379f565b60009081526005602052604090206001015490565b6103d2610801366004613c7a565b6110f5565b6103a5611136565b6103d261081c366004613c9c565b6111a6565b61041461082f3660046139f8565b6111fb565b6103d2610842366004613cd9565b611298565b6103a5610855366004613782565b6113a6565b61076c610868366004613d6b565b7ff23a6e610000000000000000000000000000000000000000000000000000000095945050505050565b6104147f000000000000000000000000000000000000000000000000000000000000000081565b6103d26108c7366004613dd4565b6113d2565b6103d26108da366004613782565b611409565b6103d26108ed366004613e09565b61145c565b60007fffffffff0000000000000000000000000000000000000000000000000000000082167f4e2312e000000000000000000000000000000000000000000000000000000000148061098557507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b600654604080517faaf10f42000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163aaf10f429160048083019260209291908290030181865afa1580156109fb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a1f9190613e4e565b905090565b3360009081526001602081905260409091205414610a6e576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a7661155e565b565b3360009081526001602081905260409091205414610ac2576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610b5c576040517f3f6cc76800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50565b33600090815260026020526040902054600114610ba8576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610c2d576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610b5c816115b6565b3360009081526001602081905260409091205414610c80576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a76611644565b610a766001611699565b6000610c9d826111fb565b9050610ca981836116c7565b5050565b3360009081526001602081905260409091205414610cf7576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d02838383611826565b505050565b3360009081526001602081905260409091205414610d51576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610df1576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610e76576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610ed582610b15565b5060009081526005602052604090205490565b600061098582610ef6611136565b60075473ffffffffffffffffffffffffffffffffffffffff16611982565b610b5c81611a80565b3360009081526001602081905260409091205414610f67576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b7fbc197c81000000000000000000000000000000000000000000000000000000005b95945050505050565b600054600203611056576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e43590000000000000000000000000000000000000000000060448201526064015b60405180910390fd5b6002600081815533815260209190915260409020546001146110a4576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff16156110e1576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110ec828233611b85565b50506001600055565b806110ff83610eca565b14610ca9576040517f66f8620a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600754604080517fa619486e000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163a619486e9160048083019260209291908290030181865afa1580156109fb573d6000803e3d6000fd5b6111c58160400151826020015184846101800151856101600151611bde565b610ca9576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006109857fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e610160015160405160200161127d9d9c9b9a99989796959493929190613eae565b60405160208183030381529060405280519060200120611c3c565b600054600203611304576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e435900000000000000000000000000000000000000000000604482015260640161104d565b600260008181553381526020919091526040902054600114611352576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff161561138f576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61139b84848484611ca5565b505060016000555050565b6000610985826113b461098b565b60065473ffffffffffffffffffffffffffffffffffffffff16611e5c565b805160005b81811015610d02576114018382815181106113f4576113f4613f4c565b6020026020010151611a80565b6001016113d7565b3360009081526001602081905260409091205414611453576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610b5c81611ebe565b6000546002036114c8576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e435900000000000000000000000000000000000000000000604482015260640161104d565b600260008181553381526020919091526040902054600114611516576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff1615611553576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110ec828233611f4c565b600380547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b60075460405173ffffffffffffffffffffffffffffffffffffffff8084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b600380547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b336000908152600460205260409020546116b4908290613faa565b3360009081526004602052604090205550565b60008160e001511180156116de5750428160e00151105b15611715576040517fc56873ba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61171f82826111a6565b6103e88161012001511115611760576040517fcd4e616700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61176d8160800151610b15565b60008281526008602052604090205460ff16156117b6576040517f7b38b76e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117f0816020015182610100015173ffffffffffffffffffffffffffffffffffffffff919091166000908152600460205260409020541490565b610ca9576040517f756688fe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8183148061183a575082158061183a575081155b15611871576040517f3f6cc76800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008381526005602052604090205415158061189a575060008281526005602052604090205415155b156118d1576040517f3a81d6fc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b60008061198e8461205a565b8051906020012090506000856040516020016119c6919073ffffffffffffffffffffffffffffffffffffffff91909116815260200190565b604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815282825280516020918201207fff000000000000000000000000000000000000000000000000000000000000008285015260609790971b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000166021840152603583019690965260558083019490945280518083039094018452607590910190525080519201919091209392505050565b602081015173ffffffffffffffffffffffffffffffffffffffff163314611ad3576040517f30cd747100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611ade826111fb565b600081815260086020526040902080549192509060ff1615611b2c576040517f7b38b76e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b81811015611bd757611bcf858281518110611ba757611ba7613f4c565b6020026020010151858381518110611bc157611bc1613f4c565b602002602001015185611f4c565b600101611b8a565b5050505050565b600080826002811115611bf357611bf3613e6b565b03611c0b57611c04868686866120eb565b9050610fdc565b6002826002811115611c1f57611c1f613e6b565b03611c3057611c0486868686612139565b611c048686868661218d565b6000610985611c496121c1565b836040517f19010000000000000000000000000000000000000000000000000000000000006020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080611cb387846122f5565b91509150600080611cc389612342565b91509150611cd78960200151308488612379565b611ce28989886123a3565b611cec84826123f5565b6101208a0151909450600090611d2e90828c61014001516001811115611d1457611d14613e6b565b14611d1f5787611d21565b865b88888e610140015161243d565b9050611d4b308b60200151848489611d469190613fc2565b612379565b611d573033848461252d565b6000611d6284612596565b90508015611d7a57611d7a308c602001518684612379565b60208b8101516040805187815292830186905282018990526060820188905260808201849052309173ffffffffffffffffffffffffffffffffffffffff9091169087907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c01516040805187815292830186905282018990526060820188905273ffffffffffffffffffffffffffffffffffffffff169086907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a35050505050505050505050565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606085901b166020820152600090611eb49083908590603401604051602081830303815290604052805190602001206126c6565b90505b9392505050565b60065460405173ffffffffffffffffffffffffffffffffffffffff8084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b81600080611f5a86846122f5565b6101208801519193509150600090611fa790825b8961014001516001811115611f8557611f85613e6b565b14611f905785611f92565b845b8960a001518a60c001518b610140015161243d565b9050600080611fb589612342565b91509150611fcf338a60200151838689611d469190613fc2565b611fdf8960200151888489612379565b6020898101516040805185815292830184905282018890526060820187905260808201859052339173ffffffffffffffffffffffffffffffffffffffff9091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a4505050505050505050565b6060604051806101a00160405280610171815260200161419261017191396040805173ffffffffffffffffffffffffffffffffffffffff8516602082015201604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526120d59291602001614005565b6040516020818303038152906040529050919050565b60008373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff1614801561212e575061212e858484612763565b90505b949350505050565b6000612146858484612763565b801561212e57508373ffffffffffffffffffffffffffffffffffffffff1661216d86610ee8565b73ffffffffffffffffffffffffffffffffffffffff161495945050505050565b600061219a858484612763565b801561212e57508373ffffffffffffffffffffffffffffffffffffffff1661216d866113a6565b60003073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614801561222757507f000000000000000000000000000000000000000000000000000000000000000046145b1561225157507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b60008061230584606001516127a5565b61230e846111fb565b905061231a81856116c7565b61232d838560a001518660c00151612817565b915061233a81858561283e565b509250929050565b60008080836101400151600181111561235d5761235d613e6b565b0361236d57505060800151600091565b50506080015190600090565b816000036123915761238c8484836128eb565b61239d565b61239d84848484612940565b50505050565b815160005b81811015611bd7576123ed858583815181106123c6576123c6613f4c565b60200260200101518584815181106123e0576123e0613f4c565b602002602001015161296d565b6001016123a8565b60008061240183612596565b905083811015611eb7576040517fdf4d808000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008515610fdc576000612452858585612a52565b905060008111801561246c5750670de0b6b3a76400008111155b1561252357600083600181111561248557612485613e6b565b036124d75761249661271082614034565b866124b2836124ad81670de0b6b3a7640000613fc2565b612ac1565b6124bc908a614034565b6124c69190614034565b6124d09190614071565b9150612523565b6124eb670de0b6b3a7640000612710614034565b86612502836124ad81670de0b6b3a7640000613fc2565b61250c908a614034565b6125169190614034565b6125209190614071565b91505b5095945050505050565b801561239d5761253f84848484612379565b604080518381526020810183905273ffffffffffffffffffffffffffffffffffffffff8516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b60008160000361264f576040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa15801561262b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061098591906140ac565b6040517efdd58e0000000000000000000000000000000000000000000000000000000081523060048201526024810183905273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e9060440161260e565b6000806126d38585612ad7565b8051602091820120604080517fff000000000000000000000000000000000000000000000000000000000000008185015260609890981b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000166021890152603588019590955260558088019190915284518088039091018152607590960190935250508251920191909120919050565b60008373ffffffffffffffffffffffffffffffffffffffff166127868484612c5a565b73ffffffffffffffffffffffffffffffffffffffff1614949350505050565b73ffffffffffffffffffffffffffffffffffffffff8116158015906127e0575073ffffffffffffffffffffffffffffffffffffffff81163314155b15610b5c576040517f5211a07900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008260000361282957506000611eb7565b826128348386614034565b611eb49190614071565b6000838152600860205260409020600181015490811561285e5781612864565b8360a001515b9150818311156128a0576040517fe2cc6ad600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6128aa8383613fc2565b9150816000036128de5780547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011781555b6001018190559392505050565b7f00000000000000000000000000000000000000000000000000000000000000003073ffffffffffffffffffffffffffffffffffffffff8516036129345761238c818484612c7e565b61239d81858585612c89565b61239d7f000000000000000000000000000000000000000000000000000000000000000085858585612c95565b60006129798484612d41565b9050612986848483612ddd565b8160008061299486846122f5565b61012088015191935091506000906129ac9082611f6e565b90506000806129ba89612342565b915091506129d186868b6020015185858c89612e89565b6020808b01518a820151604080518681529384018590528301899052606083018890526080830186905273ffffffffffffffffffffffffffffffffffffffff9182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b600080826001811115612a6757612a67613e6b565b03612a9f5782600003612a7b576000612a98565b82612a8e670de0b6b3a764000086614034565b612a989190614071565b9050611eb7565b83600003612aae576000611eb4565b83612834670de0b6b3a764000085614034565b6000818310612ad05781611eb7565b5090919050565b6040805160008082526020820190925260609190612af890604481016140c5565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152918152602080830180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f52e831dd000000000000000000000000000000000000000000000000000000001790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe60208201526c010000000000000000000000008502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be60418201526c01000000000000000000000000840260608201527f5af43d82803e903d91602b57fd5bf3000000000000000000000000000000000060748201528082604051602001612c41929190614005565b6040516020818303038152906040529250505092915050565b6000806000612c698585612f09565b91509150612c7681612f4e565b509392505050565b610d02838383613101565b61239d848484846131ba565b6040517ff242432a00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b158015612d2257600080fd5b505af1158015612d36573d6000803e3d6000fd5b505050505050505050565b6000808361014001516001811115612d5b57612d5b613e6b565b148015612d7e575060008261014001516001811115612d7c57612d7c613e6b565b145b15612d8b57506001610985565b60018361014001516001811115612da457612da4613e6b565b148015612dc7575060018261014001516001811115612dc557612dc5613e6b565b145b15612dd457506002610985565b50600092915050565b612de78383613279565b612e1d576040517f7f9a6f4600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000816002811115612e3157612e31613e6b565b03612e77578160800151836080015114610d02576040517fa0b9446500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d02836080015183608001516110f5565b612e958530868a612379565b612ea287878686866132c3565b85612eac84612596565b1015612ee4576040517fdf4d808000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612ef4308685611d46858b613fc2565b612f003033858461252d565b50505050505050565b6000808251604103612f3f5760208301516040840151606085015160001a612f338782858561334b565b94509450505050612f47565b506000905060025b9250929050565b6000816004811115612f6257612f62613e6b565b03612f6a5750565b6001816004811115612f7e57612f7e613e6b565b03612fe5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f45434453413a20696e76616c6964207369676e61747572650000000000000000604482015260640161104d565b6002816004811115612ff957612ff9613e6b565b03613060576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e67746800604482015260640161104d565b600381600481111561307457613074613e6b565b03610b5c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c60448201527f7565000000000000000000000000000000000000000000000000000000000000606482015260840161104d565b60006040517fa9059cbb000000000000000000000000000000000000000000000000000000008152836004820152826024820152602060006044836000895af13d15601f3d116001600051141617169150508061239d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600f60248201527f5452414e534645525f4641494c45440000000000000000000000000000000000604482015260640161104d565b60006040517f23b872dd0000000000000000000000000000000000000000000000000000000081528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080611bd7576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f5452414e534645525f46524f4d5f4641494c4544000000000000000000000000604482015260640161104d565b60008260c0015160001480613290575060c0820151155b1561329d57506001610985565b611eb76132a98461343a565b6132b28461343a565b856101400151856101400151613454565b60008160028111156132d7576132d7613e6b565b14611bd75760018160028111156132f0576132f0613e6b565b036133165760008281526005602052604090206001015461331190856134ee565b611bd7565b600281600281111561332a5761332a613e6b565b03611bd757600083815260056020526040902060010154613311908661361e565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156133825750600090506003613431565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156133d6573d6000803e3d6000fd5b50506040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0015191505073ffffffffffffffffffffffffffffffffffffffff811661342a57600060019250925050613431565b9150600090505b94509492505050565b60006109858260a001518360c00151846101400151612a52565b60008083600181111561346957613469613e6b565b036134ad57600082600181111561348257613482613e6b565b036134a357670de0b6b3a764000061349a8587613faa565b10159050612131565b5082841015612131565b60008260018111156134c1576134c1613e6b565b036134d0575083831015612131565b670de0b6b3a76400006134e38587613faa565b111595945050505050565b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061352457613524613f4c565b60200260200101818152505060028160018151811061354557613545613f4c565b60209081029190910101527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526135f09190600090889087908990600401614116565b600060405180830381600087803b15801561360a57600080fd5b505af1158015612f00573d6000803e3d6000fd5b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061365457613654613f4c565b60200260200101818152505060028160018151811061367557613675613f4c565b60209081029190910101527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16639e7212ad7f00000000000000000000000000000000000000000000000000000000000000006135ae565b6000602082840312156136f457600080fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611eb757600080fd5b73ffffffffffffffffffffffffffffffffffffffff81168114610b5c57600080fd5b803561375181613724565b919050565b6000806040838503121561376957600080fd5b823561377481613724565b946020939093013593505050565b60006020828403121561379457600080fd5b8135611eb781613724565b6000602082840312156137b157600080fd5b5035919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040516101a0810167ffffffffffffffff8111828210171561380b5761380b6137b8565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715613858576138586137b8565b604052919050565b80356002811061375157600080fd5b80356003811061375157600080fd5b600082601f83011261388f57600080fd5b813567ffffffffffffffff8111156138a9576138a96137b8565b6138da60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601613811565b8181528460208386010111156138ef57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a0828403121561391f57600080fd5b6139276137e7565b90508135815261393960208301613746565b602082015261394a60408301613746565b604082015261395b60608301613746565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e08201526101008083013581830152506101208083013581830152506101406139ae818401613860565b908201526101606139c083820161386f565b908201526101808281013567ffffffffffffffff8111156139e057600080fd5b6139ec8582860161387e565b82840152505092915050565b600060208284031215613a0a57600080fd5b813567ffffffffffffffff811115613a2157600080fd5b6121318482850161390c565b600080600060608486031215613a4257600080fd5b505081359360208301359350604090920135919050565b600067ffffffffffffffff821115613a7357613a736137b8565b5060051b60200190565b600082601f830112613a8e57600080fd5b81356020613aa3613a9e83613a59565b613811565b82815260059290921b84018101918181019086841115613ac257600080fd5b8286015b84811015613add5780358352918301918301613ac6565b509695505050505050565b600080600080600060a08688031215613b0057600080fd5b8535613b0b81613724565b94506020860135613b1b81613724565b9350604086013567ffffffffffffffff80821115613b3857600080fd5b613b4489838a01613a7d565b94506060880135915080821115613b5a57600080fd5b613b6689838a01613a7d565b93506080880135915080821115613b7c57600080fd5b50613b898882890161387e565b9150509295509295909350565b600082601f830112613ba757600080fd5b81356020613bb7613a9e83613a59565b82815260059290921b84018101918181019086841115613bd657600080fd5b8286015b84811015613add57803567ffffffffffffffff811115613bfa5760008081fd5b613c088986838b010161390c565b845250918301918301613bda565b60008060408385031215613c2957600080fd5b823567ffffffffffffffff80821115613c4157600080fd5b613c4d86838701613b96565b93506020850135915080821115613c6357600080fd5b50613c7085828601613a7d565b9150509250929050565b60008060408385031215613c8d57600080fd5b50508035926020909101359150565b60008060408385031215613caf57600080fd5b82359150602083013567ffffffffffffffff811115613ccd57600080fd5b613c708582860161390c565b60008060008060808587031215613cef57600080fd5b843567ffffffffffffffff80821115613d0757600080fd5b613d138883890161390c565b95506020870135915080821115613d2957600080fd5b613d3588838901613b96565b9450604087013593506060870135915080821115613d5257600080fd5b50613d5f87828801613a7d565b91505092959194509250565b600080600080600060a08688031215613d8357600080fd5b8535613d8e81613724565b94506020860135613d9e81613724565b93506040860135925060608601359150608086013567ffffffffffffffff811115613dc857600080fd5b613b898882890161387e565b600060208284031215613de657600080fd5b813567ffffffffffffffff811115613dfd57600080fd5b61213184828501613b96565b60008060408385031215613e1c57600080fd5b823567ffffffffffffffff811115613e3357600080fd5b613e3f8582860161390c565b95602094909401359450505050565b600060208284031215613e6057600080fd5b8151611eb781613724565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b60038110613eaa57613eaa613e6b565b9052565b60006101a0820190508e82528d602083015273ffffffffffffffffffffffffffffffffffffffff808e166040840152808d166060840152808c166080840152508960a08301528860c08301528760e083015286610100830152856101208301528461014083015260028410613f2557613f25613e6b565b83610160830152613f3a610180830184613e9a565b9e9d5050505050505050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60008219821115613fbd57613fbd613f7b565b500190565b600082821015613fd457613fd4613f7b565b500390565b60005b83811015613ff4578181015183820152602001613fdc565b8381111561239d5750506000910152565b60008351614017818460208801613fd9565b83519083019061402b818360208801613fd9565b01949350505050565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff048311821515161561406c5761406c613f7b565b500290565b6000826140a7577f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b500490565b6000602082840312156140be57600080fd5b5051919050565b60208152600082518060208401526140e4816040850160208701613fd9565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169190910160400192915050565b600060a0820173ffffffffffffffffffffffffffffffffffffffff881683526020878185015286604085015260a0606085015281865180845260c086019150828801935060005b818110156141795784518352938301939183019160010161415d565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a264697066735822122056df26e165b5957191bd0ff149c07ae13f5a6b4252973fb3c07a4653cce0f3b164736f6c634300080f00330000000000000000000000002e8dcfe708d44ae2e406a1c02dfe2fa13012f9610000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x1107c",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x10169",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x7d00",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x7d34",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E"
    }
  },
  "timestamp": 1664228099,
  "commit": "af3ba7f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/80001/deployExchange-1664228139.json
================================================
{
  "transactions": [
    {
      "hash": "0x4cf5ff4362abb630398f45b0ed26787e7b2524c53c4cc006641764f5f8267609",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": null,
      "arguments": [
        "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
        "0x7D8610E9567d2a6C9FBf66a5A13E9Ba8bb120d43",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "gas": "0x4d4c99",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b506040516200473f3803806200473f8339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e05161010051610120516101405161016051610180516143386200040760003960006108970152600081816104c801528181612698015281816129450152818161355201526136820152600081816105eb015281816125e3015281816128ed0152818161358e01526136be01526000612258015260006122a701526000612282015260006121db015260006122050152600061222f01526143386000f3fe608060405234801561001057600080fd5b50600436106103365760003560e01c806370480275116101b2578063d798eff6116100f9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610892578063fa950b48146108b9578063fbddd751146108cc578063fe729aaf146108df57600080fd5b8063e60f0c0514610834578063edef7d8e14610847578063f23a6e611461085a57600080fd5b8063e03ac3d0116100d3578063e03ac3d014610806578063e2eec4051461080e578063e50e4f971461082157600080fd5b8063d798eff6146107bd578063d7fb272f146107d0578063d82da838146107f357600080fd5b8063a287bdf11161015b578063b28c51c011610135578063b28c51c01461073b578063bc197c8114610759578063c10f1a751461079d57600080fd5b8063a287bdf114610702578063a6dfcf8614610715578063ac8a584a1461072857600080fd5b806383b8a5ae1161018c57806383b8a5ae146106d45780639870d7fe146106dc578063a10f3dce146106ef57600080fd5b8063704802751461068357806375d7370a146106965780637ecebe00146106b457600080fd5b8063429b62e5116102815780635893253c1161022a578063627cdcb911610204578063627cdcb91461061c578063654f0ce41461062457806368c7450f146106375780636d70f7ae1461064a57600080fd5b80635893253c146105ad5780635c1548fb146105e95780635c975abb1461060f57600080fd5b8063456068d21161025b578063456068d21461052f57806346423aa7146105375780634a2a11f5146105a557600080fd5b8063429b62e5146104f457806344bea37e146105145780634544f0551461051c57600080fd5b80631785f53c116102e357806334600901116102bd57806334600901146104b35780633b521d78146104c65780633d6d3598146104ec57600080fd5b80631785f53c1461042257806324d7806c146104355780632dff692d1461046f57600080fd5b80631031e36e116103145780631031e36e146103ca578063131e7e1c146103d457806313e7c9d8146103f457600080fd5b806301ffc9a71461033b5780630647ee201461036357806306b9d6911461039d575b600080fd5b61034e6103493660046136e2565b6108f2565b60405190151581526020015b60405180910390f35b61034e610371366004613756565b73ffffffffffffffffffffffffffffffffffffffff919091166000908152600460205260409020541490565b6103a561098b565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200161035a565b6103d2610a24565b005b6007546103a59073ffffffffffffffffffffffffffffffffffffffff1681565b610414610402366004613782565b60026020526000908152604090205481565b60405190815260200161035a565b6103d2610430366004613782565b610a78565b61034e610443366004613782565b73ffffffffffffffffffffffffffffffffffffffff166000908152600160208190526040909120541490565b61049c61047d36600461379f565b6008602052600090815260409020805460019091015460ff9091169082565b60408051921515835260208301919091520161035a565b6103d26104c136600461379f565b610b15565b7f00000000000000000000000000000000000000000000000000000000000000006103a5565b6103d2610b5f565b610414610502366004613782565b60016020526000908152604090205481565b610414600081565b6103d261052a366004613782565b610be3565b6103d2610c36565b61058861054536600461379f565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b60408051825115158152602092830151928101929092520161035a565b6103e8610414565b6105d46105bb36600461379f565b6005602052600090815260409020805460019091015482565b6040805192835260208301919091520161035a565b7f00000000000000000000000000000000000000000000000000000000000000006103a5565b60035461034e9060ff1681565b6103d2610c88565b6103d26106323660046139f8565b610c92565b6103d2610645366004613a2d565b610cad565b61034e610658366004613782565b73ffffffffffffffffffffffffffffffffffffffff1660009081526002602052604090205460011490565b6103d2610691366004613782565b610d07565b60075473ffffffffffffffffffffffffffffffffffffffff166103a5565b6104146106c2366004613782565b60046020526000908152604090205481565b6103d2610da7565b6103d26106ea366004613782565b610e2c565b6104146106fd36600461379f565b610eca565b6103a5610710366004613782565b610ee8565b6103d26107233660046139f8565b610f14565b6103d2610736366004613782565b610f1d565b60065473ffffffffffffffffffffffffffffffffffffffff166103a5565b61076c610767366004613ae8565b610fba565b6040517fffffffff00000000000000000000000000000000000000000000000000000000909116815260200161035a565b6006546103a59073ffffffffffffffffffffffffffffffffffffffff1681565b6103d26107cb366004613c16565b610fe5565b6104146107de36600461379f565b60009081526005602052604090206001015490565b6103d2610801366004613c7a565b6110f5565b6103a5611136565b6103d261081c366004613c9c565b6111a6565b61041461082f3660046139f8565b6111fb565b6103d2610842366004613cd9565b611298565b6103a5610855366004613782565b6113a6565b61076c610868366004613d6b565b7ff23a6e610000000000000000000000000000000000000000000000000000000095945050505050565b6104147f000000000000000000000000000000000000000000000000000000000000000081565b6103d26108c7366004613dd4565b6113d2565b6103d26108da366004613782565b611409565b6103d26108ed366004613e09565b61145c565b60007fffffffff0000000000000000000000000000000000000000000000000000000082167f4e2312e000000000000000000000000000000000000000000000000000000000148061098557507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b600654604080517faaf10f42000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163aaf10f429160048083019260209291908290030181865afa1580156109fb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a1f9190613e4e565b905090565b3360009081526001602081905260409091205414610a6e576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a7661155e565b565b3360009081526001602081905260409091205414610ac2576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610b5c576040517f3f6cc76800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50565b33600090815260026020526040902054600114610ba8576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610c2d576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610b5c816115b6565b3360009081526001602081905260409091205414610c80576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a76611644565b610a766001611699565b6000610c9d826111fb565b9050610ca981836116c7565b5050565b3360009081526001602081905260409091205414610cf7576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d02838383611826565b505050565b3360009081526001602081905260409091205414610d51576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610df1576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610e76576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610ed582610b15565b5060009081526005602052604090205490565b600061098582610ef6611136565b60075473ffffffffffffffffffffffffffffffffffffffff16611982565b610b5c81611a80565b3360009081526001602081905260409091205414610f67576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b7fbc197c81000000000000000000000000000000000000000000000000000000005b95945050505050565b600054600203611056576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e43590000000000000000000000000000000000000000000060448201526064015b60405180910390fd5b6002600081815533815260209190915260409020546001146110a4576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff16156110e1576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110ec828233611b85565b50506001600055565b806110ff83610eca565b14610ca9576040517f66f8620a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600754604080517fa619486e000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163a619486e9160048083019260209291908290030181865afa1580156109fb573d6000803e3d6000fd5b6111c58160400151826020015184846101800151856101600151611bde565b610ca9576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006109857fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e610160015160405160200161127d9d9c9b9a99989796959493929190613eae565b60405160208183030381529060405280519060200120611c3c565b600054600203611304576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e435900000000000000000000000000000000000000000000604482015260640161104d565b600260008181553381526020919091526040902054600114611352576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff161561138f576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61139b84848484611ca5565b505060016000555050565b6000610985826113b461098b565b60065473ffffffffffffffffffffffffffffffffffffffff16611e5c565b805160005b81811015610d02576114018382815181106113f4576113f4613f4c565b6020026020010151611a80565b6001016113d7565b3360009081526001602081905260409091205414611453576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610b5c81611ebe565b6000546002036114c8576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e435900000000000000000000000000000000000000000000604482015260640161104d565b600260008181553381526020919091526040902054600114611516576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff1615611553576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110ec828233611f4c565b600380547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b60075460405173ffffffffffffffffffffffffffffffffffffffff8084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b600380547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b336000908152600460205260409020546116b4908290613faa565b3360009081526004602052604090205550565b60008160e001511180156116de5750428160e00151105b15611715576040517fc56873ba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61171f82826111a6565b6103e88161012001511115611760576040517fcd4e616700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61176d8160800151610b15565b60008281526008602052604090205460ff16156117b6576040517f7b38b76e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117f0816020015182610100015173ffffffffffffffffffffffffffffffffffffffff919091166000908152600460205260409020541490565b610ca9576040517f756688fe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8183148061183a575082158061183a575081155b15611871576040517f3f6cc76800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008381526005602052604090205415158061189a575060008281526005602052604090205415155b156118d1576040517f3a81d6fc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b60008061198e8461205a565b8051906020012090506000856040516020016119c6919073ffffffffffffffffffffffffffffffffffffffff91909116815260200190565b604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815282825280516020918201207fff000000000000000000000000000000000000000000000000000000000000008285015260609790971b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000166021840152603583019690965260558083019490945280518083039094018452607590910190525080519201919091209392505050565b602081015173ffffffffffffffffffffffffffffffffffffffff163314611ad3576040517f30cd747100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611ade826111fb565b600081815260086020526040902080549192509060ff1615611b2c576040517f7b38b76e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b81811015611bd757611bcf858281518110611ba757611ba7613f4c565b6020026020010151858381518110611bc157611bc1613f4c565b602002602001015185611f4c565b600101611b8a565b5050505050565b600080826002811115611bf357611bf3613e6b565b03611c0b57611c04868686866120eb565b9050610fdc565b6002826002811115611c1f57611c1f613e6b565b03611c3057611c0486868686612139565b611c048686868661218d565b6000610985611c496121c1565b836040517f19010000000000000000000000000000000000000000000000000000000000006020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080611cb387846122f5565b91509150600080611cc389612342565b91509150611cd78960200151308488612379565b611ce28989886123a3565b611cec84826123f5565b6101208a0151909450600090611d2e90828c61014001516001811115611d1457611d14613e6b565b14611d1f5787611d21565b865b88888e610140015161243d565b9050611d4b308b60200151848489611d469190613fc2565b612379565b611d573033848461252d565b6000611d6284612596565b90508015611d7a57611d7a308c602001518684612379565b60208b8101516040805187815292830186905282018990526060820188905260808201849052309173ffffffffffffffffffffffffffffffffffffffff9091169087907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c01516040805187815292830186905282018990526060820188905273ffffffffffffffffffffffffffffffffffffffff169086907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a35050505050505050505050565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606085901b166020820152600090611eb49083908590603401604051602081830303815290604052805190602001206126c6565b90505b9392505050565b60065460405173ffffffffffffffffffffffffffffffffffffffff8084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b81600080611f5a86846122f5565b6101208801519193509150600090611fa790825b8961014001516001811115611f8557611f85613e6b565b14611f905785611f92565b845b8960a001518a60c001518b610140015161243d565b9050600080611fb589612342565b91509150611fcf338a60200151838689611d469190613fc2565b611fdf8960200151888489612379565b6020898101516040805185815292830184905282018890526060820187905260808201859052339173ffffffffffffffffffffffffffffffffffffffff9091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a4505050505050505050565b6060604051806101a00160405280610171815260200161419261017191396040805173ffffffffffffffffffffffffffffffffffffffff8516602082015201604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526120d59291602001614005565b6040516020818303038152906040529050919050565b60008373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff1614801561212e575061212e858484612763565b90505b949350505050565b6000612146858484612763565b801561212e57508373ffffffffffffffffffffffffffffffffffffffff1661216d86610ee8565b73ffffffffffffffffffffffffffffffffffffffff161495945050505050565b600061219a858484612763565b801561212e57508373ffffffffffffffffffffffffffffffffffffffff1661216d866113a6565b60003073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614801561222757507f000000000000000000000000000000000000000000000000000000000000000046145b1561225157507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b60008061230584606001516127a5565b61230e846111fb565b905061231a81856116c7565b61232d838560a001518660c00151612817565b915061233a81858561283e565b509250929050565b60008080836101400151600181111561235d5761235d613e6b565b0361236d57505060800151600091565b50506080015190600090565b816000036123915761238c8484836128eb565b61239d565b61239d84848484612940565b50505050565b815160005b81811015611bd7576123ed858583815181106123c6576123c6613f4c565b60200260200101518584815181106123e0576123e0613f4c565b602002602001015161296d565b6001016123a8565b60008061240183612596565b905083811015611eb7576040517fdf4d808000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008515610fdc576000612452858585612a52565b905060008111801561246c5750670de0b6b3a76400008111155b1561252357600083600181111561248557612485613e6b565b036124d75761249661271082614034565b866124b2836124ad81670de0b6b3a7640000613fc2565b612ac1565b6124bc908a614034565b6124c69190614034565b6124d09190614071565b9150612523565b6124eb670de0b6b3a7640000612710614034565b86612502836124ad81670de0b6b3a7640000613fc2565b61250c908a614034565b6125169190614034565b6125209190614071565b91505b5095945050505050565b801561239d5761253f84848484612379565b604080518381526020810183905273ffffffffffffffffffffffffffffffffffffffff8516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b60008160000361264f576040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa15801561262b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061098591906140ac565b6040517efdd58e0000000000000000000000000000000000000000000000000000000081523060048201526024810183905273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e9060440161260e565b6000806126d38585612ad7565b8051602091820120604080517fff000000000000000000000000000000000000000000000000000000000000008185015260609890981b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000166021890152603588019590955260558088019190915284518088039091018152607590960190935250508251920191909120919050565b60008373ffffffffffffffffffffffffffffffffffffffff166127868484612c5a565b73ffffffffffffffffffffffffffffffffffffffff1614949350505050565b73ffffffffffffffffffffffffffffffffffffffff8116158015906127e0575073ffffffffffffffffffffffffffffffffffffffff81163314155b15610b5c576040517f5211a07900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008260000361282957506000611eb7565b826128348386614034565b611eb49190614071565b6000838152600860205260409020600181015490811561285e5781612864565b8360a001515b9150818311156128a0576040517fe2cc6ad600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6128aa8383613fc2565b9150816000036128de5780547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011781555b6001018190559392505050565b7f00000000000000000000000000000000000000000000000000000000000000003073ffffffffffffffffffffffffffffffffffffffff8516036129345761238c818484612c7e565b61239d81858585612c89565b61239d7f000000000000000000000000000000000000000000000000000000000000000085858585612c95565b60006129798484612d41565b9050612986848483612ddd565b8160008061299486846122f5565b61012088015191935091506000906129ac9082611f6e565b90506000806129ba89612342565b915091506129d186868b6020015185858c89612e89565b6020808b01518a820151604080518681529384018590528301899052606083018890526080830186905273ffffffffffffffffffffffffffffffffffffffff9182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b600080826001811115612a6757612a67613e6b565b03612a9f5782600003612a7b576000612a98565b82612a8e670de0b6b3a764000086614034565b612a989190614071565b9050611eb7565b83600003612aae576000611eb4565b83612834670de0b6b3a764000085614034565b6000818310612ad05781611eb7565b5090919050565b6040805160008082526020820190925260609190612af890604481016140c5565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152918152602080830180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f52e831dd000000000000000000000000000000000000000000000000000000001790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe60208201526c010000000000000000000000008502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be60418201526c01000000000000000000000000840260608201527f5af43d82803e903d91602b57fd5bf3000000000000000000000000000000000060748201528082604051602001612c41929190614005565b6040516020818303038152906040529250505092915050565b6000806000612c698585612f09565b91509150612c7681612f4e565b509392505050565b610d02838383613101565b61239d848484846131ba565b6040517ff242432a00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b158015612d2257600080fd5b505af1158015612d36573d6000803e3d6000fd5b505050505050505050565b6000808361014001516001811115612d5b57612d5b613e6b565b148015612d7e575060008261014001516001811115612d7c57612d7c613e6b565b145b15612d8b57506001610985565b60018361014001516001811115612da457612da4613e6b565b148015612dc7575060018261014001516001811115612dc557612dc5613e6b565b145b15612dd457506002610985565b50600092915050565b612de78383613279565b612e1d576040517f7f9a6f4600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000816002811115612e3157612e31613e6b565b03612e77578160800151836080015114610d02576040517fa0b9446500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d02836080015183608001516110f5565b612e958530868a612379565b612ea287878686866132c3565b85612eac84612596565b1015612ee4576040517fdf4d808000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612ef4308685611d46858b613fc2565b612f003033858461252d565b50505050505050565b6000808251604103612f3f5760208301516040840151606085015160001a612f338782858561334b565b94509450505050612f47565b506000905060025b9250929050565b6000816004811115612f6257612f62613e6b565b03612f6a5750565b6001816004811115612f7e57612f7e613e6b565b03612fe5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f45434453413a20696e76616c6964207369676e61747572650000000000000000604482015260640161104d565b6002816004811115612ff957612ff9613e6b565b03613060576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e67746800604482015260640161104d565b600381600481111561307457613074613e6b565b03610b5c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c60448201527f7565000000000000000000000000000000000000000000000000000000000000606482015260840161104d565b60006040517fa9059cbb000000000000000000000000000000000000000000000000000000008152836004820152826024820152602060006044836000895af13d15601f3d116001600051141617169150508061239d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600f60248201527f5452414e534645525f4641494c45440000000000000000000000000000000000604482015260640161104d565b60006040517f23b872dd0000000000000000000000000000000000000000000000000000000081528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080611bd7576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f5452414e534645525f46524f4d5f4641494c4544000000000000000000000000604482015260640161104d565b60008260c0015160001480613290575060c0820151155b1561329d57506001610985565b611eb76132a98461343a565b6132b28461343a565b856101400151856101400151613454565b60008160028111156132d7576132d7613e6b565b14611bd75760018160028111156132f0576132f0613e6b565b036133165760008281526005602052604090206001015461331190856134ee565b611bd7565b600281600281111561332a5761332a613e6b565b03611bd757600083815260056020526040902060010154613311908661361e565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156133825750600090506003613431565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156133d6573d6000803e3d6000fd5b50506040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0015191505073ffffffffffffffffffffffffffffffffffffffff811661342a57600060019250925050613431565b9150600090505b94509492505050565b60006109858260a001518360c00151846101400151612a52565b60008083600181111561346957613469613e6b565b036134ad57600082600181111561348257613482613e6b565b036134a357670de0b6b3a764000061349a8587613faa565b10159050612131565b5082841015612131565b60008260018111156134c1576134c1613e6b565b036134d0575083831015612131565b670de0b6b3a76400006134e38587613faa565b111595945050505050565b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061352457613524613f4c565b60200260200101818152505060028160018151811061354557613545613f4c565b60209081029190910101527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526135f09190600090889087908990600401614116565b600060405180830381600087803b15801561360a57600080fd5b505af1158015612f00573d6000803e3d6000fd5b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061365457613654613f4c565b60200260200101818152505060028160018151811061367557613675613f4c565b60209081029190910101527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16639e7212ad7f00000000000000000000000000000000000000000000000000000000000000006135ae565b6000602082840312156136f457600080fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611eb757600080fd5b73ffffffffffffffffffffffffffffffffffffffff81168114610b5c57600080fd5b803561375181613724565b919050565b6000806040838503121561376957600080fd5b823561377481613724565b946020939093013593505050565b60006020828403121561379457600080fd5b8135611eb781613724565b6000602082840312156137b157600080fd5b5035919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040516101a0810167ffffffffffffffff8111828210171561380b5761380b6137b8565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715613858576138586137b8565b604052919050565b80356002811061375157600080fd5b80356003811061375157600080fd5b600082601f83011261388f57600080fd5b813567ffffffffffffffff8111156138a9576138a96137b8565b6138da60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601613811565b8181528460208386010111156138ef57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a0828403121561391f57600080fd5b6139276137e7565b90508135815261393960208301613746565b602082015261394a60408301613746565b604082015261395b60608301613746565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e08201526101008083013581830152506101208083013581830152506101406139ae818401613860565b908201526101606139c083820161386f565b908201526101808281013567ffffffffffffffff8111156139e057600080fd5b6139ec8582860161387e565b82840152505092915050565b600060208284031215613a0a57600080fd5b813567ffffffffffffffff811115613a2157600080fd5b6121318482850161390c565b600080600060608486031215613a4257600080fd5b505081359360208301359350604090920135919050565b600067ffffffffffffffff821115613a7357613a736137b8565b5060051b60200190565b600082601f830112613a8e57600080fd5b81356020613aa3613a9e83613a59565b613811565b82815260059290921b84018101918181019086841115613ac257600080fd5b8286015b84811015613add5780358352918301918301613ac6565b509695505050505050565b600080600080600060a08688031215613b0057600080fd5b8535613b0b81613724565b94506020860135613b1b81613724565b9350604086013567ffffffffffffffff80821115613b3857600080fd5b613b4489838a01613a7d565b94506060880135915080821115613b5a57600080fd5b613b6689838a01613a7d565b93506080880135915080821115613b7c57600080fd5b50613b898882890161387e565b9150509295509295909350565b600082601f830112613ba757600080fd5b81356020613bb7613a9e83613a59565b82815260059290921b84018101918181019086841115613bd657600080fd5b8286015b84811015613add57803567ffffffffffffffff811115613bfa5760008081fd5b613c088986838b010161390c565b845250918301918301613bda565b60008060408385031215613c2957600080fd5b823567ffffffffffffffff80821115613c4157600080fd5b613c4d86838701613b96565b93506020850135915080821115613c6357600080fd5b50613c7085828601613a7d565b9150509250929050565b60008060408385031215613c8d57600080fd5b50508035926020909101359150565b60008060408385031215613caf57600080fd5b82359150602083013567ffffffffffffffff811115613ccd57600080fd5b613c708582860161390c565b60008060008060808587031215613cef57600080fd5b843567ffffffffffffffff80821115613d0757600080fd5b613d138883890161390c565b95506020870135915080821115613d2957600080fd5b613d3588838901613b96565b9450604087013593506060870135915080821115613d5257600080fd5b50613d5f87828801613a7d565b91505092959194509250565b600080600080600060a08688031215613d8357600080fd5b8535613d8e81613724565b94506020860135613d9e81613724565b93506040860135925060608601359150608086013567ffffffffffffffff811115613dc857600080fd5b613b898882890161387e565b600060208284031215613de657600080fd5b813567ffffffffffffffff811115613dfd57600080fd5b61213184828501613b96565b60008060408385031215613e1c57600080fd5b823567ffffffffffffffff811115613e3357600080fd5b613e3f8582860161390c565b95602094909401359450505050565b600060208284031215613e6057600080fd5b8151611eb781613724565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b60038110613eaa57613eaa613e6b565b9052565b60006101a0820190508e82528d602083015273ffffffffffffffffffffffffffffffffffffffff808e166040840152808d166060840152808c166080840152508960a08301528860c08301528760e083015286610100830152856101208301528461014083015260028410613f2557613f25613e6b565b83610160830152613f3a610180830184613e9a565b9e9d5050505050505050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60008219821115613fbd57613fbd613f7b565b500190565b600082821015613fd457613fd4613f7b565b500390565b60005b83811015613ff4578181015183820152602001613fdc565b8381111561239d5750506000910152565b60008351614017818460208801613fd9565b83519083019061402b818360208801613fd9565b01949350505050565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff048311821515161561406c5761406c613f7b565b500290565b6000826140a7577f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b500490565b6000602082840312156140be57600080fd5b5051919050565b60208152600082518060208401526140e4816040850160208701613fd9565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169190910160400192915050565b600060a0820173ffffffffffffffffffffffffffffffffffffffff881683526020878185015286604085015260a0606085015281865180845260c086019150828801935060005b818110156141795784518352938301939183019160010161415d565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a264697066735822122056df26e165b5957191bd0ff149c07ae13f5a6b4252973fb3c07a4653cce0f3b164736f6c634300080f00330000000000000000000000002e8dcfe708d44ae2e406a1c02dfe2fa13012f9610000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x1107c",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x10169",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x7d00",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x7d34",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [
    "0x4cf5ff4362abb630398f45b0ed26787e7b2524c53c4cc006641764f5f8267609"
  ],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E"
    }
  },
  "timestamp": 1664228139,
  "commit": "af3ba7f"
}


================================================
FILE: broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json
================================================
{
  "transactions": [
    {
      "hash": "0x4cf5ff4362abb630398f45b0ed26787e7b2524c53c4cc006641764f5f8267609",
      "transactionType": "CREATE",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": null,
      "arguments": [
        "0x2E8DCfE708D44ae2e406a1c02DFE2Fa13012f961",
        "0x7D8610E9567d2a6C9FBf66a5A13E9Ba8bb120d43",
        "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "gas": "0x4d4c99",
        "value": "0x0",
        "data": "0x6101a060405260016000556003805460ff191690553480156200002157600080fd5b506040516200473f3803806200473f8339810160408190526200004491620002d6565b604080518082018252601781527f506f6c796d61726b6574204354462045786368616e67650000000000000000006020808301918252835180850185526001808252603160f81b82840190815233600090815282855287812083905560028552879020919091558451909320815190932060e08490526101008190524660a081815287517f8b73c3c69bb8fe3d512ecc4cf759cc79239f7b179b0ffacaa9a75d522b39400f818701819052818a0188905260608201859052608082019390935230818301528851808203909201825260c0019097528651969093019590952087958795879587959194938d938d9387938793909291906080523060c05261012052505050506001600160a01b0382811661014081905290821661016081905260405163095ea7b360e01b81526004810191909152600019602482015263095ea7b3906044016020604051808303816000875af1158015620001a9573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190620001cf919062000333565b50620001dd91505062000265565b610180525050600680546001600160a01b039384166001600160a01b03199182161790915560078054929093169116179055506200035e945050505050565b6040805160208101859052908101839052606081018290524660808201523060a082015260009060c0016040516020818303038152906040528051906020012090509392505050565b600060c0516001600160a01b0316306001600160a01b03161480156200028c575060a05146145b1562000299575060805190565b620002b46101205160e051610100516200021c60201b60201c565b905090565b80516001600160a01b0381168114620002d157600080fd5b919050565b60008060008060808587031215620002ed57600080fd5b620002f885620002b9565b93506200030860208601620002b9565b92506200031860408601620002b9565b91506200032860608601620002b9565b905092959194509250565b6000602082840312156200034657600080fd5b815180151581146200035757600080fd5b9392505050565b60805160a05160c05160e05161010051610120516101405161016051610180516143386200040760003960006108970152600081816104c801528181612698015281816129450152818161355201526136820152600081816105eb015281816125e3015281816128ed0152818161358e01526136be01526000612258015260006122a701526000612282015260006121db015260006122050152600061222f01526143386000f3fe608060405234801561001057600080fd5b50600436106103365760003560e01c806370480275116101b2578063d798eff6116100f9578063e60f0c05116100a2578063f698da251161007c578063f698da2514610892578063fa950b48146108b9578063fbddd751146108cc578063fe729aaf146108df57600080fd5b8063e60f0c0514610834578063edef7d8e14610847578063f23a6e611461085a57600080fd5b8063e03ac3d0116100d3578063e03ac3d014610806578063e2eec4051461080e578063e50e4f971461082157600080fd5b8063d798eff6146107bd578063d7fb272f146107d0578063d82da838146107f357600080fd5b8063a287bdf11161015b578063b28c51c011610135578063b28c51c01461073b578063bc197c8114610759578063c10f1a751461079d57600080fd5b8063a287bdf114610702578063a6dfcf8614610715578063ac8a584a1461072857600080fd5b806383b8a5ae1161018c57806383b8a5ae146106d45780639870d7fe146106dc578063a10f3dce146106ef57600080fd5b8063704802751461068357806375d7370a146106965780637ecebe00146106b457600080fd5b8063429b62e5116102815780635893253c1161022a578063627cdcb911610204578063627cdcb91461061c578063654f0ce41461062457806368c7450f146106375780636d70f7ae1461064a57600080fd5b80635893253c146105ad5780635c1548fb146105e95780635c975abb1461060f57600080fd5b8063456068d21161025b578063456068d21461052f57806346423aa7146105375780634a2a11f5146105a557600080fd5b8063429b62e5146104f457806344bea37e146105145780634544f0551461051c57600080fd5b80631785f53c116102e357806334600901116102bd57806334600901146104b35780633b521d78146104c65780633d6d3598146104ec57600080fd5b80631785f53c1461042257806324d7806c146104355780632dff692d1461046f57600080fd5b80631031e36e116103145780631031e36e146103ca578063131e7e1c146103d457806313e7c9d8146103f457600080fd5b806301ffc9a71461033b5780630647ee201461036357806306b9d6911461039d575b600080fd5b61034e6103493660046136e2565b6108f2565b60405190151581526020015b60405180910390f35b61034e610371366004613756565b73ffffffffffffffffffffffffffffffffffffffff919091166000908152600460205260409020541490565b6103a561098b565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200161035a565b6103d2610a24565b005b6007546103a59073ffffffffffffffffffffffffffffffffffffffff1681565b610414610402366004613782565b60026020526000908152604090205481565b60405190815260200161035a565b6103d2610430366004613782565b610a78565b61034e610443366004613782565b73ffffffffffffffffffffffffffffffffffffffff166000908152600160208190526040909120541490565b61049c61047d36600461379f565b6008602052600090815260409020805460019091015460ff9091169082565b60408051921515835260208301919091520161035a565b6103d26104c136600461379f565b610b15565b7f00000000000000000000000000000000000000000000000000000000000000006103a5565b6103d2610b5f565b610414610502366004613782565b60016020526000908152604090205481565b610414600081565b6103d261052a366004613782565b610be3565b6103d2610c36565b61058861054536600461379f565b6040805180820190915260008082526020820152506000908152600860209081526040918290208251808401909352805460ff1615158352600101549082015290565b60408051825115158152602092830151928101929092520161035a565b6103e8610414565b6105d46105bb36600461379f565b6005602052600090815260409020805460019091015482565b6040805192835260208301919091520161035a565b7f00000000000000000000000000000000000000000000000000000000000000006103a5565b60035461034e9060ff1681565b6103d2610c88565b6103d26106323660046139f8565b610c92565b6103d2610645366004613a2d565b610cad565b61034e610658366004613782565b73ffffffffffffffffffffffffffffffffffffffff1660009081526002602052604090205460011490565b6103d2610691366004613782565b610d07565b60075473ffffffffffffffffffffffffffffffffffffffff166103a5565b6104146106c2366004613782565b60046020526000908152604090205481565b6103d2610da7565b6103d26106ea366004613782565b610e2c565b6104146106fd36600461379f565b610eca565b6103a5610710366004613782565b610ee8565b6103d26107233660046139f8565b610f14565b6103d2610736366004613782565b610f1d565b60065473ffffffffffffffffffffffffffffffffffffffff166103a5565b61076c610767366004613ae8565b610fba565b6040517fffffffff00000000000000000000000000000000000000000000000000000000909116815260200161035a565b6006546103a59073ffffffffffffffffffffffffffffffffffffffff1681565b6103d26107cb366004613c16565b610fe5565b6104146107de36600461379f565b60009081526005602052604090206001015490565b6103d2610801366004613c7a565b6110f5565b6103a5611136565b6103d261081c366004613c9c565b6111a6565b61041461082f3660046139f8565b6111fb565b6103d2610842366004613cd9565b611298565b6103a5610855366004613782565b6113a6565b61076c610868366004613d6b565b7ff23a6e610000000000000000000000000000000000000000000000000000000095945050505050565b6104147f000000000000000000000000000000000000000000000000000000000000000081565b6103d26108c7366004613dd4565b6113d2565b6103d26108da366004613782565b611409565b6103d26108ed366004613e09565b61145c565b60007fffffffff0000000000000000000000000000000000000000000000000000000082167f4e2312e000000000000000000000000000000000000000000000000000000000148061098557507f01ffc9a7000000000000000000000000000000000000000000000000000000007fffffffff000000000000000000000000000000000000000000000000000000008316145b92915050565b600654604080517faaf10f42000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163aaf10f429160048083019260209291908290030181865afa1580156109fb573d6000803e3d6000fd5b505050506040513d601f19601f82011682018060405250810190610a1f9190613e4e565b905090565b3360009081526001602081905260409091205414610a6e576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a7661155e565b565b3360009081526001602081905260409091205414610ac2576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020526040808220829055513392917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a350565b6000818152600560205260408120549003610b5c576040517f3f6cc76800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b50565b33600090815260026020526040902054600114610ba8576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000818152600260205260408082208290555182917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a3565b3360009081526001602081905260409091205414610c2d576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610b5c816115b6565b3360009081526001602081905260409091205414610c80576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610a76611644565b610a766001611699565b6000610c9d826111fb565b9050610ca981836116c7565b5050565b3360009081526001602081905260409091205414610cf7576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d02838383611826565b505050565b3360009081526001602081905260409091205414610d51576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260016020819052604080832091909155513392917ff9ffabca9c8276e99321725bcb43fb076a6c66a54b7f21c4e8146d8519b417dc91a350565b3360009081526001602081905260409091205414610df1576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b336000818152600160205260408082208290555182917f787a2e12f4a55b658b8f573c32432ee11a5e8b51677d1e1e937aaf6a0bb5776e91a3565b3360009081526001602081905260409091205414610e76576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff811660008181526002602052604080822060019055513392917ff1e04d73c4304b5ff164f9d10c7473e2a1593b740674a6107975e2a7001c1e5c91a350565b6000610ed582610b15565b5060009081526005602052604090205490565b600061098582610ef6611136565b60075473ffffffffffffffffffffffffffffffffffffffff16611982565b610b5c81611a80565b3360009081526001602081905260409091205414610f67576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b73ffffffffffffffffffffffffffffffffffffffff8116600081815260026020526040808220829055513392917ff7262ed0443cc211121ceb1a80d69004f319245615a7488f951f1437fd91642c91a350565b7fbc197c81000000000000000000000000000000000000000000000000000000005b95945050505050565b600054600203611056576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e43590000000000000000000000000000000000000000000060448201526064015b60405180910390fd5b6002600081815533815260209190915260409020546001146110a4576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff16156110e1576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110ec828233611b85565b50506001600055565b806110ff83610eca565b14610ca9576040517f66f8620a00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b600754604080517fa619486e000000000000000000000000000000000000000000000000000000008152905160009273ffffffffffffffffffffffffffffffffffffffff169163a619486e9160048083019260209291908290030181865afa1580156109fb573d6000803e3d6000fd5b6111c58160400151826020015184846101800151856101600151611bde565b610ca9576040517f8baa579f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60006109857fa852566c4e14d00869b6db0220888a9090a13eccdaea03713ff0a3d27bf9767c836000015184602001518560400151866060015187608001518860a001518960c001518a60e001518b61010001518c61012001518d61014001518e610160015160405160200161127d9d9c9b9a99989796959493929190613eae565b60405160208183030381529060405280519060200120611c3c565b600054600203611304576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e435900000000000000000000000000000000000000000000604482015260640161104d565b600260008181553381526020919091526040902054600114611352576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff161561138f576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61139b84848484611ca5565b505060016000555050565b6000610985826113b461098b565b60065473ffffffffffffffffffffffffffffffffffffffff16611e5c565b805160005b81811015610d02576114018382815181106113f4576113f4613f4c565b6020026020010151611a80565b6001016113d7565b3360009081526001602081905260409091205414611453576040517f7bfa4b9f00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610b5c81611ebe565b6000546002036114c8576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f5245454e5452414e435900000000000000000000000000000000000000000000604482015260640161104d565b600260008181553381526020919091526040902054600114611516576040517f7c214f0400000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60035460ff1615611553576040517f9e87fac800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6110ec828233611f4c565b600380547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117905560405133907f203c4bd3e526634f661575359ff30de3b0edaba6c2cb1eac60f730b6d2d9d53690600090a2565b60075460405173ffffffffffffffffffffffffffffffffffffffff8084169216907f9726d7faf7429d6b059560dc858ed769377ccdf8b7541eabe12b22548719831f90600090a3600780547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b600380547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016905560405133907fa1e8a54850dbd7f520bcc09f47bff152294b77b2081da545a7adf531b7ea283b90600090a2565b336000908152600460205260409020546116b4908290613faa565b3360009081526004602052604090205550565b60008160e001511180156116de5750428160e00151105b15611715576040517fc56873ba00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61171f82826111a6565b6103e88161012001511115611760576040517fcd4e616700000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b61176d8160800151610b15565b60008281526008602052604090205460ff16156117b6576040517f7b38b76e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6117f0816020015182610100015173ffffffffffffffffffffffffffffffffffffffff919091166000908152600460205260409020541490565b610ca9576040517f756688fe00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b8183148061183a575082158061183a575081155b15611871576040517f3f6cc76800000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008381526005602052604090205415158061189a575060008281526005602052604090205415155b156118d1576040517f3a81d6fc00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6040805180820182528381526020808201848152600087815260058084528582209451855591516001948501558451808601865288815280840187815288835292909352848120925183559051919092015590518291849186917fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d91a48083837fbc9a2432e8aeb48327246cddd6e872ef452812b4243c04e6bfb786a2cd8faf0d60405160405180910390a4505050565b60008061198e8461205a565b8051906020012090506000856040516020016119c6919073ffffffffffffffffffffffffffffffffffffffff91909116815260200190565b604080518083037fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe001815282825280516020918201207fff000000000000000000000000000000000000000000000000000000000000008285015260609790971b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000166021840152603583019690965260558083019490945280518083039094018452607590910190525080519201919091209392505050565b602081015173ffffffffffffffffffffffffffffffffffffffff163314611ad3576040517f30cd747100000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000611ade826111fb565b600081815260086020526040902080549192509060ff1615611b2c576040517f7b38b76e00000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b80547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0016600117815560405182907f5152abf959f6564662358c2e52b702259b78bac5ee7842a0f01937e670efcc7d90600090a2505050565b825160005b81811015611bd757611bcf858281518110611ba757611ba7613f4c565b6020026020010151858381518110611bc157611bc1613f4c565b602002602001015185611f4c565b600101611b8a565b5050505050565b600080826002811115611bf357611bf3613e6b565b03611c0b57611c04868686866120eb565b9050610fdc565b6002826002811115611c1f57611c1f613e6b565b03611c3057611c0486868686612139565b611c048686868661218d565b6000610985611c496121c1565b836040517f19010000000000000000000000000000000000000000000000000000000000006020820152602281018390526042810182905260009060620160405160208183030381529060405280519060200120905092915050565b81600080611cb387846122f5565b91509150600080611cc389612342565b91509150611cd78960200151308488612379565b611ce28989886123a3565b611cec84826123f5565b6101208a0151909450600090611d2e90828c61014001516001811115611d1457611d14613e6b565b14611d1f5787611d21565b865b88888e610140015161243d565b9050611d4b308b60200151848489611d469190613fc2565b612379565b611d573033848461252d565b6000611d6284612596565b90508015611d7a57611d7a308c602001518684612379565b60208b8101516040805187815292830186905282018990526060820188905260808201849052309173ffffffffffffffffffffffffffffffffffffffff9091169087907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a46020808c01516040805187815292830186905282018990526060820188905273ffffffffffffffffffffffffffffffffffffffff169086907f63bf4d16b7fa898ef4c4b2b6d90fd201e9c56313b65638af6088d149d2ce956c9060800160405180910390a35050505050505050505050565b6040517fffffffffffffffffffffffffffffffffffffffff000000000000000000000000606085901b166020820152600090611eb49083908590603401604051602081830303815290604052805190602001206126c6565b90505b9392505050565b60065460405173ffffffffffffffffffffffffffffffffffffffff8084169216907f3053c6252a932554235c173caffc1913604dba3a41cee89516f631c4a1a50a3790600090a3600680547fffffffffffffffffffffffff00000000000000000000000000000000000000001673ffffffffffffffffffffffffffffffffffffffff92909216919091179055565b81600080611f5a86846122f5565b6101208801519193509150600090611fa790825b8961014001516001811115611f8557611f85613e6b565b14611f905785611f92565b845b8960a001518a60c001518b610140015161243d565b9050600080611fb589612342565b91509150611fcf338a60200151838689611d469190613fc2565b611fdf8960200151888489612379565b6020898101516040805185815292830184905282018890526060820187905260808201859052339173ffffffffffffffffffffffffffffffffffffffff9091169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a4505050505050505050565b6060604051806101a00160405280610171815260200161419261017191396040805173ffffffffffffffffffffffffffffffffffffffff8516602082015201604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152908290526120d59291602001614005565b6040516020818303038152906040529050919050565b60008373ffffffffffffffffffffffffffffffffffffffff168573ffffffffffffffffffffffffffffffffffffffff1614801561212e575061212e858484612763565b90505b949350505050565b6000612146858484612763565b801561212e57508373ffffffffffffffffffffffffffffffffffffffff1661216d86610ee8565b73ffffffffffffffffffffffffffffffffffffffff161495945050505050565b600061219a858484612763565b801561212e57508373ffffffffffffffffffffffffffffffffffffffff1661216d866113a6565b60003073ffffffffffffffffffffffffffffffffffffffff7f00000000000000000000000000000000000000000000000000000000000000001614801561222757507f000000000000000000000000000000000000000000000000000000000000000046145b1561225157507f000000000000000000000000000000000000000000000000000000000000000090565b50604080517f00000000000000000000000000000000000000000000000000000000000000006020808301919091527f0000000000000000000000000000000000000000000000000000000000000000828401527f000000000000000000000000000000000000000000000000000000000000000060608301524660808301523060a0808401919091528351808403909101815260c0909201909252805191012090565b60008061230584606001516127a5565b61230e846111fb565b905061231a81856116c7565b61232d838560a001518660c00151612817565b915061233a81858561283e565b509250929050565b60008080836101400151600181111561235d5761235d613e6b565b0361236d57505060800151600091565b50506080015190600090565b816000036123915761238c8484836128eb565b61239d565b61239d84848484612940565b50505050565b815160005b81811015611bd7576123ed858583815181106123c6576123c6613f4c565b60200260200101518584815181106123e0576123e0613f4c565b602002602001015161296d565b6001016123a8565b60008061240183612596565b905083811015611eb7576040517fdf4d808000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008515610fdc576000612452858585612a52565b905060008111801561246c5750670de0b6b3a76400008111155b1561252357600083600181111561248557612485613e6b565b036124d75761249661271082614034565b866124b2836124ad81670de0b6b3a7640000613fc2565b612ac1565b6124bc908a614034565b6124c69190614034565b6124d09190614071565b9150612523565b6124eb670de0b6b3a7640000612710614034565b86612502836124ad81670de0b6b3a7640000613fc2565b61250c908a614034565b6125169190614034565b6125209190614071565b91505b5095945050505050565b801561239d5761253f84848484612379565b604080518381526020810183905273ffffffffffffffffffffffffffffffffffffffff8516917facffcc86834d0f1a64b0d5a675798deed6ff0bcfc2231edd3480e7288dba7ff4910160405180910390a250505050565b60008160000361264f576040517f70a0823100000000000000000000000000000000000000000000000000000000815230600482015273ffffffffffffffffffffffffffffffffffffffff7f000000000000000000000000000000000000000000000000000000000000000016906370a08231906024015b602060405180830381865afa15801561262b573d6000803e3d6000fd5b505050506040513d601f19601f8201168201806040525081019061098591906140ac565b6040517efdd58e0000000000000000000000000000000000000000000000000000000081523060048201526024810183905273ffffffffffffffffffffffffffffffffffffffff7f0000000000000000000000000000000000000000000000000000000000000000169062fdd58e9060440161260e565b6000806126d38585612ad7565b8051602091820120604080517fff000000000000000000000000000000000000000000000000000000000000008185015260609890981b7fffffffffffffffffffffffffffffffffffffffff000000000000000000000000166021890152603588019590955260558088019190915284518088039091018152607590960190935250508251920191909120919050565b60008373ffffffffffffffffffffffffffffffffffffffff166127868484612c5a565b73ffffffffffffffffffffffffffffffffffffffff1614949350505050565b73ffffffffffffffffffffffffffffffffffffffff8116158015906127e0575073ffffffffffffffffffffffffffffffffffffffff81163314155b15610b5c576040517f5211a07900000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b60008260000361282957506000611eb7565b826128348386614034565b611eb49190614071565b6000838152600860205260409020600181015490811561285e5781612864565b8360a001515b9150818311156128a0576040517fe2cc6ad600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6128aa8383613fc2565b9150816000036128de5780547fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff001660011781555b6001018190559392505050565b7f00000000000000000000000000000000000000000000000000000000000000003073ffffffffffffffffffffffffffffffffffffffff8516036129345761238c818484612c7e565b61239d81858585612c89565b61239d7f000000000000000000000000000000000000000000000000000000000000000085858585612c95565b60006129798484612d41565b9050612986848483612ddd565b8160008061299486846122f5565b61012088015191935091506000906129ac9082611f6e565b90506000806129ba89612342565b915091506129d186868b6020015185858c89612e89565b6020808b01518a820151604080518681529384018590528301899052606083018890526080830186905273ffffffffffffffffffffffffffffffffffffffff9182169291169086907fd0a08e8c493f9c94f29311604c9de1b4e8c8d4c06bd0c789af57f2d65bfec0f69060a00160405180910390a450505050505050505050565b600080826001811115612a6757612a67613e6b565b03612a9f5782600003612a7b576000612a98565b82612a8e670de0b6b3a764000086614034565b612a989190614071565b9050611eb7565b83600003612aae576000611eb4565b83612834670de0b6b3a764000085614034565b6000818310612ad05781611eb7565b5090919050565b6040805160008082526020820190925260609190612af890604481016140c5565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0818403018152918152602080830180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167f52e831dd000000000000000000000000000000000000000000000000000000001790528151606380825260a082019093529293506000929190820181803683370190505090507f3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe60208201526c010000000000000000000000008502602d8201527f5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be60418201526c01000000000000000000000000840260608201527f5af43d82803e903d91602b57fd5bf3000000000000000000000000000000000060748201528082604051602001612c41929190614005565b6040516020818303038152906040529250505092915050565b6000806000612c698585612f09565b91509150612c7681612f4e565b509392505050565b610d02838383613101565b61239d848484846131ba565b6040517ff242432a00000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff85811660048301528481166024830152604482018490526064820183905260a06084830152600060a483015286169063f242432a9060c401600060405180830381600087803b158015612d2257600080fd5b505af1158015612d36573d6000803e3d6000fd5b505050505050505050565b6000808361014001516001811115612d5b57612d5b613e6b565b148015612d7e575060008261014001516001811115612d7c57612d7c613e6b565b145b15612d8b57506001610985565b60018361014001516001811115612da457612da4613e6b565b148015612dc7575060018261014001516001811115612dc557612dc5613e6b565b145b15612dd457506002610985565b50600092915050565b612de78383613279565b612e1d576040517f7f9a6f4600000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b6000816002811115612e3157612e31613e6b565b03612e77578160800151836080015114610d02576040517fa0b9446500000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b610d02836080015183608001516110f5565b612e958530868a612379565b612ea287878686866132c3565b85612eac84612596565b1015612ee4576040517fdf4d808000000000000000000000000000000000000000000000000000000000815260040160405180910390fd5b612ef4308685611d46858b613fc2565b612f003033858461252d565b50505050505050565b6000808251604103612f3f5760208301516040840151606085015160001a612f338782858561334b565b94509450505050612f47565b506000905060025b9250929050565b6000816004811115612f6257612f62613e6b565b03612f6a5750565b6001816004811115612f7e57612f7e613e6b565b03612fe5576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f45434453413a20696e76616c6964207369676e61747572650000000000000000604482015260640161104d565b6002816004811115612ff957612ff9613e6b565b03613060576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e67746800604482015260640161104d565b600381600481111561307457613074613e6b565b03610b5c576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c60448201527f7565000000000000000000000000000000000000000000000000000000000000606482015260840161104d565b60006040517fa9059cbb000000000000000000000000000000000000000000000000000000008152836004820152826024820152602060006044836000895af13d15601f3d116001600051141617169150508061239d576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600f60248201527f5452414e534645525f4641494c45440000000000000000000000000000000000604482015260640161104d565b60006040517f23b872dd0000000000000000000000000000000000000000000000000000000081528460048201528360248201528260448201526020600060648360008a5af13d15601f3d1160016000511416171691505080611bd7576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601460248201527f5452414e534645525f46524f4d5f4641494c4544000000000000000000000000604482015260640161104d565b60008260c0015160001480613290575060c0820151155b1561329d57506001610985565b611eb76132a98461343a565b6132b28461343a565b856101400151856101400151613454565b60008160028111156132d7576132d7613e6b565b14611bd75760018160028111156132f0576132f0613e6b565b036133165760008281526005602052604090206001015461331190856134ee565b611bd7565b600281600281111561332a5761332a613e6b565b03611bd757600083815260056020526040902060010154613311908661361e565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156133825750600090506003613431565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa1580156133d6573d6000803e3d6000fd5b50506040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0015191505073ffffffffffffffffffffffffffffffffffffffff811661342a57600060019250925050613431565b9150600090505b94509492505050565b60006109858260a001518360c00151846101400151612a52565b60008083600181111561346957613469613e6b565b036134ad57600082600181111561348257613482613e6b565b036134a357670de0b6b3a764000061349a8587613faa565b10159050612131565b5082841015612131565b60008260018111156134c1576134c1613e6b565b036134d0575083831015612131565b670de0b6b3a76400006134e38587613faa565b111595945050505050565b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061352457613524613f4c565b60200260200101818152505060028160018151811061354557613545613f4c565b60209081029190910101527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff166372ce42757f00000000000000000000000000000000000000000000000000000000000000005b6040517fffffffff0000000000000000000000000000000000000000000000000000000060e084901b1681526135f09190600090889087908990600401614116565b600060405180830381600087803b15801561360a57600080fd5b505af1158015612f00573d6000803e3d6000fd5b60408051600280825260608201835260009260208301908036833701905050905060018160008151811061365457613654613f4c565b60200260200101818152505060028160018151811061367557613675613f4c565b60209081029190910101527f000000000000000000000000000000000000000000000000000000000000000073ffffffffffffffffffffffffffffffffffffffff16639e7212ad7f00000000000000000000000000000000000000000000000000000000000000006135ae565b6000602082840312156136f457600080fd5b81357fffffffff0000000000000000000000000000000000000000000000000000000081168114611eb757600080fd5b73ffffffffffffffffffffffffffffffffffffffff81168114610b5c57600080fd5b803561375181613724565b919050565b6000806040838503121561376957600080fd5b823561377481613724565b946020939093013593505050565b60006020828403121561379457600080fd5b8135611eb781613724565b6000602082840312156137b157600080fd5b5035919050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6040516101a0810167ffffffffffffffff8111828210171561380b5761380b6137b8565b60405290565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe016810167ffffffffffffffff81118282101715613858576138586137b8565b604052919050565b80356002811061375157600080fd5b80356003811061375157600080fd5b600082601f83011261388f57600080fd5b813567ffffffffffffffff8111156138a9576138a96137b8565b6138da60207fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f84011601613811565b8181528460208386010111156138ef57600080fd5b816020850160208301376000918101602001919091529392505050565b60006101a0828403121561391f57600080fd5b6139276137e7565b90508135815261393960208301613746565b602082015261394a60408301613746565b604082015261395b60608301613746565b60608201526080820135608082015260a082013560a082015260c082013560c082015260e082013560e08201526101008083013581830152506101208083013581830152506101406139ae818401613860565b908201526101606139c083820161386f565b908201526101808281013567ffffffffffffffff8111156139e057600080fd5b6139ec8582860161387e565b82840152505092915050565b600060208284031215613a0a57600080fd5b813567ffffffffffffffff811115613a2157600080fd5b6121318482850161390c565b600080600060608486031215613a4257600080fd5b505081359360208301359350604090920135919050565b600067ffffffffffffffff821115613a7357613a736137b8565b5060051b60200190565b600082601f830112613a8e57600080fd5b81356020613aa3613a9e83613a59565b613811565b82815260059290921b84018101918181019086841115613ac257600080fd5b8286015b84811015613add5780358352918301918301613ac6565b509695505050505050565b600080600080600060a08688031215613b0057600080fd5b8535613b0b81613724565b94506020860135613b1b81613724565b9350604086013567ffffffffffffffff80821115613b3857600080fd5b613b4489838a01613a7d565b94506060880135915080821115613b5a57600080fd5b613b6689838a01613a7d565b93506080880135915080821115613b7c57600080fd5b50613b898882890161387e565b9150509295509295909350565b600082601f830112613ba757600080fd5b81356020613bb7613a9e83613a59565b82815260059290921b84018101918181019086841115613bd657600080fd5b8286015b84811015613add57803567ffffffffffffffff811115613bfa5760008081fd5b613c088986838b010161390c565b845250918301918301613bda565b60008060408385031215613c2957600080fd5b823567ffffffffffffffff80821115613c4157600080fd5b613c4d86838701613b96565b93506020850135915080821115613c6357600080fd5b50613c7085828601613a7d565b9150509250929050565b60008060408385031215613c8d57600080fd5b50508035926020909101359150565b60008060408385031215613caf57600080fd5b82359150602083013567ffffffffffffffff811115613ccd57600080fd5b613c708582860161390c565b60008060008060808587031215613cef57600080fd5b843567ffffffffffffffff80821115613d0757600080fd5b613d138883890161390c565b95506020870135915080821115613d2957600080fd5b613d3588838901613b96565b9450604087013593506060870135915080821115613d5257600080fd5b50613d5f87828801613a7d565b91505092959194509250565b600080600080600060a08688031215613d8357600080fd5b8535613d8e81613724565b94506020860135613d9e81613724565b93506040860135925060608601359150608086013567ffffffffffffffff811115613dc857600080fd5b613b898882890161387e565b600060208284031215613de657600080fd5b813567ffffffffffffffff811115613dfd57600080fd5b61213184828501613b96565b60008060408385031215613e1c57600080fd5b823567ffffffffffffffff811115613e3357600080fd5b613e3f8582860161390c565b95602094909401359450505050565b600060208284031215613e6057600080fd5b8151611eb781613724565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b60038110613eaa57613eaa613e6b565b9052565b60006101a0820190508e82528d602083015273ffffffffffffffffffffffffffffffffffffffff808e166040840152808d166060840152808c166080840152508960a08301528860c08301528760e083015286610100830152856101208301528461014083015260028410613f2557613f25613e6b565b83610160830152613f3a610180830184613e9a565b9e9d5050505050505050505050505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60008219821115613fbd57613fbd613f7b565b500190565b600082821015613fd457613fd4613f7b565b500390565b60005b83811015613ff4578181015183820152602001613fdc565b8381111561239d5750506000910152565b60008351614017818460208801613fd9565b83519083019061402b818360208801613fd9565b01949350505050565b6000817fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff048311821515161561406c5761406c613f7b565b500290565b6000826140a7577f4e487b7100000000000000000000000000000000000000000000000000000000600052601260045260246000fd5b500490565b6000602082840312156140be57600080fd5b5051919050565b60208152600082518060208401526140e4816040850160208701613fd9565b601f017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0169190910160400192915050565b600060a0820173ffffffffffffffffffffffffffffffffffffffff881683526020878185015286604085015260a0606085015281865180845260c086019150828801935060005b818110156141795784518352938301939183019160010161415d565b5050809350505050826080830152969550505050505056fe608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033a264697066735822122056df26e165b5957191bd0ff149c07ae13f5a6b4252973fb3c07a4653cce0f3b164736f6c634300080f00330000000000000000000000002e8dcfe708d44ae2e406a1c02dfe2fa13012f9610000000000000000000000007d8610e9567d2a6c9fbf66a5a13e9ba8bb120d43000000000000000000000000ab45c5a4b0c941a2f231c04c3f49182e1a254052000000000000000000000000aacfeea03eb1561c4e67d661e40682bd20e3541b",
        "nonce": "0x0",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "addAdmin(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x1107c",
        "value": "0x0",
        "data": "0x70480275000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x1",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "addOperator(address)",
      "arguments": [
        "0x665057d2bDc8F83722435712a98747EE4A7B8aEb"
      ],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x10169",
        "value": "0x0",
        "data": "0x9870d7fe000000000000000000000000665057d2bdc8f83722435712a98747ee4a7b8aeb",
        "nonce": "0x2",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "renounceAdminRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x7d00",
        "value": "0x0",
        "data": "0x83b8a5ae",
        "nonce": "0x3",
        "accessList": []
      },
      "additionalContracts": []
    },
    {
      "hash": null,
      "transactionType": "CALL",
      "contractName": "CTFExchange",
      "contractAddress": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E",
      "function": "renounceOperatorRole()",
      "arguments": [],
      "transaction": {
        "type": "0x02",
        "from": "0x81fd0e5e7372ed171f421a7c33a4b263ea9dcc25",
        "to": "0x4bfb41d5b3570defd03c39a9a4d8de6bd8b8982e",
        "gas": "0x7d34",
        "value": "0x0",
        "data": "0x3d6d3598",
        "nonce": "0x4",
        "accessList": []
      },
      "additionalContracts": []
    }
  ],
  "receipts": [],
  "libraries": [],
  "pending": [
    "0x4cf5ff4362abb630398f45b0ed26787e7b2524c53c4cc006641764f5f8267609"
  ],
  "path": "/home/jonathan/WorkSpace/polymarket/ctf-exchange/broadcast/ExchangeDeployment.s.sol/80001/deployExchange-latest.json",
  "returns": {
    "exchange": {
      "internal_type": "address",
      "value": "0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E"
    }
  },
  "timestamp": 1664228139,
  "commit": "af3ba7f"
}


================================================
FILE: deploy/scripts/deploy_exchange.sh
================================================
#!/usr/bin/env bash

LOCAL=.env.local
TESTNET=.env.testnet
MAINNET=.env

if [ -z $1 ]
then
  echo "usage: deploy_exchange.sh [local || testnet || mainnet]"
  exit 1
elif [ $1 == "local" ]
then
  ENV=$LOCAL
elif [ $1 == "testnet" ]
then
  ENV=$TESTNET
elif [ $1 == "mainnet" ]
then
  ENV=$MAINNET
else
  echo "usage: deploy_exchange.sh [local || testnet || mainnet]"
  exit 1
fi

source $ENV

echo "Deploying CTF Exchange..."

echo "Deploy args:
Admin: $ADMIN
Collateral: $COLLATERAL
ConditionalTokensFramework: $CTF
ProxyFactory: $PROXY_FACTORY
SafeFactory: $SAFE_FACTORY
"

OUTPUT="$(forge script ExchangeDeployment \
    --private-key $PK \
    --rpc-url $RPC_URL \
    --json \
    --broadcast \
    --with-gas-price 200000000000 \
    -s "deployExchange(address,address,address,address,address)" $ADMIN $COLLATERAL $CTF $PROXY_FACTORY $SAFE_FACTORY)"

EXCHANGE=$(echo "$OUTPUT" | grep "{" | jq -r .returns.exchange.value)
echo "Exchange deployed: $EXCHANGE"

echo "Complete!"



================================================
FILE: docs/CTFExchange.md
================================================
# CTFExchange

`CTFExchange` is the core binary limit order exchange contract. It inherits from various library and mixin functions and provides a concise definition of entry points. 
___


## `constructor`

Initializes the abstract contracts it inherits from including `Asset` `Signatures` and `Fees`.

Parameters:

```java
address _collateral // ERC20 collateral asset (USDC)
address _ctf //  ERC1155 outcome tokens contract (gnosis conditional tokens framework)
address _proxyFactory // Polymarket proxy factory
address _safeFactory // Gnosis safe factory contract 
address _feeReceiver // account to accumulate feed to 
```

## `pauseTrading`

Allows admin to pause trading. 

Requirements:

- caller is `admin` (`onlyAdmin`)

## `unpauseTrading`

Allows admin to unpause trading. 

Requirements:

caller is `admin` (`onlyAdmin`)

## `fillOrder`

Fills the fill amount of an order with `msg.sender` as the taker

Parameters:

```java
Order order // The order to be filled
uint256 fillAmount // The amount to be filled, always in terms of the maker amount
```

Requirements:

- caller is `operator` (`onlyOperator`)
- trading is not `paused` (`notPaused`)
- function is being called for first time in control flow or the previous function call has resolved (`nonReentrant`)


## `fillOrders`

Fills an array of orders for the corresponding fill amounts with `msg.sender` as the taker

Parameters:

```java
Order[] orders // The order to be filled
uint256[] fillAmounts // The amounts to be filled, always in terms of the maker amount
```

Requirements:

- caller is `operator` (`onlyOperator`)
- trading is not `paused` (`notPaused`)
- function is being called for first time in control flow or the previous function call has resolved (`nonReentrant`)

## `matchOrders`

Matches a taker order against an array of maker orders for the specified amounts. 

Parameters:

```java
Order takerOrder // The active order to be matched
Order[] makerOrders // The array of maker orders to be matched against the active order
uint256 takerFillAmount // The amount to fill on the taker order, always in terms of the maker amount
uint256[] makerFillAmounts // The array of amounts to fill on the maker orders, always in terms of the maker amount
```

Requirements:

- caller is `operator` (`onlyOperator`)
- trading is not `paused` (`notPaused`)
- function is being called for first time in control flow or the previous function call has resolved (`nonReentrant`)

## `setFeeReceiver`

Sets `feeReceiver` to new address. 

Parameters:

```java
address _feeReceiver // The new fee receiver address
```

Requirements:

- caller is `admin` (`onlyAdmin`)


## `setProxyFactory`

Sets `proxyFactory` to new Polymarket proxy wallet factory address. 

Parameters:

```java
address _newProxyFactory // The new Proxy Wallet factory
```

Requirements:

- caller is `admin` (`onlyAdmin`)

## `setSafeFactory`

Sets `safeFactory` to new gnosis safe factory address.

Parameters:

```java
address _newSafeFactory // The new Safe wallet factory
```
Requirements:

- caller is `admin` (`onlyAdmin`)


## `registerToken`

Registers a tokenId, its complement and its conditionId for trading.

Parameters:

```java
uint256 token // The ERC1155 (ctf) tokenId being registered
uint256 complement // The ERC1155 (ctf) token ID of the complement of token
bytes32 // The corresponding CTF conditionId
```
Requirements:

- caller is `admin` (`onlyAdmin`)


================================================
FILE: docs/Overview.md
================================================
# Exchange 

## Overview

The `CTFExchange` contract facilitates atomic swaps between binary outcome tokens (ERC1155) and the collateral asset (ERC20). It is intended to be used in a hybrid-decentralized exchange model wherein there is an operator that provides matching/ordering/execution services while settlement happens on-chain,non-custodially according to instructions in the form of signed order messages. The CTF exchange allows for matching operations that include a mint/merge operation which allows orders for complementary outcome tokens to be crossed. Orders are represented as signed typed structured data (EIP712). Additionally, the CTFExchange implements symmetric fees. When orders are matched, one side is considered the maker and the other side is considered the taker. The relationship is always either one to one or many to one (maker to taker) and any price improvement is captured by the taking agent. 

## Matching Scenarios 

### Assets

* **`A`** - ERC1155 outcome token
* **`A'`** - ERC1155 outcome token, complement of **`A`**.*
* **`C`** - ERC20 collateral token. 


*\* Complements assumes 1 outcome token and 1 of its complement can always be merged into 1 unit of collateral and 1 unit of collateral can always be split into 1 outcome token and 1 of its complement (ie **`A`** + **`A'`** = **`C`**). Also assume that outcome tokens and collateral have the same decimals/base unit. Finally, the following examples assume **`C`** is USDC for pricing.*

### Scenario 1 - `NORMAL`

#### Maker Order

- **UserA** BUY **100** token **`A`** @ **$0.50**

*(pseudo variables)*
```json
{
  "maker": "userA",
  "makerAsset": "C",
  "takerAsset": "A",
  "makerAmount": 50,
  "takerAmount": 100
}
```

#### Taker Order

- **UserB** SELL **50** token **`A`** @ **$0.50**

*(pseudo variables)*
```json
{
  "maker": "userB",
  "makerAsset": "A",
  "takerAsset": "C",
  "makerAmount": 50,
  "takerAmount": 25
}
```

#### Match Operation Overview

`matchOrders(makerOrder, [takerOrder], 50, [25])`

1. Transfer **50** token **`A`** from **userB** into `CTFExchange`
2. Transfer **25** **`C`** from **userA** into `CTFExchange`
3. Transfer **50** token **`A`** from `CTFExchange` to **userA**
4. Transfer **25** **`C`** from `CTFExchange` to **userB**

### Scenario 2 - `MINT`

#### Maker Order

- **UserA** BUY **100** token **`A`** @ **$0.50**

*(pseudo variables)*
```json
{
  "maker": "userA",
  "makerAsset": "C",
  "takerAsset": "A",
  "makerAmount": 50,
  "takerAmount": 100
}
```

#### Taker Order

- **UserB** BUY **50** token **`A'`** @ **$0.50**

*(pseudo variables)*
```json
{
  "maker": "userB",
  "makerAsset": "C",
  "takerAsset": "A''",
  "makerAmount": 25,
  "takerAmount": 50
}
```

#### Match Operation Overview

`matchOrders(makerOrder, [takerOrder], 25, 25)`

1. Transfer **25** **`C`** from **userB** into `CTFExchange`
2. Transfer **25** **`C`** from **userA** into `CTFExchange`
3. Mint **50** token sets (= **50** token **`A`** + **50** token **`A'`**)
4. Transfer **50** token **`A`** from `CTFExchange` to **userA**
5. Transfer **50** token **`A'`** from `CTFExchange` to **userB**


### Scenario 3 - `MERGE`

#### Maker Order

- **UserA** SELL **50** token **`A`** @ **$0.50**

*(pseudo variables)*
```json
{
  "maker": "userA",
  "makerAsset": "A",
  "takerAsset": "C",
  "makerAmount": 50,
  "takerAmount": 25
}
```

#### Taker Order

- **UserB** SELL **100** token **`A'`** @ **$0.50**

*(pseudo variables)*
```json
{
  "maker": "userB",
  "makerAsset": "A'",
  "takerAsset": "C'",
  "makerAmount": 100,
  "takerAmount": 50
}
```

#### Match Operation Overview

`matchOrders(makerOrder, [takerOrder], 50, 50)`

1. Transfer **50** **`A'`** from **userB** into `CTFExchange`
2. Transfer **50** **`A`** from **userA** into `CTFExchange`
3. Merge **50** token sets into **50** **`C`**(**50** token **`A`** + **50** token **`A'`** = **50** **`C`**)
4. Transfer **25** **`C`** from `CTFExchange` to **userA**
5. Transfer **25** **`C`** from `CTFExchange` to **userB**

## Fees

Fees are levied in the output asset (proceeds). Fees for binary options with a complementary relationship (ie **`A`** + **`A'`** = **`C`**) must be symmetric to preserve market integrity. Symmetric means that someone selling 100 shares of `A` @ $0.99 should pay the same fee value as someone buying 100 `A'` @ $0.01. An intuition for this requires understanding that minting/merging a complementary token set for collateral can happen at any time. Fees are thus implemented in the following manner. 

If buying (ie receiving **`A`** or **`A'`**), the fee is levied on the proceed tokens. If selling (ie receiving **`C`**), the fee is levied on the proceed collateral. The base fee rate (`baseFeeRate`) is signed into the order struct. The base fee rate corresponds to 2x the fee rate (collateral per unit of outcome token) paid by traders when the price of the two tokens is equal (ie $0.50 and $0.50). Moving away from a centered price, the following formulas are used to calculate the fees making sure to maintain symmetry.

usdcFee =  baseRate * min(price, 1-price) * outcomeShareCount

**Case 1:** If selling outcome tokens (base) for collateral (quote):

$feeQuote =  baseRate * \min(price, 1-price) * size$

**Case 2:** If buying outcome tokens (base) with collateral (quote):

$feeBase =  baseRate * \min(price, 1-price) * \frac{size}{price}$

### Fee Examples:

*(assume the full order is filled)*

`baseFeeRate` = 0.02 (usdc/condition)

____

BUY **100** **`A`** @ **$0.50** 

`fee` = 2 **`A`**

($1.00 in value)
___

SELL **100** **`A'`** @ **$0.50** 

`fee` = 1.0 **`C`**

($1.00 in value)
___

BUY **100** **`A`** @ **$0.10** 

`fee` = 2 **`A`**

($0.20 in value)
___

SELL **100** **`A``** @ **$0.90** 

`fee` = 0.20 **`C`**

($0.20 in value)

___

BUY **100** **`A`** @ **$0.90** 

`fee` = .222 **`A`**

($0.20 in value)
___

SELL **100** **`A``** @ **$0.10** 

`fee` = 0.20 **`C`**

($0.20 in value)

## Package Layout

The [`exchange/`]() package includes libraries, mixins, interface definitions and tests supporting the primary contract `CTFExchange`. Mixins are primarily full implementations of related interfaces that are then inherited by the `CTFExchange`. These contracts define the core logic and are supported by library contracts. Mixins are designated as abstract functions because they are intended to always be inherited from. Interfaces are generally separated into those that define function signatures and those that define events and errors (EE).


  




================================================
FILE: docs/mixins/AssetOperations.md
================================================
# Asset Operations

Provides balance fetching, transferring and ctf utilities as an abstract contract. Implements both the `IAssetOperations` and `IAssets` interface. 

## `_getBalance`

Gets the contract's balance of collateral (`tokenID` == 0) or the contract's balance of the conditional token. 

Parameters:

```java
uint256 tokenId // ERC1155 tokenID for ctf, or 0 for getting collateral (ERC20) balance
```

Returns:

```java
uint256 // token balance
```

## `_transfer`

Transfers a quantity of assets, defined by a tokenID, from one address to another address. Calls either `_transferCollateral` or `TransferHelper._transferFromERC1155`. 

Parameters:

```java
address from // account from which to transfer assets
address to // account to which to transfer assets
uint256 id // ID of asset to transfer. ERC1155 tokenID for ctf, or 0 for getting collateral (ERC20) balance
uint256 value // amount of asset to transfer
```

## `_transferCollateral`

Called by `_transfer` in the case that `id` == 0. Transfers ERC20 collateral using the `TransferHelper` library which in turn uses either the `transfer` or `transferFrom` ERC20 interface methods. The choice of transfer method depends on whether or not the from address is the contract itself. 

Parameters:

```java
address from // account from which to transfer the ERC20 tokens
address to // account to which to transfer the ERC20 tokens
uint256 value // amount of ERC20 tokens to transfer
```

## `_mint`

Mints a full conditional token set from collateral by calling the `splitPostion` function ont he ctf contract with the provided `conditionId`. This will convert X units of collateral (ERC20) into X units of complementary outcome tokens (ERC1155). The zeroed bytes32 is used as the `parentCollectionId` and the partition is the simple binary case [1,2]. You can read more about Gnosis Conditional Tokens [here](https://docs.gnosis.io/conditionaltokens/docs/devguide01/).

Parameters:

```java
bytes32 conditionId // id of condition on which to split
uint256 amount // quantity of collateral to split. Note the collateral and minted conditional tokens will use the same number of decimals.
```


## `_merge`

Opposite of `_mint`. Takes complete sets (equal parts of two complementary outcome tokens) and merges (burns) them by calling the `mergePositions` function on the ctf contract with the provided `conditionId`. Specifically this will convert X complete sets (X of token A (ERC1155) and X of its its complement token A' (ERC1155)) into X units of collateral (ERC20). This function assumes merging happens on a binary set and for the zeroed bytes32 `parentCollectionId`. You can read more about Gnosis Conditional Tokens [here](https://docs.gnosis.io/conditionaltokens/docs/devguide01/).

Parameters:

```java
bytes32 conditionId // id of condition on which to merge
uint256 amount // quantity of complete sets to burn for their underlying collateral.
```


================================================
FILE: docs/mixins/Assets.md
================================================
# Assets

Stores the addresses of the ERC20 collateral and ERC155 outcome tokens. 

## `constructor`

Initializes the contract, setting the collateral token address and ctf token address state variables. Also approves the ctf contract to spend usdc on the contract's behalf.

Parameters:

```java
address _collateral // collateral token (ERC20)
address _ctf // ctf outcome token (ERC1155)
```

## `getCollateral`

Gets the stored `collateral` address.

Returns:

```java
address // collateral token address
```

## `getCtf`

Gets the stored `ctf` address.

Returns:

```java
address // ctf address
```


================================================
FILE: docs/mixins/Auth.md
================================================
# Auth

Manages authenticated address with two distinct tiers: `admins` and `operators`. Both roles are represented through mappings allowing for any address to be designated to either role. Admins ultimately have the highest role based access control as they are capable of modifying (adding/removing) the `operators` mapping. All `admins` are equal and can add/remove other `admins`. Initially the contract deployer is the only authorized admin and `operator`. Implements `IAuth` interface.

## `onlyAdmin`

Modifier that reverts in the case that the `msg.sender` is not an admin specifically, it checks the value of `msg.sender` in the `admins` mapping, reverting if it's not `1`. 

## `onlyOperator`

Modifier that reverts in the case that the `msg.sender` is not an operator specifically, it checks the value of `msg.sender` in the `operators` mapping, reverting if it's not `1`. 

## `constructor`

Initializes the contract, designating the deployer as the sole admin and operator.

## `isAdmin`

Gets a boolean indicating whether or not a specified address has been designated as an admin. 

Parameters:

```java
address usr // address to check for admin status
```

Returns:

```java
bool // true if usr is an admin, false if not
```

## `isOperator`

Gets a boolean indicating whether or not a specified address has been designated as an operator. 

Parameters:

```java
address usr // address to check for operator status
```

Returns:

```java
bool // true if usr is an operator, false if not
```

## `addAdmin`

Adds an admin by setting the value of a specified address key to `1` in the `admins` mapping. 

Requirements:

- caller is `admin` (`onlyAdmin`)

Parameters:

```java
address admin_ // address to add as an admin
```

Emits:

- `NewAdmin(admin_, msg.sender)`

## `addOperator`

Adds an operator by setting the value of a specified address key to `1` in the `operators` mapping. 

Requirements:

- caller is `admin` (`onlyAdmin`)

Parameters:

```java
address operator_ // address to add as an operator
```

Emits:

- `NewOperator(operator_, msg.sender)`

## `removeAdmin`

Removes an admin by setting the value of a specified address key to `0` in the `admins` mapping. 

Requirements:

- caller is `admin` (`onlyAdmin`)

Parameters:

```java
address admin // address to remove as an admin
```

Emits:

- `RemovedAdmin(admin, msg.sender)`

## `removeOperator`

Removes an operator by setting the value of a specified address key to `0` in the `operator` mapping. 

Requirements:

- caller is `admin` (`onlyAdmin`)

Parameters:

```java
address operator // address to remove as an admin
```

Emits:

- `RemovedOperator(operator, msg.sender)`


================================================
FILE: docs/mixins/Fees.md
================================================
# Fees

Provides simple utilities related to setting/getting a max fee rate

## `getMaxFeeRate`

Gets the max fee rate which is hard coded to 1000 bps.

Returns:

```java
uint256 // max fee rate that can be signed into an order
```

## `getFeeReceiver`

Gets the fee receiver.

Returns:

```java
address feeReceiver // address to which fees should be sent
```

## `_setFeeReceiver`

Sets a new fee receiver.

Parameters:

```java
address _feeReceiver // address to which fees should be sent
```


================================================
FILE: docs/mixins/Hashing.md
================================================
# Hashing

Provides a simple EIP712 typed structured data hashing utility function. Inherits from Open Zeppelin's draft-EIP712 contract and implements the `IHashing` interface. 

## `constructor`

Initializes the `Hasing` contract, setting the domainSeparator state variable via the parent `EIP712` contract's `_domainSeparatorV4` function and also calls the `EIP712` parent constructor. 

Parameters:

```java
address name // name of the signing domain
address version // current major version of signing domain
```


## `hashOrder`

Hashes an `Order` object according to the EIP712 procedure for hashing and signing of typed structured data. This will mirror the hashing done in client libraries used to prepare and sign orders. 


Parameters:

```java
Order order // order object to hash 
```

Returns:

```java
bytes32 // typed data hash of order object
```


================================================
FILE: docs/mixins/NonceManager.md
================================================
# Nonce Manager

The nonce manager is a mixin responsible for maintaining a mapping of account nonces. These account nonces are used to determine the validity of an order and allow users to cancel orders via nonce changes and increments. Specifically, an order is only valid if the nonce included in the signed order matches the signers current nonce value. Note that nonces can only increase therefore if an account sets their nonce to the max unint256, they will no longer be able to cancel orders via nonce increments. 


## `incrementNonce`

Increments (by 1) an account's nonce. `msg.sender` is used to determine the account of which to increment the nonce for. 

## `updateNonce`

Updates an account's nonce by adding a specific uint256 `val` to the user's current nonce value. Again, `msg.sender` is used to determine the account for nonce addition.

Parameters:

```java
uint256 val // value to add to user's current nonce
```

## `isValidNonce`

Provided a user address and a nonce, returns a boolean indicating whether or not the specified nonce matches the user's nonce stored in the `nonces` state variable mapping. 

Parameters:

```java
address usr // account to match nonce for
uint256 nonce // nonce value to compare against
```

Returns:

```java
bool // indicates whether a supplied nonce matches the user's nonce as stored in nonces mapping
```


================================================
FILE: docs/mixins/Pausable.md
================================================
# Pausable

Used to provide a trading "kill switch". Specifically, the primary entry points to `CTFExchange` are all decorated with the `notPaused` modifier, meaning trading can be paused if needed. This contract provides simple utilities for pausing/unpausing. 

## `notPaused`

Modifier that reverts in the case that the state variable `paused` is `true` (`bool`). Otherwise, execution of modified function continues without disruption.

## `_pauseTrading`

Internal function that sets the `paused` state variable to `true` which result in any `notPaused` decorated function to revert. 

Emits:

- `TradingPaused(msg.sender)`

## `unpauseTrading`

Internal function that sets the `paused` state variable to `false`. This unpauses trading by making the `notPaused` modifier not hit the revert path.

Emits:

- `TradingUnpaused(msg.sender)`





================================================
FILE: docs/mixins/ProxyFactoryHelper.md
================================================
# ProxyFactoryHelper

`PolyFactoryHelper` manages referenced proxy wallet factory addresses and provides wrappers around functions contained in both `PolySafeLib` and `PolyProxyLib` which calculate wallet addresses given the "owning" or "signing" EOA addresses of the proxy wallets. The `CTFExchange` supports two signature types related to contract wallets. Users of Polymarket's interface trade from contract wallets. Originally, these wallets were a custom implementation, but later, Gnosis safes were used. In order to maintain backwards compatibility, both types are supported by the `CTFExchange`. In both cases, the EOA that deploys/creates the proxy wallet is the approved "owner" of that wallet. This means that they are able to sign/execute transaction on the behalf of the contract. User's funds live in these proxy wallets, thus in order to support off-chain order signing (EOAs), the `CTFExchange` must be able to relate a signer to a corresponding wallet address. This contract along with the supporting library functions allow exactly that. 

## `constructor`

Sets the `proxyFactory` and `safeFactory` state variables. 

Parameters:

```java
address _proxyFactory // address of Polymarket proxy wallet factory
address _safeFactory // address of gnosis safe factory
```

## `getProxyFactory`

Getter for the Polymarket proxy factory address.

Returns:

```java
address // address of Polymarket proxy wallet factory
```

## `getSafeFactory`

Getter for the Gnosis safe factory address.

Returns:

```java
address // address of gnosis safe factory
```

## `getPolyProxyFactoryImplementation`

Calls the `getImplementation` function on the `proxyFactory` which should return the address of the proxy wallet implementation that is cloned when a new wallet is created via the factory. 

Returns:

```java
address // the Polymarket Proxy factory implementation
```

## `getSafeFactoryImplementation`

Calls the `masterCopy` function on the `safeFactory` which should return the address of the gnosis safe implementation that is cloned when a new wallet is created via the factory. 

Returns:

```java
address // the Safe factory implementation
```

## `getPolyProxyWalletAddress`

Uses the `PolyProxyLib`'s `computeProxyWalletAddress` function, called with the provided owner address, the stored Polymarket proxy factory implementation address and the proxy factory address to return the wallet address of the owner. 

Parameters:

```java
address _addr // the owner's address for which to calculate their proxy address
```

Returns:

```java
address // the _addr's owned proxy wallet address
```

## `getSafeAddress`

Uses the `PolySafeLib`'s `getSafeAddress` function, called with the provided owner address, the stored safe factory implementation address and the safe factory address to return the wallet address of the owner. 

Parameters:

```java
address _addr // the owner's address for which to calculate their safe address
```

Returns:

```java
address // the _addr's owned safe address
```

## `_setProxyFactory`

Internal function to set the `proxyFactory` address. 

Parameters:

```java
address _proxyFactory // Polymarket proxy factory address
```

Emits:

- `ProxyFactoryUpdated(proxyFactory, _proxyFactory)`


## `_setSafeFactory`

Parameters:

```java
address _safeFactory // Gnosis safe factory address
```

Internal function to set the `safeFactory` address. 

Emits:

- `SafeFactoryUpdated(safeFactory, _safeFactory)`


================================================
FILE: docs/mixins/Registry.md
================================================
# Registry

The `CTFExchange` supports "binary matching". This assumes that two complementary tokens are always worth, in sum, 1 unit of underlying collateral. This is enforced by the CTF contract which always allows minting and merging of full sets (complete collection of outcomes, in our case `A` and its binary complement `A'`). What this ultimately unlocks for the `CTFExchange` is matching between buy orders of `A` and `A'` (via a preceeding "mint" operation), and sell orders of `A` and `A'` (via a succeeding "merge" operation). The `CTFExchange` gets orders to match and is able to determine whether or not a "mint" or "merge" operation is ncessary. The challenge, is that the "mint"/"merge" operation requires knowing the order's base asset's (conditional token) corresponding `conditionId`. Thus, there needs to be a way for the `conditionId` to be gotten from the `tokenId`. The `Registry` is responsible for this function and maintains a mapping of `tokenId`s to `OutcomeToken` objects which include information relating to the specific `tokenId` including the `complement`'s `tokenId`, and the parent `conditionId`. It is the responsibility of operators to register new outcome tokens. Note all methods assume benevolent input by the operator, specifically that they are registering the correct tokenIds/complements/conditions and that they are all binary outcomes that are valid in the context of the CTF contract.


## `getConditionId`

Gets the associated `conditionId` for a `tokenId` by looking it up in the `registry` mapping and returning the `conditionId` value.

Parameters:

```java
uint256 token // token id for which to get conditionId for
```

Returns:

```java
bytes32 // parent conditionId of the token according to the registry
```

## `getComplement`

Gets the complementary `tokenId` for a specified `tokenId` by looking it up in the `registry` mapping and returning the `complement` value. 

Parameters:

```java
uint256 token // token id for which to get complement token id for
```

Returns:

```java
uint256 // complement token id
```

## `validateComplement`

Checks whether the `token` id and `complement` id correspond according to `token`'s value in the `registry` mapping. Reverts if not.

Parameters:

```java
uint256 token // token id for which to check complement
uint256 complement // suspected complement token id of token
```

## `validateTokenId`

Checks whether a valid token id (`!=0`) has been registered. Reverts if not

Parameters:

```java
uint256 tokenId // token id to validate registration for
```

## `validateMatchingTokenIds`

Checks whether the `token0` id and `token1` id are equal if it has been registered. Reverts if not.

Parameters:

```java
uint256 token0 // first token id to compare for equality 
uint256 token1 // second token id to compare for equality
```

## `_registerToken`

Registers complementary token pair.

Parameters:

```java
uint256 token0 // first token id of pair
uint256 token1 // second token id of pair
bytes32 conditionID // cft conditionId for the pair
```

Requirements:

- `token0` and `token1` are not equal
- neither `token0` or `token1` are zero
- neither `token0` or `token` have been registered


Emits:

- `TokenRegistered(token0, token1, conditionId)`
- `TokenRegistered(token1, token0, conditionId)`



================================================
FILE: docs/mixins/Signatures.md
================================================
# Signatures 

The `CTFExchange` supports three distinct signature types:

- **EOA** - ECDSA EIP712 signatures signed by EOAs
- **POLY_PROXY** - EIP712 signatures signed by EOAs that own Polymarket Proxy wallets
- **POLY_GNOSIS_SAFE** - EIP712 signatures signed by EOAs that own Polymarket Gnosis safes

The `Signatures` contract provides functions for validating signatures and associated utilities. 

## `validateOrderSignature`

Validates the signature of an order. Calls `isValidSignature`, reverts if not truthy. 

Parameters:

```java
bytes32 orderHash // the has of the order
Order order // the order which includes the signature
```

## `isValidSignature`

Verifies a signature for signed Order structs. Follows validation paths based on the signature type. Returns boolean indicating signature validity

Parameters:

```java
address signer // Address of the signer
address associated //Address associated with the signer. For POLY_PROXY and POLY_GNOSIS_SAFE signature types, this is the address of the proxy or the safe. For EOA, this is the same as the signer address and is not used.
bytes32 structHash // hash of the struct being verified
bytes signature // signature to be verified
uint256 signatureType // signature type EOA, POLY_PROXY or POLY_GNOSIS_SAFE
```

Returns:

```java
bool // indicates validity of signature
```

## `verifyECDSASignature`

Verifies that a given ECDSA signature was that of the provided `signer` over the given `structHash`. Uses `SilentECDSA` library. Returns boolean indicating signature validity.

Parameters:

```java
address signer // signer address
bytes32 structHash // hash of the struct being verified
bytes signature // signature to be verified
```

Returns:

```java
bool // indicates validity of signature
```

## `verifyPolyProxySignature`

Verifies a signature created by the owner of a Polymarket proxy wallet. Specifically it verifies that:

- ECDSA signature is valid 
- Proxy wallet is owned by the signer

Parameters:

```java
address signer // signer
address proxyWallet // Polymarket proxy wallet (should be one "owned" by signer)
bytes32 structHash // Hash of the struct being verified
bytes signature // Signature to be verified
```

Returns:

```java
bool // indicates validity of signature
```

## `verifyPolySafeSignature`

Verifies a signature created by the owner of a Polymarket Gnosis safe. Specifically it verifies that:

- ECDSA signature is valid 
- PSafe is owned by the signer

Parameters:

```java
address signer // signer
address safeAddress // gnosis safe (should be one "owned" by signer)
bytes32 structHash // Hash of the struct being verified
bytes signature // Signature to be verified
```

Returns:

```java
bool // indicates validity of signature
```

## `getSignatureType`

Returns the associated `SignatureType` enum value provided an index.

Parameters:

```java
uint256 signatureType // index of signature type
```

Returns:

```java
SignatureType // SignatureType enum value of index
```


================================================
FILE: docs/mixins/Trading.md
================================================
# Trading

Trading implements the core exchange logic for trading CTF assets. 

*Note a core assumption that is made is that the collateral and conditional tokens have the same number of decimals. This is true for any CTF token.*

## `getOrderStatus`

Get the status of an order. An order can either be not-filled, partially filled or fully filled. If an order has not been filled, its hash will not exist in the `orderStatus` mapping. If it has been partially filled its hash will exist in this mapping and the maker amount `remaining` will be defined. If the order has been fully filled the hash will exist and the `isCompleted` bool in the `OrderStatus` object will be `true`

Parameters:

```java
bytes32 orderHash // hash of the order
```

Returns:

```java
OrderStatus // status object for the order hash
```

## `validateOrder`

Validates an order. Hashes an order and calls `_validateOrder` with the order hash and order object.

Parameters:

```java
Order order // order to be validated
```

## `cancelOrder`

Cancels an order. Calls `_cancelOrder` with the order. An order can only be cancelled by its maker, the address which holds funds for the order.

Parameters:

```java
Order order // order to be cancelled
```

## `cancelOrders`

Cancels a set of orders by calling `_cancelOrder` on each order is provided order array. 

Parameters:

```java
Order[] orders // orders to be cancelled
```

## `_cancelOrder`

Cancels an order by setting its status to completed.

Requirements:

- order's `maker` must be `msg.sender`
- order cannot have already been filled

Parameters:

```java
Order order // order  to cancel
```

Emits:

- `OrderCancelled(orderHash)`

## `_validateOrder`

Validates an order alongside its hash. Reverts if order is not valid.

Requirements:

- order is not expired
- order fee rate is not greater than configured max fee rate
- order signature is valid for order
- order is not already filled
- order has valid nonce

Parameters:

```java
bytes32 orderHash // hash of order to validate
Order order // order object corresponding to orderHash
```

## `_fillOrder`

Fills an order against the caller. First validates the order, then fills it up to the amount specified by `fillAmount`, updates the status and takes calculated fee. 

Parameters:

```java
Order order // order to fill
uint256 fillAmount // amount to be filled, always in terms of the maker amount
address to // address to receive proceeds from filling the order
```

Emits:

- `emit OrderFilled(orderHash, msg.sender, order.makerAssetId, order.takerAssetId, making, remaining, fee)`


## `_fillOrders`

Fills a set of orders against the caller by calling `_fillOrders` for each order and corresponding fill amount. 

```java
Order[] orders // orders to fill
uint256[] fillAmounts // amounts to be filled for each order in orders, always in terms of the maker amount
address to // address to receive proceeds from filling the orders
```

## `_matchOrders`

Matches a taker order against an array of maker orders up to the amounts specified. Validation is performed to make sure each maker order is able to be filled with the taker order up to the amount specified. The order of transfer operations in the fill is:

1. transfer making amount from taker order to exchange
2. Fill each maker order
   1. Transfer making amount for maker order into exchange
   2. Execute match call (merge or mint)
   3. Transfer taking amount for maker order to the maker order's maker
   4. Fee charged on maker
3. transfer taking amount (calculated based on maker order fills, will include any price improvement for buying) to taker order maker
4. Fee charged on taker
5. transfer any excess making amount left from exchange to taker order maker (price improvement in case of selling)

Requirements:

- all orders valid
- making amounts are valid for each order
- taker order provides enough assets for the filling of all maker orders to the amounts specified
- each maker order is marketable against taker order
- taker gets at least as much proceeds as they expect

Parameters:

```java
Order takerOrder // taker order to be matched
Order[] makerOrders // array of maker orders to be matched against the taker order
uint256 takerFillAmount // amount to fill on the taker order, in terms of the maker amount
uint256[] memory makerFillAmounts // array of amounts to fill on the maker orders, in terms of the maker amount
```

Emits:

- `OrderFilled(orderHash, address(this), takerOrder.makerAssetId, takerOrder.takerAssetId, making, remaining, fee)`
- `OrdersMatched(orderHash, takerOrder.makerAssetId, takerOrder.takerAssetId, making, taking)`

## `_fillMakerOrders`

Fills an array of maker orders for the specified amounts. 

Parameters:

```java
Order takerOrder // taker order
Order[] makerOrders // maker orders
uint256[] makerFillAmounts // maker amounts to fill on each maker order
```

## `_fillMakerOrder`

Fills a maker order. In doing so, validates it is marketable with a supplied taker order, derives the pre/post matching operation and charges fees.

Requirements:

- valid taker and maker order
- maker and taker order can be crossed
- amount provided is fillable for maker order

Parameters:

```java
Order takerOrder // taker order object
Order makerOrder // maker order object
uint256 fillAmount // maker amount to be filled on makerOrder
```

Emits:

- `OrderFilled(hashOrder(makerOrder), takerOrder.maker, makerOrder.makerAssetId, makerOrder.takerAssetId, making, remaining, fee)`


## `_validateOrderAndCalcTaking`

Performs common order validation and calculates taking amount for a matched order. The taking amount is proportional to the making amount that is being filled. Additionally the order status is updated to reflect the amount that is being filled. 

Requirements:

- Order is valid
- Making amount can be filled on order

Parameters:

```java
Order order // order being validated
uint256 making // maker amount to be filled of order
```

Returns:

```java
uint256 takingAmount // amount of taking amount corresponding to supplied taking amount 
uint256 remainingAmount // maker amount remaining on the order. 
```

## `_fillFacingExchange`

Fills a maker order using the Exchange as the counterparty. Follows the following steps:

1. Transfers makingAmount of maker asset from the order maker to the exchange
2. Executes the match call
   1. In the case a buy + sell is being matching nothing happens
   2. In the case a buy + buy is being matched, a mint (split) happens, since the taker order's collateral is already available and the maker order's collateral was just transferred there should be enough to mint takingAmount.
   3. In the case a sell + sell is being matched a merge happens, since the taker order's conditional tokens will have already been transferred to the exchange and the taker order's conditional tokens were just transferred, there should be enough conditional tokens to merge makingAmount.
3. Transfer taking amount of taker asset to the order maker

Parameters:

```java
uint256 makingAmount // Amount to be filled in terms of maker amount
uint256 takingAmount // Amount to be filled in terms of taker amount
Order order // the order to be filed
MatchType matchType // the match type
```

## `_deriveMatchType`

Provided a taker and maker order determines the matching operation that is needed. 

Parameters:

```java
Order takerOrder // the taker order
Order makerOrder // the maker order
```

Returns:

```java
MatchType // type of match NORMAL, MINT or MERGE
```

## `_executeMatchCall`

Executes a CTF call to match orders by minting new Outcome tokens or merging Outcome tokens into collateral.

Parameters:

```java
uint256 makingAmount // Amount to be filled in terms of maker amount, used as amount in merge case
uint256 takingAmount // Amount to be filled in terms of taker amount, used as amount in mint case
Order order // order to be filled
MatchType matchType // the match type
```

## `_validateTakerAndMaker`

Ensures the taker and maker orders can be matched against each other.

Requirements:

- orders are crossing
- in case of NORMAL, conditional tokenIds match across maker and taker order
- in case of MINT, conditional tokenIds should be complementary
- in case of MERGE, conditional tokenIds should be complementary

Parameters:

```java
Order takerOrder // the taker order
Order makerOrder // the maker order
MatchType matchType // the match type
```

## `_chargeFee`

Charges a fee from a payer to the receiver.

Parameters:

```java
address payer // fee payer
address receiver // fee recipient
uint256 tokenId // token id of fee, 0 if collateral
uint256 fee // fee amount
```

Emits:

- `FeeReceived(payer, receiver, tokenId, fee)`

## `_updateOrderStatus`

Updates the order status. Will mark as completed if the making amount plus any already filled amount of order is equal to total order size, otherwise will calculate and store the remaining amount. 

Parameters:

```java
bytes32 orderHash // order hash
Order order // order object
uint256 makingAmount // making amount
```

Returns:

```java
uint256 // remaining maker amount for order
```

## `_updateTakingWithSurplus`

Checks to see how much of the tokenId the exchange contract has received and verifies it is greater than the min amount and returns the max(actualAmount, minimumAmount).

Parameters:

```java
uint256 minimumAmount // minimum amount exchange should have of tokenId
uint256 tokenId // tokenId to get balance of
```

Returns:

```java
uint256 // amount of tokenId in contract
```


================================================
FILE: src/common/ERC20.sol
================================================
// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity ^0.8.15;

import {IERC20} from "common/interfaces/IERC20.sol";

/// @notice Forked to have non-constant decimals, to set after deployment.
/// @author Polymarket
/// @author Modified from Solmate (https://github.com/Rari-Capital/solmate/blob/main/src/utils/ReentrancyGuard.sol)
/// @dev Do not manually set balances without updating totalSupply, as the sum of all user balances must not exceed it.
abstract contract ERC20 is IERC20 {
    /*//////////////////////////////////////////////////////////////
                                 EVENTS
    //////////////////////////////////////////////////////////////*/

    event Transfer(address indexed from, address indexed to, uint256 amount);

    event Approval(address indexed owner, address indexed spender, uint256 amount);

    /*//////////////////////////////////////////////////////////////
                            METADATA STORAGE
    //////////////////////////////////////////////////////////////*/

    string public name;

    string public symbol;

    uint8 public decimals;

    /*//////////////////////////////////////////////////////////////
                              ERC20 STORAGE
    //////////////////////////////////////////////////////////////*/

    uint256 public totalSupply;

    mapping(address => uint256) public balanceOf;

    mapping(address => mapping(address => uint256)) public allowance;

    /*//////////////////////////////////////////////////////////////
                            EIP-2612 STORAGE
    //////////////////////////////////////////////////////////////*/

    uint256 internal immutable INITIAL_CHAIN_ID;

    bytes32 internal immutable INITIAL_DOMAIN_SEPARATOR;

    mapping(address => uint256) public nonces;

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    constructor(string memory _name, string memory _symbol, uint8 _decimals) {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;

        INITIAL_CHAIN_ID = block.chainid;
        INITIAL_DOMAIN_SEPARATOR = computeDomainSeparator();
    }

    /*//////////////////////////////////////////////////////////////
                               ERC20 LOGIC
    //////////////////////////////////////////////////////////////*/

    function approve(address spender, uint256 amount) public virtual returns (bool) {
        allowance[msg.sender][spender] = amount;

        emit Approval(msg.sender, spender, amount);

        return true;
    }

    function transfer(address to, uint256 amount) public virtual returns (bool) {
        balanceOf[msg.sender] -= amount;

        // Cannot overflow because the sum of all user
        // balances can't exceed the max uint256 value.
        unchecked {
            balanceOf[to] += amount;
        }

        emit Transfer(msg.sender, to, amount);

        return true;
    }

    function transferFrom(address from, address to, uint256 amount) public virtual returns (bool) {
        uint256 allowed = allowance[from][msg.sender]; // Saves gas for limited approvals.

        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;

        balanceOf[from] -= amount;

        // Cannot overflow because the sum of all user
        // balances can't exceed the max uint256 value.
        unchecked {
            balanceOf[to] += amount;
        }

        emit Transfer(from, to, amount);

        return true;
    }

    /*//////////////////////////////////////////////////////////////
                             EIP-2612 LOGIC
    //////////////////////////////////////////////////////////////*/

    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s)
        public
        virtual
    {
        require(deadline >= block.timestamp, "PERMIT_DEADLINE_EXPIRED");

        // Unchecked because the only math done is incrementing
        // the owner's nonce which cannot realistically overflow.
        unchecked {
            address recoveredAddress = ecrecover(
                keccak256(
                    abi.encodePacked(
                        "\x19\x01",
                        DOMAIN_SEPARATOR(),
                        keccak256(
                            abi.encode(
                                keccak256(
                                    "Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)"
                                ),
                                owner,
                                spender,
                                value,
                                nonces[owner]++,
                                deadline
                            )
                        )
                    )
                ),
                v,
                r,
                s
            );

            require(recoveredAddress != address(0) && recoveredAddress == owner, "INVALID_SIGNER");

            allowance[recoveredAddress][spender] = value;
        }

        emit Approval(owner, spender, value);
    }

    function DOMAIN_SEPARATOR() public view virtual returns (bytes32) {
        return block.chainid == INITIAL_CHAIN_ID ? INITIAL_DOMAIN_SEPARATOR : computeDomainSeparator();
    }

    function computeDomainSeparator() internal view virtual returns (bytes32) {
        return keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes(name)),
                keccak256("1"),
                block.chainid,
                address(this)
            )
        );
    }

    /*//////////////////////////////////////////////////////////////
                        INTERNAL MINT/BURN LOGIC
    //////////////////////////////////////////////////////////////*/

    function _mint(address to, uint256 amount) internal virtual {
        totalSupply += amount;

        // Cannot overflow because the sum of all user
        // balances can't exceed the max uint256 value.
        unchecked {
            balanceOf[to] += amount;
        }

        emit Transfer(address(0), to, amount);
    }

    function _burn(address from, uint256 amount) internal virtual {
        balanceOf[from] -= amount;

        // Cannot underflow because a user's balance
        // will never be larger than the total supply.
        unchecked {
            totalSupply -= amount;
        }

        emit Transfer(from, address(0), amount);
    }
}



================================================
FILE: src/common/ReentrancyGuard.sol
================================================
// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity >=0.8.0;

/// @notice Forked from Solmate to handle clones.
/// @author Polymarket
/// @author Modified from Solmate (https://github.com/Rari-Capital/solmate/blob/main/src/utils/ReentrancyGuard.sol)
abstract contract ReentrancyGuard {
    uint256 private locked = 1;

    modifier nonReentrant() virtual {
        require(locked != 2, "REENTRANCY");

        locked = 2;

        _;

        locked = 1;
    }
}



================================================
FILE: src/common/auth/Authorized.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Owned} from "solmate/auth/Owned.sol";
import {IAuthorized, IAuthorizedEE} from "common/auth/interfaces/IAuthorized.sol";

abstract contract Authorized is Owned, IAuthorized {
    mapping(address => bool) public authorized;

    constructor(address _owner) Owned(_owner) {}

    modifier onlyAuthorized() {
        if (!authorized[msg.sender]) revert OnlyAuthorized();
        _;
    }

    function addAuthorization(address _account) external onlyOwner {
        authorized[_account] = true;

        emit AuthorizationAdded(_account);
    }

    function removeAuthorization(address _account) external onlyOwner {
        authorized[_account] = false;

        emit AuthorizationRemoved(_account);
    }
}



================================================
FILE: src/common/auth/Ownable.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

interface IOwnable {
    error OnlyOwner();
}

///@notice Ownable with transfer and accept
///@notice We might not use this for anything.
abstract contract Ownable is IOwnable {
    address public owner;
    address public nextOwner;

    constructor(address _owner) {
        owner = _owner;
    }

    modifier onlyOwner() {
        if (msg.sender != owner) revert OnlyOwner();
        _;
    }

    function transferOwnership(address _nextOwner) external onlyOwner {
        nextOwner = _nextOwner;
    }

    function acceptOwnership() external {
        if (msg.sender != nextOwner) revert();
        owner = msg.sender;
    }
}



================================================
FILE: src/common/auth/Owned.sol
================================================
// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity >=0.8.0;

import {IOwned, IOwnedEE} from "common/auth/interfaces/IOwned.sol";

/// @notice Forked from solmate to add interface and custom errors.
/// @notice Simple single owner authorization mixin.
/// @author Modified from Solmate (https://github.com/transmissions11/solmate/blob/main/src/auth/Owned.sol)
abstract contract Owned is IOwned {
    /*//////////////////////////////////////////////////////////////
                            OWNERSHIP STORAGE
    //////////////////////////////////////////////////////////////*/

    address public owner;

    modifier onlyOwner() virtual {
        if (msg.sender != owner) revert OnlyOwner();

        _;
    }

    /*//////////////////////////////////////////////////////////////
                               CONSTRUCTOR
    //////////////////////////////////////////////////////////////*/

    constructor(address _owner) {
        owner = _owner;

        emit OwnerUpdated(address(0), _owner);
    }

    /*//////////////////////////////////////////////////////////////
                             OWNERSHIP LOGIC
    //////////////////////////////////////////////////////////////*/

    function setOwner(address newOwner) public virtual onlyOwner {
        owner = newOwner;

        emit OwnerUpdated(msg.sender, newOwner);
    }
}



================================================
FILE: src/common/auth/interfaces/IAuthorized.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

interface IAuthorizedEE {
    event AuthorizationAdded(address _account);
    event AuthorizationRemoved(address _account);

    error OnlyAuthorized();
}

interface IAuthorized is IAuthorizedEE {
    function addAuthorization(address _account) external;

    function removeAuthorization(address _account) external;
}



================================================
FILE: src/common/auth/interfaces/IOwned.sol
================================================
// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity >=0.8.0;

interface IOwnedEE {
    event OwnerUpdated(address indexed user, address indexed newOwner);

    error OnlyOwner();
}

interface IOwned is IOwnedEE {
    function setOwner(address newOwner) external;
}



================================================
FILE: src/common/interfaces/IERC20.sol
================================================
// SPDX-License-Identifier: AGPL-3.0-only
pragma solidity >=0.8.0;

// interface for Solmate ERC20
interface IERC20 {
    // event Approval(address indexed owner, address indexed spender, uint256 amount);
    // event Transfer(address indexed from, address indexed to, uint256 amount);

    function DOMAIN_SEPARATOR() external view returns (bytes32);

    function allowance(address, address) external view returns (uint256);

    function approve(address spender, uint256 amount) external returns (bool);

    function balanceOf(address) external view returns (uint256);

    function decimals() external view returns (uint8);

    function name() external view returns (string memory);

    function nonces(address) external view returns (uint256);

    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s)
        external;

    function symbol() external view returns (string memory);

    function totalSupply() external view returns (uint256);

    function transfer(address to, uint256 amount) external returns (bool);

    function transferFrom(address from, address to, uint256 amount) external returns (bool);
}



================================================
FILE: src/common/libraries/SafeTransferLib.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {SafeTransferLib, ERC20} from "solmate/utils/SafeTransferLib.sol";



================================================
FILE: src/dev/TestHelper.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test, console2 as console, stdStorage, StdStorage} from "forge-std/Test.sol";
import {ERC20} from "common/ERC20.sol";
import {TestMath} from "dev/libraries/TestMath.sol";

abstract contract TestHelper is Test {
    using TestMath for uint64;
    using TestMath for uint256;

    mapping(address => mapping(address => uint256)) private balanceCheckpoints;

    address alice = address(1);
    address brian = address(2);
    address carly = address(3);
    address dylan = address(4);
    address erica = address(5);
    address frank = address(6);
    address grace = address(7);
    address henry = address(8);

    constructor() {
        vm.label(alice, "alice");
        vm.label(brian, "brian");
        vm.label(carly, "carly");
        vm.label(dylan, "dylan");
        vm.label(erica, "erica");
        vm.label(frank, "frank");
        vm.label(grace, "grace");
        vm.label(henry, "henry");
    }

    modifier with(address _account) {
        vm.startPrank(_account);
        _;
        vm.stopPrank();
    }

    function hashAddress(bytes memory _digest) internal pure returns (address) {
        return address(uint160(uint256(keccak256(_digest))));
    }

    function assertBalance(address _token, address _who, uint256 _amount) internal {
        assertEq(ERC20(_token).balanceOf(_who), balanceCheckpoints[_token][_who] + _amount);
    }

    function checkpointBalance(address _token, address _who) internal {
        balanceCheckpoints[_token][_who] = ERC20(_token).balanceOf(_who);
    }

    function balanceOf(address _token, address _who) internal view returns (uint256) {
        return ERC20(_token).balanceOf(_who);
    }

    function approve(address _token, address _spender, uint256 _amount) internal {
        ERC20(_token).approve(_spender, _amount);
    }

    ///@dev msg.sender is the owner of the approved tokens
    function dealAndApprove(address _token, address _to, address _spender, uint256 _amount) internal {
        deal(_token, _to, _amount);
        approve(_token, _spender, _amount);
    }

    function add(uint256 _a, uint256 _b) internal pure returns (uint256) {
        return _a + _b;
    }

    function advance(uint256 _delta) internal {
        vm.roll(block.number + _delta);
    }

    function _address(bytes memory _seed) internal pure returns (address) {
        return address(bytes20(keccak256(_seed)));
    }
}



================================================
FILE: src/dev/libraries/TestMath.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

library TestMath {
    // these binary operators implicitly cast smaller uints
    // into uint256 before the operatiuon
    function mul(uint256 _x, uint256 _y) internal pure returns (uint256) {
        return _x * _y;
    }

    function add(uint256 _x, uint256 _y) internal pure returns (uint256) {
        return _x + _y;
    }
}



================================================
FILE: src/dev/mocks/ERC1271Mock.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { ECDSA } from "openzeppelin-contracts/utils/cryptography/ECDSA.sol";

contract ERC1271Mock {
    address public signer;

    bytes4 internal constant MAGIC_VALUE_1271 = 0x1626ba7e;

    constructor(address _signer) {
        signer = _signer;
    }

    function isValidSignature(bytes32 hash, bytes memory signature) public view returns (bytes4) {
        return ECDSA.recover(hash, signature) == signer ? MAGIC_VALUE_1271 : bytes4(0);
    }
}



================================================
FILE: src/dev/mocks/ERC20.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {ERC20 as BaseERC20} from "solmate/tokens/ERC20.sol";

/// @dev always has 18 decimals
contract ERC20 is BaseERC20 {
    constructor(string memory _name, string memory _symbol) BaseERC20(_name, _symbol, 18) {}

    function mint(address _to, uint256 _amount) external {
        _mint(_to, _amount);
    }
}



================================================
FILE: src/dev/mocks/USDC.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {ERC20} from "common/ERC20.sol";

contract USDC is ERC20 {
    constructor() ERC20("USDC", "USDC", 6) {}

    function mint(address _to, uint256 _amount) external {
        _mint(_to, _amount);
    }
}



================================================
FILE: src/dev/script/callTest.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script, console2 as console} from "forge-std/Script.sol";
import {ERC20} from "openzeppelin/token/ERC20/ERC20.sol";

contract call_test is Script {
    // returns true
    function run(address _callee) public returns (bool) {
        address from = address(1);
        address to = address(2);
        uint256 value = 1;
        (bool success,) = _callee.call(abi.encodeWithSelector(ERC20.transferFrom.selector, from, to, value));
        return (success);
    }

    // returns false
    function run() public returns (bool) {
        address token = address(new ERC20("", ""));
        address from = address(1);
        address to = address(2);
        uint256 value = 1;
        (bool success,) = token.call(abi.encodeWithSelector(ERC20.transferFrom.selector, from, to, value));
        return (success);
    }
}



================================================
FILE: src/dev/script/ffi.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test, console2 as console} from "forge-std/Test.sol";

// simple demo of ffi with echo
contract ffi is Test {
    function run() public {
        string[] memory inputs = new string[](3);
        inputs[0] = "echo";
        inputs[1] = "-n";
        inputs[2] = "0xcafe";
        // or
        // inputs[2] = 'cafe';

        bytes memory result = vm.ffi(inputs);
        console.logBytes(result);
    }
}



================================================
FILE: src/dev/script/poolBytecodeHash.s.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script, console2 as console} from "forge-std/Test.sol";
import {Json} from "dev/util/Json.sol";

contract PoolBytecodeHash is Script {
    function run() public {
        bytes memory result = Json.readData("artifacts/UniswapV3Pool.json", ".bytecode");
        console.logBytes32(keccak256(result));
    }
}



================================================
FILE: src/dev/script/useSolcVersion.s.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script, console2 as console} from "forge-std/Script.sol";
import {ERC20} from "openzeppelin/token/ERC20/ERC20.sol";

// forces forge to download the specified solc version
contract useSolcVersion is Script {
    // returns true
    function run() public view returns (bool) {
        console.log("using 0.8.15");
        return true;
    }
}



================================================
FILE: src/dev/script/ZeroTx.s.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script, console2 as console} from "forge-std/Test.sol";
import {Json} from "dev/util/Json.sol";

contract ZeroTx is Script {
    function run() public {
        vm.startBroadcast();
        payable(address(this)).transfer(0);
    }
}



================================================
FILE: src/dev/util/Ascii.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

/// @notice ascii functions
library Ascii {
    /// @notice converts a uint256 to string
    /// @param _value, uint256, the value to convert
    /// @return result the resulting string
    function encodeUint256(uint256 _value) internal pure returns (string memory result) {
        if (_value == 0) return "0";

        assembly {
            // largest uint = 2^256-1 has 78 digits
            // reserve 110 = 78 + 32 bytes of data in memory
            // (first 32 are for string length)

            // get 110 bytes of free memory
            result := add(mload(0x40), 110)
            mstore(0x40, result)

            // keep track of digits
            let digits := 0

            for {} gt(_value, 0) {} {
                // increment digits
                digits := add(digits, 1)
                // go back one byte
                result := sub(result, 1)
                // compute ascii char
                let c := add(mod(_value, 10), 48)
                // store byte
                mstore8(result, c)
                // advance to next digit
                _value := div(_value, 10)
            }
            // go back 32 bytes
            result := sub(result, 32)
            // store the length
            mstore(result, digits)
        }
    }

    function encodeBytes(bytes memory _data) internal pure returns (string memory) {
        if (_data.length == 0) return "00";
        string memory table = "0123456789abcdef";
        uint256 length = _data.length;
        bytes memory result = new bytes(2 * length + 2);
        assembly {
            //
            let resultPtr := add(result, 32)
            //
            let tablePtr := add(table, 1)
            //
            let dataPtr := add(_data, 1)

            // write two bytes '0x' at most significant digits
            // this is actually not necessary for ffi
            mstore8(resultPtr, 48)
            resultPtr := add(resultPtr, 1)
            mstore8(resultPtr, 120)
            resultPtr := add(resultPtr, 1)

            for { let i := 0 } lt(i, length) { i := add(i, 1) } {
                let c := mload(dataPtr)
                // first 4 bits
                let c1 := and(0x0f, shr(4, c))
                // second 4 bits
                let c2 := and(0x0f, c)

                mstore8(resultPtr, mload(add(tablePtr, c1)))
                resultPtr := add(resultPtr, 1)
                mstore8(resultPtr, mload(add(tablePtr, c2)))
                resultPtr := add(resultPtr, 1)

                dataPtr := add(dataPtr, 1)
            }
        }
        return string(result);
    }
}



================================================
FILE: src/dev/util/Deployer.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {vm} from "dev/util/vm.sol";
import {Json} from "dev/util/Json.sol";

library Deployer {
    function deployCode(string memory _what) internal returns (address addr) {
        addr = deployCode(_what, "", "");
    }

    function deployCode(string memory _what, bytes memory _args, string memory _salt) internal returns (address addr) {
        bytes memory bytecode = abi.encodePacked(vm.std_cheats.getCode(_what), _args);
        assembly {
            addr := create2(0, add(bytecode, 0x20), mload(bytecode), _salt)
        }
    }

    function deployBytecode(bytes memory _initcode, bytes memory _args, string memory _salt)
        internal
        returns (address addr)
    {
        bytes memory bytecode = abi.encodePacked(_initcode, _args);
        assembly {
            addr := create2(0, add(bytecode, 0x20), mload(bytecode), _salt)
        }
    }

    function ConditionalTokens() public returns (address) {
        bytes memory initcode = Json.readData("artifacts/ConditionalTokens.json", ".bytecode.object");
        return deployBytecode(initcode, "", "");
    }
}



================================================
FILE: src/dev/util/Io.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test, console2 as console} from "forge-std/Test.sol";
import {vm} from "./vm.sol";

library Io {
    string constant tempFolder = "tmp";

    function read(string memory filePath) internal returns (bytes memory) {
        string[] memory c = new string[](3);
        c[0] = "bash";
        c[1] = "-c";
        c[2] = string.concat("cat ", filePath);

        bytes memory result = vm.std_cheats.ffi(c);
        return result;
    }

    function write(string memory filePath, string memory _content) internal {
        _prepareTempFolder();
        string[] memory c = new string[](3);
        c[0] = "bash";
        c[1] = "-c";
        c[2] = string.concat("echo -n ", _content, " > ", filePath);

        vm.std_cheats.ffi(c);
    }

    function _prepareTempFolder() internal {
        string[] memory c = new string[](3);
        c[0] = "bash";
        c[1] = "-c";
        // c[2] = 'cast ae "x(string)" $(pwd)';
        c[2] = "mkdir -p tmp && echo -n 0x00";
        // c[0] = string.concat("mdkir -p ", tempFolder);

        vm.std_cheats.ffi(c);
    }
}



================================================
FILE: src/dev/util/Json.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Test, console2 as console} from "forge-std/Test.sol";
import {vm} from "./vm.sol";

library Json {
    function read(string memory _path, string memory _filter) internal returns (bytes memory) {
        string[] memory c = new string[](3);
        c[0] = "bash";
        c[1] = "-c";
        c[2] = string.concat('cast ae "response(bytes)" $(jq -j ', _filter, " ", _path, " | xxd -p)");
        // in general should dump with xxd -p (or whatever)
        bytes memory data = vm.std_cheats.ffi(c);

        return data;
    }

    function readData(string memory _path, string memory _filter) internal returns (bytes memory) {
        string[] memory c = new string[](3);
        c[0] = "bash";
        c[1] = "-c";
        c[2] = string.concat('cast ae "response(bytes)" $(jq -j ', _filter, " ", _path, ")");

        bytes memory data = vm.std_cheats.ffi(c);
        bytes memory result = abi.decode(data, (bytes));

        return result;
    }

    // function write(string memory filePath, string memory data) internal {
    //     string[] memory writeInputs = new string[](3);
    //     writeInputs[0] = "scripts/io_write.sh";
    //     writeInputs[1] = filePath;
    //     writeInputs[2] = data;

    //     vm.std_cheats.ffi(writeInputs);
    // }
}



================================================
FILE: src/dev/util/Log.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {console} from "forge-std/console.sol";
import {vm} from "dev/util/vm.sol";
import {Ascii} from "dev/util/Ascii.sol";

library Log {
    function logERC20(string memory label, uint256 value) internal {
        string[] memory inputs = new string[](2);
        inputs[0] = "scripts/formatERC20.sh";
        inputs[1] = string(Ascii.encodeUint256(value));

        string memory result = string(vm.std_cheats.ffi(inputs));
        console.log(string.concat(label, ": ", result));
    }

    function logX96(string memory label, uint256 value) internal {
        string[] memory inputs = new string[](2);
        inputs[0] = "scripts/formatX96.sh";
        inputs[1] = string(Ascii.encodeUint256(value));

        string memory result = string(vm.std_cheats.ffi(inputs));
        console.log(string.concat(label, ": ", result));
    }
}



================================================
FILE: src/dev/util/Predictor.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {vm} from "./vm.sol";
import {Ascii} from "./Ascii.sol";

// modified from https://ethereum.stackexchange.com/questions/760/how-is-the-address-of-an-ethereum-contract-computed
library Predictor {
    function addressFrom(address _origin, uint256 _nonce) public pure returns (address) {
        if (_nonce == 0x00) return addressHash(abi.encodePacked(bytes1(0xd6), bytes1(0x94), _origin, bytes1(0x80)));
        if (_nonce <= 0x7f) {
            return addressHash(abi.encodePacked(bytes1(0xd6), bytes1(0x94), _origin, bytes1(uint8(_nonce))));
        }
        if (_nonce <= 0xff) {
            return addressHash(abi.encodePacked(bytes1(0xd7), bytes1(0x94), _origin, bytes1(0x81), uint8(_nonce)));
        }
        if (_nonce <= 0xffff) {
            return addressHash(abi.encodePacked(bytes1(0xd8), bytes1(0x94), _origin, bytes1(0x82), uint16(_nonce)));
        }
        if (_nonce <= 0xffffff) {
            return addressHash(abi.encodePacked(bytes1(0xd9), bytes1(0x94), _origin, bytes1(0x83), uint24(_nonce)));
        }
        return addressHash(abi.encodePacked(bytes1(0xda), bytes1(0x94), _origin, bytes1(0x84), uint32(_nonce))); // more than 2^32 nonces not realisti);
    }

    function addressHash(bytes memory _digest) public pure returns (address) {
        return address(uint160(uint256(keccak256(_digest))));
    }
}



================================================
FILE: src/dev/util/Reader.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {vm} from "./vm.sol";

library Reader {
    function read(string memory _path) internal returns (bytes memory) {
        string[] memory c = new string[](3);
        c[0] = "bash";
        c[1] = "-c";
        c[2] = string.concat('cast ae "response(bytes)" $(xxd -p ', _path, ")");

        bytes memory result = vm.std_cheats.ffi(c);
        return result;
    }

    // function write(string memory filePath, string memory data) internal {
    //     string[] memory writeInputs = new string[](3);
    //     writeInputs[0] = "scripts/io_write.sh";
    //     writeInputs[1] = filePath;
    //     writeInputs[2] = data;

    //     vm.std_cheats.ffi(writeInputs);
    // }
}



================================================
FILE: src/dev/util/vm.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Vm} from "forge-std/Vm.sol";

library vm {
    Vm constant std_cheats = Vm(0x7109709ECfa91a80626fF3989D68f67F5b1DD12D);
}



================================================
FILE: src/dev/util/script/prepareTempFolder.s.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import {Script, console2 as console} from "forge-std/Script.sol";
import {Io} from "dev/util/Io.sol";

// forces forge to download the specified solc version
contract prepareTempFolder is Script {
    // returns true
    function run() public returns (bool) {
        Io._prepareTempFolder();
        return true;
    }
}



================================================
FILE: src/exchange/BaseExchange.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { ERC1155Holder } from "openzeppelin-contracts/token/ERC1155/utils/ERC1155Holder.sol";
import { ReentrancyGuard } from "common/ReentrancyGuard.sol";

abstract contract BaseExchange is ERC1155Holder, ReentrancyGuard { }



================================================
FILE: src/exchange/CTFExchange.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity 0.8.15;

import { Auth } from "./mixins/Auth.sol";
import { Fees } from "./mixins/Fees.sol";
import { Assets } from "./mixins/Assets.sol";
import { Hashing } from "./mixins/Hashing.sol";
import { Trading } from "./mixins/Trading.sol";
import { Registry } from "./mixins/Registry.sol";
import { Pausable } from "./mixins/Pausable.sol";
import { Signatures } from "./mixins/Signatures.sol";
import { NonceManager } from "./mixins/NonceManager.sol";
import { AssetOperations } from "./mixins/AssetOperations.sol";

import { BaseExchange } from "./BaseExchange.sol";

import { Order } from "./libraries/OrderStructs.sol";

/// @title CTF Exchange
/// @notice Implements logic for trading CTF assets
/// @author Polymarket
contract CTFExchange is
    BaseExchange,
    Auth,
    Assets,
    Fees,
    Pausable,
    AssetOperations,
    Hashing("Polymarket CTF Exchange", "1"),
    NonceManager,
    Registry,
    Signatures,
    Trading
{
    constructor(address _collateral, address _ctf, address _proxyFactory, address _safeFactory)
        Assets(_collateral, _ctf)
        Signatures(_proxyFactory, _safeFactory)
    { }

    /*//////////////////////////////////////////////////////////////
                        PAUSE
    //////////////////////////////////////////////////////////////*/

    /// @notice Pause trading on the Exchange
    function pauseTrading() external onlyAdmin {
        _pauseTrading();
    }

    /// @notice Unpause trading on the Exchange
    function unpauseTrading() external onlyAdmin {
        _unpauseTrading();
    }

    /*//////////////////////////////////////////////////////////////
                        TRADING
    //////////////////////////////////////////////////////////////*/

    /// @notice Fills an order
    /// @param order        - The order to be filled
    /// @param fillAmount   - The amount to be filled, always in terms of the maker amount
    function fillOrder(Order memory order, uint256 fillAmount) external nonReentrant onlyOperator notPaused {
        _fillOrder(order, fillAmount, msg.sender);
    }

    /// @notice Fills a set of orders
    /// @param orders       - The order to be filled
    /// @param fillAmounts  - The amounts to be filled, always in terms of the maker amount
    function fillOrders(Order[] memory orders, uint256[] memory fillAmounts)
        external
        nonReentrant
        onlyOperator
        notPaused
    {
        _fillOrders(orders, fillAmounts, msg.sender);
    }

    /// @notice Matches a taker order against a list of maker orders
    /// @param takerOrder       - The active order to be matched
    /// @param makerOrders      - The array of maker orders to be matched against the active order
    /// @param takerFillAmount  - The amount to fill on the taker order, always in terms of the maker amount
    /// @param makerFillAmounts - The array of amounts to fill on the maker orders, always in terms of the maker amount
    function matchOrders(
        Order memory takerOrder,
        Order[] memory makerOrders,
        uint256 takerFillAmount,
        uint256[] memory makerFillAmounts
    ) external nonReentrant onlyOperator notPaused {
        _matchOrders(takerOrder, makerOrders, takerFillAmount, makerFillAmounts);
    }

    /*//////////////////////////////////////////////////////////////
                        CONFIGURATION
    //////////////////////////////////////////////////////////////*/

    /// @notice Sets a new Proxy Wallet factory for the Exchange
    /// @param _newProxyFactory - The new Proxy Wallet factory
    function setProxyFactory(address _newProxyFactory) external onlyAdmin {
        _setProxyFactory(_newProxyFactory);
    }

    /// @notice Sets a new safe factory for the Exchange
    /// @param _newSafeFactory  - The new Safe wallet factory
    function setSafeFactory(address _newSafeFactory) external onlyAdmin {
        _setSafeFactory(_newSafeFactory);
    }

    /// @notice Registers a tokenId, its complement and its conditionId for trading on the Exchange
    /// @param token        - The tokenId being registered
    /// @param complement   - The complement of the tokenId
    /// @param conditionId  - The CTF conditionId
    function registerToken(uint256 token, uint256 complement, bytes32 conditionId) external onlyAdmin {
        _registerToken(token, complement, conditionId);
    }
}



================================================
FILE: src/exchange/interfaces/IAssetOperations.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

abstract contract IAssetOperations {
    function _getBalance(uint256 tokenId) internal virtual returns (uint256);

    function _transfer(address from, address to, uint256 id, uint256 value) internal virtual;

    function _mint(bytes32 conditionId, uint256 amount) internal virtual;

    function _merge(bytes32 conditionId, uint256 amount) internal virtual;
}



================================================
FILE: src/exchange/interfaces/IAssets.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

abstract contract IAssets {
    function getCollateral() public virtual returns (address);

    function getCtf() public virtual returns (address);
}



================================================
FILE: src/exchange/interfaces/IAuth.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

interface IAuthEE {
    error NotAdmin();
    error NotOperator();

    /// @notice Emitted when a new admin is added
    event NewAdmin(address indexed newAdminAddress, address indexed admin);

    /// @notice Emitted when a new operator is added
    event NewOperator(address indexed newOperatorAddress, address indexed admin);

    /// @notice Emitted when an admin is removed
    event RemovedAdmin(address indexed removedAdmin, address indexed admin);

    /// @notice Emitted when an operator is removed
    event RemovedOperator(address indexed removedOperator, address indexed admin);
}

interface IAuth is IAuthEE {
    function isAdmin(address) external view returns (bool);

    function isOperator(address) external view returns (bool);

    function addAdmin(address) external;

    function addOperator(address) external;

    function removeAdmin(address) external;

    function removeOperator(address) external;

    function renounceAdminRole() external;

    function renounceOperatorRole() external;
}



================================================
FILE: src/exchange/interfaces/IConditionalTokens.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { IERC20 } from "openzeppelin-contracts/token/ERC20/IERC20.sol";

/// @title IConditionalTokens
/// @notice Interface for the Gnosis ConditionalTokensFramework: https://github.com/gnosis/conditional-tokens-contracts/blob/master/contracts/ConditionalTokens.sol
interface IConditionalTokens {
    function payoutNumerators(bytes32 conditionId, uint256 index) external view returns (uint256);

    function payoutDenominator(bytes32 conditionId) external view returns (uint256);

    /// @dev This function prepares a condition by initializing a payout vector associated with the condition.
    /// @param oracle The account assigned to report the result for the prepared condition.
    /// @param questionId An identifier for the question to be answered by the oracle.
    /// @param outcomeSlotCount The number of outcome slots which should be used for this condition. Must not exceed 256.
    function prepareCondition(address oracle, bytes32 questionId, uint256 outcomeSlotCount) external;

    /// @dev Called by the oracle for reporting results of conditions. Will set the payout vector for the condition with the ID ``keccak256(abi.encodePacked(oracle, questionId, outcomeSlotCount))``, where oracle is the message sender, questionId is one of the parameters of this function, and outcomeSlotCount is the length of the payouts parameter, which contains the payoutNumerators for each outcome slot of the condition.
    /// @param questionId The question ID the oracle is answering for
    /// @param payouts The oracle's answer
    function reportPayouts(bytes32 questionId, uint256[] calldata payouts) external;

    /// @dev This function splits a position. If splitting from the collateral, this contract will attempt to transfer `amount` collateral from the message sender to itself. Otherwise, this contract will burn `amount` stake held by the message sender in the position being split worth of EIP 1155 tokens. Regardless, if successful, `amount` stake will be minted in the split target positions. If any of the transfers, mints, or burns fail, the transaction will revert. The transaction will also revert if the given partition is trivial, invalid, or refers to more slots than the condition is prepared with.
    /// @param collateralToken The address of the positions' backing collateral token.
    /// @param parentCollectionId The ID of the outcome collections common to the position being split and the split target positions. May be null, in which only the collateral is shared.
    /// @param conditionId The ID of the condition to split on.
    /// @param partition An array of disjoint index sets representing a nontrivial partition of the outcome slots of the given condition. E.g. A|B and C but not A|B and B|C (is not disjoint). Each element's a number which, together with the condition, represents the outcome collection. E.g. 0b110 is A|B, 0b010 is B, etc.
    /// @param amount The amount of collateral or stake to split.
    function splitPosition(
        IERC20 collateralToken,
        bytes32 parentCollectionId,
        bytes32 conditionId,
        uint256[] calldata partition,
        uint256 amount
    ) external;

    /// @dev This function merges CTF tokens into the underlying collateral.
    /// @param collateralToken The address of the positions' backing collateral token.
    /// @param parentCollectionId The ID of the outcome collections common to the position being split and the split target positions. May be null, in which only the collateral is shared.
    /// @param conditionId The ID of the condition to split on.
    /// @param partition An array of disjoint index sets representing a nontrivial partition of the outcome slots of the given condition. E.g. A|B and C but not A|B and B|C (is not disjoint). Each element's a number which, together with the condition, represents the outcome collection. E.g. 0b110 is A|B, 0b010 is B, etc.
    /// @param amount The amount of collateral or stake to split.
    function mergePositions(
        IERC20 collateralToken,
        bytes32 parentCollectionId,
        bytes32 conditionId,
        uint256[] calldata partition,
        uint256 amount
    ) external;

    /// @dev This function redeems a CTF ERC1155 token for the underlying collateral
    /// @param collateralToken The address of the positions' backing collateral token.
    /// @param parentCollectionId The ID of the outcome collections common to the position
    /// @param conditionId The ID of the condition to split on.
    /// @param indexSets Index sets of the outcome collection to combine with the parent outcome collection
    function redeemPositions(
        IERC20 collateralToken,
        bytes32 parentCollectionId,
        bytes32 conditionId,
        uint256[] calldata indexSets
    ) external;

    /// @dev Gets the outcome slot count of a condition.
    /// @param conditionId ID of the condition.
    /// @return Number of outcome slots associated with a condition, or zero if condition has not been prepared yet.
    function getOutcomeSlotCount(bytes32 conditionId) external view returns (uint256);

    /// @dev Constructs a condition ID from an oracle, a question ID, and the outcome slot count for the question.
    /// @param oracle The account assigned to report the result for the prepared condition.
    /// @param questionId An identifier for the question to be answered by the oracle.
    /// @param outcomeSlotCount The number of outcome slots which should be used for this condition. Must not exceed 256.
    function getConditionId(address oracle, bytes32 questionId, uint256 outcomeSlotCount)
        external
        pure
        returns (bytes32);

    /// @dev Constructs an outcome collection ID from a parent collection and an outcome collection.
    /// @param parentCollectionId Collection ID of the parent outcome collection, or bytes32(0) if there's no parent.
    /// @param conditionId Condition ID of the outcome collection to combine with the parent outcome collection.
    /// @param indexSet Index set of the outcome collection to combine with the parent outcome collection.
    function getCollectionId(bytes32 parentCollectionId, bytes32 conditionId, uint256 indexSet)
        external
        view
        returns (bytes32);

    /// @dev Constructs a position ID from a collateral token and an outcome collection. These IDs are used as the ERC-1155 ID for this contract.
    /// @param collateralToken Collateral token which backs the position.
    /// @param collectionId ID of the outcome collection associated with this position.
    function getPositionId(IERC20 collateralToken, bytes32 collectionId) external pure returns (uint256);
}



================================================
FILE: src/exchange/interfaces/IFees.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

interface IFeesEE {
    error FeeTooHigh();

    /// @notice Emitted when a fee is charged
    event FeeCharged(address indexed receiver, uint256 tokenId, uint256 amount);
}

abstract contract IFees is IFeesEE {
    function getMaxFeeRate() public pure virtual returns (uint256);
}



================================================
FILE: src/exchange/interfaces/IHashing.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { Order } from "../libraries/OrderStructs.sol";

abstract contract IHashing {
    function hashOrder(Order memory order) public view virtual returns (bytes32);
}



================================================
FILE: src/exchange/interfaces/INonceManager.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

abstract contract INonceManager {
    function incrementNonce() external virtual;

    function isValidNonce(address user, uint256 userNonce) public view virtual returns (bool);
}



================================================
FILE: src/exchange/interfaces/IPausable.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

interface IPausableEE {
    error Paused();

    event TradingPaused(address indexed pauser);

    event TradingUnpaused(address indexed pauser);
}

abstract contract IPausable is IPausableEE {
    function _pauseTrading() internal virtual;

    function _unpauseTrading() internal virtual;
}



================================================
FILE: src/exchange/interfaces/IRegistry.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

interface IRegistryEE {
    error InvalidComplement();
    error InvalidTokenId();
    error AlreadyRegistered();

    /// @notice Emitted when a token is registered
    event TokenRegistered(uint256 indexed token0, uint256 indexed token1, bytes32 indexed conditionId);
}

abstract contract IRegistry is IRegistryEE {
    function getConditionId(uint256 tokenId) public view virtual returns (bytes32);

    function getComplement(uint256 tokenId) public view virtual returns (uint256);

    function validateTokenId(uint256 tokenId) public view virtual;

    function validateComplement(uint256 token0, uint256 token1) public view virtual;
}



================================================
FILE: src/exchange/interfaces/ISignatures.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { Order } from "../libraries/OrderStructs.sol";

interface ISignaturesEE {
    error InvalidSignature();
}

abstract contract ISignatures is ISignaturesEE {
    function validateOrderSignature(bytes32 orderHash, Order memory order) public view virtual;
}



================================================
FILE: src/exchange/interfaces/ITrading.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { OrderStatus, Order } from "../libraries/OrderStructs.sol";

interface ITradingEE {
    error NotOwner();
    error NotTaker();
    error OrderFilledOrCancelled();
    error OrderExpired();
    error InvalidNonce();
    error MakingGtRemaining();
    error NotCrossing();
    error TooLittleTokensReceived();
    error MismatchedTokenIds();

    /// @notice Emitted when an order is cancelled
    event OrderCancelled(bytes32 indexed orderHash);

    /// @notice Emitted when an order is filled
    event OrderFilled(
        bytes32 indexed orderHash,
        address indexed maker,
        address indexed taker,
        uint256 makerAssetId,
        uint256 takerAssetId,
        uint256 makerAmountFilled,
        uint256 takerAmountFilled,
        uint256 fee
    );

    /// @notice Emitted when a set of orders is matched
    event OrdersMatched(
        bytes32 indexed takerOrderHash,
        address indexed takerOrderMaker,
        uint256 makerAssetId,
        uint256 takerAssetId,
        uint256 makerAmountFilled,
        uint256 takerAmountFilled
    );
}

interface ITrading is ITradingEE { }



================================================
FILE: src/exchange/libraries/CalculatorHelper.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { Order, Side } from "../libraries/OrderStructs.sol";

library CalculatorHelper {
    uint256 internal constant ONE = 10 ** 18;

    uint256 internal constant BPS_DIVISOR = 10_000;

    function calculateTakingAmount(uint256 makingAmount, uint256 makerAmount, uint256 takerAmount)
        internal
        pure
        returns (uint256)
    {
        if (makerAmount == 0) return 0;
        return makingAmount * takerAmount / makerAmount;
    }

    /// @notice Calculates the fee for an order
    /// @dev Fees are calculated based on amount of outcome tokens and the order's feeRate
    /// @param feeRateBps       - Fee rate, in basis points
    /// @param outcomeTokens    - The number of outcome tokens
    /// @param makerAmount      - The maker amount of the order
    /// @param takerAmount      - The taker amount of the order
    /// @param side             - The side of the order
    function calculateFee(
        uint256 feeRateBps,
        uint256 outcomeTokens,
        uint256 makerAmount,
        uint256 takerAmount,
        Side side
    ) internal pure returns (uint256 fee) {
        if (feeRateBps > 0) {
            uint256 price = _calculatePrice(makerAmount, takerAmount, side);
            if (price > 0 && price <= ONE) {
                if (side == Side.BUY) {
                    // Fee charged on Token Proceeds:
                    // baseRate * min(price, 1-price) * (outcomeTokens/price)
                    fee = (feeRateBps * min(price, ONE - price) * outcomeTokens) / (price * BPS_DIVISOR);
                } else {
                    // Fee charged on Collateral proceeds:
                    // baseRate * min(price, 1-price) * outcomeTokens
                    fee = feeRateBps * min(price, ONE - price) * outcomeTokens / (BPS_DIVISOR * ONE);
                }
            }
        }
    }

    function min(uint256 a, uint256 b) internal pure returns (uint256) {
        return a < b ? a : b;
    }

    function calculatePrice(Order memory order) internal pure returns (uint256) {
        return _calculatePrice(order.makerAmount, order.takerAmount, order.side);
    }

    function _calculatePrice(uint256 makerAmount, uint256 takerAmount, Side side) internal pure returns (uint256) {
        if (side == Side.BUY) return takerAmount != 0 ? makerAmount * ONE / takerAmount : 0;
        return makerAmount != 0 ? takerAmount * ONE / makerAmount : 0;
    }

    function isCrossing(Order memory a, Order memory b) internal pure returns (bool) {
        if (a.takerAmount == 0 || b.takerAmount == 0) return true;

        return _isCrossing(calculatePrice(a), calculatePrice(b), a.side, b.side);
    }

    function _isCrossing(uint256 priceA, uint256 priceB, Side sideA, Side sideB) internal pure returns (bool) {
        if (sideA == Side.BUY) {
            if (sideB == Side.BUY) {
                // if a and b are bids
                return priceA + priceB >= ONE;
            }
            // if a is bid and b is ask
            return priceA >= priceB;
        }
        if (sideB == Side.BUY) {
            // if a is ask and b is bid
            return priceB >= priceA;
        }
        // if a and b are asks
        return priceA + priceB <= ONE;
    }
}



================================================
FILE: src/exchange/libraries/OrderStructs.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

bytes32 constant ORDER_TYPEHASH = keccak256(
    "Order(uint256 salt,address maker,address signer,address taker,uint256 tokenId,uint256 makerAmount,uint256 takerAmount,uint256 expiration,uint256 nonce,uint256 feeRateBps,uint8 side,uint8 signatureType)"
);

struct Order {
    /// @notice Unique salt to ensure entropy
    uint256 salt;
    /// @notice Maker of the order, i.e the source of funds for the order
    address maker;
    /// @notice Signer of the order
    address signer;
    /// @notice Address of the order taker. The zero address is used to indicate a public order
    address taker;
    /// @notice Token Id of the CTF ERC1155 asset to be bought or sold
    /// If BUY, this is the tokenId of the asset to be bought, i.e the makerAssetId
    /// If SELL, this is the tokenId of the asset to be sold, i.e the takerAssetId
    uint256 tokenId;
    /// @notice Maker amount, i.e the maximum amount of tokens to be sold
    uint256 makerAmount;
    /// @notice Taker amount, i.e the minimum amount of tokens to be received
    uint256 takerAmount;
    /// @notice Timestamp after which the order is expired
    uint256 expiration;
    /// @notice Nonce used for onchain cancellations
    uint256 nonce;
    /// @notice Fee rate, in basis points, charged to the order maker, charged on proceeds
    uint256 feeRateBps;
    /// @notice The side of the order: BUY or SELL
    Side side;
    /// @notice Signature type used by the Order: EOA, POLY_PROXY or POLY_GNOSIS_SAFE
    SignatureType signatureType;
    /// @notice The order signature
    bytes signature;
}

enum SignatureType {
    // 0: ECDSA EIP712 signatures signed by EOAs
    EOA,
    // 1: EIP712 signatures signed by EOAs that own Polymarket Proxy wallets
    POLY_PROXY,
    // 2: EIP712 signatures signed by EOAs that own Polymarket Gnosis safes
    POLY_GNOSIS_SAFE,
    // 3: EIP1271 signatures signed by smart contracts. To be used by smart contract wallets or vaults
    POLY_1271
}

enum Side {
    // 0: buy
    BUY,
    // 1: sell
    SELL
}

enum MatchType {
    // 0: buy vs sell
    COMPLEMENTARY,
    // 1: both buys
    MINT,
    // 2: both sells
    MERGE
}

struct OrderStatus {
    bool isFilledOrCancelled;
    uint256 remaining;
}



================================================
FILE: src/exchange/libraries/PolyProxyLib.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

/// @notice Helper library to compute polymarket proxy wallet addresses
library PolyProxyLib {
    /// @notice Gets the polymarket proxy address for a signer
    /// @param signer - Address of the signer
    function getProxyWalletAddress(address signer, address implementation, address deployer)
        internal
        pure
        returns (address proxyWallet)
    {
        return _computeCreate2Address(deployer, implementation, keccak256(abi.encodePacked(signer)));
    }

    function _computeCreate2Address(address from, address target, bytes32 salt) internal pure returns (address) {
        bytes32 bytecodeHash = keccak256(_computeCreationCode(from, target));
        bytes32 _data = keccak256(abi.encodePacked(bytes1(0xff), from, salt, bytecodeHash));
        return address(uint160(uint256(_data)));
    }

    function _computeCreationCode(address deployer, address target) internal pure returns (bytes memory clone) {
        bytes memory consData = abi.encodeWithSignature("cloneConstructor(bytes)", new bytes(0));
        bytes memory buffer = new bytes(99);
        assembly {
            mstore(add(buffer, 0x20), 0x3d3d606380380380913d393d73bebebebebebebebebebebebebebebebebebebe)
            mstore(add(buffer, 0x2d), mul(deployer, 0x01000000000000000000000000))
            mstore(add(buffer, 0x41), 0x5af4602a57600080fd5b602d8060366000396000f3363d3d373d3d3d363d73be)
            mstore(add(buffer, 0x60), mul(target, 0x01000000000000000000000000))
            mstore(add(buffer, 116), 0x5af43d82803e903d91602b57fd5bf30000000000000000000000000000000000)
        }
        // clone = bytes.concat(buffer, consData);
        clone = abi.encodePacked(buffer, consData);
        return clone;
    }
}



================================================
FILE: src/exchange/libraries/PolySafeLib.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

/// @title PolySafeLib
/// @notice Helper library to compute Polymarket gnosis safe addresses
library PolySafeLib {
    bytes private constant proxyCreationCode =
        hex"608060405234801561001057600080fd5b5060405161017138038061017183398101604081905261002f916100b9565b6001600160a01b0381166100945760405162461bcd60e51b815260206004820152602260248201527f496e76616c69642073696e676c65746f6e20616464726573732070726f766964604482015261195960f21b606482015260840160405180910390fd5b600080546001600160a01b0319166001600160a01b03929092169190911790556100e7565b6000602082840312156100ca578081fd5b81516001600160a01b03811681146100e0578182fd5b9392505050565b607c806100f56000396000f3fe6080604052600080546001600160a01b0316813563530ca43760e11b1415602857808252602082f35b3682833781823684845af490503d82833e806041573d82fd5b503d81f3fea264697066735822122015938e3bf2c49f5df5c1b7f9569fa85cc5d6f3074bb258a2dc0c7e299bc9e33664736f6c63430008040033";

    /// @notice Gets the Polymarket Gnosis safe address for a signer
    /// @param signer   - Address of the signer
    /// @param deployer - Address of the deployer contract
    function getSafeAddress(address signer, address implementation, address deployer)
        internal
        pure
        returns (address safe)
    {
        bytes32 bytecodeHash = keccak256(getContractBytecode(implementation));
        bytes32 salt = keccak256(abi.encode(signer));
        safe = _computeCreate2Address(deployer, bytecodeHash, salt);
    }

    function getContractBytecode(address masterCopy) internal pure returns (bytes memory) {
        return abi.encodePacked(proxyCreationCode, abi.encode(masterCopy));
    }

    function _computeCreate2Address(address deployer, bytes32 bytecodeHash, bytes32 salt)
        internal
        pure
        returns (address)
    {
        bytes32 _data = keccak256(abi.encodePacked(bytes1(0xff), deployer, salt, bytecodeHash));
        return address(uint160(uint256(_data)));
    }
}



================================================
FILE: src/exchange/libraries/TransferHelper.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { IERC1155 } from "openzeppelin-contracts/token/ERC1155/IERC1155.sol";

import { SafeTransferLib, ERC20 } from "common/libraries/SafeTransferLib.sol";

/// @title TransferHelper
/// @notice Helper method to transfer tokens
library TransferHelper {
    /// @notice Transfers tokens from msg.sender to a recipient
    /// @param token    - The contract address of the token which will be transferred
    /// @param to       - The recipient of the transfer
    /// @param amount   - The amount to be transferred
    function _transferERC20(address token, address to, uint256 amount) internal {
        SafeTransferLib.safeTransfer(ERC20(token), to, amount);
    }

    /// @notice Transfers tokens from the targeted address to the given destination
    /// @param token    - The contract address of the token to be transferred
    /// @param from     - The originating address from which the tokens will be transferred
    /// @param to       - The destination address of the transfer
    /// @param amount   - The amount to be transferred
    function _transferFromERC20(address token, address from, address to, uint256 amount) internal {
        SafeTransferLib.safeTransferFrom(ERC20(token), from, to, amount);
    }

    /// @notice Transfer an ERC1155 token
    /// @param token    - The contract address of the token to be transferred
    /// @param from     - The originating address from which the tokens will be transferred
    /// @param to       - The destination address of the transfer
    /// @param id       - The tokenId of the token to be transferred
    /// @param amount   - The amount to be transferred
    function _transferFromERC1155(address token, address from, address to, uint256 id, uint256 amount) internal {
        IERC1155(token).safeTransferFrom(from, to, id, amount, "");
    }

    /// @notice Transfers a set of ERC1155 tokens
    /// @param token    - The contract address of the token to be transferred
    /// @param from     - The originating address from which the tokens will be transferred
    /// @param to       - The destination address of the transfer
    /// @param ids      - The tokenId of the token to be transferred
    /// @param amounts  - The amount to be transferred
    function _batchTransferFromERC1155(
        address token,
        address from,
        address to,
        uint256[] memory ids,
        uint256[] memory amounts
    ) internal {
        IERC1155(token).safeBatchTransferFrom(from, to, ids, amounts, "");
    }
}



================================================
FILE: src/exchange/mixins/AssetOperations.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { IERC20 } from "openzeppelin-contracts/token/ERC20/IERC20.sol";
import { IERC1155 } from "openzeppelin-contracts/token/ERC1155/IERC1155.sol";

import { IAssets } from "../interfaces/IAssets.sol";
import { IAssetOperations } from "../interfaces/IAssetOperations.sol";
import { IConditionalTokens } from "../interfaces/IConditionalTokens.sol";

import { TransferHelper } from "../libraries/TransferHelper.sol";

/// @title Asset Operations
/// @notice Operations on the CTF and Collateral assets
abstract contract AssetOperations is IAssetOperations, IAssets {
    bytes32 public constant parentCollectionId = bytes32(0);

    function _getBalance(uint256 tokenId) internal override returns (uint256) {
        if (tokenId == 0) return IERC20(getCollateral()).balanceOf(address(this));
        return IERC1155(getCtf()).balanceOf(address(this), tokenId);
    }

    function _transfer(address from, address to, uint256 id, uint256 value) internal override {
        if (id == 0) return _transferCollateral(from, to, value);
        return _transferCTF(from, to, id, value);
    }

    function _transferCollateral(address from, address to, uint256 value) internal {
        address token = getCollateral();
        if (from == address(this)) TransferHelper._transferERC20(token, to, value);
        else TransferHelper._transferFromERC20(token, from, to, value);
    }

    function _transferCTF(address from, address to, uint256 id, uint256 value) internal {
        TransferHelper._transferFromERC1155(getCtf(), from, to, id, value);
    }

    function _mint(bytes32 conditionId, uint256 amount) internal override {
        uint256[] memory partition = new uint256[](2);
        partition[0] = 1;
        partition[1] = 2;
        IConditionalTokens(getCtf()).splitPosition(
            IERC20(getCollateral()), parentCollectionId, conditionId, partition, amount
        );
    }

    function _merge(bytes32 conditionId, uint256 amount) internal override {
        uint256[] memory partition = new uint256[](2);
        partition[0] = 1;
        partition[1] = 2;

        IConditionalTokens(getCtf()).mergePositions(
            IERC20(getCollateral()), parentCollectionId, conditionId, partition, amount
        );
    }
}



================================================
FILE: src/exchange/mixins/Assets.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { IERC20 } from "openzeppelin-contracts/token/ERC20/IERC20.sol";

import { IAssets } from "../interfaces/IAssets.sol";

abstract contract Assets is IAssets {
    address internal immutable collateral;
    address internal immutable ctf;

    constructor(address _collateral, address _ctf) {
        collateral = _collateral;
        ctf = _ctf;
        IERC20(collateral).approve(ctf, type(uint256).max);
    }

    function getCollateral() public view override returns (address) {
        return collateral;
    }

    function getCtf() public view override returns (address) {
        return ctf;
    }
}



================================================
FILE: src/exchange/mixins/Auth.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { IAuth } from "../interfaces/IAuth.sol";

/// @title Auth
/// @notice Provides admin and operator roles and access control modifiers
abstract contract Auth is IAuth {
    /// @dev The set of addresses authorized as Admins
    mapping(address => uint256) public admins;

    /// @dev The set of addresses authorized as Operators
    mapping(address => uint256) public operators;

    modifier onlyAdmin() {
        if (admins[msg.sender] != 1) revert NotAdmin();
        _;
    }

    modifier onlyOperator() {
        if (operators[msg.sender] != 1) revert NotOperator();
        _;
    }

    constructor() {
        admins[msg.sender] = 1;
        operators[msg.sender] = 1;
    }

    function isAdmin(address usr) external view returns (bool) {
        return admins[usr] == 1;
    }

    function isOperator(address usr) external view returns (bool) {
        return operators[usr] == 1;
    }

    /// @notice Adds a new admin
    /// Can only be called by a current admin
    /// @param admin_ - The new admin
    function addAdmin(address admin_) external onlyAdmin {
        admins[admin_] = 1;
        emit NewAdmin(admin_, msg.sender);
    }

    /// @notice Adds a new operator
    /// Can only be called by a current admin
    /// @param operator_ - The new operator
    function addOperator(address operator_) external onlyAdmin {
        operators[operator_] = 1;
        emit NewOperator(operator_, msg.sender);
    }

    /// @notice Removes an existing Admin
    /// Can only be called by a current admin
    /// @param admin - The admin to be removed
    function removeAdmin(address admin) external onlyAdmin {
        admins[admin] = 0;
        emit RemovedAdmin(admin, msg.sender);
    }

    /// @notice Removes an existing operator
    /// Can only be called by a current admin
    /// @param operator - The operator to be removed
    function removeOperator(address operator) external onlyAdmin {
        operators[operator] = 0;
        emit RemovedOperator(operator, msg.sender);
    }

    /// @notice Removes the admin role for the caller
    /// Can only be called by an existing admin
    function renounceAdminRole() external onlyAdmin {
        admins[msg.sender] = 0;
        emit RemovedAdmin(msg.sender, msg.sender);
    }

    /// @notice Removes the operator role for the caller
    /// Can only be called by an exiting operator
    function renounceOperatorRole() external onlyOperator {
        operators[msg.sender] = 0;
        emit RemovedOperator(msg.sender, msg.sender);
    }
}



================================================
FILE: src/exchange/mixins/Fees.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { IFees } from "../interfaces/IFees.sol";

abstract contract Fees is IFees {
    /// @notice Maximum fee rate that can be signed into an Order
    uint256 internal constant MAX_FEE_RATE_BIPS = 1000; // 1000 bips or 10%

    /// @notice Returns the maximum fee rate for an order
    function getMaxFeeRate() public pure override returns (uint256) {
        return MAX_FEE_RATE_BIPS;
    }
}



================================================
FILE: src/exchange/mixins/Hashing.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { EIP712 } from "openzeppelin-contracts/utils/cryptography/draft-EIP712.sol";

import { IHashing } from "../interfaces/IHashing.sol";

import { Order, ORDER_TYPEHASH } from "../libraries/OrderStructs.sol";

abstract contract Hashing is EIP712, IHashing {
    bytes32 public immutable domainSeparator;

    constructor(string memory name, string memory version) EIP712(name, version) {
        domainSeparator = _domainSeparatorV4();
    }

    /// @notice Computes the hash for an order
    /// @param order - The order to be hashed
    function hashOrder(Order memory order) public view override returns (bytes32) {
        return _hashTypedDataV4(
            keccak256(
                abi.encode(
                    ORDER_TYPEHASH,
                    order.salt,
                    order.maker,
                    order.signer,
                    order.taker,
                    order.tokenId,
                    order.makerAmount,
                    order.takerAmount,
                    order.expiration,
                    order.nonce,
                    order.feeRateBps,
                    order.side,
                    order.signatureType
                )
            )
        );
    }
}



================================================
FILE: src/exchange/mixins/NonceManager.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { INonceManager } from "../interfaces/INonceManager.sol";

abstract contract NonceManager is INonceManager {
    mapping(address => uint256) public nonces;

    function incrementNonce() external override {
        updateNonce(1);
    }

    function updateNonce(uint256 val) internal {
        nonces[ msg.sender] = nonces[ msg.sender] + val;
    }

    function isValidNonce(address usr, uint256 nonce) public view override returns (bool) {
        return nonces[ usr] == nonce;
    }
}



================================================
FILE: src/exchange/mixins/Pausable.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { IPausable } from "../interfaces/IPausable.sol";

abstract contract Pausable is IPausable {
    bool public paused = false;

    modifier notPaused() {
        if (paused) revert Paused();
        _;
    }

    function _pauseTrading() internal override {
        paused = true;
        emit TradingPaused(msg.sender);
    }

    function _unpauseTrading() internal override {
        paused = false;
        emit TradingUnpaused(msg.sender);
    }
}



================================================
FILE: src/exchange/mixins/PolyFactoryHelper.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { PolySafeLib } from "../libraries/PolySafeLib.sol";
import { PolyProxyLib } from "../libraries/PolyProxyLib.sol";

interface IPolyProxyFactory {
    function getImplementation() external view returns (address);
}

interface IPolySafeFactory {
    function masterCopy() external view returns (address);
}

abstract contract PolyFactoryHelper {
    /// @notice The Polymarket Proxy Wallet Factory Contract
    address public proxyFactory;
    /// @notice The Polymarket Gnosis Safe Factory Contract
    address public safeFactory;

    event ProxyFactoryUpdated(address indexed oldProxyFactory, address indexed newProxyFactory);

    event SafeFactoryUpdated(address indexed oldSafeFactory, address indexed newSafeFactory);

    constructor(address _proxyFactory, address _safeFactory) {
        proxyFactory = _proxyFactory;
        safeFactory = _safeFactory;
    }

    /// @notice Gets the Proxy factory address
    function getProxyFactory() public view returns (address) {
        return proxyFactory;
    }

    /// @notice Gets the Safe factory address
    function getSafeFactory() public view returns (address) {
        return safeFactory;
    }

    /// @notice Gets the Polymarket Proxy factory implementation address
    function getPolyProxyFactoryImplementation() public view returns (address) {
        return IPolyProxyFactory(proxyFactory).getImplementation();
    }

    /// @notice Gets the Safe factory implementation address
    function getSafeFactoryImplementation() public view returns (address) {
        return IPolySafeFactory(safeFactory).masterCopy();
    }

    /// @notice Gets the Polymarket proxy wallet address for an address
    /// @param _addr    - The address that owns the proxy wallet
    function getPolyProxyWalletAddress(address _addr) public view returns (address) {
        return PolyProxyLib.getProxyWalletAddress(_addr, getPolyProxyFactoryImplementation(), proxyFactory);
    }

    /// @notice Gets the Polymarket Gnosis Safe address for an address
    /// @param _addr    - The address that owns the proxy wallet
    function getSafeAddress(address _addr) public view returns (address) {
        return PolySafeLib.getSafeAddress(_addr, getSafeFactoryImplementation(), safeFactory);
    }

    function _setProxyFactory(address _proxyFactory) internal {
        emit ProxyFactoryUpdated(proxyFactory, _proxyFactory);
        proxyFactory = _proxyFactory;
    }

    function _setSafeFactory(address _safeFactory) internal {
        emit SafeFactoryUpdated(safeFactory, _safeFactory);
        safeFactory = _safeFactory;
    }
}



================================================
FILE: src/exchange/mixins/Registry.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { IRegistry } from "../interfaces/IRegistry.sol";

struct OutcomeToken {
    uint256 complement;
    bytes32 conditionId;
}

/// @title Registry
abstract contract Registry is IRegistry {
    mapping(uint256 => OutcomeToken) public registry;

    /// @notice Gets the conditionId from a tokenId
    /// @param token    - The token
    function getConditionId(uint256 token) public view override returns (bytes32) {
        return registry[ token].conditionId;
    }

    /// @notice Gets the complement of a tokenId
    /// @param token    - The token
    function getComplement(uint256 token) public view override returns (uint256) {
        validateTokenId(token);
        return registry[ token].complement;
    }

    /// @notice Validates the complement of a tokenId
    /// @param token        - The tokenId
    /// @param complement   - The complement to be validated
    function validateComplement(uint256 token, uint256 complement) public view override {
        if (getComplement(token) != complement) revert InvalidComplement();
    }
    /// @notice Validates that a tokenId is registered
    /// @param tokenId - The tokenId

    function validateTokenId(uint256 tokenId) public view override {
        if (registry[ tokenId].complement == 0) revert InvalidTokenId();
    }

    function _registerToken(uint256 token0, uint256 token1, bytes32 conditionId) internal {
        if (token0 == token1 || (token0 == 0 || token1 == 0)) revert InvalidTokenId();
        if (registry[ token0].complement != 0 || registry[ token1].complement != 0) revert AlreadyRegistered();

        registry[ token0] = OutcomeToken({complement: token1, conditionId: conditionId});

        registry[ token1] = OutcomeToken({complement: token0, conditionId: conditionId});

        emit TokenRegistered(token0, token1, conditionId);
        emit TokenRegistered(token1, token0, conditionId);
    }
}



================================================
FILE: src/exchange/mixins/Signatures.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.10;

import { SignatureCheckerLib } from "solady/utils/SignatureCheckerLib.sol";
import { ECDSA } from "openzeppelin-contracts/utils/cryptography/ECDSA.sol";

import { SignatureType, Order } from "../libraries/OrderStructs.sol";

import { ISignatures } from "../interfaces/ISignatures.sol";

import { PolyFactoryHelper } from "./PolyFactoryHelper.sol";

/// @title Signatures
/// @notice Maintains logic that defines the various signature types and validates them
abstract contract Signatures is ISignatures, PolyFactoryHelper {
    constructor(address _proxyFactory, address _safeFactory) PolyFactoryHelper(_proxyFactory, _safeFactory) { }

    /// @notice Validates the signature of an order
    /// @param orderHash - The hash of the order
    /// @param order     - The order
    function validateOrderSignature(bytes32 orderHash, Order memory order) public view override {
        if (!isValidSignature(order.signer, order.maker, orderHash, order.signature, order.signatureType)) {
            revert InvalidSignature();
        }
    }

    /// @notice Verifies a signature for signed Order structs
    /// @param signer           - Address of the signer
    /// @param associated       - Address associated with the signer.
    ///                           For signature type EOA, this MUST be the same as the signer address.
    ///                           For signature types POLY_PROXY and POLY_GNOSIS_SAFE, this is the address of the proxy
    ///                           or the safe
    ///                           For signature type POLY_1271, this is the address of the contract
    /// @param structHash       - The hash of the struct being verified
    /// @param signature        - The signature to be verified
    /// @param signatureType    - The signature type to be verified
    function isValidSignature(
        address signer,
        address associated,
        bytes32 structHash,
        bytes memory signature,
        SignatureType signatureType
    ) internal view returns (bool) {
        if (signatureType == SignatureType.EOA) {
            // EOA
            return verifyEOASignature(signer, associated, structHash, signature);
        } else if (signatureType == SignatureType.POLY_GNOSIS_SAFE) {
            // POLY_GNOSIS_SAFE
            return verifyPolySafeSignature(signer, associated, structHash, signature);
        } else if (signatureType == SignatureType.POLY_1271) {
            // POLY_1271
            return verifyPoly1271Signature(signer, associated, structHash, signature);
        } else {
            // POLY_PROXY
            return verifyPolyProxySignature(signer, associated, structHash, signature);
        }
    }

    /// @notice Verifies an EOA ECDSA signature
    /// Verifies that:
    /// 1) the signature is valid
    /// 2) the signer and maker are the same
    /// @param signer      - The address of the signer
    /// @param maker       - The address of the maker
    /// @param structHash  - The hash of the struct being verified
    /// @param signature   - The signature to be verified
    function verifyEOASignature(address signer, address maker, bytes32 structHash, bytes memory signature)
        internal
        pure
        returns (bool)
    {
        return (signer == maker) && verifyECDSASignature(signer, structHash, signature);
    }

    /// @notice Verifies an ECDSA signature
    /// @dev Reverts if the signature length is invalid or the recovered signer is the zero address
    /// @param signer      - Address of the signer
    /// @param structHash  - The hash of the struct being verified
    /// @param signature   - The signature to be verified
    function verifyECDSASignature(address signer, bytes32 structHash, bytes memory signature)
        internal
        pure
        returns (bool)
    {
        return ECDSA.recover(structHash, signature) == signer;
    }

    /// @notice Verifies a signature signed by a Polymarket proxy wallet
    // Verifies that:
    // 1) the ECDSA signature is valid
    // 2) the Proxy wallet is owned by the signer
    /// @param signer       - Address of the signer
    /// @param proxyWallet  - Address of the poly proxy wallet
    /// @param structHash   - Hash of the struct being verified
    /// @param signature    - Signature to be verified
    function verifyPolyProxySignature(address signer, address proxyWallet, bytes32 structHash, bytes memory signature)
        internal
        view
        returns (bool)
    {
        return verifyECDSASignature(signer, structHash, signature) && getPolyProxyWalletAddress(signer) == proxyWallet;
    }

    /// @notice Verifies a signature signed by a Polymarket Gnosis safe
    // Verifies that:
    // 1) the ECDSA signature is valid
    // 2) the Safe is owned by the signer
    /// @param signer      - Address of the signer
    /// @param safeAddress - Address of the safe
    /// @param hash        - Hash of the struct being verified
    /// @param signature   - Signature to be verified
    function verifyPolySafeSignature(address signer, address safeAddress, bytes32 hash, bytes memory signature)
        internal
        view
        returns (bool)
    {
        return verifyECDSASignature(signer, hash, signature) && getSafeAddress(signer) == safeAddress;
    }

    /// @notice Verifies a signature signed by a smart contract
    /// @param signer           - Address of the 1271 smart contract
    /// @param maker            - Address of the 1271 smart contract
    /// @param hash             - Hash of the struct being verified
    /// @param signature        - Signature to be verified
    function verifyPoly1271Signature(address signer, address maker, bytes32 hash, bytes memory signature)
        internal
        view
        returns (bool)
    {
        return (signer == maker) && maker.code.length > 0
            && SignatureCheckerLib.isValidSignatureNow(maker, hash, signature);
    }
}



================================================
FILE: src/exchange/mixins/Trading.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { IFees } from "../interfaces/IFees.sol";
import { IHashing } from "../interfaces/IHashing.sol";
import { ITrading } from "../interfaces/ITrading.sol";
import { IRegistry } from "../interfaces/IRegistry.sol";
import { ISignatures } from "../interfaces/ISignatures.sol";
import { INonceManager } from "../interfaces/INonceManager.sol";
import { IAssetOperations } from "../interfaces/IAssetOperations.sol";

import { CalculatorHelper } from "../libraries/CalculatorHelper.sol";
import { Order, Side, MatchType, OrderStatus } from "../libraries/OrderStructs.sol";

/// @title Trading
/// @notice Implements logic for trading CTF assets
abstract contract Trading is IFees, ITrading, IHashing, IRegistry, ISignatures, INonceManager, IAssetOperations {
    /// @notice Mapping of orders to their current status
    mapping(bytes32 => OrderStatus) public orderStatus;

    /// @notice Gets the status of an order
    /// @param orderHash    - The hash of the order
    function getOrderStatus(bytes32 orderHash) public view returns (OrderStatus memory) {
        return orderStatus[ orderHash];
    }

    /// @notice Validates an order
    /// @notice order - The order to be validated
    function validateOrder(Order memory order) public view {
        bytes32 orderHash = hashOrder(order);
        _validateOrder(orderHash, order);
    }

    /// @notice Cancels an order
    /// An order can only be cancelled by its maker, the address which holds funds for the order
    /// @notice order - The order to be cancelled
    function cancelOrder(Order memory order) external {
        _cancelOrder(order);
    }

    /// @notice Cancels a set of orders
    /// @notice orders - The set of orders to be cancelled
    function cancelOrders(Order[] memory orders) external {
        uint256 length = orders.length;
        uint256 i = 0;
        for (; i < length;) {
            _cancelOrder(orders[ i]);
            unchecked {
                ++i;
            }
        }
    }

    function _cancelOrder(Order memory order) internal {
        if (order.maker != msg.sender) revert NotOwner();

        bytes32 orderHash = hashOrder(order);
        OrderStatus storage status = orderStatus[orderHash];
        if (status.isFilledOrCancelled) revert OrderFilledOrCancelled();

        status.isFilledOrCancelled = true;
        emit OrderCancelled(orderHash);
    }

    function _validateOrder(bytes32 orderHash, Order memory order) internal view {
        // Validate order expiration
        if (order.expiration > 0 && order.expiration < block.timestamp) revert OrderExpired();

        // Validate signature
        validateOrderSignature(orderHash, order);

        // Validate fee
        if (order.feeRateBps > getMaxFeeRate()) revert FeeTooHigh();

        // Validate the token to be traded
        validateTokenId(order.tokenId);

        // Validate that the order can be filled
        if (orderStatus[orderHash].isFilledOrCancelled) revert OrderFilledOrCancelled();

        // Validate nonce
        if (!isValidNonce(order.maker, order.nonce)) revert InvalidNonce();
    }

    /// @notice Fills an order against the caller
    /// @param order        - The order to be filled
    /// @param fillAmount   - The amount to be filled, always in terms of the maker amount
    /// @param to           - The address to receive assets from filling the order
    function _fillOrder(Order memory order, uint256 fillAmount, address to) internal {
        uint256 making = fillAmount;
        (uint256 taking, bytes32 orderHash) = _performOrderChecks(order, making);

        uint256 fee = CalculatorHelper.calculateFee(
            order.feeRateBps, order.side == Side.BUY ? taking : making, order.makerAmount, order.takerAmount, order.side
        );

        (uint256 makerAssetId, uint256 takerAssetId) = _deriveAssetIds(order);

        // Transfer order proceeds minus fees from msg.sender to order maker
        _transfer(msg.sender, order.maker, takerAssetId, taking - fee);

        // Transfer makingAmount from order maker to `to`
        _transfer(order.maker, to, makerAssetId, making);

        // NOTE: Fees are "collected" by the Operator implicitly,
        // since the fee is deducted from the assets paid by the Operator

        emit OrderFilled(orderHash, order.maker, msg.sender, makerAssetId, takerAssetId, making, taking, fee);
    }

    /// @notice Fills a set of orders against the caller
    /// @param orders       - The order to be filled
    /// @param fillAmounts  - The amounts to be filled, always in terms of the maker amount
    /// @param to           - The address to receive assets from filling the order
    function _fillOrders(Order[] memory orders, uint256[] memory fillAmounts, address to) internal {
        uint256 length = orders.length;
        uint256 i = 0;
        for (; i < length;) {
            _fillOrder(orders[i], fillAmounts[i], to);
            unchecked {
                ++i;
            }
        }
    }

    /// @notice Matches orders against each other
    /// Matches a taker order against a list of maker orders
    /// @param takerOrder       - The active order to be matched
    /// @param makerOrders      - The array of passive orders to be matched against the active order
    /// @param takerFillAmount  - The amount to fill on the taker order, in terms of the maker amount
    /// @param makerFillAmounts - The array of amounts to fill on the maker orders, in terms of the maker amount
    function _matchOrders(
        Order memory takerOrder,
        Order[] memory makerOrders,
        uint256 takerFillAmount,
        uint256[] memory makerFillAmounts
    ) internal {
        uint256 making = takerFillAmount;

        (uint256 taking, bytes32 orderHash) = _performOrderChecks(takerOrder, making);
        (uint256 makerAssetId, uint256 takerAssetId) = _deriveAssetIds(takerOrder);

        // Transfer takerOrder making amount from taker order to the Exchange
        _transfer(takerOrder.maker, address(this), makerAssetId, making);

        // Fill the maker orders
        _fillMakerOrders(takerOrder, makerOrders, makerFillAmounts);

        taking = _updateTakingWithSurplus(taking, takerAssetId);
        uint256 fee = CalculatorHelper.calculateFee(
            takerOrder.feeRateBps, takerOrder.side == Side.BUY ? taking : making, making, taking, takerOrder.side
        );

        // Execute transfers

        // Transfer order proceeds post fees from the Exchange to the taker order maker
        _transfer(address(this), takerOrder.maker, takerAssetId, taking - fee);

        // Charge the fee to taker order maker, explicitly transferring the fee from the Exchange to the Operator
        _chargeFee(address(this), msg.sender, takerAssetId, fee);

        // Refund any leftover tokens pulled from the taker to the taker order
        uint256 refund = _getBalance(makerAssetId);
        if (refund > 0) _transfer(address(this), takerOrder.maker, makerAssetId, refund);

        emit OrderFilled(
            orderHash, takerOrder.maker, address(this), makerAssetId, takerAssetId, making, taking, fee
        );

        emit OrdersMatched(orderHash, takerOrder.maker, makerAssetId, takerAssetId, making, taking);

        
    }

    function _fillMakerOrders(Order memory takerOrder, Order[] memory makerOrders, uint256[] memory makerFillAmounts)
        internal
    {
        uint256 length = makerOrders.length;
        uint256 i = 0;
        for (; i < length;) {
            _fillMakerOrder(takerOrder, makerOrders[i], makerFillAmounts[i]);
            unchecked {
                ++i;
            }
        }
    }

    /// @notice Fills a Maker order
    /// @param takerOrder   - The taker order
    /// @param makerOrder   - The maker order
    /// @param fillAmount   - The fill amount
    function _fillMakerOrder(Order memory takerOrder, Order memory makerOrder, uint256 fillAmount) internal {
        MatchType matchType = _deriveMatchType(takerOrder, makerOrder);

        // Ensure taker order and maker order match
        _validateTakerAndMaker(takerOrder, makerOrder, matchType);

        uint256 making = fillAmount;
        (uint256 taking, bytes32 orderHash) = _performOrderChecks(makerOrder, making);
        uint256 fee = CalculatorHelper.calculateFee(
            makerOrder.feeRateBps,
            makerOrder.side == Side.BUY ? taking : making,
            makerOrder.makerAmount,
            makerOrder.takerAmount,
            makerOrder.side
        );
        (uint256 makerAssetId, uint256 takerAssetId) = _deriveAssetIds(makerOrder);

        _fillFacingExchange(making, taking, makerOrder.maker, makerAssetId, takerAssetId, matchType, fee);

        emit OrderFilled(
            orderHash, makerOrder.maker, takerOrder.maker, makerAssetId, takerAssetId, making, taking, fee
        );
    }

    /// @notice Performs common order computations and validation
    /// 1) Validates the order taker
    /// 2) Computes the order hash
    /// 3) Validates the order
    /// 4) Computes taking amount
    /// 5) Updates the order status in storage
    /// @param order    - The order being prepared
    /// @param making   - The amount of the order being filled, in terms of maker amount
    function _performOrderChecks(Order memory order, uint256 making)
        internal
        returns (uint256 takingAmount, bytes32 orderHash)
    {
        _validateTaker(order.taker);

        orderHash = hashOrder(order);

        // Validate order
        _validateOrder(orderHash, order);

        // Calculate taking amount
        takingAmount = CalculatorHelper.calculateTakingAmount(making, order.makerAmount, order.takerAmount);

        // Update the order status in storage
        _updateOrderStatus(orderHash, order, making);
    }

    /// @notice Fills a maker order using the Exchange as the counterparty
    /// @param makingAmount - Amount to be filled in terms of maker amount
    /// @param takingAmount - Amount to be filled in terms of taker amount
    /// @param maker        - The order maker
    /// @param makerAssetId - The Token Id of the Asset to be sold
    /// @param takerAssetId - The Token Id of the Asset to be received
    /// @param matchType    - The match type
    /// @param fee          - The fee charged to the Order maker
    function _fillFacingExchange(
        uint256 makingAmount,
        uint256 takingAmount,
        address maker,
        uint256 makerAssetId,
        uint256 takerAssetId,
        MatchType matchType,
        uint256 fee
    ) internal {
        // Transfer makingAmount tokens from order maker to Exchange
        _transfer(maker, address(this), makerAssetId, makingAmount);

        // Executes a match call based on match type
        _executeMatchCall(makingAmount, takingAmount, makerAssetId, takerAssetId, matchType);

        // Ensure match action generated enough tokens to fill the order
        if (_getBalance(takerAssetId) < takingAmount) revert TooLittleTokensReceived();

        // Transfer order proceeds minus fees from the Exchange to the order maker
        _transfer(address(this), maker, takerAssetId, takingAmount - fee);

        // Transfer fees from Exchange to the Operator
        _chargeFee(address(this), msg.sender, takerAssetId, fee);
    }

    function _deriveMatchType(Order memory takerOrder, Order memory makerOrder) internal pure returns (MatchType) {
        if (takerOrder.side == Side.BUY && makerOrder.side == Side.BUY) return MatchType.MINT;
        if (takerOrder.side == Side.SELL && makerOrder.side == Side.SELL) return MatchType.MERGE;
        return MatchType.COMPLEMENTARY;
    }

    function _deriveAssetIds(Order memory order) internal pure returns (uint256 makerAssetId, uint256 takerAssetId) {
        if (order.side == Side.BUY) return (0, order.tokenId);
        return (order.tokenId, 0);
    }

    /// @notice Executes a CTF call to match orders by minting new Outcome tokens
    /// or merging Outcome tokens into collateral.
    /// @param makingAmount - Amount to be filled in terms of maker amount
    /// @param takingAmount - Amount to be filled in terms of taker amount
    /// @param makerAssetId - The Token Id of the Asset to be sold
    /// @param takerAssetId - The Token Id of the Asset to be received
    /// @param matchType    - The match type
    function _executeMatchCall(
        uint256 makingAmount,
        uint256 takingAmount,
        uint256 makerAssetId,
        uint256 takerAssetId,
        MatchType matchType
    ) internal {
        if (matchType == MatchType.COMPLEMENTARY) {
            // Indicates a buy vs sell order
            // no match action needed
            return;
        }
        if (matchType == MatchType.MINT) {
            // Indicates matching 2 buy orders
            // Mint new Outcome tokens using Exchange collateral balance and fill buys
            return _mint(getConditionId(takerAssetId), takingAmount);
        }
        if (matchType == MatchType.MERGE) {
            // Indicates matching 2 sell orders
            // Merge the Exchange Outcome token balance into collateral and fill sells
            return _merge(getConditionId(makerAssetId), makingAmount);
        }
    }

    /// @notice Ensures the taker and maker orders can be matched against each other
    /// @param takerOrder   - The taker order
    /// @param makerOrder   - The maker order
    function _validateTakerAndMaker(Order memory takerOrder, Order memory makerOrder, MatchType matchType)
        internal
        view
    {
        if (!CalculatorHelper.isCrossing(takerOrder, makerOrder)) revert NotCrossing();

        // Ensure orders match
        if (matchType == MatchType.COMPLEMENTARY) {
            if (takerOrder.tokenId != makerOrder.tokenId) revert MismatchedTokenIds();
        } else {
            // both bids or both asks
            validateComplement(takerOrder.tokenId, makerOrder.tokenId);
        }
    }

    function _validateTaker(address taker) internal view {
        if (taker != address(0) && taker != msg.sender) revert NotTaker();
    }

    function _chargeFee(address payer, address receiver, uint256 tokenId, uint256 fee) internal {
        // Charge fee to the payer if any
        if (fee > 0) {
            _transfer(payer, receiver, tokenId, fee);
            emit FeeCharged(receiver, tokenId, fee);
        }
    }

    function _updateOrderStatus(bytes32 orderHash, Order memory order, uint256 makingAmount)
        internal
        returns (uint256 remaining)
    {
        OrderStatus storage status = orderStatus[orderHash];
        // Fetch remaining amount from storage
        remaining = status.remaining;

        // Update remaining if the order is new/has not been filled
        remaining = remaining == 0 ? order.makerAmount : remaining;

        // Throw if the makingAmount(amount to be filled) is greater than the amount available
        if (makingAmount > remaining) revert MakingGtRemaining();

        // Update remaining using the makingAmount
        remaining = remaining - makingAmount;

        // If order is completely filled, update isFilledOrCancelled in storage
        if (remaining == 0) status.isFilledOrCancelled = true;

        // Update remaining in storage
        status.remaining = remaining;
    }

    function _updateTakingWithSurplus(uint256 minimumAmount, uint256 tokenId) internal returns (uint256) {
        uint256 actualAmount = _getBalance(tokenId);
        if (actualAmount < minimumAmount) revert TooLittleTokensReceived();
        return actualAmount;
    }
}



================================================
FILE: src/exchange/scripts/ExchangeDeployment.s.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.15;

import { Script } from "forge-std/Script.sol";
import { CTFExchange } from "exchange/CTFExchange.sol";

/// @title ExchangeDeployment
/// @notice Script to deploy the CTF Exchange
/// @author Polymarket
contract ExchangeDeployment is Script {
    /// @notice Deploys the Exchange contract
    /// @param admin        - The admin for the Exchange
    /// @param collateral   - The collateral token address
    /// @param ctf          - The CTF address
    /// @param proxyFactory - The Polymarket proxy factory address
    /// @param safeFactory  - The Polymarket Gnosis Safe factory address
    function deployExchange(address admin, address collateral, address ctf, address proxyFactory, address safeFactory)
        public
        returns (address exchange)
    {
        vm.startBroadcast();

        CTFExchange exch = new CTFExchange(collateral, ctf, proxyFactory, safeFactory);

        // Grant Auth privileges to the Admin address
        exch.addAdmin(admin);
        exch.addOperator(admin);

        // Revoke the deployer's authorization
        exch.renounceAdminRole();
        exch.renounceOperatorRole();

        exchange = address(exch);
    }
}



================================================
FILE: src/exchange/test/BaseExchangeTest.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { TestHelper } from "dev/TestHelper.sol";

import { USDC } from "dev/mocks/USDC.sol";
import { ERC1271Mock } from "dev/mocks/ERC1271Mock.sol";

import { Deployer } from "dev/util/Deployer.sol";

import { IERC20 } from "openzeppelin-contracts/token/ERC20/IERC20.sol";
import { IERC1155 } from "openzeppelin-contracts/token/ERC1155/IERC1155.sol";

import { CTFExchange } from "exchange/CTFExchange.sol";
import { IAuthEE } from "exchange/interfaces/IAuth.sol";
import { IFeesEE } from "exchange/interfaces/IFees.sol";
import { ITradingEE } from "exchange/interfaces/ITrading.sol";
import { IPausableEE } from "exchange/interfaces/IPausable.sol";
import { IRegistryEE } from "exchange/interfaces/IRegistry.sol";
import { ISignaturesEE } from "exchange/interfaces/ISignatures.sol";

import { IConditionalTokens } from "exchange/interfaces/IConditionalTokens.sol";

import { CalculatorHelper } from "exchange/libraries/CalculatorHelper.sol";
import { Order, Side, SignatureType } from "exchange/libraries/OrderStructs.sol";

contract BaseExchangeTest is TestHelper, IAuthEE, IFeesEE, IRegistryEE, IPausableEE, ITradingEE, ISignaturesEE {
    mapping(address => mapping(address => mapping(uint256 => uint256))) private _checkpoints1155;

    USDC public usdc;
    IConditionalTokens public ctf;
    CTFExchange public exchange;

    bytes32 public constant questionID = hex"1234";
    bytes32 public conditionId;
    uint256 public yes;
    uint256 public no;

    address public admin = alice;
    uint256 internal bobPK = 0xB0B;
    uint256 internal carlaPK = 0xCA414;
    address public bob;
    address public carla;

    ERC1271Mock public contractWallet;

    // ERC20 transfer event
    event Transfer(address indexed from, address indexed to, uint256 value);

    // ERC1155 transfer event
    event TransferSingle(
        address indexed operator, address indexed from, address indexed to, uint256 id, uint256 amount
    );

    function setUp() public virtual {
        bob = vm.addr(bobPK);
        vm.label(bob, "bob");
        carla = vm.addr(carlaPK);
        vm.label(carla, "carla");

        usdc = new USDC();
        vm.label(address(usdc), "USDC");
        ctf = IConditionalTokens(Deployer.ConditionalTokens());
        vm.label(address(ctf), "CTF");

        conditionId = _prepareCondition(admin, questionID);
        yes = _getPositionId(2);
        no = _getPositionId(1);

        // Deploy a 1271 contract and set carla as the signer
        contractWallet = new ERC1271Mock(carla);

        vm.startPrank(admin);
        exchange = new CTFExchange(address(usdc), address(ctf), address(0), address(0));
        exchange.registerToken(yes, no, conditionId);
        exchange.addOperator(bob);
        exchange.addOperator(carla);
        vm.stopPrank();
    }

    function _prepareCondition(address oracle, bytes32 _questionId) internal returns (bytes32) {
        ctf.prepareCondition(oracle, _questionId, 2);
        return ctf.getConditionId(oracle, _questionId, 2);
    }

    function _getPositionId(uint256 indexSet) internal view returns (uint256) {
        return ctf.getPositionId(IERC20(address(usdc)), ctf.getCollectionId(bytes32(0), conditionId, indexSet));
    }

    function _createAndSignOrderWithFee(
        uint256 pk,
        uint256 tokenId,
        uint256 makerAmount,
        uint256 takerAmount,
        uint256 feeRateBps,
        Side side
    ) internal returns (Order memory) {
        address maker = vm.addr(pk);
        Order memory order = _createOrder(maker, tokenId, makerAmount, takerAmount, side);
        order.feeRateBps = feeRateBps;
        order.signature = _signMessage(pk, exchange.hashOrder(order));
        return order;
    }

    function _createAndSignOrder(uint256 pk, uint256 tokenId, uint256 makerAmount, uint256 takerAmount, Side side)
        internal
        returns (Order memory)
    {
        address maker = vm.addr(pk);
        Order memory order = _createOrder(maker, tokenId, makerAmount, takerAmount, side);
        order.signature = _signMessage(pk, exchange.hashOrder(order));
        return order;
    }

    function _createAndSign1271Order(uint256 signerPk, address wallet, uint256 tokenId, uint256 makerAmount, uint256 takerAmount, Side side) 
        internal
        returns (Order memory)
    {
        Order memory order = _createOrder(wallet, tokenId, makerAmount, takerAmount, side);
        order.signatureType = SignatureType.POLY_1271;
        order.signature = _signMessage(signerPk, exchange.hashOrder(order));
        return order;
    }

    function _createOrder(address maker, uint256 tokenId, uint256 makerAmount, uint256 takerAmount, Side side)
        internal
        pure
        returns (Order memory)
    {
        Order memory order = Order({
            salt: 1,
            signer: maker,
            maker: maker,
            taker: address(0),
            tokenId: tokenId,
            makerAmount: makerAmount,
            takerAmount: takerAmount,
            expiration: 0,
            nonce: 0,
            feeRateBps: 0,
            signatureType: SignatureType.EOA,
            side: side,
            signature: new bytes(0)
        });
        return order;
    }

    function _signMessage(uint256 pk, bytes32 message) internal returns (bytes memory sig) {
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(pk, message);
        sig = abi.encodePacked(r, s, v);
    }

    function _mintTestTokens(address to, address spender, uint256 amount) internal {
        uint256[] memory partition = new uint256[](2);
        partition[0] = 1;
        partition[1] = 2;

        vm.startPrank(to);
        approve(address(usdc), address(ctf), type(uint256).max);

        dealAndApprove(address(usdc), to, spender, amount);
        IERC1155(address(ctf)).setApprovalForAll(spender, true);

        uint256 splitAmount = amount / 2;
        IConditionalTokens(ctf).splitPosition(IERC20(address(usdc)), bytes32(0), conditionId, partition, splitAmount);
        vm.stopPrank();
    }

    function assertCollateralBalance(address _who, uint256 _amount) public {
        assertBalance(address(usdc), _who, _amount);
    }

    function assertCTFBalance(address _who, uint256 _tokenId, uint256 _amount) public {
        assertBalance1155(address(ctf), _who, _tokenId, _amount);
    }

    function checkpointCollateral(address _who) public {
        checkpointBalance(address(usdc), _who);
    }

    function checkpointCTF(address _who, uint256 _tokenId) public {
        checkpointBalance1155(address(ctf), _who, _tokenId);
    }

    function getCTFBalance(address _who, uint256 _tokenId) public view returns (uint256) {
        return IERC1155(address(ctf)).balanceOf(_who, _tokenId);
    }

    function assertBalance1155(address _token, address _who, uint256 _tokenId, uint256 _amount) public {
        assertEq(getCTFBalance(_who, _tokenId), _checkpoints1155[_token][_who][_tokenId] + _amount);
    }

    function checkpointBalance1155(address _token, address _who, uint256 _tokenId) public {
        _checkpoints1155[_token][_who][_tokenId] = getCTFBalance(_who, _tokenId);
    }

    function calculatePrice(uint256 makerAmount, uint256 takerAmount, Side side) public pure returns (uint256) {
        return CalculatorHelper._calculatePrice(makerAmount, takerAmount, side);
    }

    function calculateFee(uint256 _feeRate, uint256 _amount, uint256 makerAmount, uint256 takerAmount, Side side)
        internal
        pure
        returns (uint256)
    {
        return CalculatorHelper.calculateFee(_feeRate, _amount, makerAmount, takerAmount, side);
    }

    function _getTakingAmount(uint256 _making, uint256 _makerAmount, uint256 _takerAmount)
        internal
        pure
        returns (uint256)
    {
        return _making * _takerAmount / _makerAmount;
    }
}



================================================
FILE: src/exchange/test/CTFExchange.t.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { BaseExchangeTest } from "exchange/test/BaseExchangeTest.sol";
import { Order, Side, MatchType, OrderStatus, SignatureType } from "exchange/libraries/OrderStructs.sol";

contract CTFExchangeTest is BaseExchangeTest {
    function testSetup() public {
        assertTrue(exchange.isAdmin(admin));
        assertTrue(exchange.isOperator(admin));
        assertFalse(exchange.isAdmin(brian));
        assertFalse(exchange.isOperator(brian));
    }

    function testAuth() public {
        vm.expectEmit(true, true, true, true);
        emit NewAdmin(henry, admin);
        emit NewOperator(henry, admin);

        vm.startPrank(admin);
        exchange.addAdmin(henry);
        exchange.addOperator(henry);
        vm.stopPrank();

        assertTrue(exchange.isOperator(henry));
        assertTrue(exchange.isAdmin(henry));
    }

    function testAuthRemoveAdmin() public {
        vm.expectEmit(true, true, true, true);
        emit RemovedAdmin(henry, admin);
        emit RemovedOperator(henry, admin);

        vm.startPrank(admin);
        exchange.removeAdmin(henry);
        exchange.removeOperator(henry);
        vm.stopPrank();

        assertFalse(exchange.isAdmin(henry));
        assertFalse(exchange.isOperator(henry));
    }

    function testAuthNotAdmin() public {
        vm.expectRevert(NotAdmin.selector);
        exchange.addAdmin(address(1));
    }

    function testAuthRenounce() public {
        // Non admin cannot renounce
        vm.expectRevert(NotAdmin.selector);
        vm.prank(address(12));
        exchange.renounceAdminRole();

        assertTrue(exchange.isAdmin(admin));
        assertTrue(exchange.isOperator(admin));

        // Successfully renounces the admin role
        vm.prank(admin);
        exchange.renounceAdminRole();
        assertFalse(exchange.isAdmin(admin));
        assertTrue(exchange.isOperator(admin));

        // Successfully renounces the operator role
        vm.prank(admin);
        exchange.renounceOperatorRole();
        assertFalse(exchange.isOperator(admin));
    }

    function testPause() public {
        vm.expectEmit(true, true, true, false);
        emit TradingPaused(admin);

        vm.prank(admin);
        exchange.pauseTrading();

        _mintTestTokens(bob, address(exchange), 1_000_000_000);
        _mintTestTokens(carla, address(exchange), 1_000_000_000);

        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);

        vm.expectRevert(Paused.selector);
        vm.prank(carla);
        exchange.fillOrder(order, 50_000_000);

        vm.expectEmit(true, true, true, true);
        emit TradingUnpaused(admin);

        vm.prank(admin);
        exchange.unpauseTrading();

        // Order can be filled after unpausing
        vm.prank(carla);
        exchange.fillOrder(order, 50_000_000);
        emit OrderFilled(exchange.hashOrder(order), bob, carla, 0, yes, 50_000_000, 100_000_000, 0);
    }

    function testRegisterToken(uint256 _token0, uint256 _token1, uint256 _conditionId) public {
        vm.assume(
            _token0 != yes && _token0 != no && _token1 != yes && _token1 != no && _token1 != _token0 && _token0 > 0
                && _token1 > 0
        );
        bytes32 tokenConditionId = bytes32(_conditionId);

        vm.expectEmit(true, true, true, false);
        emit TokenRegistered(_token0, _token1, tokenConditionId);
        emit TokenRegistered(_token1, _token0, tokenConditionId);
        vm.prank(admin);
        exchange.registerToken(_token0, _token1, tokenConditionId);

        assertEq(exchange.getComplement(_token0), _token1);
        assertEq(exchange.getComplement(_token1), _token0);
        assertEq(exchange.getConditionId(_token0), tokenConditionId);
    }

    function testRegisterTokenRevertCases() public {
        vm.startPrank(admin);
        vm.expectRevert(InvalidTokenId.selector);
        exchange.registerToken(0, 0, bytes32(0));

        vm.expectRevert(AlreadyRegistered.selector);
        exchange.registerToken(no, yes, bytes32(0));
    }

    function testHashOrder() public {
        Order memory order = _createOrder(bob, 1, 50_000_000, 100_000_000, Side.BUY);

        bytes32 expectedHash = 0xea9d5909ecf95a08c9906dc3cfafa62ca6b505f5e1c37c33e0d01099c0565c8f;

        assertEq(exchange.hashOrder(order), expectedHash);
    }

    function testValidate() public {
        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        exchange.validateOrder(order);
    }

    function testValidateInvalidSig() public {
        Order memory order = _createOrder(bob, yes, 50_000_000, 100_000_000, Side.BUY);

        // Incorrect signature(note: signed by carla)
        order.signature = _signMessage(carlaPK, exchange.hashOrder(order));
        vm.expectRevert(InvalidSignature.selector);
        exchange.validateOrder(order);
    }

    function testValidateInvalidSigLength() public {
        Order memory order = _createOrder(bob, yes, 50_000_000, 100_000_000, Side.BUY);
        order.signature = hex"";
        vm.expectRevert("ECDSA: invalid signature length");
        exchange.validateOrder(order);
    }

    function testValidateInvalidNonce() public {
        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        vm.prank(bob);
        exchange.incrementNonce();
        vm.expectRevert(InvalidNonce.selector);
        exchange.validateOrder(order);

        order.nonce = 1;
        order.signature = _signMessage(bobPK, exchange.hashOrder(order));
        exchange.validateOrder(order);
    }

    function testValidateInvalidSignerMaker() public {
        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        // For EOA signature type, signer and maker MUST be the same
        order.maker = carla;
        order.signatureType = SignatureType.EOA;
        order.signature = _signMessage(bobPK, exchange.hashOrder(order));

        vm.expectRevert(InvalidSignature.selector);
        exchange.validateOrder(order);
    }

    function testValidateInvalidExpiration() public {
        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        vm.warp(block.timestamp + 1000);
        order.expiration = 50;
        vm.expectRevert(OrderExpired.selector);
        exchange.validateOrder(order);
    }

    function testValidateDuplicateOrder() public {
        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);

        _mintTestTokens(bob, address(exchange), 1_000_000_000);
        _mintTestTokens(carla, address(exchange), 1_000_000_000);
        vm.prank(carla);
        exchange.fillOrder(order, 50_000_000);

        // attempting to fill this order again reverts
        vm.expectRevert(OrderFilledOrCancelled.selector);
        vm.prank(carla);
        exchange.fillOrder(order, 50_000_000);
    }

    function testValidateFeeTooHigh() public {
        Order memory order = _createAndSignOrderWithFee(
            bobPK,
            yes,
            50_000_000,
            100_000_000,
            10000, // Fee of 100%
            Side.BUY
        );

        vm.expectRevert(FeeTooHigh.selector);
        exchange.validateOrder(order);
    }

    function testFillOrder() public {
        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);

        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        bytes32 orderHash = exchange.hashOrder(order);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(orderHash, bob, carla, 0, yes, 25_000_000, 50_000_000, 0);

        // Checkpoint USDC balance for carla and Outcome token balance for bob
        checkpointCollateral(carla);
        checkpointCTF(bob, yes);

        // Partially fill the order with carla
        vm.prank(carla);
        exchange.fillOrder(order, 25_000_000);

        // Check balances post fill
        assertCollateralBalance(carla, 25_000_000);
        assertCTFBalance(bob, yes, 50_000_000);

        // Ensure the order status is as expected
        OrderStatus memory status = exchange.getOrderStatus(orderHash);
        assertEq(status.remaining, 25_000_000);
        assertFalse(status.isFilledOrCancelled);
    }

    function testFillOrderPartial() public {
        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);

        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        bytes32 orderHash = exchange.hashOrder(order);

        // Partially fill the order with carla
        vm.startPrank(carla);
        exchange.fillOrder(order, 25_000_000);

        // Fill the order again
        exchange.fillOrder(order, 25_000_000);

        // Ensure the order status is as expected
        OrderStatus memory status = exchange.getOrderStatus(orderHash);
        assertEq(status.remaining, 0);
        assertTrue(status.isFilledOrCancelled);
    }

    function testFillOrderWithFees() public {
        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);

        Order memory order = _createAndSignOrderWithFee(
            bobPK,
            yes,
            50_000_000,
            100_000_000,
            100, // 1% or 100 bips
            Side.BUY
        );
        bytes32 orderHash = exchange.hashOrder(order);

        // Fees are charged on order proceeds, in this case Outcome tokens
        uint256 expectedFee = calculateFee(100, 50_000_000, order.makerAmount, order.takerAmount, order.side);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(orderHash, bob, carla, 0, yes, 25_000_000, 50_000_000, expectedFee);

        vm.prank(carla);
        exchange.fillOrder(order, 25_000_000);

        // Ensure the order status is as expected
        OrderStatus memory status = exchange.getOrderStatus(orderHash);
        assertEq(status.remaining, 25_000_000);
        assertFalse(status.isFilledOrCancelled);
    }

    function testFuzzFillOrderWithFees(uint128 fillAmount, uint16 feeRateBps) public {
        uint256 makerAmount = 50_000_000;
        uint256 takerAmount = 100_000_000;

        vm.assume(fillAmount <= makerAmount && feeRateBps < exchange.getMaxFeeRate());

        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);

        Order memory order = _createAndSignOrderWithFee(bobPK, yes, makerAmount, takerAmount, feeRateBps, Side.BUY);
        bytes32 orderHash = exchange.hashOrder(order);

        uint256 remaining = makerAmount - fillAmount;
        uint256 taking = fillAmount * order.takerAmount / order.makerAmount;
        uint256 expectedFee = calculateFee(feeRateBps, taking, order.makerAmount, order.takerAmount, order.side);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(orderHash, bob, carla, 0, yes, fillAmount, taking, expectedFee);

        checkpointCTF(bob, yes);
        checkpointCollateral(carla);

        vm.prank(carla);
        exchange.fillOrder(order, fillAmount);

        // Ensure the order status is as expected
        OrderStatus memory status = exchange.getOrderStatus(orderHash);
        assertEq(status.remaining, remaining);

        // Assert the token transfers from the order maker to the filler
        assertCTFBalance(bob, yes, taking - expectedFee);
        assertCollateralBalance(carla, fillAmount);
    }

    function testFillOrderNonTaker() public {
        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);
        _mintTestTokens(admin, address(exchange), 20_000_000_000);

        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        order.taker = carla;
        bytes32 orderHash = exchange.hashOrder(order);
        order.signature = _signMessage(bobPK, orderHash);

        // A non taker operator attempting to fill the order will revert
        vm.expectRevert(NotTaker.selector);
        vm.prank(admin);
        exchange.fillOrder(order, 50_000_000);

        // The taker specified operator will successfully fill the order
        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(order), bob, carla, 0, yes, 50_000_000, 100_000_000, 0);

        vm.prank(carla);
        exchange.fillOrder(order, 50_000_000);
    }

    function testFillOrders() public {
        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);

        Order[] memory orders = new Order[](3);
        uint256[] memory amounts = new uint256[](3);

        Order memory yesBuy = _createAndSignOrderWithFee(
            bobPK,
            yes,
            50_000_000,
            100_000_000,
            100, // 1% or 100 bips
            Side.BUY
        );

        Order memory noBuy = _createAndSignOrderWithFee(
            bobPK,
            no,
            50_000_000,
            100_000_000,
            100, // 1% or 100 bips
            Side.BUY
        );

        Order memory yesSell = _createAndSignOrderWithFee(
            bobPK,
            yes,
            100_000_000,
            60_000_000,
            100, // 1% or 100 bips
            Side.SELL
        );

        orders[0] = yesBuy;
        orders[1] = noBuy;
        orders[2] = yesSell;

        amounts[0] = 50_000_000;
        amounts[1] = 50_000_000;
        amounts[2] = 100_000_000;

        uint256 expectedFeeYesBuy = calculateFee(100, 100_000_000, yesBuy.makerAmount, yesBuy.takerAmount, yesBuy.side);
        uint256 expectedFeeNoBuy = calculateFee(100, 100_000_000, noBuy.makerAmount, noBuy.takerAmount, noBuy.side);
        uint256 expectedFeeYesSell =
            calculateFee(100, 100_000_000, yesSell.makerAmount, yesSell.takerAmount, yesSell.side);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(yesBuy), bob, carla, 0, yes, 50_000_000, 100_000_000, expectedFeeYesBuy);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(noBuy), bob, carla, 0, no, 50_000_000, 100_000_000, expectedFeeNoBuy);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(yesSell), bob, carla, yes, 0, 100_000_000, 60_000_000, expectedFeeYesSell);

        vm.prank(carla);
        exchange.fillOrders(orders, amounts);
    }

    function testFillOrderZeroMakerAmount() public {
        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);

        // Create a non-standard order with 0 maker amount
        Order memory order = _createAndSignOrder(bobPK, yes, 0, 100_000_000, Side.BUY);

        // Reverts since the order does not allocate any tokens to be sold, i.e zero maker amount
        vm.expectRevert(MakingGtRemaining.selector);
        vm.prank(carla);
        exchange.fillOrder(order, 50_000_000);
    }

    function testFillOrderZeroTakerAmount() public {
        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);

        // Create a non-standard order with 0 taker amount
        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 0, Side.BUY);

        // As such, the order can be successfully filled with *nothing*.
        // Note: it is up to the user to provide sensible maker and taker amounts
        // See the below CTF ERC1155 transfer event:
        // Transferring 0 YES tokens from carla in return for all of the USDC in the order
        vm.expectEmit(true, true, true, true);
        emit TransferSingle(address(exchange), carla, bob, yes, 0);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(order), bob, carla, 0, yes, 50_000_000, 0, 0);

        uint256 fillAmount = 50_000_000;
        vm.prank(carla);
        exchange.fillOrder(order, fillAmount);
    }

    function testFillOrderMaliciousOperator() public {
        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);

        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);

        // A malicious operator could attempt to pull tokens available in the order maker's wallet
        // Exchange will protect against this and revert
        uint256 fillAmount = usdc.balanceOf(bob);

        vm.expectRevert(MakingGtRemaining.selector);
        vm.prank(carla);
        exchange.fillOrder(order, fillAmount);
    }

    function testCancelOrder(uint256 makerAmount, uint256 takerAmount, uint256 tokenId) public {
        vm.assume(tokenId > 0);

        Order memory order = _createAndSignOrder(bobPK, tokenId, makerAmount, takerAmount, Side.BUY);
        bytes32 orderHash = exchange.hashOrder(order);

        vm.expectEmit(true, true, true, true);
        emit OrderCancelled(orderHash);
        vm.prank(bob);
        exchange.cancelOrder(order);
    }

    function testCancelOrders(uint256 makerAmount, uint256 takerAmount, uint256 tokenId) public {
        vm.assume(tokenId > 0);

        Order memory o1 = _createAndSignOrder(bobPK, tokenId, makerAmount, takerAmount, Side.BUY);
        bytes32 o1Hash = exchange.hashOrder(o1);

        Order memory o2 = _createAndSignOrder(bobPK, tokenId, makerAmount, takerAmount, Side.SELL);
        bytes32 o2Hash = exchange.hashOrder(o2);

        Order[] memory orders = new Order[](2);
        orders[0] = o1;
        orders[1] = o2;

        vm.expectEmit(true, true, true, true);
        emit OrderCancelled(o1Hash);
        emit OrderCancelled(o2Hash);

        vm.prank(bob);
        exchange.cancelOrders(orders);
    }

    function testCancelOrderNotOwner() public {
        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        vm.expectRevert(NotOwner.selector);
        vm.prank(carla);
        exchange.cancelOrder(order);
    }

    function testCancelOrderOrderFilledOrCancelled() public {
        _mintTestTokens(bob, address(exchange), 1_000_000_000);
        _mintTestTokens(carla, address(exchange), 1_000_000_000);

        Order memory order = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);

        vm.prank(carla);
        exchange.fillOrder(order, 50_000_000);

        vm.expectRevert(OrderFilledOrCancelled.selector);
        vm.prank(bob);
        exchange.cancelOrder(order);
    }

    function testCancelOrderNonExistent() public {
        Order memory order = _createAndSignOrder(bobPK, 1, 50_000_000, 100_000_000, Side.BUY);

        // Cancelling a new order is valid, the order will now be unfillable
        vm.prank(bob);
        exchange.cancelOrder(order);

        OrderStatus memory status = exchange.getOrderStatus(exchange.hashOrder(order));
        assertTrue(status.isFilledOrCancelled);
        assertEq(status.remaining, 0);

        vm.expectRevert(OrderFilledOrCancelled.selector);
        vm.prank(bob);
        exchange.cancelOrder(order);
    }

    function testCalculateFeeBuy() public {
        uint256 feeRateBps = 100; // 1%
        uint256 proceeds;
        uint256 expectedFee;
        uint256 actualFee;
        Order memory order;

        order = _createOrder(bob, yes, 40_000_000, 100_000_000, Side.BUY);
        proceeds = 100_000_000;
        expectedFee = 1000000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 20_000_000, 100_000_000, Side.BUY);
        proceeds = 100_000_000;
        expectedFee = 1000000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 60_000_000, 100_000_000, Side.BUY);
        proceeds = 100_000_000;
        expectedFee = 666666;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 80_000_000, 100_000_000, Side.BUY);
        proceeds = 100_000_000;
        expectedFee = 250000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 99_000_000, 100_000_000, Side.BUY);
        proceeds = 100_000_000;
        expectedFee = 10101;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 1_000_000, 100_000_000, Side.BUY);
        proceeds = 100_000_000;
        expectedFee = 1_000_000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 100_000_000, 500_000_000, Side.BUY);
        proceeds = 500_000_000;
        expectedFee = 5_000_000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 1_000, 2_000, Side.BUY);
        proceeds = 2_000;
        expectedFee = 20;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);
    }

    function testCalculateFeeSell() public {
        uint256 feeRateBps = 100; // 1%
        uint256 proceeds = 100_000_000;
        uint256 expectedFee;
        uint256 actualFee;
        Order memory order;

        order = _createOrder(bob, yes, 100_000_000, 40_000_000, Side.SELL);
        proceeds = 100_000_000;
        expectedFee = 400000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 100_000_000, 20_000_000, Side.SELL);
        expectedFee = 200000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 100_000_000, 60_000_000, Side.SELL);
        expectedFee = 400000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 100_000_000, 80_000_000, Side.SELL);
        expectedFee = 200000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 100_000_000, 99_000_000, Side.SELL);
        expectedFee = 10000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 100_000_000, 1_000_000, Side.SELL);
        expectedFee = 10000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 500_000_000, 100_000_000, Side.SELL);
        proceeds = 500_000_000;
        expectedFee = 1_000_000;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);

        order = _createOrder(bob, yes, 2_000, 1_000, Side.SELL);
        proceeds = 2_000;
        expectedFee = 10;
        actualFee = calculateFee(feeRateBps, proceeds, order.makerAmount, order.takerAmount, order.side);
        assertEq(actualFee, expectedFee);
    }

    function testFuzzCalculateFee(uint128 fillAmount, uint16 feeRateBps, uint128 makerAmount, uint128 takerAmount)
        public
    {
        vm.assume(
            makerAmount > 0 && takerAmount > makerAmount && fillAmount <= makerAmount
                && feeRateBps < exchange.getMaxFeeRate()
        );

        uint256 expectedProceeds = _getTakingAmount(fillAmount, makerAmount, takerAmount);
        calculateFee(feeRateBps, expectedProceeds, makerAmount, takerAmount, Side.BUY);
        calculateFee(feeRateBps, expectedProceeds, takerAmount, makerAmount, Side.SELL);
    }

    function testCalculateFeeLargePrice() public {
        // Possible for an order to have a price that breaks fee calculation:
        // Implies a price of 100 USD per YES token
        uint256 makerAmount = 1_000_000; // yes tokens
        uint256 takerAmount = 100_000_000; // cash

        Side side = Side.SELL;
        uint256 feeRateBps = 100;
        uint256 outcomeTokens = takerAmount;

        // ignore these orders in fee calculation
        uint256 fee = calculateFee(feeRateBps, outcomeTokens, makerAmount, takerAmount, side);
        assertEq(fee, 0);
    }
}



================================================
FILE: src/exchange/test/ERC1271Signature.t.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { BaseExchangeTest } from "exchange/test/BaseExchangeTest.sol";
import { Order, Side, SignatureType } from "exchange/libraries/OrderStructs.sol";

contract ERC1271SignatureTest is BaseExchangeTest {
    function test_validate1271Signature() public {
        Order memory order =
            _createAndSign1271Order(carlaPK, address(contractWallet), yes, 50_000_000, 100_000_000, Side.BUY);
        exchange.validateOrderSignature(exchange.hashOrder(order), order);
    }

    function test_validate1271Signature_revert_incorrectSigner() public {
        Order memory order = _createOrder(address(contractWallet), yes, 50_000_000, 100_000_000, Side.BUY);
        order.signatureType = SignatureType.POLY_1271;
        bytes32 orderHash = exchange.hashOrder(order);
        order.signature = _signMessage(bobPK, orderHash);
        vm.expectRevert(InvalidSignature.selector);
        exchange.validateOrderSignature(orderHash, order);
    }

    function test_validate1271Signature_revert_sigType() public {
        Order memory order = _createOrder(address(contractWallet), yes, 50_000_000, 100_000_000, Side.BUY);
        order.signatureType = SignatureType.EOA;
        bytes32 orderHash = exchange.hashOrder(order);
        order.signature = _signMessage(carlaPK, orderHash);
        vm.expectRevert(InvalidSignature.selector);
        exchange.validateOrderSignature(orderHash, order);
    }

    function test_validate1271Signature_revert_nonContract() public {
        Order memory order = _createOrder(carla, yes, 50_000_000, 100_000_000, Side.BUY);
        order.signatureType = SignatureType.POLY_1271;
        bytes32 orderHash = exchange.hashOrder(order);
        order.signature = _signMessage(carlaPK, orderHash);
        vm.expectRevert(InvalidSignature.selector);
        exchange.validateOrderSignature(orderHash, order);
    }

    function test_validate1271Signature_revert_invalidContract() public {
        // revert when using a non 1271 contract
        Order memory order = _createOrder(address(usdc), yes, 50_000_000, 100_000_000, Side.BUY);
        order.signatureType = SignatureType.POLY_1271;
        bytes32 orderHash = exchange.hashOrder(order);
        order.signature = _signMessage(carlaPK, orderHash);
        vm.expectRevert(InvalidSignature.selector);
        exchange.validateOrderSignature(orderHash, order);
    }

    function test_validate1271Signature_revert_invalidSignerMaker() public {
        Order memory order = _createOrder(address(contractWallet), yes, 50_000_000, 100_000_000, Side.BUY);
        order.signatureType = SignatureType.POLY_1271;
        // signer == carla, maker == contractWallet
        order.signer = carla;
        bytes32 orderHash = exchange.hashOrder(order);
        order.signature = _signMessage(carlaPK, orderHash);
        vm.expectRevert(InvalidSignature.selector);
        exchange.validateOrderSignature(orderHash, order);
    }
}



================================================
FILE: src/exchange/test/MatchOrders.t.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { BaseExchangeTest } from "exchange/test/BaseExchangeTest.sol";

import { Order, Side } from "exchange/libraries/OrderStructs.sol";

contract MatchOrdersTest is BaseExchangeTest {
    function setUp() public override {
        super.setUp();
        _mintTestTokens(bob, address(exchange), 20_000_000_000);
        _mintTestTokens(carla, address(exchange), 20_000_000_000);
    }

    function testMatchTypeComplementary() public {
        // Init a match with a yes buy against a list of yes sells
        Order memory buy = _createAndSignOrder(bobPK, yes, 60_000_000, 100_000_000, Side.BUY);
        Order memory sellA = _createAndSignOrder(carlaPK, yes, 50_000_000, 25_000_000, Side.SELL);
        Order memory sellB = _createAndSignOrder(carlaPK, yes, 100_000_000, 50_000_000, Side.SELL);
        Order[] memory makerOrders = new Order[](2);
        makerOrders[ 0] = sellA;
        makerOrders[ 1] = sellB;

        uint256[] memory fillAmounts = new uint256[](2);
        fillAmounts[ 0] = 50_000_000;
        fillAmounts[ 1] = 70_000_000;

        checkpointCollateral(carla);
        checkpointCTF(bob, yes);

        // Check fill events
        // First maker order is filled completely
        vm.expectEmit(true, true, true, false);
        emit OrderFilled(exchange.hashOrder(sellA), carla, bob, yes, 0, 50_000_000, 25_000_000, 0);

        // Second maker order is partially filled
        vm.expectEmit(true, true, true, false);
        emit OrderFilled(exchange.hashOrder(sellB), carla, bob, yes, 0, 70_000_000, 35_000_000, 0);

        // The taker order is filled completely
        vm.expectEmit(true, true, true, false);
        emit OrderFilled(exchange.hashOrder(buy), bob, address(exchange), 0, yes, 60_000_000, 120_000_000, 0);

        vm.expectEmit(true, true, true, false);
        emit OrdersMatched(exchange.hashOrder(buy), bob, 0, yes, 60_000_000, 120_000_000);

        vm.prank(admin);
        exchange.matchOrders(buy, makerOrders, 60_000_000, fillAmounts);

        // Ensure balances have been updated post match
        assertCollateralBalance(carla, 60_000_000);
        assertCTFBalance(bob, yes, 120_000_000);

        // Ensure onchain state for orders is as expected
        bytes32 buyHash = exchange.hashOrder(buy);
        assertEq(exchange.getOrderStatus(buyHash).remaining, 0);
        assertTrue(exchange.getOrderStatus(buyHash).isFilledOrCancelled);
    }

    function testMatchTypeMint() public {
        // Init Match with YES buy against a YES sell and a NO buy
        // To match the YES buy with the NO buy, CTF Exchange will MINT new Outcome tokens using it's collateral
        // balance. Then will fill the YES buy and NO buy with the resulting Outcome tokens
        Order memory buy = _createAndSignOrder(bobPK, yes, 60_000_000, 100_000_000, Side.BUY);
        Order memory yesSell = _createAndSignOrder(carlaPK, yes, 50_000_000, 25_000_000, Side.SELL);
        Order memory noBuy = _createAndSignOrder(carlaPK, no, 16_000_000, 40_000_000, Side.BUY);
        Order[] memory makerOrders = new Order[](2);
        makerOrders[0] = yesSell;
        makerOrders[1] = noBuy;

        uint256[] memory fillAmounts = new uint256[](2);
        fillAmounts[0] = 50_000_000;
        fillAmounts[1] = 16_000_000;

        uint256 takerOrderFillAmount = 49_000_000;

        checkpointCollateral(carla);
        checkpointCTF(bob, yes);
        checkpointCTF(carla, no);

        vm.prank(admin);
        exchange.matchOrders(buy, makerOrders, takerOrderFillAmount, fillAmounts);

        // Ensure balances have been updated post match
        assertCTFBalance(bob, yes, 90_000_000);

        assertCollateralBalance(carla, 9_000_000);
        assertCTFBalance(carla, no, 40_000_000);

        // Ensure onchain state for orders is as expected
        // The taker order is partially filled
        assertEq(exchange.getOrderStatus(exchange.hashOrder(buy)).remaining, 11_000_000);
        assertFalse(exchange.getOrderStatus(exchange.hashOrder(buy)).isFilledOrCancelled);

        // The maker orders get completely filled
        assertEq(exchange.getOrderStatus(exchange.hashOrder(yesSell)).remaining, 0);
        assertTrue(exchange.getOrderStatus(exchange.hashOrder(yesSell)).isFilledOrCancelled);

        assertEq(exchange.getOrderStatus(exchange.hashOrder(noBuy)).remaining, 0);
        assertTrue(exchange.getOrderStatus(exchange.hashOrder(noBuy)).isFilledOrCancelled);
    }

    function testMatchTypeMerge() public {
        // Init Match with YES sell against a NO sell and a Yes buy
        // To match the YES sell with the NO sell, CTF Exchange will MERGE Outcome tokens into collateral
        // Then will fill the YES sell and the NO sell with the resulting collateral
        Order memory yesSell = _createAndSignOrder(bobPK, yes, 100_000_000, 60_000_000, Side.SELL);

        Order memory noSell = _createAndSignOrder(carlaPK, no, 75_000_000, 30_000_000, Side.SELL);

        Order memory yesBuy = _createAndSignOrder(carlaPK, yes, 24_000_000, 40_000_000, Side.BUY);
        Order[] memory makerOrders = new Order[](2);
        makerOrders[0] = noSell;
        makerOrders[1] = yesBuy;

        uint256[] memory fillAmounts = new uint256[](2);
        fillAmounts[0] = 75_000_000;
        fillAmounts[1] = 15_000_000;

        uint256 takerOrderFillAmount = 100_000_000;

        checkpointCollateral(bob);

        checkpointCTF(carla, yes);
        checkpointCollateral(carla);

        vm.prank(admin);
        exchange.matchOrders(yesSell, makerOrders, takerOrderFillAmount, fillAmounts);

        // Ensure balances have been updated post match
        assertCollateralBalance(bob, 60_000_000);

        assertCTFBalance(carla, yes, 25_000_000);
        assertCollateralBalance(carla, 15_000_000);

        // Ensure onchain state for orders is as expected
        // The taker order is fully filled
        assertEq(exchange.getOrderStatus(exchange.hashOrder(yesSell)).remaining, 0);
        assertTrue(exchange.getOrderStatus(exchange.hashOrder(yesSell)).isFilledOrCancelled);

        // The first maker order gets completely filled
        assertEq(exchange.getOrderStatus(exchange.hashOrder(noSell)).remaining, 0);
        assertTrue(exchange.getOrderStatus(exchange.hashOrder(noSell)).isFilledOrCancelled);

        // The second maker order is partially filled
        assertEq(exchange.getOrderStatus(exchange.hashOrder(yesBuy)).remaining, 9_000_000);
        assertFalse(exchange.getOrderStatus(exchange.hashOrder(yesBuy)).isFilledOrCancelled);
    }

    function testMatchTypeComplementaryFuzz(uint128 fillAmount, uint16 takerFeeRateBps, uint16 makerFeeRateBps)
        public
    {
        uint256 makerAmount = 50_000_000;
        uint256 takerAmount = 100_000_000;

        vm.assume(
            fillAmount <= makerAmount && takerFeeRateBps < exchange.getMaxFeeRate()
                && makerFeeRateBps < exchange.getMaxFeeRate()
        );

        // Init a match with a yes buy against a yes sell
        Order memory buy = _createAndSignOrderWithFee(bobPK, yes, makerAmount, takerAmount, takerFeeRateBps, Side.BUY);
        Order memory sell =
            _createAndSignOrderWithFee(carlaPK, yes, takerAmount, makerAmount, makerFeeRateBps, Side.SELL);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = sell;

        uint256[] memory fillAmounts = new uint256[](1);
        uint256 makerFillAmount = _getTakingAmount(fillAmount, makerAmount, takerAmount);
        fillAmounts[0] = makerFillAmount;

        checkpointCollateral(carla);
        checkpointCTF(bob, yes);

        uint256 makerFee = calculateFee(makerFeeRateBps, makerFillAmount, sell.makerAmount, sell.takerAmount, sell.side);
        if (makerFee > 0) {
            vm.expectEmit(true, true, true, false);
            emit FeeCharged(admin, 0, makerFee);
        }

        uint256 takerFee = calculateFee(takerFeeRateBps, fillAmount, fillAmount, makerFillAmount, buy.side);
        if (takerFee > 0) {
            // TakerFee could be >= expected taker fee due to surplus
            vm.expectEmit(true, true, false, false);
            emit FeeCharged(admin, buy.tokenId, takerFee);
        }

        vm.prank(admin);
        exchange.matchOrders(buy, makerOrders, fillAmount, fillAmounts);

        // Ensure balances have been updated post match
        assertCollateralBalance(carla, fillAmount - makerFee);
        assertGe(getCTFBalance(bob, yes), makerFillAmount);
    }

    function testMatchTypeMintFuzz(uint128 fillAmount, uint16 takerFeeRateBps, uint16 makerFeeRateBps) public {
        uint256 makerAmount = 50_000_000;
        uint256 takerAmount = 100_000_000;

        vm.assume(
            fillAmount <= makerAmount && takerFeeRateBps < exchange.getMaxFeeRate()
                && makerFeeRateBps < exchange.getMaxFeeRate()
        );

        // Init a match with a YES buy against a NO buy
        Order memory yesBuy =
            _createAndSignOrderWithFee(bobPK, yes, makerAmount, takerAmount, takerFeeRateBps, Side.BUY);

        Order memory noBuy =
            _createAndSignOrderWithFee(carlaPK, no, makerAmount, takerAmount, makerFeeRateBps, Side.BUY);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = noBuy;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = fillAmount;

        uint256 taking = _getTakingAmount(fillAmount, makerAmount, takerAmount);

        uint256 makerFee = calculateFee(makerFeeRateBps, taking, noBuy.makerAmount, noBuy.takerAmount, noBuy.side);
        if (makerFee > 0) {
            vm.expectEmit(true, true, true, false);
            emit FeeCharged(admin, yes, makerFee);
        }

        uint256 takerFee = calculateFee(takerFeeRateBps, taking, fillAmount, taking, yesBuy.side);
        if (takerFee > 0) {
            vm.expectEmit(true, true, true, false);
            emit FeeCharged(admin, no, takerFee);
        }

        checkpointCTF(carla, no);
        checkpointCTF(bob, yes);

        vm.prank(admin);
        exchange.matchOrders(yesBuy, makerOrders, fillAmount, fillAmounts);

        // Ensure balances have been updated post match
        assertCTFBalance(carla, no, taking - makerFee);
        assertCTFBalance(bob, yes, taking - takerFee);
    }

    function testMatchTypeMergeFuzz(uint128 fillAmount, uint16 takerFeeRateBps, uint16 makerFeeRateBps) public {
        uint256 makerAmount = 100_000_000;
        uint256 takerAmount = 50_000_000;

        vm.assume(
            fillAmount <= makerAmount && takerFeeRateBps < exchange.getMaxFeeRate()
                && makerFeeRateBps < exchange.getMaxFeeRate()
        );

        // Init a match with a YES sell against a NO sell
        Order memory yesSell =
            _createAndSignOrderWithFee(bobPK, yes, makerAmount, takerAmount, takerFeeRateBps, Side.SELL);

        Order memory noSell =
            _createAndSignOrderWithFee(carlaPK, no, makerAmount, takerAmount, makerFeeRateBps, Side.SELL);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = noSell;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = fillAmount;
        uint256 taking = _getTakingAmount(fillAmount, makerAmount, takerAmount);

        uint256 makerFee =
            calculateFee(makerFeeRateBps, fillAmount, noSell.makerAmount, noSell.takerAmount, noSell.side);
        if (makerFee > 0) {
            vm.expectEmit(true, true, true, true);
            emit FeeCharged(admin, 0, makerFee);
        }

        uint256 takerFee = calculateFee(takerFeeRateBps, fillAmount, fillAmount, taking, yesSell.side);
        if (takerFee > 0) {
            // TakerFee could be >= expected taker fee due to surplus
            vm.expectEmit(true, true, true, false);
            emit FeeCharged(admin, 0, takerFee);
        }

        checkpointCollateral(carla);
        checkpointCollateral(bob);

        vm.prank(admin);
        exchange.matchOrders(yesSell, makerOrders, fillAmount, fillAmounts);

        // Ensure balances have been updated post match
        assertCollateralBalance(carla, taking - makerFee);
        assertGe(usdc.balanceOf(bob), taking - takerFee);
    }

    function testTakerRefund() public {
        // Init match with takerFillAmount >> amount needed to fill the maker orders
        // The excess tokens should be refunded to the taker
        Order memory buy = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);

        Order memory sell = _createAndSignOrder(carlaPK, yes, 100_000_000, 40_000_000, Side.SELL);
        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = sell;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 100_000_000;

        // If fill amount is miscalculated, refund the caller any leftover tokens
        // In this test, 40 USDC is needed to fill the sell.
        // The Exchange will refund the taker order maker 10 USDC
        uint256 takerFillAmount = 50_000_000;
        uint256 expectedRefund = 10_000_000;

        vm.expectEmit(true, true, true, false);
        // Assert the refund transfer to the taker order maker
        emit Transfer(address(exchange), bob, expectedRefund);

        vm.prank(admin);
        exchange.matchOrders(buy, makerOrders, takerFillAmount, fillAmounts);
    }

    function testWithFees() public {
        vm.startPrank(admin);

        // Init a yes BUY taker order at 50c with a 10% taker fee
        uint256 takerFeeRate = 1000;
        Order memory buy = _createAndSignOrderWithFee(
            bobPK,
            yes,
            50_000_000,
            100_000_000,
            takerFeeRate, // Taker fee of 10%
            Side.BUY
        );

        // Init a yes SELL order at 50c with a 1% maker fee
        uint256 makerFeeRate = 100;
        Order memory sell = _createAndSignOrderWithFee(
            carlaPK,
            yes,
            100_000_000,
            50_000_000,
            makerFeeRate, // Maker fee of 1%
            Side.SELL
        );

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = sell;
        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 50_000_000;

        uint256 takerFillAmount = 25_000_000;
        uint256 expectedTakerFee = calculateFee(takerFeeRate, 50_000_000, buy.makerAmount, buy.takerAmount, buy.side);
        uint256 expectedMakerFee = calculateFee(makerFeeRate, 50_000_000, sell.makerAmount, sell.takerAmount, sell.side);

        if (expectedMakerFee > 0) {
            vm.expectEmit(true, true, true, false);
            emit FeeCharged(admin, yes, expectedMakerFee);
        }

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(sell), carla, bob, yes, 0, 50_000_000, 25_000_000, expectedMakerFee);

        if (expectedMakerFee > 0) {
            vm.expectEmit(true, true, true, false);
            emit FeeCharged(admin, yes, expectedTakerFee);
        }

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(
            exchange.hashOrder(buy), bob, address(exchange), 0, yes, 25_000_000, 50_000_000, expectedTakerFee
        );

        // Match the orders
        exchange.matchOrders(buy, makerOrders, takerFillAmount, fillAmounts);
    }

    function testWithFeesWithSurplus() public {
        vm.startPrank(admin);

        // Init a yes SELL taker order at 50c with a 1% taker fee
        uint256 takerFeeRate = 100;
        Order memory sell = _createAndSignOrderWithFee(
            bobPK,
            yes,
            100_000_000,
            50_000_000,
            takerFeeRate, // Taker fee of 1%
            Side.SELL
        );

        // Init a yes BUY order at 60c with a 0% maker fee
        uint256 makerFeeRate = 0;
        Order memory buy = _createAndSignOrderWithFee(carlaPK, yes, 60_000_000, 100_000_000, makerFeeRate, Side.BUY);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = buy;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 60_000_000;

        uint256 takerFillAmount = 100_000_000;

        // NOTE: the fee is calculated on the *actual* fill price, vs the price implied by the sell order
        // thus the fee is inclusive of any surplus/price improvements generated
        uint256 expectedTakerFee = calculateFee(takerFeeRate, takerFillAmount, 100_000_000, 60_000_000, sell.side);
        uint256 expectedMakerFee =
            calculateFee(makerFeeRate, takerFillAmount, buy.makerAmount, buy.takerAmount, buy.side);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(buy), carla, bob, 0, yes, 60_000_000, 100_000_000, expectedMakerFee);

        if (expectedTakerFee > 0) {
            vm.expectEmit(true, true, true, true);
            emit FeeCharged(admin, 0, expectedTakerFee);
        }

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(sell), bob, address(exchange), yes, 0, 100_000_000, 60_000_000, expectedTakerFee);

        vm.expectEmit(true, true, true, true);
        emit OrdersMatched(exchange.hashOrder(sell), bob, yes, 0, 100_000_000, 60_000_000);

        // Match the orders
        exchange.matchOrders(sell, makerOrders, takerFillAmount, fillAmounts);
    }

    function testMintWithFees() public {
        vm.startPrank(admin);

        // Init a YES BUY taker order at 50c with a 1% taker fee
        uint256 takerFeeRate = 100;
        Order memory buy = _createAndSignOrderWithFee(
            bobPK,
            yes,
            50_000_000,
            100_000_000,
            takerFeeRate, // Taker fee of 1%
            Side.BUY
        );

        // Init a NO BUY order at 50c with a 0.3% maker fee
        uint256 makerFeeRate = 30;
        Order memory noBuy = _createAndSignOrderWithFee(
            carlaPK,
            no,
            50_000_000,
            100_000_000,
            makerFeeRate, // Maker fee of 0.3%
            Side.BUY
        );

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = noBuy;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 50_000_000;

        uint256 takerFillAmount = 50_000_000;

        uint256 expectedTakerFee = calculateFee(takerFeeRate, 100_000_000, buy.makerAmount, buy.takerAmount, buy.side);
        uint256 expectedMakerFee =
            calculateFee(makerFeeRate, 100_000_000, noBuy.makerAmount, noBuy.takerAmount, noBuy.side);

        vm.expectEmit(true, true, true, true);
        emit FeeCharged(admin, no, expectedMakerFee);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(noBuy), carla, bob, 0, no, 50_000_000, 100_000_000, expectedMakerFee);

        vm.expectEmit(true, true, true, true);
        emit FeeCharged(admin, yes, expectedTakerFee);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(buy), bob, address(exchange), 0, yes, 50_000_000, 100_000_000, expectedTakerFee);

        // Match the orders
        exchange.matchOrders(buy, makerOrders, takerFillAmount, fillAmounts);

        assertCTFBalance(admin, yes, expectedTakerFee);
        assertCTFBalance(admin, no, expectedMakerFee);
    }

    function testMergeWithFees() public {
        vm.startPrank(admin);

        // Init a YES SELL taker order at 50c with a 1% taker fee
        uint256 takerFeeRate = 100;
        Order memory yesSell = _createAndSignOrderWithFee(
            bobPK,
            yes,
            100_000_000,
            50_000_000,
            takerFeeRate, // Taker fee of 1%
            Side.SELL
        );

        // Init a NO SELL order at 50c with a 0.3% maker fee
        uint256 makerFeeRate = 30;
        Order memory noSell = _createAndSignOrderWithFee(
            carlaPK,
            no,
            100_000_000,
            50_000_000,
            makerFeeRate, // Maker fee of 0.3%
            Side.SELL
        );

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = noSell;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 100_000_000;

        uint256 takerFillAmount = 100_000_000;

        uint256 expectedTakerFee =
            calculateFee(takerFeeRate, 100_000_000, yesSell.makerAmount, yesSell.takerAmount, yesSell.side);
        uint256 expectedMakerFee =
            calculateFee(makerFeeRate, 100_000_000, noSell.makerAmount, noSell.takerAmount, noSell.side);

        vm.expectEmit(true, true, true, true);
        emit FeeCharged(admin, 0, expectedMakerFee);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(noSell), carla, bob, no, 0, 100_000_000, 50_000_000, expectedMakerFee);

        vm.expectEmit(true, true, true, true);
        emit FeeCharged(admin, 0, expectedTakerFee);

        vm.expectEmit(true, true, true, true);
        emit OrderFilled(exchange.hashOrder(yesSell), bob, address(exchange), yes, 0, 100_000_000, 50_000_000, expectedTakerFee);

        // Match the orders
        exchange.matchOrders(yesSell, makerOrders, takerFillAmount, fillAmounts);
    }

    /*//////////////////////////////////////////////////////////////
                               FAIL CASES
    //////////////////////////////////////////////////////////////*/

    function testNotCrossingSells() public {
        // 60c YES sell
        Order memory yesSell = _createAndSignOrder(bobPK, yes, 100_000_000, 60_000_000, Side.SELL);

        // 60c NO sell
        Order memory noSell = _createAndSignOrder(carlaPK, no, 100_000_000, 60_000_000, Side.SELL);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = noSell;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 100_000_000;

        uint256 takerOrderFillAmount = 100_000_000;

        // Sells can only match if priceYesSell + priceNoSell < 1
        vm.expectRevert(NotCrossing.selector);
        vm.prank(admin);
        exchange.matchOrders(yesSell, makerOrders, takerOrderFillAmount, fillAmounts);
    }

    function testNotCrossingBuys() public {
        // 50c YES buy
        Order memory yesBuy = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);

        // 40c NO buy
        Order memory noBuy = _createAndSignOrder(carlaPK, no, 40_000_000, 100_000_000, Side.BUY);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = noBuy;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 40_000_000;

        uint256 takerOrderFillAmount = 50_000_000;

        // Buys can only match if priceYesBuy + priceNoBuy > 1
        vm.expectRevert(NotCrossing.selector);
        vm.prank(admin);
        exchange.matchOrders(yesBuy, makerOrders, takerOrderFillAmount, fillAmounts);
    }

    function testNotCrossingBuyVsSell() public {
        // 50c YES buy
        Order memory buy = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);

        // 60c YES sell
        Order memory sell = _createAndSignOrder(carlaPK, no, 100_000_000, 60_000_000, Side.SELL);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = sell;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 0;

        uint256 takerOrderFillAmount = 0;

        vm.expectRevert(NotCrossing.selector);
        vm.prank(admin);
        exchange.matchOrders(buy, makerOrders, takerOrderFillAmount, fillAmounts);
    }

    function testInvalidTrade() public {
        Order memory buy = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        Order memory sell = _createAndSignOrder(carlaPK, no, 100_000_000, 50_000_000, Side.SELL);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = sell;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 100_000_000;

        uint256 takerOrderFillAmount = 50_000_000;

        // Attempt to match a yes buy with a no sell, reverts as this is invalid
        vm.expectRevert(MismatchedTokenIds.selector);
        vm.prank(admin);
        exchange.matchOrders(buy, makerOrders, takerOrderFillAmount, fillAmounts);
    }

    function testMatchNonTaker() public {
        Order memory buy = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);
        buy.taker = carla;
        buy.signature = _signMessage(bobPK, exchange.hashOrder(buy));

        // Sell with taker zero
        Order memory sell = _createAndSignOrder(carlaPK, yes, 100_000_000, 50_000_000, Side.SELL);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = sell;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 100_000_000;

        uint256 takerOrderFillAmount = 50_000_000;

        // Attempt to match orders with admin, incompatible with the taker for the buy order
        // Reverts
        vm.expectRevert(NotTaker.selector);
        vm.prank(admin);
        exchange.matchOrders(buy, makerOrders, takerOrderFillAmount, fillAmounts);

        // Matching with carla suceeds as expected
        vm.expectEmit(true, true, true, true);
        emit OrdersMatched(exchange.hashOrder(buy), bob, 0, yes, 50_000_000, 100_000_000);
        vm.prank(carla);
        exchange.matchOrders(buy, makerOrders, takerOrderFillAmount, fillAmounts);
    }

    function testMatchZeroTakerAmount() public {
        // Create a non-standard buy order with zero taker amount
        Order memory buy = _createAndSignOrder(bobPK, yes, 50_000_000, 0, Side.BUY);

        // Any valid sell order will be able to drain the buy order
        // Init a sell order priced absurdly high
        Order memory sell = _createAndSignOrder(carlaPK, yes, 1, 50_000_000, Side.SELL);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = sell;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 1;

        uint256 takerOrderFillAmount = 50_000_000;

        vm.expectEmit(true, true, true, true);
        emit OrdersMatched(exchange.hashOrder(buy), bob, 0, yes, 50_000_000, 1);

        // The orders are successfully matched
        vm.prank(admin);
        exchange.matchOrders(buy, makerOrders, takerOrderFillAmount, fillAmounts);
    }

    function testMatchInvalidFillAmount() public {
        Order memory buy = _createAndSignOrder(bobPK, yes, 50_000_000, 100_000_000, Side.BUY);

        Order memory sell = _createAndSignOrder(carlaPK, yes, 1_000_000_000, 500_000_000, Side.SELL);

        Order[] memory makerOrders = new Order[](1);
        makerOrders[0] = sell;

        uint256[] memory fillAmounts = new uint256[](1);
        fillAmounts[0] = 1_000_000_000;

        uint256 takerOrderFillAmount = 500_000_000;

        // Attempt to match the above buy and sell, with fillAmount >>> the maker amount of the buy
        // Reverts
        vm.expectRevert(MakingGtRemaining.selector);
        vm.prank(admin);
        exchange.matchOrders(buy, makerOrders, takerOrderFillAmount, fillAmounts);
    }
}



================================================
FILE: src/exchange/test/libraries/CalculatorHelper.t.sol
================================================
// SPDX-License-Identifier: MIT
pragma solidity <0.9.0;

import { Test } from "forge-std/Test.sol";

import { CalculatorHelper } from "exchange/libraries/CalculatorHelper.sol";
import { Side } from "exchange/libraries/OrderStructs.sol";

contract CalculatorHelperTest is Test {
    function testFuzzCalculateTakingAmount(uint64 making, uint128 makerAmount, uint128 takerAmount) public {
        vm.assume(makerAmount > 0 && making <= makerAmount);
        // Explicitly cast to 256 to avoid overflows
        uint256 expected = making * uint256(takerAmount) / uint256(makerAmount);
        assertEq(CalculatorHelper.calculateTakingAmount(making, makerAmount, takerAmount), expected);
    }

    function testFuzzCalculatePrice(uint128 makerAmount, uint128 takerAmount, uint8 sideInt) public {
        vm.assume(sideInt <= 1);
        Side side = Side(sideInt);
        // Asserts not needed, test checks that we can calculate price safely without unexpected reverts

        CalculatorHelper._calculatePrice(makerAmount, takerAmount, side);
    }

    function testFuzzIsCrossing(
        uint128 makerAmountA,
        uint128 takerAmountA,
        uint8 sideIntA,
        uint128 makerAmountB,
        uint128 takerAmountB,
        uint8 sideIntB
    ) public {
        vm.assume(sideIntA <= 1 && sideIntB <= 1);
        Side sideA = Side(sideIntA);
        Side sideB = Side(sideIntB);
        uint256 priceA = CalculatorHelper._calculatePrice(makerAmountA, takerAmountA, sideA);
        uint256 priceB = CalculatorHelper._calculatePrice(makerAmountB, takerAmountB, sideB);

        // Asserts not needed, test checks that we can check isCrossing safely without unexpected reverts
        CalculatorHelper._isCrossing(priceA, priceB, sideA, sideB);
    }
}



================================================
FILE: .github/workflows/Tests.yml
================================================
name: Tests

on: [push]

jobs:
  check:
    name: Tests
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: recursive

      - name: Install Foundry
        uses: foundry-rs/foundry-toolchain@v1
        with:
          version: nightly

      - name: Run tests
        run: forge test -vvv


