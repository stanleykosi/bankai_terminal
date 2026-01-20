Directory structure:
└── polymarket-resolution-subgraph/
    ├── README.md
    ├── docker-compose.yaml
    ├── LICENSE
    ├── networks.json
    ├── package.json
    ├── schema.graphql
    ├── subgraph.yaml
    ├── tsconfig.json
    ├── .env.example
    ├── abis/
    │   ├── ModRegistry.json
    │   ├── OptimisticOracleOld.json
    │   ├── OptimisticOracleV2.json
    │   ├── UmaCtfAdapterOld.json
    │   ├── UmaCtfAdapterV2.json
    │   ├── UmaCtfAdapterV31.json
    │   └── UmaCtfAdapterV4.json
    ├── generated/
    │   ├── schema.ts
    │   ├── OptimisticOracleOld/
    │   │   └── OptimisticOracleOld.ts
    │   ├── OptimisticOracleV2/
    │   │   └── OptimisticOracleV2.ts
    │   └── UmaCtfAdapterOld/
    │       └── UmaCtfAdapterOld.ts
    ├── src/
    │   ├── managed-oo-v2.ts
    │   ├── mod-registry.ts
    │   ├── optimistic-oracle-old.ts
    │   ├── optimistic-oracle-v-2.ts
    │   ├── uma-ctf-adapter-old.ts
    │   ├── uma-ctf-adapter.ts
    │   └── utils/
    │       ├── constants.ts
    │       └── qualifier.ts
    └── tests/
        └── qualifier.test.ts


Files Content:

================================================
FILE: README.md
================================================
# resolutions-subgraph

## Codegen

Run the following command to generate the `generated` folder:

```bash
yarn codegen
```

## Environment Variables

Create a `.env` file with the following variables:

```bash
MATIC_RPC_URL=
```

## Local Development

### Run graph node locally

```bash
docker compose up
```

## Restart graph node and clear volumes

```bash
docker compose down
```

```bash
sudo docker rm resolution-subgraph-graph-node-1 && sudo docker rm resolution-subgraph-ipfs-1 && sudo docker rm resolution-subgraph-postgres-1 && sudo docker rm resolution-subgraph-ganache-1
```

## Create and deploy subgraph

While local subgraph node is running run:

```bash
yarn create-local
```

```bash
yarn deploy-local
```

Access the GraphQL editor at:

[`http://localhost:8000/subgraphs/name/resolutions-subgraph/graphql`](http://localhost:8000/subgraphs/name/resolutions-subgraph/graphql)

**Example query:**

```graphQL
query marketResolutions {
  marketResolutions {
    id
    author
    ancillaryData
    lastUpdateTimestamp
    status
    wasDisputed
    proposedPrice
    reproposedPrice
    price
    updates
  }
}
```

## Goldsky

Build the subgraph with `yarn build` and then run the following to deploy:

```bash
goldsky subgraph deploy resolutions-subgraph/<version> --path .
```



================================================
FILE: docker-compose.yaml
================================================
version: "3.8"
services:

  postgres:
    image: postgres
    environment:
      POSTGRES_PASSWORD: letmein
      POSTGRES_INITDB_ARGS: "-E UTF8 --locale=C"

  ipfs:
    image: ipfs/go-ipfs:v0.4.23
    ports:
      - "5001:5001"

  ganache:
    image: trufflesuite/ganache-cli
    ports:
      - "8545:8545"

  graph-node:
    image: graphprotocol/graph-node
    ports:
      - "8000:8000"
      - "8001:8001"
      - "8020:8020"
      - "8030:8030"
      - "8040:8040"
    environment:
      postgres_host: postgres
      postgres_port: 5432
      postgres_user: postgres
      postgres_pass: letmein
      postgres_db: postgres
      ipfs: "ipfs:5001"
      ethereum: "matic:${MATIC_RPC_URL}"



================================================
FILE: LICENSE
================================================
MIT License

Copyright (c) 2023 Polymarket

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.



================================================
FILE: networks.json
================================================
{
  "matic": {
    "umaCtfAdapterV4": {
      "address": "0x65070BE91477460D8A7AeEb94ef92fe056C2f2A7",
      "startBlock": 74797879
    },
    "umaCtfAdapterV31": {
      "address": "0x157Ce2d672854c848c9b79C49a8Cc6cc89176a49",
      "startBlock": 71214173
    },
    "UmaCtfAdapterV2": {
      "address": "0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74",
      "startBlock": 35203539
    },
    "NegRiskUmaCtfAdapter": {
      "address": "0x2F5e3684cb1F318ec51b00Edba38d79Ac2c0aA9d",
      "startBlock": 50505488
    },
    "ManagedOptimisticOracleV2": {
      "address": "0x2C0367a9DB231dDeBd88a94b4f6461a6e47C58B1",
      "startBlock": 74677419
    },
    "OptimisticOracleV2": {
      "address": "0xeE3Afe347D5C74317041E2618C49534dAf887c24",
      "startBlock": 35203539
    },
    "UmaCtfAdapterOld": {
      "address": "0xCB1822859cEF82Cd2Eb4E6276C7916e692995130",
      "startBlock": 23569780
    },
    "OptimisticOracleOld": {
      "address": "0xBb1A8db2D4350976a11cdfA60A1d43f97710Da49",
      "startBlock": 23569780
    }
  }
}



================================================
FILE: package.json
================================================
{
  "name": "resolutions-subgraph",
  "license": "UNLICENSED",
  "scripts": {
    "codegen": "graph codegen",
    "build": "graph build",
    "deploy": "graph deploy --node https://api.studio.thegraph.com/deploy/ resolutions-subgraph",
    "create-local": "graph create --node http://localhost:8020/ resolutions-subgraph",
    "remove-local": "graph remove --node http://localhost:8020/ resolutions-subgraph",
    "deploy-local": "graph deploy --node http://localhost:8020/ --ipfs http://localhost:5001 resolutions-subgraph",
    "test": "graph test -d"
  },
  "dependencies": {
    "@graphprotocol/graph-cli": "0.51.1",
    "@graphprotocol/graph-ts": "0.30.0",
    "assemblyscript-regex": "^1.6.4"
  },
  "devDependencies": {
    "matchstick-as": "0.5.0"
  }
}



================================================
FILE: schema.graphql
================================================
type MarketResolution @entity {
  id: String! # questionID
  newVersionQ: Boolean! # bool
  author: Bytes! # address
  ancillaryData: Bytes! # bytes
  lastUpdateTimestamp: BigInt! # uint256
  status: String! # initialized/posed/proposed/challenged/reproposed/disputed/resolved
  wasDisputed: Boolean! # bool
  proposedPrice: BigInt! # int256
  reproposedPrice: BigInt! # int256
  price: BigInt! # int256
  updates: String! # comma separated updates
  transactionHash: String # txHash
  logIndex: BigInt # uint256
  approved: Boolean # bool
}

type AncillaryDataHashToQuestionId @entity {
  id: String! # ancillaryDataHash
  questionId: String! # questionID
}

type Moderator @entity {
  id: String! # address
  canMod: Boolean! # bool
}

type Revision @entity {
  id: String!
  moderator: String! # address
  questionId: String! # questionID of the update
  timestamp: BigInt! # uint256
  update: String! # the update posted
  transactionHash: String! # transaction hash
}



================================================
FILE: subgraph.yaml
================================================
specVersion: 0.0.5
schema:
  file: ./schema.graphql
dataSources:
  - kind: ethereum
    name: UmaCtfAdapterV2
    network: matic
    source:
      abi: UmaCtfAdapterV2
      address: "0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74"
      startBlock: 35203539
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.7
      language: wasm/assemblyscript
      entities:
        - MarketResolution
        - Revision
      abis:
        - name: UmaCtfAdapterV2
          file: ./abis/UmaCtfAdapterV2.json
      callHandlers:
        - function: postUpdate(bytes32,bytes)
          handler: handleAncillaryDataUpdated
      eventHandlers:
        - event: QuestionInitialized(indexed bytes32,indexed uint256,indexed
            address,bytes,address,uint256,uint256)
          handler: handleQuestionInitialized
        - event: QuestionReset(indexed bytes32)
          handler: handleQuestionReset
        - event: QuestionResolved(indexed bytes32,indexed int256,uint256[])
          handler: handleQuestionResolved
      file: ./src/uma-ctf-adapter.ts
  - kind: ethereum
    name: UmaCtfAdapterV31
    network: matic
    source:
      abi: UmaCtfAdapterV31
      address: "0x157Ce2d672854c848c9b79C49a8Cc6cc89176a49"
      startBlock: 46755254
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.7
      language: wasm/assemblyscript
      entities:
        - MarketResolution
        - Revision
      abis:
        - name: UmaCtfAdapterV31
          file: ./abis/UmaCtfAdapterV31.json
      callHandlers:
        - function: postUpdate(bytes32,bytes)
          handler: handleAncillaryDataUpdated
      eventHandlers:
        - event: QuestionInitialized(indexed bytes32,indexed uint256,indexed
            address,bytes,address,uint256,uint256)
          handler: handleQuestionInitialized
        - event: QuestionReset(indexed bytes32)
          handler: handleQuestionReset
        - event: QuestionResolved(indexed bytes32,indexed int256,uint256[])
          handler: handleQuestionResolved
      file: ./src/uma-ctf-adapter.ts
  - kind: ethereum
    name: UmaCtfAdapterV4
    network: matic
    source:
      abi: UmaCtfAdapterV4
      address: "0x65070BE91477460D8A7AeEb94ef92fe056C2f2A7"
      startBlock: 74797879
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.7
      language: wasm/assemblyscript
      entities:
        - MarketResolution
        - Revision
      abis:
        - name: UmaCtfAdapterV4
          file: ./abis/UmaCtfAdapterV4.json
      callHandlers:
        - function: postUpdate(bytes32,bytes)
          handler: handleAncillaryDataUpdated
      eventHandlers:
        - event: QuestionInitialized(indexed bytes32,indexed uint256,indexed
            address,bytes,address,uint256,uint256)
          handler: handleQuestionInitialized
        - event: QuestionReset(indexed bytes32)
          handler: handleQuestionReset
        - event: QuestionResolved(indexed bytes32,indexed int256,uint256[])
          handler: handleQuestionResolved
      file: ./src/uma-ctf-adapter.ts
  - kind: ethereum
    name: NegRiskUmaCtfAdapter
    network: matic
    source:
      address: "0x2F5e3684cb1F318ec51b00Edba38d79Ac2c0aA9d"
      abi: UmaCtfAdapterV31
      startBlock: 50505488
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.7
      language: wasm/assemblyscript
      entities:
        - MarketResolution
        - Revision
      abis:
        - name: UmaCtfAdapterV31
          file: ./abis/UmaCtfAdapterV31.json
      callHandlers:
        - function: postUpdate(bytes32,bytes)
          handler: handleAncillaryDataUpdated
      eventHandlers:
        - event: QuestionInitialized(indexed bytes32,indexed uint256,indexed
            address,bytes,address,uint256,uint256)
          handler: handleQuestionInitialized
        - event: QuestionReset(indexed bytes32)
          handler: handleQuestionReset
        - event: QuestionResolved(indexed bytes32,indexed int256,uint256[])
          handler: handleQuestionResolved
      file: ./src/uma-ctf-adapter.ts
  - kind: ethereum
    name: OptimisticOracleV2
    network: matic
    source:
      address: "0xeE3Afe347D5C74317041E2618C49534dAf887c24"
      abi: OptimisticOracleV2
      startBlock: 35203539
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.7
      language: wasm/assemblyscript
      entities:
        - MarketResolution
      abis:
        - name: OptimisticOracleV2
          file: ./abis/OptimisticOracleV2.json
      eventHandlers:
        - event: DisputePrice(indexed address,indexed address,indexed
            address,bytes32,uint256,bytes,int256)
          handler: handleDisputePrice
        - event: ProposePrice(indexed address,indexed
            address,bytes32,uint256,bytes,int256,uint256,address)
          handler: handleProposePrice
        - event: RequestPrice(indexed
            address,bytes32,uint256,bytes,address,uint256,uint256)
          handler: handleRequestPrice
      file: ./src/optimistic-oracle-v-2.ts
  - kind: ethereum
    name: ManagedOptimisticOracleV2
    network: matic
    source:
      address: "0x2C0367a9DB231dDeBd88a94b4f6461a6e47C58B1"
      abi: ManagedOptimisticOracleV2
      startBlock: 74677419
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.7
      language: wasm/assemblyscript
      entities:
        - MarketResolution
      abis:
        - name: ManagedOptimisticOracleV2
          file: ./abis/ManagedOptimisticOracleV2.json
      eventHandlers:
        - event: DisputePrice(indexed address,indexed address,indexed
            address,bytes32,uint256,bytes,int256)
          handler: handleDisputePrice
        - event: ProposePrice(indexed address,indexed
            address,bytes32,uint256,bytes,int256,uint256,address)
          handler: handleProposePrice
        - event: RequestPrice(indexed
            address,bytes32,uint256,bytes,address,uint256,uint256)
          handler: handleRequestPrice
      file: ./src/managed-oo-v2.ts
  - kind: ethereum
    name: UmaCtfAdapterOld
    network: matic
    source:
      address: "0xCB1822859cEF82Cd2Eb4E6276C7916e692995130"
      abi: UmaCtfAdapterOld
      startBlock: 23569780
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.7
      language: wasm/assemblyscript
      entities:
        - MarketResolution
        - AncillaryDataHashToQuestionId
      abis:
        - name: UmaCtfAdapterOld
          file: ./abis/UmaCtfAdapterOld.json
      eventHandlers:
        - event: QuestionInitialized(indexed
            bytes32,bytes,uint256,address,uint256,uint256,bool)
          handler: handleQuestionInitialized
        - event: QuestionReset(indexed bytes32)
          handler: handleQuestionReset
        - event: QuestionResolved(indexed bytes32,indexed bool)
          handler: handleQuestionResolved
        - event: QuestionSettled(indexed bytes32,indexed int256,indexed bool)
          handler: handleQuestionSettled
      file: ./src/uma-ctf-adapter-old.ts
  - kind: ethereum
    name: OptimisticOracleOld
    network: matic
    source:
      address: "0xBb1A8db2D4350976a11cdfA60A1d43f97710Da49"
      abi: OptimisticOracleOld
      startBlock: 23569780
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.7
      language: wasm/assemblyscript
      entities:
        - MarketResolution
        - AncillaryDataHashToQuestionId
      abis:
        - name: OptimisticOracleOld
          file: ./abis/OptimisticOracleOld.json
      eventHandlers:
        - event: DisputePrice(indexed address,indexed address,indexed
            address,bytes32,uint256,bytes,int256)
          handler: handleDisputePrice
        - event: ProposePrice(indexed address,indexed
            address,bytes32,uint256,bytes,int256,uint256,address)
          handler: handleProposePrice
        - event: RequestPrice(indexed
            address,bytes32,uint256,bytes,address,uint256,uint256)
          handler: handleRequestPrice
      file: ./src/optimistic-oracle-old.ts
  - kind: ethereum
    name: ModRegistry
    network: matic
    source:
      abi: ModRegistry
      address: "0xe1c9271516930B9e1355b87232556a0f39D3aBD3"
      startBlock: 52699785
    mapping:
      kind: ethereum/events
      apiVersion: 0.0.7
      language: wasm/assemblyscript
      entities:
        - Moderator
      abis:
        - name: ModRegistry
          file: ./abis/ModRegistry.json
      eventHandlers:
        - event: ModAdded(indexed address,indexed address)
          handler: handleModAdded
        - event: ModRemoved(indexed address,indexed address)
          handler: handleModRemoved
      file: ./src/mod-registry.ts



================================================
FILE: tsconfig.json
================================================
{
  "extends": "@graphprotocol/graph-ts/types/tsconfig.base.json",
  "include": ["src", "tests"]
}



================================================
FILE: .env.example
================================================
MATIC_RPC_URL=


================================================
FILE: abis/ModRegistry.json
================================================
[
    {
        "type": "function",
        "name": "addAdmin",
        "inputs": [
            {
                "name": "admin",
                "type": "address",
                "internalType": "address"
            }
        ],
        "outputs": [],
        "stateMutability": "nonpayable"
    },
    {
        "type": "function",
        "name": "addMod",
        "inputs": [
            {
                "name": "user",
                "type": "address",
                "internalType": "address"
            }
        ],
        "outputs": [],
        "stateMutability": "nonpayable"
    },
    {
        "type": "function",
        "name": "admins",
        "inputs": [
            {
                "name": "",
                "type": "address",
                "internalType": "address"
            }
        ],
        "outputs": [
            {
                "name": "",
                "type": "uint256",
                "internalType": "uint256"
            }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "isAdmin",
        "inputs": [
            {
                "name": "addr",
                "type": "address",
                "internalType": "address"
            }
        ],
        "outputs": [
            {
                "name": "",
                "type": "bool",
                "internalType": "bool"
            }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "isMod",
        "inputs": [
            {
                "name": "user",
                "type": "address",
                "internalType": "address"
            }
        ],
        "outputs": [
            {
                "name": "",
                "type": "bool",
                "internalType": "bool"
            }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "moderators",
        "inputs": [
            {
                "name": "",
                "type": "address",
                "internalType": "address"
            }
        ],
        "outputs": [
            {
                "name": "",
                "type": "bool",
                "internalType": "bool"
            }
        ],
        "stateMutability": "view"
    },
    {
        "type": "function",
        "name": "removeAdmin",
        "inputs": [
            {
                "name": "admin",
                "type": "address",
                "internalType": "address"
            }
        ],
        "outputs": [],
        "stateMutability": "nonpayable"
    },
    {
        "type": "function",
        "name": "removeMod",
        "inputs": [
            {
                "name": "user",
                "type": "address",
                "internalType": "address"
            }
        ],
        "outputs": [],
        "stateMutability": "nonpayable"
    },
    {
        "type": "function",
        "name": "renounceAdmin",
        "inputs": [],
        "outputs": [],
        "stateMutability": "nonpayable"
    },
    {
        "type": "event",
        "name": "ModAdded",
        "inputs": [
            {
                "name": "admin",
                "type": "address",
                "indexed": true,
                "internalType": "address"
            },
            {
                "name": "addedMod",
                "type": "address",
                "indexed": true,
                "internalType": "address"
            }
        ],
        "anonymous": false
    },
    {
        "type": "event",
        "name": "ModRemoved",
        "inputs": [
            {
                "name": "admin",
                "type": "address",
                "indexed": true,
                "internalType": "address"
            },
            {
                "name": "removedMod",
                "type": "address",
                "indexed": true,
                "internalType": "address"
            }
        ],
        "anonymous": false
    },
    {
        "type": "event",
        "name": "NewAdmin",
        "inputs": [
            {
                "name": "admin",
                "type": "address",
                "indexed": true,
                "internalType": "address"
            },
            {
                "name": "newAdminAddress",
                "type": "address",
                "indexed": true,
                "internalType": "address"
            }
        ],
        "anonymous": false
    },
    {
        "type": "event",
        "name": "RemovedAdmin",
        "inputs": [
            {
                "name": "admin",
                "type": "address",
                "indexed": true,
                "internalType": "address"
            },
            {
                "name": "removedAdmin",
                "type": "address",
                "indexed": true,
                "internalType": "address"
            }
        ],
        "anonymous": false
    },
    {
        "type": "error",
        "name": "NotAdmin",
        "inputs": []
    }
]


================================================
FILE: abis/OptimisticOracleOld.json
================================================
[
  {
    "inputs": [
      { "internalType": "uint256", "name": "_liveness", "type": "uint256" },
      {
        "internalType": "address",
        "name": "_finderAddress",
        "type": "address"
      },
      { "internalType": "address", "name": "_timerAddress", "type": "address" }
    ],
    "stateMutability": "nonpayable",
    "type": "constructor"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "requester",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "proposer",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "disputer",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "identifier",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "int256",
        "name": "proposedPrice",
        "type": "int256"
      }
    ],
    "name": "DisputePrice",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "requester",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "proposer",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "identifier",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "int256",
        "name": "proposedPrice",
        "type": "int256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "expirationTimestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "currency",
        "type": "address"
      }
    ],
    "name": "ProposePrice",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "requester",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "identifier",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "currency",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "reward",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "finalFee",
        "type": "uint256"
      }
    ],
    "name": "RequestPrice",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "requester",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "proposer",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "disputer",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "identifier",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "int256",
        "name": "price",
        "type": "int256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "payout",
        "type": "uint256"
      }
    ],
    "name": "Settle",
    "type": "event"
  },
  {
    "inputs": [],
    "name": "ancillaryBytesLimit",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "defaultLiveness",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "disputePrice",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "disputer", "type": "address" },
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "disputePriceFor",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "finder",
    "outputs": [
      {
        "internalType": "contract FinderInterface",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "getCurrentTime",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "getRequest",
    "outputs": [
      {
        "components": [
          { "internalType": "address", "name": "proposer", "type": "address" },
          { "internalType": "address", "name": "disputer", "type": "address" },
          {
            "internalType": "contract IERC20",
            "name": "currency",
            "type": "address"
          },
          { "internalType": "bool", "name": "settled", "type": "bool" },
          { "internalType": "bool", "name": "refundOnDispute", "type": "bool" },
          {
            "internalType": "int256",
            "name": "proposedPrice",
            "type": "int256"
          },
          {
            "internalType": "int256",
            "name": "resolvedPrice",
            "type": "int256"
          },
          {
            "internalType": "uint256",
            "name": "expirationTime",
            "type": "uint256"
          },
          { "internalType": "uint256", "name": "reward", "type": "uint256" },
          { "internalType": "uint256", "name": "finalFee", "type": "uint256" },
          { "internalType": "uint256", "name": "bond", "type": "uint256" },
          {
            "internalType": "uint256",
            "name": "customLiveness",
            "type": "uint256"
          }
        ],
        "internalType": "struct OptimisticOracleInterface.Request",
        "name": "",
        "type": "tuple"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "getState",
    "outputs": [
      {
        "internalType": "enum OptimisticOracleInterface.State",
        "name": "",
        "type": "uint8"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "hasPrice",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "int256", "name": "proposedPrice", "type": "int256" }
    ],
    "name": "proposePrice",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "proposer", "type": "address" },
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "int256", "name": "proposedPrice", "type": "int256" }
    ],
    "name": "proposePriceFor",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      {
        "internalType": "contract IERC20",
        "name": "currency",
        "type": "address"
      },
      { "internalType": "uint256", "name": "reward", "type": "uint256" }
    ],
    "name": "requestPrice",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
    "name": "requests",
    "outputs": [
      { "internalType": "address", "name": "proposer", "type": "address" },
      { "internalType": "address", "name": "disputer", "type": "address" },
      {
        "internalType": "contract IERC20",
        "name": "currency",
        "type": "address"
      },
      { "internalType": "bool", "name": "settled", "type": "bool" },
      { "internalType": "bool", "name": "refundOnDispute", "type": "bool" },
      { "internalType": "int256", "name": "proposedPrice", "type": "int256" },
      { "internalType": "int256", "name": "resolvedPrice", "type": "int256" },
      {
        "internalType": "uint256",
        "name": "expirationTime",
        "type": "uint256"
      },
      { "internalType": "uint256", "name": "reward", "type": "uint256" },
      { "internalType": "uint256", "name": "finalFee", "type": "uint256" },
      { "internalType": "uint256", "name": "bond", "type": "uint256" },
      { "internalType": "uint256", "name": "customLiveness", "type": "uint256" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "uint256", "name": "bond", "type": "uint256" }
    ],
    "name": "setBond",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "time", "type": "uint256" }
    ],
    "name": "setCurrentTime",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "uint256", "name": "customLiveness", "type": "uint256" }
    ],
    "name": "setCustomLiveness",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "setRefundOnDispute",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "settle",
    "outputs": [
      { "internalType": "uint256", "name": "payout", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "settleAndGetPrice",
    "outputs": [{ "internalType": "int256", "name": "", "type": "int256" }],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "address", "name": "requester", "type": "address" }
    ],
    "name": "stampAncillaryData",
    "outputs": [{ "internalType": "bytes", "name": "", "type": "bytes" }],
    "stateMutability": "pure",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "timerAddress",
    "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
    "stateMutability": "view",
    "type": "function"
  }
]



================================================
FILE: abis/OptimisticOracleV2.json
================================================
[
  {
    "inputs": [
      { "internalType": "uint256", "name": "_liveness", "type": "uint256" },
      {
        "internalType": "address",
        "name": "_finderAddress",
        "type": "address"
      },
      { "internalType": "address", "name": "_timerAddress", "type": "address" }
    ],
    "stateMutability": "nonpayable",
    "type": "constructor"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "requester",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "proposer",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "disputer",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "identifier",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "int256",
        "name": "proposedPrice",
        "type": "int256"
      }
    ],
    "name": "DisputePrice",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "requester",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "proposer",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "identifier",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "int256",
        "name": "proposedPrice",
        "type": "int256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "expirationTimestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "currency",
        "type": "address"
      }
    ],
    "name": "ProposePrice",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "requester",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "identifier",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "currency",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "reward",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "finalFee",
        "type": "uint256"
      }
    ],
    "name": "RequestPrice",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "requester",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "proposer",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "disputer",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "identifier",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "int256",
        "name": "price",
        "type": "int256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "payout",
        "type": "uint256"
      }
    ],
    "name": "Settle",
    "type": "event"
  },
  {
    "inputs": [],
    "name": "OO_ANCILLARY_DATA_LIMIT",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "TOO_EARLY_RESPONSE",
    "outputs": [{ "internalType": "int256", "name": "", "type": "int256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "ancillaryBytesLimit",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "defaultLiveness",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "disputePrice",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "disputer", "type": "address" },
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "disputePriceFor",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "finder",
    "outputs": [
      {
        "internalType": "contract FinderInterface",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "getCurrentTime",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "getRequest",
    "outputs": [
      {
        "components": [
          { "internalType": "address", "name": "proposer", "type": "address" },
          { "internalType": "address", "name": "disputer", "type": "address" },
          {
            "internalType": "contract IERC20",
            "name": "currency",
            "type": "address"
          },
          { "internalType": "bool", "name": "settled", "type": "bool" },
          {
            "components": [
              { "internalType": "bool", "name": "eventBased", "type": "bool" },
              {
                "internalType": "bool",
                "name": "refundOnDispute",
                "type": "bool"
              },
              {
                "internalType": "bool",
                "name": "callbackOnPriceProposed",
                "type": "bool"
              },
              {
                "internalType": "bool",
                "name": "callbackOnPriceDisputed",
                "type": "bool"
              },
              {
                "internalType": "bool",
                "name": "callbackOnPriceSettled",
                "type": "bool"
              },
              { "internalType": "uint256", "name": "bond", "type": "uint256" },
              {
                "internalType": "uint256",
                "name": "customLiveness",
                "type": "uint256"
              }
            ],
            "internalType": "struct OptimisticOracleV2Interface.RequestSettings",
            "name": "requestSettings",
            "type": "tuple"
          },
          {
            "internalType": "int256",
            "name": "proposedPrice",
            "type": "int256"
          },
          {
            "internalType": "int256",
            "name": "resolvedPrice",
            "type": "int256"
          },
          {
            "internalType": "uint256",
            "name": "expirationTime",
            "type": "uint256"
          },
          { "internalType": "uint256", "name": "reward", "type": "uint256" },
          { "internalType": "uint256", "name": "finalFee", "type": "uint256" }
        ],
        "internalType": "struct OptimisticOracleV2Interface.Request",
        "name": "",
        "type": "tuple"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "getState",
    "outputs": [
      {
        "internalType": "enum OptimisticOracleV2Interface.State",
        "name": "",
        "type": "uint8"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "hasPrice",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "int256", "name": "proposedPrice", "type": "int256" }
    ],
    "name": "proposePrice",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "proposer", "type": "address" },
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "int256", "name": "proposedPrice", "type": "int256" }
    ],
    "name": "proposePriceFor",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      {
        "internalType": "contract IERC20",
        "name": "currency",
        "type": "address"
      },
      { "internalType": "uint256", "name": "reward", "type": "uint256" }
    ],
    "name": "requestPrice",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
    "name": "requests",
    "outputs": [
      { "internalType": "address", "name": "proposer", "type": "address" },
      { "internalType": "address", "name": "disputer", "type": "address" },
      {
        "internalType": "contract IERC20",
        "name": "currency",
        "type": "address"
      },
      { "internalType": "bool", "name": "settled", "type": "bool" },
      {
        "components": [
          { "internalType": "bool", "name": "eventBased", "type": "bool" },
          { "internalType": "bool", "name": "refundOnDispute", "type": "bool" },
          {
            "internalType": "bool",
            "name": "callbackOnPriceProposed",
            "type": "bool"
          },
          {
            "internalType": "bool",
            "name": "callbackOnPriceDisputed",
            "type": "bool"
          },
          {
            "internalType": "bool",
            "name": "callbackOnPriceSettled",
            "type": "bool"
          },
          { "internalType": "uint256", "name": "bond", "type": "uint256" },
          {
            "internalType": "uint256",
            "name": "customLiveness",
            "type": "uint256"
          }
        ],
        "internalType": "struct OptimisticOracleV2Interface.RequestSettings",
        "name": "requestSettings",
        "type": "tuple"
      },
      { "internalType": "int256", "name": "proposedPrice", "type": "int256" },
      { "internalType": "int256", "name": "resolvedPrice", "type": "int256" },
      {
        "internalType": "uint256",
        "name": "expirationTime",
        "type": "uint256"
      },
      { "internalType": "uint256", "name": "reward", "type": "uint256" },
      { "internalType": "uint256", "name": "finalFee", "type": "uint256" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "uint256", "name": "bond", "type": "uint256" }
    ],
    "name": "setBond",
    "outputs": [
      { "internalType": "uint256", "name": "totalBond", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      {
        "internalType": "bool",
        "name": "callbackOnPriceProposed",
        "type": "bool"
      },
      {
        "internalType": "bool",
        "name": "callbackOnPriceDisputed",
        "type": "bool"
      },
      {
        "internalType": "bool",
        "name": "callbackOnPriceSettled",
        "type": "bool"
      }
    ],
    "name": "setCallbacks",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "uint256", "name": "time", "type": "uint256" }
    ],
    "name": "setCurrentTime",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "uint256", "name": "customLiveness", "type": "uint256" }
    ],
    "name": "setCustomLiveness",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "setEventBased",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "setRefundOnDispute",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "requester", "type": "address" },
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "settle",
    "outputs": [
      { "internalType": "uint256", "name": "payout", "type": "uint256" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "identifier", "type": "bytes32" },
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "name": "settleAndGetPrice",
    "outputs": [{ "internalType": "int256", "name": "", "type": "int256" }],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "address", "name": "requester", "type": "address" }
    ],
    "name": "stampAncillaryData",
    "outputs": [{ "internalType": "bytes", "name": "", "type": "bytes" }],
    "stateMutability": "pure",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "timerAddress",
    "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
    "stateMutability": "view",
    "type": "function"
  }
]



================================================
FILE: abis/UmaCtfAdapterOld.json
================================================
[
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "conditionalTokenAddress",
        "type": "address"
      },
      {
        "internalType": "address",
        "name": "umaFinderAddress",
        "type": "address"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "constructor"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "usr",
        "type": "address"
      }
    ],
    "name": "AuthorizedUser",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "usr",
        "type": "address"
      }
    ],
    "name": "DeauthorizedUser",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "address",
        "name": "oldFinder",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "newFinder",
        "type": "address"
      }
    ],
    "name": "NewFinderAddress",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionFlaggedForAdminResolution",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "resolutionTime",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "rewardToken",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "reward",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "proposalBond",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bool",
        "name": "earlyResolutionEnabled",
        "type": "bool"
      }
    ],
    "name": "QuestionInitialized",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionPaused",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionReset",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "bool",
        "name": "emergencyReport",
        "type": "bool"
      }
    ],
    "name": "QuestionResolved",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "int256",
        "name": "settledPrice",
        "type": "int256"
      },
      {
        "indexed": true,
        "internalType": "bool",
        "name": "earlyResolution",
        "type": "bool"
      }
    ],
    "name": "QuestionSettled",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionUnpaused",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "resolutionTime",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "rewardToken",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "reward",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "proposalBond",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bool",
        "name": "earlyResolutionEnabled",
        "type": "bool"
      }
    ],
    "name": "QuestionUpdated",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "requestor",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "uint256",
        "name": "requestTimestamp",
        "type": "uint256"
      },
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "bytes32",
        "name": "identifier",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "rewardToken",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "reward",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "proposalBond",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "bool",
        "name": "earlyResolution",
        "type": "bool"
      }
    ],
    "name": "ResolutionDataRequested",
    "type": "event"
  },
  {
    "inputs": [],
    "name": "conditionalTokenContract",
    "outputs": [
      {
        "internalType": "contract IConditionalTokens",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [{ "internalType": "address", "name": "usr", "type": "address" }],
    "name": "deny",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" },
      { "internalType": "uint256[]", "name": "payouts", "type": "uint256[]" }
    ],
    "name": "emergencyReportPayouts",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "emergencySafetyPeriod",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "flagQuestionForEmergencyResolution",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "getExpectedPayouts",
    "outputs": [
      { "internalType": "uint256[]", "name": "", "type": "uint256[]" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "identifier",
    "outputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "ignorePrice",
    "outputs": [{ "internalType": "int256", "name": "", "type": "int256" }],
    "stateMutability": "pure",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      {
        "internalType": "uint256",
        "name": "resolutionTime",
        "type": "uint256"
      },
      { "internalType": "address", "name": "rewardToken", "type": "address" },
      { "internalType": "uint256", "name": "reward", "type": "uint256" },
      { "internalType": "uint256", "name": "proposalBond", "type": "uint256" },
      {
        "internalType": "bool",
        "name": "earlyResolutionEnabled",
        "type": "bool"
      }
    ],
    "name": "initializeQuestion",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "isQuestionFlaggedForEmergencyResolution",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "isQuestionInitialized",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "pauseQuestion",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      {
        "internalType": "uint256",
        "name": "resolutionTime",
        "type": "uint256"
      },
      { "internalType": "address", "name": "rewardToken", "type": "address" },
      { "internalType": "uint256", "name": "reward", "type": "uint256" },
      { "internalType": "uint256", "name": "proposalBond", "type": "uint256" },
      {
        "internalType": "bool",
        "name": "earlyResolutionEnabled",
        "type": "bool"
      }
    ],
    "name": "prepareAndInitialize",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
    "name": "questions",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "resolutionTime",
        "type": "uint256"
      },
      { "internalType": "uint256", "name": "reward", "type": "uint256" },
      { "internalType": "uint256", "name": "proposalBond", "type": "uint256" },
      { "internalType": "uint256", "name": "settled", "type": "uint256" },
      {
        "internalType": "uint256",
        "name": "requestTimestamp",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "adminResolutionTimestamp",
        "type": "uint256"
      },
      {
        "internalType": "bool",
        "name": "earlyResolutionEnabled",
        "type": "bool"
      },
      { "internalType": "bool", "name": "resolved", "type": "bool" },
      { "internalType": "bool", "name": "paused", "type": "bool" },
      { "internalType": "address", "name": "rewardToken", "type": "address" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "readyToRequestResolution",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "readyToSettle",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [{ "internalType": "address", "name": "usr", "type": "address" }],
    "name": "rely",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "reportPayouts",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "requestResolutionData",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "newFinderAddress",
        "type": "address"
      }
    ],
    "name": "setFinderAddress",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "settle",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "umaFinder",
    "outputs": [{ "internalType": "address", "name": "", "type": "address" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "unPauseQuestion",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      {
        "internalType": "uint256",
        "name": "resolutionTime",
        "type": "uint256"
      },
      { "internalType": "address", "name": "rewardToken", "type": "address" },
      { "internalType": "uint256", "name": "reward", "type": "uint256" },
      { "internalType": "uint256", "name": "proposalBond", "type": "uint256" },
      {
        "internalType": "bool",
        "name": "earlyResolutionEnabled",
        "type": "bool"
      }
    ],
    "name": "updateQuestion",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [{ "internalType": "address", "name": "", "type": "address" }],
    "name": "wards",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  }
]



================================================
FILE: abis/UmaCtfAdapterV2.json
================================================
[
  {
    "inputs": [
      { "internalType": "address", "name": "_ctf", "type": "address" },
      { "internalType": "address", "name": "_finder", "type": "address" }
    ],
    "stateMutability": "nonpayable",
    "type": "constructor"
  },
  { "inputs": [], "name": "Flagged", "type": "error" },
  { "inputs": [], "name": "Initialized", "type": "error" },
  { "inputs": [], "name": "InvalidAncillaryData", "type": "error" },
  { "inputs": [], "name": "InvalidOOPrice", "type": "error" },
  { "inputs": [], "name": "InvalidPayouts", "type": "error" },
  { "inputs": [], "name": "NotAdmin", "type": "error" },
  { "inputs": [], "name": "NotFlagged", "type": "error" },
  { "inputs": [], "name": "NotInitialized", "type": "error" },
  { "inputs": [], "name": "NotOptimisticOracle", "type": "error" },
  { "inputs": [], "name": "NotReadyToResolve", "type": "error" },
  { "inputs": [], "name": "Paused", "type": "error" },
  { "inputs": [], "name": "PriceNotAvailable", "type": "error" },
  { "inputs": [], "name": "Resolved", "type": "error" },
  { "inputs": [], "name": "SafetyPeriodNotPassed", "type": "error" },
  { "inputs": [], "name": "UnsupportedToken", "type": "error" },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "owner",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "update",
        "type": "bytes"
      }
    ],
    "name": "AncillaryDataUpdated",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "admin",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "newAdminAddress",
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
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256[]",
        "name": "payouts",
        "type": "uint256[]"
      }
    ],
    "name": "QuestionEmergencyResolved",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionFlagged",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "uint256",
        "name": "requestTimestamp",
        "type": "uint256"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "creator",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "rewardToken",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "reward",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "proposalBond",
        "type": "uint256"
      }
    ],
    "name": "QuestionInitialized",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionPaused",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionReset",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "int256",
        "name": "settledPrice",
        "type": "int256"
      },
      {
        "indexed": false,
        "internalType": "uint256[]",
        "name": "payouts",
        "type": "uint256[]"
      }
    ],
    "name": "QuestionResolved",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionUnpaused",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "admin",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "removedAdmin",
        "type": "address"
      }
    ],
    "name": "RemovedAdmin",
    "type": "event"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "admin", "type": "address" }
    ],
    "name": "addAdmin",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [{ "internalType": "address", "name": "", "type": "address" }],
    "name": "admins",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "collateralWhitelist",
    "outputs": [
      {
        "internalType": "contract IAddressWhitelist",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "ctf",
    "outputs": [
      {
        "internalType": "contract IConditionalTokens",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" },
      { "internalType": "uint256[]", "name": "payouts", "type": "uint256[]" }
    ],
    "name": "emergencyResolve",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "emergencySafetyPeriod",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "flag",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "getExpectedPayouts",
    "outputs": [
      { "internalType": "uint256[]", "name": "", "type": "uint256[]" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" },
      { "internalType": "address", "name": "owner", "type": "address" }
    ],
    "name": "getLatestUpdate",
    "outputs": [
      {
        "components": [
          { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
          { "internalType": "bytes", "name": "update", "type": "bytes" }
        ],
        "internalType": "struct BulletinBoard.AncillaryDataUpdate",
        "name": "",
        "type": "tuple"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "getQuestion",
    "outputs": [
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "requestTimestamp",
            "type": "uint256"
          },
          { "internalType": "uint256", "name": "reward", "type": "uint256" },
          {
            "internalType": "uint256",
            "name": "proposalBond",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "emergencyResolutionTimestamp",
            "type": "uint256"
          },
          { "internalType": "bool", "name": "resolved", "type": "bool" },
          { "internalType": "bool", "name": "paused", "type": "bool" },
          { "internalType": "bool", "name": "reset", "type": "bool" },
          {
            "internalType": "address",
            "name": "rewardToken",
            "type": "address"
          },
          { "internalType": "address", "name": "creator", "type": "address" },
          { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
        ],
        "internalType": "struct QuestionData",
        "name": "",
        "type": "tuple"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" },
      { "internalType": "address", "name": "owner", "type": "address" }
    ],
    "name": "getUpdates",
    "outputs": [
      {
        "components": [
          { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
          { "internalType": "bytes", "name": "update", "type": "bytes" }
        ],
        "internalType": "struct BulletinBoard.AncillaryDataUpdate[]",
        "name": "",
        "type": "tuple[]"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "address", "name": "rewardToken", "type": "address" },
      { "internalType": "uint256", "name": "reward", "type": "uint256" },
      { "internalType": "uint256", "name": "proposalBond", "type": "uint256" }
    ],
    "name": "initialize",
    "outputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "addr", "type": "address" }
    ],
    "name": "isAdmin",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "isFlagged",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "isInitialized",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "maxAncillaryData",
    "outputs": [{ "internalType": "uint256", "name": "", "type": "uint256" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "optimisticOracle",
    "outputs": [
      {
        "internalType": "contract IOptimisticOracleV2",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "pause",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" },
      { "internalType": "bytes", "name": "update", "type": "bytes" }
    ],
    "name": "postUpdate",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "", "type": "bytes32" },
      { "internalType": "uint256", "name": "", "type": "uint256" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" },
      { "internalType": "uint256", "name": "", "type": "uint256" }
    ],
    "name": "priceDisputed",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
    "name": "questions",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "requestTimestamp",
        "type": "uint256"
      },
      { "internalType": "uint256", "name": "reward", "type": "uint256" },
      { "internalType": "uint256", "name": "proposalBond", "type": "uint256" },
      {
        "internalType": "uint256",
        "name": "emergencyResolutionTimestamp",
        "type": "uint256"
      },
      { "internalType": "bool", "name": "resolved", "type": "bool" },
      { "internalType": "bool", "name": "paused", "type": "bool" },
      { "internalType": "bool", "name": "reset", "type": "bool" },
      { "internalType": "address", "name": "rewardToken", "type": "address" },
      { "internalType": "address", "name": "creator", "type": "address" },
      { "internalType": "bytes", "name": "ancillaryData", "type": "bytes" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "ready",
    "outputs": [{ "internalType": "bool", "name": "", "type": "bool" }],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "address", "name": "admin", "type": "address" }
    ],
    "name": "removeAdmin",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "renounceAdmin",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "reset",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "resolve",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "questionID", "type": "bytes32" }
    ],
    "name": "unpause",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      { "internalType": "bytes32", "name": "", "type": "bytes32" },
      { "internalType": "uint256", "name": "", "type": "uint256" }
    ],
    "name": "updates",
    "outputs": [
      { "internalType": "uint256", "name": "timestamp", "type": "uint256" },
      { "internalType": "bytes", "name": "update", "type": "bytes" }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "yesOrNoIdentifier",
    "outputs": [{ "internalType": "bytes32", "name": "", "type": "bytes32" }],
    "stateMutability": "view",
    "type": "function"
  }
]



================================================
FILE: abis/UmaCtfAdapterV31.json
================================================
[
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "_ctf",
        "type": "address"
      },
      {
        "internalType": "address",
        "name": "_finder",
        "type": "address"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "constructor"
  },
  {
    "inputs": [],
    "name": "Flagged",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "Initialized",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "InvalidAncillaryData",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "InvalidOOPrice",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "InvalidPayouts",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "NotAdmin",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "NotFlagged",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "NotInitialized",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "NotOptimisticOracle",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "NotReadyToResolve",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "Paused",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "PriceNotAvailable",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "Resolved",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "SafetyPeriodNotPassed",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "SafetyPeriodPassed",
    "type": "error"
  },
  {
    "inputs": [],
    "name": "UnsupportedToken",
    "type": "error"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "owner",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "update",
        "type": "bytes"
      }
    ],
    "name": "AncillaryDataUpdated",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "admin",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "newAdminAddress",
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
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": false,
        "internalType": "uint256[]",
        "name": "payouts",
        "type": "uint256[]"
      }
    ],
    "name": "QuestionEmergencyResolved",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionFlagged",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "uint256",
        "name": "requestTimestamp",
        "type": "uint256"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "creator",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "indexed": false,
        "internalType": "address",
        "name": "rewardToken",
        "type": "address"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "reward",
        "type": "uint256"
      },
      {
        "indexed": false,
        "internalType": "uint256",
        "name": "proposalBond",
        "type": "uint256"
      }
    ],
    "name": "QuestionInitialized",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionPaused",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionReset",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "indexed": true,
        "internalType": "int256",
        "name": "settledPrice",
        "type": "int256"
      },
      {
        "indexed": false,
        "internalType": "uint256[]",
        "name": "payouts",
        "type": "uint256[]"
      }
    ],
    "name": "QuestionResolved",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionUnflagged",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "QuestionUnpaused",
    "type": "event"
  },
  {
    "anonymous": false,
    "inputs": [
      {
        "indexed": true,
        "internalType": "address",
        "name": "admin",
        "type": "address"
      },
      {
        "indexed": true,
        "internalType": "address",
        "name": "removedAdmin",
        "type": "address"
      }
    ],
    "name": "RemovedAdmin",
    "type": "event"
  },
  {
    "inputs": [],
    "name": "EMERGENCY_SAFETY_PERIOD",
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
    "inputs": [],
    "name": "MAX_ANCILLARY_DATA",
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
    "inputs": [],
    "name": "YES_OR_NO_IDENTIFIER",
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
        "internalType": "address",
        "name": "admin",
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
    "inputs": [],
    "name": "collateralWhitelist",
    "outputs": [
      {
        "internalType": "contract IAddressWhitelist",
        "name": "",
        "type": "address"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [],
    "name": "ctf",
    "outputs": [
      {
        "internalType": "contract IConditionalTokens",
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
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "internalType": "uint256[]",
        "name": "payouts",
        "type": "uint256[]"
      }
    ],
    "name": "emergencyResolve",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "flag",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "getExpectedPayouts",
    "outputs": [
      {
        "internalType": "uint256[]",
        "name": "",
        "type": "uint256[]"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "internalType": "address",
        "name": "owner",
        "type": "address"
      }
    ],
    "name": "getLatestUpdate",
    "outputs": [
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "timestamp",
            "type": "uint256"
          },
          {
            "internalType": "bytes",
            "name": "update",
            "type": "bytes"
          }
        ],
        "internalType": "struct AncillaryDataUpdate",
        "name": "",
        "type": "tuple"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "getQuestion",
    "outputs": [
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "requestTimestamp",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "reward",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "proposalBond",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "liveness",
            "type": "uint256"
          },
          {
            "internalType": "uint256",
            "name": "emergencyResolutionTimestamp",
            "type": "uint256"
          },
          {
            "internalType": "bool",
            "name": "resolved",
            "type": "bool"
          },
          {
            "internalType": "bool",
            "name": "paused",
            "type": "bool"
          },
          {
            "internalType": "bool",
            "name": "reset",
            "type": "bool"
          },
          {
            "internalType": "bool",
            "name": "refund",
            "type": "bool"
          },
          {
            "internalType": "address",
            "name": "rewardToken",
            "type": "address"
          },
          {
            "internalType": "address",
            "name": "creator",
            "type": "address"
          },
          {
            "internalType": "bytes",
            "name": "ancillaryData",
            "type": "bytes"
          }
        ],
        "internalType": "struct QuestionData",
        "name": "",
        "type": "tuple"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "internalType": "address",
        "name": "owner",
        "type": "address"
      }
    ],
    "name": "getUpdates",
    "outputs": [
      {
        "components": [
          {
            "internalType": "uint256",
            "name": "timestamp",
            "type": "uint256"
          },
          {
            "internalType": "bytes",
            "name": "update",
            "type": "bytes"
          }
        ],
        "internalType": "struct AncillaryDataUpdate[]",
        "name": "",
        "type": "tuple[]"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "internalType": "address",
        "name": "rewardToken",
        "type": "address"
      },
      {
        "internalType": "uint256",
        "name": "reward",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "proposalBond",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "liveness",
        "type": "uint256"
      }
    ],
    "name": "initialize",
    "outputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "address",
        "name": "addr",
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
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "isFlagged",
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
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "isInitialized",
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
    "name": "optimisticOracle",
    "outputs": [
      {
        "internalType": "contract IOptimisticOracleV2",
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
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "pause",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      },
      {
        "internalType": "bytes",
        "name": "update",
        "type": "bytes"
      }
    ],
    "name": "postUpdate",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "",
        "type": "bytes32"
      },
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      },
      {
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      },
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "name": "priceDisputed",
    "outputs": [],
    "stateMutability": "nonpayable",
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
    "name": "questions",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "requestTimestamp",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "reward",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "proposalBond",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "liveness",
        "type": "uint256"
      },
      {
        "internalType": "uint256",
        "name": "emergencyResolutionTimestamp",
        "type": "uint256"
      },
      {
        "internalType": "bool",
        "name": "resolved",
        "type": "bool"
      },
      {
        "internalType": "bool",
        "name": "paused",
        "type": "bool"
      },
      {
        "internalType": "bool",
        "name": "reset",
        "type": "bool"
      },
      {
        "internalType": "bool",
        "name": "refund",
        "type": "bool"
      },
      {
        "internalType": "address",
        "name": "rewardToken",
        "type": "address"
      },
      {
        "internalType": "address",
        "name": "creator",
        "type": "address"
      },
      {
        "internalType": "bytes",
        "name": "ancillaryData",
        "type": "bytes"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "ready",
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
    "inputs": [],
    "name": "renounceAdmin",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "reset",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "resolve",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "unflag",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "questionID",
        "type": "bytes32"
      }
    ],
    "name": "unpause",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  },
  {
    "inputs": [
      {
        "internalType": "bytes32",
        "name": "",
        "type": "bytes32"
      },
      {
        "internalType": "uint256",
        "name": "",
        "type": "uint256"
      }
    ],
    "name": "updates",
    "outputs": [
      {
        "internalType": "uint256",
        "name": "timestamp",
        "type": "uint256"
      },
      {
        "internalType": "bytes",
        "name": "update",
        "type": "bytes"
      }
    ],
    "stateMutability": "view",
    "type": "function"
  }
]



================================================
FILE: abis/UmaCtfAdapterV4.json
================================================
[
    {
        "inputs": [
            {
                "internalType": "address",
                "name": "_ctf",
                "type": "address"
            },
            {
                "internalType": "address",
                "name": "_finder",
                "type": "address"
            },
            {
                "internalType": "address",
                "name": "_oo",
                "type": "address"
            }
        ],
        "stateMutability": "nonpayable",
        "type": "constructor"
    },
    {
        "inputs": [],
        "name": "Flagged",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "Initialized",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "InvalidAncillaryData",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "InvalidOOPrice",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "InvalidPayouts",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "NotAdmin",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "NotFlagged",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "NotInitialized",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "NotOptimisticOracle",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "NotReadyToResolve",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "Paused",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "PriceNotAvailable",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "Resolved",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "SafetyPeriodNotPassed",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "SafetyPeriodPassed",
        "type": "error"
    },
    {
        "inputs": [],
        "name": "UnsupportedToken",
        "type": "error"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            },
            {
                "indexed": true,
                "internalType": "address",
                "name": "owner",
                "type": "address"
            },
            {
                "indexed": false,
                "internalType": "bytes",
                "name": "update",
                "type": "bytes"
            }
        ],
        "name": "AncillaryDataUpdated",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "address",
                "name": "admin",
                "type": "address"
            },
            {
                "indexed": true,
                "internalType": "address",
                "name": "newAdminAddress",
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
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "QuestionFlagged",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            },
            {
                "indexed": true,
                "internalType": "uint256",
                "name": "requestTimestamp",
                "type": "uint256"
            },
            {
                "indexed": true,
                "internalType": "address",
                "name": "creator",
                "type": "address"
            },
            {
                "indexed": false,
                "internalType": "bytes",
                "name": "ancillaryData",
                "type": "bytes"
            },
            {
                "indexed": false,
                "internalType": "address",
                "name": "rewardToken",
                "type": "address"
            },
            {
                "indexed": false,
                "internalType": "uint256",
                "name": "reward",
                "type": "uint256"
            },
            {
                "indexed": false,
                "internalType": "uint256",
                "name": "proposalBond",
                "type": "uint256"
            }
        ],
        "name": "QuestionInitialized",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            },
            {
                "indexed": false,
                "internalType": "uint256[]",
                "name": "payouts",
                "type": "uint256[]"
            }
        ],
        "name": "QuestionManuallyResolved",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "QuestionPaused",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "QuestionReset",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            },
            {
                "indexed": true,
                "internalType": "int256",
                "name": "settledPrice",
                "type": "int256"
            },
            {
                "indexed": false,
                "internalType": "uint256[]",
                "name": "payouts",
                "type": "uint256[]"
            }
        ],
        "name": "QuestionResolved",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "QuestionUnflagged",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "QuestionUnpaused",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "internalType": "address",
                "name": "admin",
                "type": "address"
            },
            {
                "indexed": true,
                "internalType": "address",
                "name": "removedAdmin",
                "type": "address"
            }
        ],
        "name": "RemovedAdmin",
        "type": "event"
    },
    {
        "inputs": [],
        "name": "MAX_ANCILLARY_DATA",
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
        "inputs": [],
        "name": "SAFETY_PERIOD",
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
        "inputs": [],
        "name": "YES_OR_NO_IDENTIFIER",
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
                "internalType": "address",
                "name": "admin",
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
        "inputs": [],
        "name": "collateralWhitelist",
        "outputs": [
            {
                "internalType": "contract IAddressWhitelist",
                "name": "",
                "type": "address"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [],
        "name": "ctf",
        "outputs": [
            {
                "internalType": "contract IConditionalTokens",
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
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "flag",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "getExpectedPayouts",
        "outputs": [
            {
                "internalType": "uint256[]",
                "name": "",
                "type": "uint256[]"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            },
            {
                "internalType": "address",
                "name": "owner",
                "type": "address"
            }
        ],
        "name": "getLatestUpdate",
        "outputs": [
            {
                "components": [
                    {
                        "internalType": "uint256",
                        "name": "timestamp",
                        "type": "uint256"
                    },
                    {
                        "internalType": "bytes",
                        "name": "update",
                        "type": "bytes"
                    }
                ],
                "internalType": "struct AncillaryDataUpdate",
                "name": "",
                "type": "tuple"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "getQuestion",
        "outputs": [
            {
                "components": [
                    {
                        "internalType": "uint256",
                        "name": "requestTimestamp",
                        "type": "uint256"
                    },
                    {
                        "internalType": "uint256",
                        "name": "reward",
                        "type": "uint256"
                    },
                    {
                        "internalType": "uint256",
                        "name": "proposalBond",
                        "type": "uint256"
                    },
                    {
                        "internalType": "uint256",
                        "name": "liveness",
                        "type": "uint256"
                    },
                    {
                        "internalType": "uint256",
                        "name": "manualResolutionTimestamp",
                        "type": "uint256"
                    },
                    {
                        "internalType": "bool",
                        "name": "resolved",
                        "type": "bool"
                    },
                    {
                        "internalType": "bool",
                        "name": "paused",
                        "type": "bool"
                    },
                    {
                        "internalType": "bool",
                        "name": "reset",
                        "type": "bool"
                    },
                    {
                        "internalType": "bool",
                        "name": "refund",
                        "type": "bool"
                    },
                    {
                        "internalType": "address",
                        "name": "rewardToken",
                        "type": "address"
                    },
                    {
                        "internalType": "address",
                        "name": "creator",
                        "type": "address"
                    },
                    {
                        "internalType": "bytes",
                        "name": "ancillaryData",
                        "type": "bytes"
                    }
                ],
                "internalType": "struct QuestionData",
                "name": "",
                "type": "tuple"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            },
            {
                "internalType": "address",
                "name": "owner",
                "type": "address"
            }
        ],
        "name": "getUpdates",
        "outputs": [
            {
                "components": [
                    {
                        "internalType": "uint256",
                        "name": "timestamp",
                        "type": "uint256"
                    },
                    {
                        "internalType": "bytes",
                        "name": "update",
                        "type": "bytes"
                    }
                ],
                "internalType": "struct AncillaryDataUpdate[]",
                "name": "",
                "type": "tuple[]"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes",
                "name": "ancillaryData",
                "type": "bytes"
            },
            {
                "internalType": "address",
                "name": "rewardToken",
                "type": "address"
            },
            {
                "internalType": "uint256",
                "name": "reward",
                "type": "uint256"
            },
            {
                "internalType": "uint256",
                "name": "proposalBond",
                "type": "uint256"
            },
            {
                "internalType": "uint256",
                "name": "liveness",
                "type": "uint256"
            }
        ],
        "name": "initialize",
        "outputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "address",
                "name": "addr",
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
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "isFlagged",
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
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "isInitialized",
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
        "name": "optimisticOracle",
        "outputs": [
            {
                "internalType": "contract IOptimisticOracleV2",
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
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "pause",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            },
            {
                "internalType": "bytes",
                "name": "update",
                "type": "bytes"
            }
        ],
        "name": "postUpdate",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "",
                "type": "bytes32"
            },
            {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
            },
            {
                "internalType": "bytes",
                "name": "ancillaryData",
                "type": "bytes"
            },
            {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
            }
        ],
        "name": "priceDisputed",
        "outputs": [],
        "stateMutability": "nonpayable",
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
        "name": "questions",
        "outputs": [
            {
                "internalType": "uint256",
                "name": "requestTimestamp",
                "type": "uint256"
            },
            {
                "internalType": "uint256",
                "name": "reward",
                "type": "uint256"
            },
            {
                "internalType": "uint256",
                "name": "proposalBond",
                "type": "uint256"
            },
            {
                "internalType": "uint256",
                "name": "liveness",
                "type": "uint256"
            },
            {
                "internalType": "uint256",
                "name": "manualResolutionTimestamp",
                "type": "uint256"
            },
            {
                "internalType": "bool",
                "name": "resolved",
                "type": "bool"
            },
            {
                "internalType": "bool",
                "name": "paused",
                "type": "bool"
            },
            {
                "internalType": "bool",
                "name": "reset",
                "type": "bool"
            },
            {
                "internalType": "bool",
                "name": "refund",
                "type": "bool"
            },
            {
                "internalType": "address",
                "name": "rewardToken",
                "type": "address"
            },
            {
                "internalType": "address",
                "name": "creator",
                "type": "address"
            },
            {
                "internalType": "bytes",
                "name": "ancillaryData",
                "type": "bytes"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "ready",
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
        "inputs": [],
        "name": "renounceAdmin",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "reset",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "resolve",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            },
            {
                "internalType": "uint256[]",
                "name": "payouts",
                "type": "uint256[]"
            }
        ],
        "name": "resolveManually",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "unflag",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "questionID",
                "type": "bytes32"
            }
        ],
        "name": "unpause",
        "outputs": [],
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "inputs": [
            {
                "internalType": "bytes32",
                "name": "",
                "type": "bytes32"
            },
            {
                "internalType": "uint256",
                "name": "",
                "type": "uint256"
            }
        ],
        "name": "updates",
        "outputs": [
            {
                "internalType": "uint256",
                "name": "timestamp",
                "type": "uint256"
            },
            {
                "internalType": "bytes",
                "name": "update",
                "type": "bytes"
            }
        ],
        "stateMutability": "view",
        "type": "function"
    }
]


================================================
FILE: generated/schema.ts
================================================
// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  TypedMap,
  Entity,
  Value,
  ValueKind,
  store,
  Bytes,
  BigInt,
  BigDecimal
} from "@graphprotocol/graph-ts";

export class MarketResolution extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save MarketResolution entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type MarketResolution must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("MarketResolution", id.toString(), this);
    }
  }

  static loadInBlock(id: string): MarketResolution | null {
    return changetype<MarketResolution | null>(
      store.get_in_block("MarketResolution", id)
    );
  }

  static load(id: string): MarketResolution | null {
    return changetype<MarketResolution | null>(
      store.get("MarketResolution", id)
    );
  }

  get id(): string {
    let value = this.get("id");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get newVersionQ(): boolean {
    let value = this.get("newVersionQ");
    if (!value || value.kind == ValueKind.NULL) {
      return false;
    } else {
      return value.toBoolean();
    }
  }

  set newVersionQ(value: boolean) {
    this.set("newVersionQ", Value.fromBoolean(value));
  }

  get author(): Bytes {
    let value = this.get("author");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toBytes();
    }
  }

  set author(value: Bytes) {
    this.set("author", Value.fromBytes(value));
  }

  get ancillaryData(): Bytes {
    let value = this.get("ancillaryData");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toBytes();
    }
  }

  set ancillaryData(value: Bytes) {
    this.set("ancillaryData", Value.fromBytes(value));
  }

  get lastUpdateTimestamp(): BigInt {
    let value = this.get("lastUpdateTimestamp");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toBigInt();
    }
  }

  set lastUpdateTimestamp(value: BigInt) {
    this.set("lastUpdateTimestamp", Value.fromBigInt(value));
  }

  get status(): string {
    let value = this.get("status");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set status(value: string) {
    this.set("status", Value.fromString(value));
  }

  get wasDisputed(): boolean {
    let value = this.get("wasDisputed");
    if (!value || value.kind == ValueKind.NULL) {
      return false;
    } else {
      return value.toBoolean();
    }
  }

  set wasDisputed(value: boolean) {
    this.set("wasDisputed", Value.fromBoolean(value));
  }

  get proposedPrice(): BigInt {
    let value = this.get("proposedPrice");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toBigInt();
    }
  }

  set proposedPrice(value: BigInt) {
    this.set("proposedPrice", Value.fromBigInt(value));
  }

  get reproposedPrice(): BigInt {
    let value = this.get("reproposedPrice");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toBigInt();
    }
  }

  set reproposedPrice(value: BigInt) {
    this.set("reproposedPrice", Value.fromBigInt(value));
  }

  get price(): BigInt {
    let value = this.get("price");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toBigInt();
    }
  }

  set price(value: BigInt) {
    this.set("price", Value.fromBigInt(value));
  }

  get updates(): string {
    let value = this.get("updates");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set updates(value: string) {
    this.set("updates", Value.fromString(value));
  }

  get transactionHash(): string | null {
    let value = this.get("transactionHash");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toString();
    }
  }

  set transactionHash(value: string | null) {
    if (!value) {
      this.unset("transactionHash");
    } else {
      this.set("transactionHash", Value.fromString(<string>value));
    }
  }

  get logIndex(): BigInt | null {
    let value = this.get("logIndex");
    if (!value || value.kind == ValueKind.NULL) {
      return null;
    } else {
      return value.toBigInt();
    }
  }

  set logIndex(value: BigInt | null) {
    if (!value) {
      this.unset("logIndex");
    } else {
      this.set("logIndex", Value.fromBigInt(<BigInt>value));
    }
  }

  get approved(): boolean {
    let value = this.get("approved");
    if (!value || value.kind == ValueKind.NULL) {
      return false;
    } else {
      return value.toBoolean();
    }
  }

  set approved(value: boolean) {
    this.set("approved", Value.fromBoolean(value));
  }
}

export class AncillaryDataHashToQuestionId extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(
      id != null,
      "Cannot save AncillaryDataHashToQuestionId entity without an ID"
    );
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type AncillaryDataHashToQuestionId must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("AncillaryDataHashToQuestionId", id.toString(), this);
    }
  }

  static loadInBlock(id: string): AncillaryDataHashToQuestionId | null {
    return changetype<AncillaryDataHashToQuestionId | null>(
      store.get_in_block("AncillaryDataHashToQuestionId", id)
    );
  }

  static load(id: string): AncillaryDataHashToQuestionId | null {
    return changetype<AncillaryDataHashToQuestionId | null>(
      store.get("AncillaryDataHashToQuestionId", id)
    );
  }

  get id(): string {
    let value = this.get("id");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get questionId(): string {
    let value = this.get("questionId");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set questionId(value: string) {
    this.set("questionId", Value.fromString(value));
  }
}

export class Moderator extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Moderator entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Moderator must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Moderator", id.toString(), this);
    }
  }

  static loadInBlock(id: string): Moderator | null {
    return changetype<Moderator | null>(store.get_in_block("Moderator", id));
  }

  static load(id: string): Moderator | null {
    return changetype<Moderator | null>(store.get("Moderator", id));
  }

  get id(): string {
    let value = this.get("id");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get canMod(): boolean {
    let value = this.get("canMod");
    if (!value || value.kind == ValueKind.NULL) {
      return false;
    } else {
      return value.toBoolean();
    }
  }

  set canMod(value: boolean) {
    this.set("canMod", Value.fromBoolean(value));
  }
}

export class Revision extends Entity {
  constructor(id: string) {
    super();
    this.set("id", Value.fromString(id));
  }

  save(): void {
    let id = this.get("id");
    assert(id != null, "Cannot save Revision entity without an ID");
    if (id) {
      assert(
        id.kind == ValueKind.STRING,
        `Entities of type Revision must have an ID of type String but the id '${id.displayData()}' is of type ${id.displayKind()}`
      );
      store.set("Revision", id.toString(), this);
    }
  }

  static loadInBlock(id: string): Revision | null {
    return changetype<Revision | null>(store.get_in_block("Revision", id));
  }

  static load(id: string): Revision | null {
    return changetype<Revision | null>(store.get("Revision", id));
  }

  get id(): string {
    let value = this.get("id");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set id(value: string) {
    this.set("id", Value.fromString(value));
  }

  get moderator(): string {
    let value = this.get("moderator");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set moderator(value: string) {
    this.set("moderator", Value.fromString(value));
  }

  get questionId(): string {
    let value = this.get("questionId");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set questionId(value: string) {
    this.set("questionId", Value.fromString(value));
  }

  get timestamp(): BigInt {
    let value = this.get("timestamp");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toBigInt();
    }
  }

  set timestamp(value: BigInt) {
    this.set("timestamp", Value.fromBigInt(value));
  }

  get update(): string {
    let value = this.get("update");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set update(value: string) {
    this.set("update", Value.fromString(value));
  }

  get transactionHash(): string {
    let value = this.get("transactionHash");
    if (!value || value.kind == ValueKind.NULL) {
      throw new Error("Cannot return null for a required field.");
    } else {
      return value.toString();
    }
  }

  set transactionHash(value: string) {
    this.set("transactionHash", Value.fromString(value));
  }
}



================================================
FILE: generated/OptimisticOracleOld/OptimisticOracleOld.ts
================================================
// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  ethereum,
  JSONValue,
  TypedMap,
  Entity,
  Bytes,
  Address,
  BigInt
} from "@graphprotocol/graph-ts";

export class DisputePrice extends ethereum.Event {
  get params(): DisputePrice__Params {
    return new DisputePrice__Params(this);
  }
}

export class DisputePrice__Params {
  _event: DisputePrice;

  constructor(event: DisputePrice) {
    this._event = event;
  }

  get requester(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get proposer(): Address {
    return this._event.parameters[1].value.toAddress();
  }

  get disputer(): Address {
    return this._event.parameters[2].value.toAddress();
  }

  get identifier(): Bytes {
    return this._event.parameters[3].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._event.parameters[4].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[5].value.toBytes();
  }

  get proposedPrice(): BigInt {
    return this._event.parameters[6].value.toBigInt();
  }
}

export class ProposePrice extends ethereum.Event {
  get params(): ProposePrice__Params {
    return new ProposePrice__Params(this);
  }
}

export class ProposePrice__Params {
  _event: ProposePrice;

  constructor(event: ProposePrice) {
    this._event = event;
  }

  get requester(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get proposer(): Address {
    return this._event.parameters[1].value.toAddress();
  }

  get identifier(): Bytes {
    return this._event.parameters[2].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._event.parameters[3].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[4].value.toBytes();
  }

  get proposedPrice(): BigInt {
    return this._event.parameters[5].value.toBigInt();
  }

  get expirationTimestamp(): BigInt {
    return this._event.parameters[6].value.toBigInt();
  }

  get currency(): Address {
    return this._event.parameters[7].value.toAddress();
  }
}

export class RequestPrice extends ethereum.Event {
  get params(): RequestPrice__Params {
    return new RequestPrice__Params(this);
  }
}

export class RequestPrice__Params {
  _event: RequestPrice;

  constructor(event: RequestPrice) {
    this._event = event;
  }

  get requester(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get identifier(): Bytes {
    return this._event.parameters[1].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._event.parameters[2].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[3].value.toBytes();
  }

  get currency(): Address {
    return this._event.parameters[4].value.toAddress();
  }

  get reward(): BigInt {
    return this._event.parameters[5].value.toBigInt();
  }

  get finalFee(): BigInt {
    return this._event.parameters[6].value.toBigInt();
  }
}

export class Settle extends ethereum.Event {
  get params(): Settle__Params {
    return new Settle__Params(this);
  }
}

export class Settle__Params {
  _event: Settle;

  constructor(event: Settle) {
    this._event = event;
  }

  get requester(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get proposer(): Address {
    return this._event.parameters[1].value.toAddress();
  }

  get disputer(): Address {
    return this._event.parameters[2].value.toAddress();
  }

  get identifier(): Bytes {
    return this._event.parameters[3].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._event.parameters[4].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[5].value.toBytes();
  }

  get price(): BigInt {
    return this._event.parameters[6].value.toBigInt();
  }

  get payout(): BigInt {
    return this._event.parameters[7].value.toBigInt();
  }
}

export class OptimisticOracleOld__getRequestResultValue0Struct extends ethereum.Tuple {
  get proposer(): Address {
    return this[0].toAddress();
  }

  get disputer(): Address {
    return this[1].toAddress();
  }

  get currency(): Address {
    return this[2].toAddress();
  }

  get settled(): boolean {
    return this[3].toBoolean();
  }

  get refundOnDispute(): boolean {
    return this[4].toBoolean();
  }

  get proposedPrice(): BigInt {
    return this[5].toBigInt();
  }

  get resolvedPrice(): BigInt {
    return this[6].toBigInt();
  }

  get expirationTime(): BigInt {
    return this[7].toBigInt();
  }

  get reward(): BigInt {
    return this[8].toBigInt();
  }

  get finalFee(): BigInt {
    return this[9].toBigInt();
  }

  get bond(): BigInt {
    return this[10].toBigInt();
  }

  get customLiveness(): BigInt {
    return this[11].toBigInt();
  }
}

export class OptimisticOracleOld__requestsResult {
  value0: Address;
  value1: Address;
  value2: Address;
  value3: boolean;
  value4: boolean;
  value5: BigInt;
  value6: BigInt;
  value7: BigInt;
  value8: BigInt;
  value9: BigInt;
  value10: BigInt;
  value11: BigInt;

  constructor(
    value0: Address,
    value1: Address,
    value2: Address,
    value3: boolean,
    value4: boolean,
    value5: BigInt,
    value6: BigInt,
    value7: BigInt,
    value8: BigInt,
    value9: BigInt,
    value10: BigInt,
    value11: BigInt
  ) {
    this.value0 = value0;
    this.value1 = value1;
    this.value2 = value2;
    this.value3 = value3;
    this.value4 = value4;
    this.value5 = value5;
    this.value6 = value6;
    this.value7 = value7;
    this.value8 = value8;
    this.value9 = value9;
    this.value10 = value10;
    this.value11 = value11;
  }

  toMap(): TypedMap<string, ethereum.Value> {
    let map = new TypedMap<string, ethereum.Value>();
    map.set("value0", ethereum.Value.fromAddress(this.value0));
    map.set("value1", ethereum.Value.fromAddress(this.value1));
    map.set("value2", ethereum.Value.fromAddress(this.value2));
    map.set("value3", ethereum.Value.fromBoolean(this.value3));
    map.set("value4", ethereum.Value.fromBoolean(this.value4));
    map.set("value5", ethereum.Value.fromSignedBigInt(this.value5));
    map.set("value6", ethereum.Value.fromSignedBigInt(this.value6));
    map.set("value7", ethereum.Value.fromUnsignedBigInt(this.value7));
    map.set("value8", ethereum.Value.fromUnsignedBigInt(this.value8));
    map.set("value9", ethereum.Value.fromUnsignedBigInt(this.value9));
    map.set("value10", ethereum.Value.fromUnsignedBigInt(this.value10));
    map.set("value11", ethereum.Value.fromUnsignedBigInt(this.value11));
    return map;
  }

  getProposer(): Address {
    return this.value0;
  }

  getDisputer(): Address {
    return this.value1;
  }

  getCurrency(): Address {
    return this.value2;
  }

  getSettled(): boolean {
    return this.value3;
  }

  getRefundOnDispute(): boolean {
    return this.value4;
  }

  getProposedPrice(): BigInt {
    return this.value5;
  }

  getResolvedPrice(): BigInt {
    return this.value6;
  }

  getExpirationTime(): BigInt {
    return this.value7;
  }

  getReward(): BigInt {
    return this.value8;
  }

  getFinalFee(): BigInt {
    return this.value9;
  }

  getBond(): BigInt {
    return this.value10;
  }

  getCustomLiveness(): BigInt {
    return this.value11;
  }
}

export class OptimisticOracleOld extends ethereum.SmartContract {
  static bind(address: Address): OptimisticOracleOld {
    return new OptimisticOracleOld("OptimisticOracleOld", address);
  }

  ancillaryBytesLimit(): BigInt {
    let result = super.call(
      "ancillaryBytesLimit",
      "ancillaryBytesLimit():(uint256)",
      []
    );

    return result[0].toBigInt();
  }

  try_ancillaryBytesLimit(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "ancillaryBytesLimit",
      "ancillaryBytesLimit():(uint256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  defaultLiveness(): BigInt {
    let result = super.call(
      "defaultLiveness",
      "defaultLiveness():(uint256)",
      []
    );

    return result[0].toBigInt();
  }

  try_defaultLiveness(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "defaultLiveness",
      "defaultLiveness():(uint256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  disputePrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): BigInt {
    let result = super.call(
      "disputePrice",
      "disputePrice(address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBigInt();
  }

  try_disputePrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "disputePrice",
      "disputePrice(address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  disputePriceFor(
    disputer: Address,
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): BigInt {
    let result = super.call(
      "disputePriceFor",
      "disputePriceFor(address,address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(disputer),
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBigInt();
  }

  try_disputePriceFor(
    disputer: Address,
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "disputePriceFor",
      "disputePriceFor(address,address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(disputer),
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  finder(): Address {
    let result = super.call("finder", "finder():(address)", []);

    return result[0].toAddress();
  }

  try_finder(): ethereum.CallResult<Address> {
    let result = super.tryCall("finder", "finder():(address)", []);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  getCurrentTime(): BigInt {
    let result = super.call("getCurrentTime", "getCurrentTime():(uint256)", []);

    return result[0].toBigInt();
  }

  try_getCurrentTime(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "getCurrentTime",
      "getCurrentTime():(uint256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  getRequest(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): OptimisticOracleOld__getRequestResultValue0Struct {
    let result = super.call(
      "getRequest",
      "getRequest(address,bytes32,uint256,bytes):((address,address,address,bool,bool,int256,int256,uint256,uint256,uint256,uint256,uint256))",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return changetype<OptimisticOracleOld__getRequestResultValue0Struct>(
      result[0].toTuple()
    );
  }

  try_getRequest(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<OptimisticOracleOld__getRequestResultValue0Struct> {
    let result = super.tryCall(
      "getRequest",
      "getRequest(address,bytes32,uint256,bytes):((address,address,address,bool,bool,int256,int256,uint256,uint256,uint256,uint256,uint256))",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(
      changetype<OptimisticOracleOld__getRequestResultValue0Struct>(
        value[0].toTuple()
      )
    );
  }

  getState(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): i32 {
    let result = super.call(
      "getState",
      "getState(address,bytes32,uint256,bytes):(uint8)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toI32();
  }

  try_getState(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<i32> {
    let result = super.tryCall(
      "getState",
      "getState(address,bytes32,uint256,bytes):(uint8)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toI32());
  }

  hasPrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): boolean {
    let result = super.call(
      "hasPrice",
      "hasPrice(address,bytes32,uint256,bytes):(bool)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBoolean();
  }

  try_hasPrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<boolean> {
    let result = super.tryCall(
      "hasPrice",
      "hasPrice(address,bytes32,uint256,bytes):(bool)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBoolean());
  }

  proposePrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    proposedPrice: BigInt
  ): BigInt {
    let result = super.call(
      "proposePrice",
      "proposePrice(address,bytes32,uint256,bytes,int256):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromSignedBigInt(proposedPrice)
      ]
    );

    return result[0].toBigInt();
  }

  try_proposePrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    proposedPrice: BigInt
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "proposePrice",
      "proposePrice(address,bytes32,uint256,bytes,int256):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromSignedBigInt(proposedPrice)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  proposePriceFor(
    proposer: Address,
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    proposedPrice: BigInt
  ): BigInt {
    let result = super.call(
      "proposePriceFor",
      "proposePriceFor(address,address,bytes32,uint256,bytes,int256):(uint256)",
      [
        ethereum.Value.fromAddress(proposer),
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromSignedBigInt(proposedPrice)
      ]
    );

    return result[0].toBigInt();
  }

  try_proposePriceFor(
    proposer: Address,
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    proposedPrice: BigInt
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "proposePriceFor",
      "proposePriceFor(address,address,bytes32,uint256,bytes,int256):(uint256)",
      [
        ethereum.Value.fromAddress(proposer),
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromSignedBigInt(proposedPrice)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  requestPrice(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    currency: Address,
    reward: BigInt
  ): BigInt {
    let result = super.call(
      "requestPrice",
      "requestPrice(bytes32,uint256,bytes,address,uint256):(uint256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromAddress(currency),
        ethereum.Value.fromUnsignedBigInt(reward)
      ]
    );

    return result[0].toBigInt();
  }

  try_requestPrice(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    currency: Address,
    reward: BigInt
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "requestPrice",
      "requestPrice(bytes32,uint256,bytes,address,uint256):(uint256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromAddress(currency),
        ethereum.Value.fromUnsignedBigInt(reward)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  requests(param0: Bytes): OptimisticOracleOld__requestsResult {
    let result = super.call(
      "requests",
      "requests(bytes32):(address,address,address,bool,bool,int256,int256,uint256,uint256,uint256,uint256,uint256)",
      [ethereum.Value.fromFixedBytes(param0)]
    );

    return new OptimisticOracleOld__requestsResult(
      result[0].toAddress(),
      result[1].toAddress(),
      result[2].toAddress(),
      result[3].toBoolean(),
      result[4].toBoolean(),
      result[5].toBigInt(),
      result[6].toBigInt(),
      result[7].toBigInt(),
      result[8].toBigInt(),
      result[9].toBigInt(),
      result[10].toBigInt(),
      result[11].toBigInt()
    );
  }

  try_requests(
    param0: Bytes
  ): ethereum.CallResult<OptimisticOracleOld__requestsResult> {
    let result = super.tryCall(
      "requests",
      "requests(bytes32):(address,address,address,bool,bool,int256,int256,uint256,uint256,uint256,uint256,uint256)",
      [ethereum.Value.fromFixedBytes(param0)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(
      new OptimisticOracleOld__requestsResult(
        value[0].toAddress(),
        value[1].toAddress(),
        value[2].toAddress(),
        value[3].toBoolean(),
        value[4].toBoolean(),
        value[5].toBigInt(),
        value[6].toBigInt(),
        value[7].toBigInt(),
        value[8].toBigInt(),
        value[9].toBigInt(),
        value[10].toBigInt(),
        value[11].toBigInt()
      )
    );
  }

  setBond(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    bond: BigInt
  ): BigInt {
    let result = super.call(
      "setBond",
      "setBond(bytes32,uint256,bytes,uint256):(uint256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromUnsignedBigInt(bond)
      ]
    );

    return result[0].toBigInt();
  }

  try_setBond(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    bond: BigInt
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "setBond",
      "setBond(bytes32,uint256,bytes,uint256):(uint256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromUnsignedBigInt(bond)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  settle(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): BigInt {
    let result = super.call(
      "settle",
      "settle(address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBigInt();
  }

  try_settle(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "settle",
      "settle(address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  settleAndGetPrice(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): BigInt {
    let result = super.call(
      "settleAndGetPrice",
      "settleAndGetPrice(bytes32,uint256,bytes):(int256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBigInt();
  }

  try_settleAndGetPrice(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "settleAndGetPrice",
      "settleAndGetPrice(bytes32,uint256,bytes):(int256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  stampAncillaryData(ancillaryData: Bytes, requester: Address): Bytes {
    let result = super.call(
      "stampAncillaryData",
      "stampAncillaryData(bytes,address):(bytes)",
      [
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromAddress(requester)
      ]
    );

    return result[0].toBytes();
  }

  try_stampAncillaryData(
    ancillaryData: Bytes,
    requester: Address
  ): ethereum.CallResult<Bytes> {
    let result = super.tryCall(
      "stampAncillaryData",
      "stampAncillaryData(bytes,address):(bytes)",
      [
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromAddress(requester)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBytes());
  }

  timerAddress(): Address {
    let result = super.call("timerAddress", "timerAddress():(address)", []);

    return result[0].toAddress();
  }

  try_timerAddress(): ethereum.CallResult<Address> {
    let result = super.tryCall("timerAddress", "timerAddress():(address)", []);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }
}

export class ConstructorCall extends ethereum.Call {
  get inputs(): ConstructorCall__Inputs {
    return new ConstructorCall__Inputs(this);
  }

  get outputs(): ConstructorCall__Outputs {
    return new ConstructorCall__Outputs(this);
  }
}

export class ConstructorCall__Inputs {
  _call: ConstructorCall;

  constructor(call: ConstructorCall) {
    this._call = call;
  }

  get _liveness(): BigInt {
    return this._call.inputValues[0].value.toBigInt();
  }

  get _finderAddress(): Address {
    return this._call.inputValues[1].value.toAddress();
  }

  get _timerAddress(): Address {
    return this._call.inputValues[2].value.toAddress();
  }
}

export class ConstructorCall__Outputs {
  _call: ConstructorCall;

  constructor(call: ConstructorCall) {
    this._call = call;
  }
}

export class DisputePriceCall extends ethereum.Call {
  get inputs(): DisputePriceCall__Inputs {
    return new DisputePriceCall__Inputs(this);
  }

  get outputs(): DisputePriceCall__Outputs {
    return new DisputePriceCall__Outputs(this);
  }
}

export class DisputePriceCall__Inputs {
  _call: DisputePriceCall;

  constructor(call: DisputePriceCall) {
    this._call = call;
  }

  get requester(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[3].value.toBytes();
  }
}

export class DisputePriceCall__Outputs {
  _call: DisputePriceCall;

  constructor(call: DisputePriceCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class DisputePriceForCall extends ethereum.Call {
  get inputs(): DisputePriceForCall__Inputs {
    return new DisputePriceForCall__Inputs(this);
  }

  get outputs(): DisputePriceForCall__Outputs {
    return new DisputePriceForCall__Outputs(this);
  }
}

export class DisputePriceForCall__Inputs {
  _call: DisputePriceForCall;

  constructor(call: DisputePriceForCall) {
    this._call = call;
  }

  get disputer(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get requester(): Address {
    return this._call.inputValues[1].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[3].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[4].value.toBytes();
  }
}

export class DisputePriceForCall__Outputs {
  _call: DisputePriceForCall;

  constructor(call: DisputePriceForCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class ProposePriceCall extends ethereum.Call {
  get inputs(): ProposePriceCall__Inputs {
    return new ProposePriceCall__Inputs(this);
  }

  get outputs(): ProposePriceCall__Outputs {
    return new ProposePriceCall__Outputs(this);
  }
}

export class ProposePriceCall__Inputs {
  _call: ProposePriceCall;

  constructor(call: ProposePriceCall) {
    this._call = call;
  }

  get requester(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[3].value.toBytes();
  }

  get proposedPrice(): BigInt {
    return this._call.inputValues[4].value.toBigInt();
  }
}

export class ProposePriceCall__Outputs {
  _call: ProposePriceCall;

  constructor(call: ProposePriceCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class ProposePriceForCall extends ethereum.Call {
  get inputs(): ProposePriceForCall__Inputs {
    return new ProposePriceForCall__Inputs(this);
  }

  get outputs(): ProposePriceForCall__Outputs {
    return new ProposePriceForCall__Outputs(this);
  }
}

export class ProposePriceForCall__Inputs {
  _call: ProposePriceForCall;

  constructor(call: ProposePriceForCall) {
    this._call = call;
  }

  get proposer(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get requester(): Address {
    return this._call.inputValues[1].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[3].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[4].value.toBytes();
  }

  get proposedPrice(): BigInt {
    return this._call.inputValues[5].value.toBigInt();
  }
}

export class ProposePriceForCall__Outputs {
  _call: ProposePriceForCall;

  constructor(call: ProposePriceForCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class RequestPriceCall extends ethereum.Call {
  get inputs(): RequestPriceCall__Inputs {
    return new RequestPriceCall__Inputs(this);
  }

  get outputs(): RequestPriceCall__Outputs {
    return new RequestPriceCall__Outputs(this);
  }
}

export class RequestPriceCall__Inputs {
  _call: RequestPriceCall;

  constructor(call: RequestPriceCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get currency(): Address {
    return this._call.inputValues[3].value.toAddress();
  }

  get reward(): BigInt {
    return this._call.inputValues[4].value.toBigInt();
  }
}

export class RequestPriceCall__Outputs {
  _call: RequestPriceCall;

  constructor(call: RequestPriceCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class SetBondCall extends ethereum.Call {
  get inputs(): SetBondCall__Inputs {
    return new SetBondCall__Inputs(this);
  }

  get outputs(): SetBondCall__Outputs {
    return new SetBondCall__Outputs(this);
  }
}

export class SetBondCall__Inputs {
  _call: SetBondCall;

  constructor(call: SetBondCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get bond(): BigInt {
    return this._call.inputValues[3].value.toBigInt();
  }
}

export class SetBondCall__Outputs {
  _call: SetBondCall;

  constructor(call: SetBondCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class SetCurrentTimeCall extends ethereum.Call {
  get inputs(): SetCurrentTimeCall__Inputs {
    return new SetCurrentTimeCall__Inputs(this);
  }

  get outputs(): SetCurrentTimeCall__Outputs {
    return new SetCurrentTimeCall__Outputs(this);
  }
}

export class SetCurrentTimeCall__Inputs {
  _call: SetCurrentTimeCall;

  constructor(call: SetCurrentTimeCall) {
    this._call = call;
  }

  get time(): BigInt {
    return this._call.inputValues[0].value.toBigInt();
  }
}

export class SetCurrentTimeCall__Outputs {
  _call: SetCurrentTimeCall;

  constructor(call: SetCurrentTimeCall) {
    this._call = call;
  }
}

export class SetCustomLivenessCall extends ethereum.Call {
  get inputs(): SetCustomLivenessCall__Inputs {
    return new SetCustomLivenessCall__Inputs(this);
  }

  get outputs(): SetCustomLivenessCall__Outputs {
    return new SetCustomLivenessCall__Outputs(this);
  }
}

export class SetCustomLivenessCall__Inputs {
  _call: SetCustomLivenessCall;

  constructor(call: SetCustomLivenessCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get customLiveness(): BigInt {
    return this._call.inputValues[3].value.toBigInt();
  }
}

export class SetCustomLivenessCall__Outputs {
  _call: SetCustomLivenessCall;

  constructor(call: SetCustomLivenessCall) {
    this._call = call;
  }
}

export class SetRefundOnDisputeCall extends ethereum.Call {
  get inputs(): SetRefundOnDisputeCall__Inputs {
    return new SetRefundOnDisputeCall__Inputs(this);
  }

  get outputs(): SetRefundOnDisputeCall__Outputs {
    return new SetRefundOnDisputeCall__Outputs(this);
  }
}

export class SetRefundOnDisputeCall__Inputs {
  _call: SetRefundOnDisputeCall;

  constructor(call: SetRefundOnDisputeCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }
}

export class SetRefundOnDisputeCall__Outputs {
  _call: SetRefundOnDisputeCall;

  constructor(call: SetRefundOnDisputeCall) {
    this._call = call;
  }
}

export class SettleCall extends ethereum.Call {
  get inputs(): SettleCall__Inputs {
    return new SettleCall__Inputs(this);
  }

  get outputs(): SettleCall__Outputs {
    return new SettleCall__Outputs(this);
  }
}

export class SettleCall__Inputs {
  _call: SettleCall;

  constructor(call: SettleCall) {
    this._call = call;
  }

  get requester(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[3].value.toBytes();
  }
}

export class SettleCall__Outputs {
  _call: SettleCall;

  constructor(call: SettleCall) {
    this._call = call;
  }

  get payout(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class SettleAndGetPriceCall extends ethereum.Call {
  get inputs(): SettleAndGetPriceCall__Inputs {
    return new SettleAndGetPriceCall__Inputs(this);
  }

  get outputs(): SettleAndGetPriceCall__Outputs {
    return new SettleAndGetPriceCall__Outputs(this);
  }
}

export class SettleAndGetPriceCall__Inputs {
  _call: SettleAndGetPriceCall;

  constructor(call: SettleAndGetPriceCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }
}

export class SettleAndGetPriceCall__Outputs {
  _call: SettleAndGetPriceCall;

  constructor(call: SettleAndGetPriceCall) {
    this._call = call;
  }

  get value0(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}



================================================
FILE: generated/OptimisticOracleV2/OptimisticOracleV2.ts
================================================
// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  ethereum,
  JSONValue,
  TypedMap,
  Entity,
  Bytes,
  Address,
  BigInt
} from "@graphprotocol/graph-ts";

export class DisputePrice extends ethereum.Event {
  get params(): DisputePrice__Params {
    return new DisputePrice__Params(this);
  }
}

export class DisputePrice__Params {
  _event: DisputePrice;

  constructor(event: DisputePrice) {
    this._event = event;
  }

  get requester(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get proposer(): Address {
    return this._event.parameters[1].value.toAddress();
  }

  get disputer(): Address {
    return this._event.parameters[2].value.toAddress();
  }

  get identifier(): Bytes {
    return this._event.parameters[3].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._event.parameters[4].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[5].value.toBytes();
  }

  get proposedPrice(): BigInt {
    return this._event.parameters[6].value.toBigInt();
  }
}

export class ProposePrice extends ethereum.Event {
  get params(): ProposePrice__Params {
    return new ProposePrice__Params(this);
  }
}

export class ProposePrice__Params {
  _event: ProposePrice;

  constructor(event: ProposePrice) {
    this._event = event;
  }

  get requester(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get proposer(): Address {
    return this._event.parameters[1].value.toAddress();
  }

  get identifier(): Bytes {
    return this._event.parameters[2].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._event.parameters[3].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[4].value.toBytes();
  }

  get proposedPrice(): BigInt {
    return this._event.parameters[5].value.toBigInt();
  }

  get expirationTimestamp(): BigInt {
    return this._event.parameters[6].value.toBigInt();
  }

  get currency(): Address {
    return this._event.parameters[7].value.toAddress();
  }
}

export class RequestPrice extends ethereum.Event {
  get params(): RequestPrice__Params {
    return new RequestPrice__Params(this);
  }
}

export class RequestPrice__Params {
  _event: RequestPrice;

  constructor(event: RequestPrice) {
    this._event = event;
  }

  get requester(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get identifier(): Bytes {
    return this._event.parameters[1].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._event.parameters[2].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[3].value.toBytes();
  }

  get currency(): Address {
    return this._event.parameters[4].value.toAddress();
  }

  get reward(): BigInt {
    return this._event.parameters[5].value.toBigInt();
  }

  get finalFee(): BigInt {
    return this._event.parameters[6].value.toBigInt();
  }
}

export class Settle extends ethereum.Event {
  get params(): Settle__Params {
    return new Settle__Params(this);
  }
}

export class Settle__Params {
  _event: Settle;

  constructor(event: Settle) {
    this._event = event;
  }

  get requester(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get proposer(): Address {
    return this._event.parameters[1].value.toAddress();
  }

  get disputer(): Address {
    return this._event.parameters[2].value.toAddress();
  }

  get identifier(): Bytes {
    return this._event.parameters[3].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._event.parameters[4].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[5].value.toBytes();
  }

  get price(): BigInt {
    return this._event.parameters[6].value.toBigInt();
  }

  get payout(): BigInt {
    return this._event.parameters[7].value.toBigInt();
  }
}

export class OptimisticOracleV2__getRequestResultValue0Struct extends ethereum.Tuple {
  get proposer(): Address {
    return this[0].toAddress();
  }

  get disputer(): Address {
    return this[1].toAddress();
  }

  get currency(): Address {
    return this[2].toAddress();
  }

  get settled(): boolean {
    return this[3].toBoolean();
  }

  get requestSettings(): OptimisticOracleV2__getRequestResultValue0RequestSettingsStruct {
    return changetype<
      OptimisticOracleV2__getRequestResultValue0RequestSettingsStruct
    >(this[4].toTuple());
  }

  get proposedPrice(): BigInt {
    return this[5].toBigInt();
  }

  get resolvedPrice(): BigInt {
    return this[6].toBigInt();
  }

  get expirationTime(): BigInt {
    return this[7].toBigInt();
  }

  get reward(): BigInt {
    return this[8].toBigInt();
  }

  get finalFee(): BigInt {
    return this[9].toBigInt();
  }
}

export class OptimisticOracleV2__getRequestResultValue0RequestSettingsStruct extends ethereum.Tuple {
  get eventBased(): boolean {
    return this[0].toBoolean();
  }

  get refundOnDispute(): boolean {
    return this[1].toBoolean();
  }

  get callbackOnPriceProposed(): boolean {
    return this[2].toBoolean();
  }

  get callbackOnPriceDisputed(): boolean {
    return this[3].toBoolean();
  }

  get callbackOnPriceSettled(): boolean {
    return this[4].toBoolean();
  }

  get bond(): BigInt {
    return this[5].toBigInt();
  }

  get customLiveness(): BigInt {
    return this[6].toBigInt();
  }
}

export class OptimisticOracleV2__requestsResultRequestSettingsStruct extends ethereum.Tuple {
  get eventBased(): boolean {
    return this[0].toBoolean();
  }

  get refundOnDispute(): boolean {
    return this[1].toBoolean();
  }

  get callbackOnPriceProposed(): boolean {
    return this[2].toBoolean();
  }

  get callbackOnPriceDisputed(): boolean {
    return this[3].toBoolean();
  }

  get callbackOnPriceSettled(): boolean {
    return this[4].toBoolean();
  }

  get bond(): BigInt {
    return this[5].toBigInt();
  }

  get customLiveness(): BigInt {
    return this[6].toBigInt();
  }
}

export class OptimisticOracleV2__requestsResult {
  value0: Address;
  value1: Address;
  value2: Address;
  value3: boolean;
  value4: OptimisticOracleV2__requestsResultRequestSettingsStruct;
  value5: BigInt;
  value6: BigInt;
  value7: BigInt;
  value8: BigInt;
  value9: BigInt;

  constructor(
    value0: Address,
    value1: Address,
    value2: Address,
    value3: boolean,
    value4: OptimisticOracleV2__requestsResultRequestSettingsStruct,
    value5: BigInt,
    value6: BigInt,
    value7: BigInt,
    value8: BigInt,
    value9: BigInt
  ) {
    this.value0 = value0;
    this.value1 = value1;
    this.value2 = value2;
    this.value3 = value3;
    this.value4 = value4;
    this.value5 = value5;
    this.value6 = value6;
    this.value7 = value7;
    this.value8 = value8;
    this.value9 = value9;
  }

  toMap(): TypedMap<string, ethereum.Value> {
    let map = new TypedMap<string, ethereum.Value>();
    map.set("value0", ethereum.Value.fromAddress(this.value0));
    map.set("value1", ethereum.Value.fromAddress(this.value1));
    map.set("value2", ethereum.Value.fromAddress(this.value2));
    map.set("value3", ethereum.Value.fromBoolean(this.value3));
    map.set("value4", ethereum.Value.fromTuple(this.value4));
    map.set("value5", ethereum.Value.fromSignedBigInt(this.value5));
    map.set("value6", ethereum.Value.fromSignedBigInt(this.value6));
    map.set("value7", ethereum.Value.fromUnsignedBigInt(this.value7));
    map.set("value8", ethereum.Value.fromUnsignedBigInt(this.value8));
    map.set("value9", ethereum.Value.fromUnsignedBigInt(this.value9));
    return map;
  }

  getProposer(): Address {
    return this.value0;
  }

  getDisputer(): Address {
    return this.value1;
  }

  getCurrency(): Address {
    return this.value2;
  }

  getSettled(): boolean {
    return this.value3;
  }

  getRequestSettings(): OptimisticOracleV2__requestsResultRequestSettingsStruct {
    return this.value4;
  }

  getProposedPrice(): BigInt {
    return this.value5;
  }

  getResolvedPrice(): BigInt {
    return this.value6;
  }

  getExpirationTime(): BigInt {
    return this.value7;
  }

  getReward(): BigInt {
    return this.value8;
  }

  getFinalFee(): BigInt {
    return this.value9;
  }
}

export class OptimisticOracleV2 extends ethereum.SmartContract {
  static bind(address: Address): OptimisticOracleV2 {
    return new OptimisticOracleV2("OptimisticOracleV2", address);
  }

  OO_ANCILLARY_DATA_LIMIT(): BigInt {
    let result = super.call(
      "OO_ANCILLARY_DATA_LIMIT",
      "OO_ANCILLARY_DATA_LIMIT():(uint256)",
      []
    );

    return result[0].toBigInt();
  }

  try_OO_ANCILLARY_DATA_LIMIT(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "OO_ANCILLARY_DATA_LIMIT",
      "OO_ANCILLARY_DATA_LIMIT():(uint256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  TOO_EARLY_RESPONSE(): BigInt {
    let result = super.call(
      "TOO_EARLY_RESPONSE",
      "TOO_EARLY_RESPONSE():(int256)",
      []
    );

    return result[0].toBigInt();
  }

  try_TOO_EARLY_RESPONSE(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "TOO_EARLY_RESPONSE",
      "TOO_EARLY_RESPONSE():(int256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  ancillaryBytesLimit(): BigInt {
    let result = super.call(
      "ancillaryBytesLimit",
      "ancillaryBytesLimit():(uint256)",
      []
    );

    return result[0].toBigInt();
  }

  try_ancillaryBytesLimit(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "ancillaryBytesLimit",
      "ancillaryBytesLimit():(uint256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  defaultLiveness(): BigInt {
    let result = super.call(
      "defaultLiveness",
      "defaultLiveness():(uint256)",
      []
    );

    return result[0].toBigInt();
  }

  try_defaultLiveness(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "defaultLiveness",
      "defaultLiveness():(uint256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  disputePrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): BigInt {
    let result = super.call(
      "disputePrice",
      "disputePrice(address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBigInt();
  }

  try_disputePrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "disputePrice",
      "disputePrice(address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  disputePriceFor(
    disputer: Address,
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): BigInt {
    let result = super.call(
      "disputePriceFor",
      "disputePriceFor(address,address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(disputer),
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBigInt();
  }

  try_disputePriceFor(
    disputer: Address,
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "disputePriceFor",
      "disputePriceFor(address,address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(disputer),
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  finder(): Address {
    let result = super.call("finder", "finder():(address)", []);

    return result[0].toAddress();
  }

  try_finder(): ethereum.CallResult<Address> {
    let result = super.tryCall("finder", "finder():(address)", []);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  getCurrentTime(): BigInt {
    let result = super.call("getCurrentTime", "getCurrentTime():(uint256)", []);

    return result[0].toBigInt();
  }

  try_getCurrentTime(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "getCurrentTime",
      "getCurrentTime():(uint256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  getRequest(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): OptimisticOracleV2__getRequestResultValue0Struct {
    let result = super.call(
      "getRequest",
      "getRequest(address,bytes32,uint256,bytes):((address,address,address,bool,(bool,bool,bool,bool,bool,uint256,uint256),int256,int256,uint256,uint256,uint256))",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return changetype<OptimisticOracleV2__getRequestResultValue0Struct>(
      result[0].toTuple()
    );
  }

  try_getRequest(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<OptimisticOracleV2__getRequestResultValue0Struct> {
    let result = super.tryCall(
      "getRequest",
      "getRequest(address,bytes32,uint256,bytes):((address,address,address,bool,(bool,bool,bool,bool,bool,uint256,uint256),int256,int256,uint256,uint256,uint256))",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(
      changetype<OptimisticOracleV2__getRequestResultValue0Struct>(
        value[0].toTuple()
      )
    );
  }

  getState(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): i32 {
    let result = super.call(
      "getState",
      "getState(address,bytes32,uint256,bytes):(uint8)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toI32();
  }

  try_getState(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<i32> {
    let result = super.tryCall(
      "getState",
      "getState(address,bytes32,uint256,bytes):(uint8)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toI32());
  }

  hasPrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): boolean {
    let result = super.call(
      "hasPrice",
      "hasPrice(address,bytes32,uint256,bytes):(bool)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBoolean();
  }

  try_hasPrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<boolean> {
    let result = super.tryCall(
      "hasPrice",
      "hasPrice(address,bytes32,uint256,bytes):(bool)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBoolean());
  }

  proposePrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    proposedPrice: BigInt
  ): BigInt {
    let result = super.call(
      "proposePrice",
      "proposePrice(address,bytes32,uint256,bytes,int256):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromSignedBigInt(proposedPrice)
      ]
    );

    return result[0].toBigInt();
  }

  try_proposePrice(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    proposedPrice: BigInt
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "proposePrice",
      "proposePrice(address,bytes32,uint256,bytes,int256):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromSignedBigInt(proposedPrice)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  proposePriceFor(
    proposer: Address,
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    proposedPrice: BigInt
  ): BigInt {
    let result = super.call(
      "proposePriceFor",
      "proposePriceFor(address,address,bytes32,uint256,bytes,int256):(uint256)",
      [
        ethereum.Value.fromAddress(proposer),
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromSignedBigInt(proposedPrice)
      ]
    );

    return result[0].toBigInt();
  }

  try_proposePriceFor(
    proposer: Address,
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    proposedPrice: BigInt
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "proposePriceFor",
      "proposePriceFor(address,address,bytes32,uint256,bytes,int256):(uint256)",
      [
        ethereum.Value.fromAddress(proposer),
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromSignedBigInt(proposedPrice)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  requestPrice(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    currency: Address,
    reward: BigInt
  ): BigInt {
    let result = super.call(
      "requestPrice",
      "requestPrice(bytes32,uint256,bytes,address,uint256):(uint256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromAddress(currency),
        ethereum.Value.fromUnsignedBigInt(reward)
      ]
    );

    return result[0].toBigInt();
  }

  try_requestPrice(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    currency: Address,
    reward: BigInt
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "requestPrice",
      "requestPrice(bytes32,uint256,bytes,address,uint256):(uint256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromAddress(currency),
        ethereum.Value.fromUnsignedBigInt(reward)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  requests(param0: Bytes): OptimisticOracleV2__requestsResult {
    let result = super.call(
      "requests",
      "requests(bytes32):(address,address,address,bool,(bool,bool,bool,bool,bool,uint256,uint256),int256,int256,uint256,uint256,uint256)",
      [ethereum.Value.fromFixedBytes(param0)]
    );

    return new OptimisticOracleV2__requestsResult(
      result[0].toAddress(),
      result[1].toAddress(),
      result[2].toAddress(),
      result[3].toBoolean(),
      changetype<OptimisticOracleV2__requestsResultRequestSettingsStruct>(
        result[4].toTuple()
      ),
      result[5].toBigInt(),
      result[6].toBigInt(),
      result[7].toBigInt(),
      result[8].toBigInt(),
      result[9].toBigInt()
    );
  }

  try_requests(
    param0: Bytes
  ): ethereum.CallResult<OptimisticOracleV2__requestsResult> {
    let result = super.tryCall(
      "requests",
      "requests(bytes32):(address,address,address,bool,(bool,bool,bool,bool,bool,uint256,uint256),int256,int256,uint256,uint256,uint256)",
      [ethereum.Value.fromFixedBytes(param0)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(
      new OptimisticOracleV2__requestsResult(
        value[0].toAddress(),
        value[1].toAddress(),
        value[2].toAddress(),
        value[3].toBoolean(),
        changetype<OptimisticOracleV2__requestsResultRequestSettingsStruct>(
          value[4].toTuple()
        ),
        value[5].toBigInt(),
        value[6].toBigInt(),
        value[7].toBigInt(),
        value[8].toBigInt(),
        value[9].toBigInt()
      )
    );
  }

  setBond(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    bond: BigInt
  ): BigInt {
    let result = super.call(
      "setBond",
      "setBond(bytes32,uint256,bytes,uint256):(uint256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromUnsignedBigInt(bond)
      ]
    );

    return result[0].toBigInt();
  }

  try_setBond(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes,
    bond: BigInt
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "setBond",
      "setBond(bytes32,uint256,bytes,uint256):(uint256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromUnsignedBigInt(bond)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  settle(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): BigInt {
    let result = super.call(
      "settle",
      "settle(address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBigInt();
  }

  try_settle(
    requester: Address,
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "settle",
      "settle(address,bytes32,uint256,bytes):(uint256)",
      [
        ethereum.Value.fromAddress(requester),
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  settleAndGetPrice(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): BigInt {
    let result = super.call(
      "settleAndGetPrice",
      "settleAndGetPrice(bytes32,uint256,bytes):(int256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );

    return result[0].toBigInt();
  }

  try_settleAndGetPrice(
    identifier: Bytes,
    timestamp: BigInt,
    ancillaryData: Bytes
  ): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "settleAndGetPrice",
      "settleAndGetPrice(bytes32,uint256,bytes):(int256)",
      [
        ethereum.Value.fromFixedBytes(identifier),
        ethereum.Value.fromUnsignedBigInt(timestamp),
        ethereum.Value.fromBytes(ancillaryData)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  stampAncillaryData(ancillaryData: Bytes, requester: Address): Bytes {
    let result = super.call(
      "stampAncillaryData",
      "stampAncillaryData(bytes,address):(bytes)",
      [
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromAddress(requester)
      ]
    );

    return result[0].toBytes();
  }

  try_stampAncillaryData(
    ancillaryData: Bytes,
    requester: Address
  ): ethereum.CallResult<Bytes> {
    let result = super.tryCall(
      "stampAncillaryData",
      "stampAncillaryData(bytes,address):(bytes)",
      [
        ethereum.Value.fromBytes(ancillaryData),
        ethereum.Value.fromAddress(requester)
      ]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBytes());
  }

  timerAddress(): Address {
    let result = super.call("timerAddress", "timerAddress():(address)", []);

    return result[0].toAddress();
  }

  try_timerAddress(): ethereum.CallResult<Address> {
    let result = super.tryCall("timerAddress", "timerAddress():(address)", []);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }
}

export class ConstructorCall extends ethereum.Call {
  get inputs(): ConstructorCall__Inputs {
    return new ConstructorCall__Inputs(this);
  }

  get outputs(): ConstructorCall__Outputs {
    return new ConstructorCall__Outputs(this);
  }
}

export class ConstructorCall__Inputs {
  _call: ConstructorCall;

  constructor(call: ConstructorCall) {
    this._call = call;
  }

  get _liveness(): BigInt {
    return this._call.inputValues[0].value.toBigInt();
  }

  get _finderAddress(): Address {
    return this._call.inputValues[1].value.toAddress();
  }

  get _timerAddress(): Address {
    return this._call.inputValues[2].value.toAddress();
  }
}

export class ConstructorCall__Outputs {
  _call: ConstructorCall;

  constructor(call: ConstructorCall) {
    this._call = call;
  }
}

export class DisputePriceCall extends ethereum.Call {
  get inputs(): DisputePriceCall__Inputs {
    return new DisputePriceCall__Inputs(this);
  }

  get outputs(): DisputePriceCall__Outputs {
    return new DisputePriceCall__Outputs(this);
  }
}

export class DisputePriceCall__Inputs {
  _call: DisputePriceCall;

  constructor(call: DisputePriceCall) {
    this._call = call;
  }

  get requester(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[3].value.toBytes();
  }
}

export class DisputePriceCall__Outputs {
  _call: DisputePriceCall;

  constructor(call: DisputePriceCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class DisputePriceForCall extends ethereum.Call {
  get inputs(): DisputePriceForCall__Inputs {
    return new DisputePriceForCall__Inputs(this);
  }

  get outputs(): DisputePriceForCall__Outputs {
    return new DisputePriceForCall__Outputs(this);
  }
}

export class DisputePriceForCall__Inputs {
  _call: DisputePriceForCall;

  constructor(call: DisputePriceForCall) {
    this._call = call;
  }

  get disputer(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get requester(): Address {
    return this._call.inputValues[1].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[3].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[4].value.toBytes();
  }
}

export class DisputePriceForCall__Outputs {
  _call: DisputePriceForCall;

  constructor(call: DisputePriceForCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class ProposePriceCall extends ethereum.Call {
  get inputs(): ProposePriceCall__Inputs {
    return new ProposePriceCall__Inputs(this);
  }

  get outputs(): ProposePriceCall__Outputs {
    return new ProposePriceCall__Outputs(this);
  }
}

export class ProposePriceCall__Inputs {
  _call: ProposePriceCall;

  constructor(call: ProposePriceCall) {
    this._call = call;
  }

  get requester(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[3].value.toBytes();
  }

  get proposedPrice(): BigInt {
    return this._call.inputValues[4].value.toBigInt();
  }
}

export class ProposePriceCall__Outputs {
  _call: ProposePriceCall;

  constructor(call: ProposePriceCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class ProposePriceForCall extends ethereum.Call {
  get inputs(): ProposePriceForCall__Inputs {
    return new ProposePriceForCall__Inputs(this);
  }

  get outputs(): ProposePriceForCall__Outputs {
    return new ProposePriceForCall__Outputs(this);
  }
}

export class ProposePriceForCall__Inputs {
  _call: ProposePriceForCall;

  constructor(call: ProposePriceForCall) {
    this._call = call;
  }

  get proposer(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get requester(): Address {
    return this._call.inputValues[1].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[3].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[4].value.toBytes();
  }

  get proposedPrice(): BigInt {
    return this._call.inputValues[5].value.toBigInt();
  }
}

export class ProposePriceForCall__Outputs {
  _call: ProposePriceForCall;

  constructor(call: ProposePriceForCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class RequestPriceCall extends ethereum.Call {
  get inputs(): RequestPriceCall__Inputs {
    return new RequestPriceCall__Inputs(this);
  }

  get outputs(): RequestPriceCall__Outputs {
    return new RequestPriceCall__Outputs(this);
  }
}

export class RequestPriceCall__Inputs {
  _call: RequestPriceCall;

  constructor(call: RequestPriceCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get currency(): Address {
    return this._call.inputValues[3].value.toAddress();
  }

  get reward(): BigInt {
    return this._call.inputValues[4].value.toBigInt();
  }
}

export class RequestPriceCall__Outputs {
  _call: RequestPriceCall;

  constructor(call: RequestPriceCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class SetBondCall extends ethereum.Call {
  get inputs(): SetBondCall__Inputs {
    return new SetBondCall__Inputs(this);
  }

  get outputs(): SetBondCall__Outputs {
    return new SetBondCall__Outputs(this);
  }
}

export class SetBondCall__Inputs {
  _call: SetBondCall;

  constructor(call: SetBondCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get bond(): BigInt {
    return this._call.inputValues[3].value.toBigInt();
  }
}

export class SetBondCall__Outputs {
  _call: SetBondCall;

  constructor(call: SetBondCall) {
    this._call = call;
  }

  get totalBond(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class SetCallbacksCall extends ethereum.Call {
  get inputs(): SetCallbacksCall__Inputs {
    return new SetCallbacksCall__Inputs(this);
  }

  get outputs(): SetCallbacksCall__Outputs {
    return new SetCallbacksCall__Outputs(this);
  }
}

export class SetCallbacksCall__Inputs {
  _call: SetCallbacksCall;

  constructor(call: SetCallbacksCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get callbackOnPriceProposed(): boolean {
    return this._call.inputValues[3].value.toBoolean();
  }

  get callbackOnPriceDisputed(): boolean {
    return this._call.inputValues[4].value.toBoolean();
  }

  get callbackOnPriceSettled(): boolean {
    return this._call.inputValues[5].value.toBoolean();
  }
}

export class SetCallbacksCall__Outputs {
  _call: SetCallbacksCall;

  constructor(call: SetCallbacksCall) {
    this._call = call;
  }
}

export class SetCurrentTimeCall extends ethereum.Call {
  get inputs(): SetCurrentTimeCall__Inputs {
    return new SetCurrentTimeCall__Inputs(this);
  }

  get outputs(): SetCurrentTimeCall__Outputs {
    return new SetCurrentTimeCall__Outputs(this);
  }
}

export class SetCurrentTimeCall__Inputs {
  _call: SetCurrentTimeCall;

  constructor(call: SetCurrentTimeCall) {
    this._call = call;
  }

  get time(): BigInt {
    return this._call.inputValues[0].value.toBigInt();
  }
}

export class SetCurrentTimeCall__Outputs {
  _call: SetCurrentTimeCall;

  constructor(call: SetCurrentTimeCall) {
    this._call = call;
  }
}

export class SetCustomLivenessCall extends ethereum.Call {
  get inputs(): SetCustomLivenessCall__Inputs {
    return new SetCustomLivenessCall__Inputs(this);
  }

  get outputs(): SetCustomLivenessCall__Outputs {
    return new SetCustomLivenessCall__Outputs(this);
  }
}

export class SetCustomLivenessCall__Inputs {
  _call: SetCustomLivenessCall;

  constructor(call: SetCustomLivenessCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }

  get customLiveness(): BigInt {
    return this._call.inputValues[3].value.toBigInt();
  }
}

export class SetCustomLivenessCall__Outputs {
  _call: SetCustomLivenessCall;

  constructor(call: SetCustomLivenessCall) {
    this._call = call;
  }
}

export class SetEventBasedCall extends ethereum.Call {
  get inputs(): SetEventBasedCall__Inputs {
    return new SetEventBasedCall__Inputs(this);
  }

  get outputs(): SetEventBasedCall__Outputs {
    return new SetEventBasedCall__Outputs(this);
  }
}

export class SetEventBasedCall__Inputs {
  _call: SetEventBasedCall;

  constructor(call: SetEventBasedCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }
}

export class SetEventBasedCall__Outputs {
  _call: SetEventBasedCall;

  constructor(call: SetEventBasedCall) {
    this._call = call;
  }
}

export class SetRefundOnDisputeCall extends ethereum.Call {
  get inputs(): SetRefundOnDisputeCall__Inputs {
    return new SetRefundOnDisputeCall__Inputs(this);
  }

  get outputs(): SetRefundOnDisputeCall__Outputs {
    return new SetRefundOnDisputeCall__Outputs(this);
  }
}

export class SetRefundOnDisputeCall__Inputs {
  _call: SetRefundOnDisputeCall;

  constructor(call: SetRefundOnDisputeCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }
}

export class SetRefundOnDisputeCall__Outputs {
  _call: SetRefundOnDisputeCall;

  constructor(call: SetRefundOnDisputeCall) {
    this._call = call;
  }
}

export class SettleCall extends ethereum.Call {
  get inputs(): SettleCall__Inputs {
    return new SettleCall__Inputs(this);
  }

  get outputs(): SettleCall__Outputs {
    return new SettleCall__Outputs(this);
  }
}

export class SettleCall__Inputs {
  _call: SettleCall;

  constructor(call: SettleCall) {
    this._call = call;
  }

  get requester(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get identifier(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[3].value.toBytes();
  }
}

export class SettleCall__Outputs {
  _call: SettleCall;

  constructor(call: SettleCall) {
    this._call = call;
  }

  get payout(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}

export class SettleAndGetPriceCall extends ethereum.Call {
  get inputs(): SettleAndGetPriceCall__Inputs {
    return new SettleAndGetPriceCall__Inputs(this);
  }

  get outputs(): SettleAndGetPriceCall__Outputs {
    return new SettleAndGetPriceCall__Outputs(this);
  }
}

export class SettleAndGetPriceCall__Inputs {
  _call: SettleAndGetPriceCall;

  constructor(call: SettleAndGetPriceCall) {
    this._call = call;
  }

  get identifier(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get timestamp(): BigInt {
    return this._call.inputValues[1].value.toBigInt();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[2].value.toBytes();
  }
}

export class SettleAndGetPriceCall__Outputs {
  _call: SettleAndGetPriceCall;

  constructor(call: SettleAndGetPriceCall) {
    this._call = call;
  }

  get value0(): BigInt {
    return this._call.outputValues[0].value.toBigInt();
  }
}



================================================
FILE: generated/UmaCtfAdapterOld/UmaCtfAdapterOld.ts
================================================
// THIS IS AN AUTOGENERATED FILE. DO NOT EDIT THIS FILE DIRECTLY.

import {
  ethereum,
  JSONValue,
  TypedMap,
  Entity,
  Bytes,
  Address,
  BigInt
} from "@graphprotocol/graph-ts";

export class AuthorizedUser extends ethereum.Event {
  get params(): AuthorizedUser__Params {
    return new AuthorizedUser__Params(this);
  }
}

export class AuthorizedUser__Params {
  _event: AuthorizedUser;

  constructor(event: AuthorizedUser) {
    this._event = event;
  }

  get usr(): Address {
    return this._event.parameters[0].value.toAddress();
  }
}

export class DeauthorizedUser extends ethereum.Event {
  get params(): DeauthorizedUser__Params {
    return new DeauthorizedUser__Params(this);
  }
}

export class DeauthorizedUser__Params {
  _event: DeauthorizedUser;

  constructor(event: DeauthorizedUser) {
    this._event = event;
  }

  get usr(): Address {
    return this._event.parameters[0].value.toAddress();
  }
}

export class NewFinderAddress extends ethereum.Event {
  get params(): NewFinderAddress__Params {
    return new NewFinderAddress__Params(this);
  }
}

export class NewFinderAddress__Params {
  _event: NewFinderAddress;

  constructor(event: NewFinderAddress) {
    this._event = event;
  }

  get oldFinder(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get newFinder(): Address {
    return this._event.parameters[1].value.toAddress();
  }
}

export class QuestionFlaggedForAdminResolution extends ethereum.Event {
  get params(): QuestionFlaggedForAdminResolution__Params {
    return new QuestionFlaggedForAdminResolution__Params(this);
  }
}

export class QuestionFlaggedForAdminResolution__Params {
  _event: QuestionFlaggedForAdminResolution;

  constructor(event: QuestionFlaggedForAdminResolution) {
    this._event = event;
  }

  get questionID(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }
}

export class QuestionInitialized extends ethereum.Event {
  get params(): QuestionInitialized__Params {
    return new QuestionInitialized__Params(this);
  }
}

export class QuestionInitialized__Params {
  _event: QuestionInitialized;

  constructor(event: QuestionInitialized) {
    this._event = event;
  }

  get questionID(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[1].value.toBytes();
  }

  get resolutionTime(): BigInt {
    return this._event.parameters[2].value.toBigInt();
  }

  get rewardToken(): Address {
    return this._event.parameters[3].value.toAddress();
  }

  get reward(): BigInt {
    return this._event.parameters[4].value.toBigInt();
  }

  get proposalBond(): BigInt {
    return this._event.parameters[5].value.toBigInt();
  }

  get earlyResolutionEnabled(): boolean {
    return this._event.parameters[6].value.toBoolean();
  }
}

export class QuestionPaused extends ethereum.Event {
  get params(): QuestionPaused__Params {
    return new QuestionPaused__Params(this);
  }
}

export class QuestionPaused__Params {
  _event: QuestionPaused;

  constructor(event: QuestionPaused) {
    this._event = event;
  }

  get questionID(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }
}

export class QuestionReset extends ethereum.Event {
  get params(): QuestionReset__Params {
    return new QuestionReset__Params(this);
  }
}

export class QuestionReset__Params {
  _event: QuestionReset;

  constructor(event: QuestionReset) {
    this._event = event;
  }

  get questionID(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }
}

export class QuestionResolved extends ethereum.Event {
  get params(): QuestionResolved__Params {
    return new QuestionResolved__Params(this);
  }
}

export class QuestionResolved__Params {
  _event: QuestionResolved;

  constructor(event: QuestionResolved) {
    this._event = event;
  }

  get questionID(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }

  get emergencyReport(): boolean {
    return this._event.parameters[1].value.toBoolean();
  }
}

export class QuestionSettled extends ethereum.Event {
  get params(): QuestionSettled__Params {
    return new QuestionSettled__Params(this);
  }
}

export class QuestionSettled__Params {
  _event: QuestionSettled;

  constructor(event: QuestionSettled) {
    this._event = event;
  }

  get questionID(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }

  get settledPrice(): BigInt {
    return this._event.parameters[1].value.toBigInt();
  }

  get earlyResolution(): boolean {
    return this._event.parameters[2].value.toBoolean();
  }
}

export class QuestionUnpaused extends ethereum.Event {
  get params(): QuestionUnpaused__Params {
    return new QuestionUnpaused__Params(this);
  }
}

export class QuestionUnpaused__Params {
  _event: QuestionUnpaused;

  constructor(event: QuestionUnpaused) {
    this._event = event;
  }

  get questionID(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }
}

export class QuestionUpdated extends ethereum.Event {
  get params(): QuestionUpdated__Params {
    return new QuestionUpdated__Params(this);
  }
}

export class QuestionUpdated__Params {
  _event: QuestionUpdated;

  constructor(event: QuestionUpdated) {
    this._event = event;
  }

  get questionID(): Bytes {
    return this._event.parameters[0].value.toBytes();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[1].value.toBytes();
  }

  get resolutionTime(): BigInt {
    return this._event.parameters[2].value.toBigInt();
  }

  get rewardToken(): Address {
    return this._event.parameters[3].value.toAddress();
  }

  get reward(): BigInt {
    return this._event.parameters[4].value.toBigInt();
  }

  get proposalBond(): BigInt {
    return this._event.parameters[5].value.toBigInt();
  }

  get earlyResolutionEnabled(): boolean {
    return this._event.parameters[6].value.toBoolean();
  }
}

export class ResolutionDataRequested extends ethereum.Event {
  get params(): ResolutionDataRequested__Params {
    return new ResolutionDataRequested__Params(this);
  }
}

export class ResolutionDataRequested__Params {
  _event: ResolutionDataRequested;

  constructor(event: ResolutionDataRequested) {
    this._event = event;
  }

  get requestor(): Address {
    return this._event.parameters[0].value.toAddress();
  }

  get requestTimestamp(): BigInt {
    return this._event.parameters[1].value.toBigInt();
  }

  get questionID(): Bytes {
    return this._event.parameters[2].value.toBytes();
  }

  get identifier(): Bytes {
    return this._event.parameters[3].value.toBytes();
  }

  get ancillaryData(): Bytes {
    return this._event.parameters[4].value.toBytes();
  }

  get rewardToken(): Address {
    return this._event.parameters[5].value.toAddress();
  }

  get reward(): BigInt {
    return this._event.parameters[6].value.toBigInt();
  }

  get proposalBond(): BigInt {
    return this._event.parameters[7].value.toBigInt();
  }

  get earlyResolution(): boolean {
    return this._event.parameters[8].value.toBoolean();
  }
}

export class UmaCtfAdapterOld__questionsResult {
  value0: BigInt;
  value1: BigInt;
  value2: BigInt;
  value3: BigInt;
  value4: BigInt;
  value5: BigInt;
  value6: boolean;
  value7: boolean;
  value8: boolean;
  value9: Address;
  value10: Bytes;

  constructor(
    value0: BigInt,
    value1: BigInt,
    value2: BigInt,
    value3: BigInt,
    value4: BigInt,
    value5: BigInt,
    value6: boolean,
    value7: boolean,
    value8: boolean,
    value9: Address,
    value10: Bytes
  ) {
    this.value0 = value0;
    this.value1 = value1;
    this.value2 = value2;
    this.value3 = value3;
    this.value4 = value4;
    this.value5 = value5;
    this.value6 = value6;
    this.value7 = value7;
    this.value8 = value8;
    this.value9 = value9;
    this.value10 = value10;
  }

  toMap(): TypedMap<string, ethereum.Value> {
    let map = new TypedMap<string, ethereum.Value>();
    map.set("value0", ethereum.Value.fromUnsignedBigInt(this.value0));
    map.set("value1", ethereum.Value.fromUnsignedBigInt(this.value1));
    map.set("value2", ethereum.Value.fromUnsignedBigInt(this.value2));
    map.set("value3", ethereum.Value.fromUnsignedBigInt(this.value3));
    map.set("value4", ethereum.Value.fromUnsignedBigInt(this.value4));
    map.set("value5", ethereum.Value.fromUnsignedBigInt(this.value5));
    map.set("value6", ethereum.Value.fromBoolean(this.value6));
    map.set("value7", ethereum.Value.fromBoolean(this.value7));
    map.set("value8", ethereum.Value.fromBoolean(this.value8));
    map.set("value9", ethereum.Value.fromAddress(this.value9));
    map.set("value10", ethereum.Value.fromBytes(this.value10));
    return map;
  }

  getResolutionTime(): BigInt {
    return this.value0;
  }

  getReward(): BigInt {
    return this.value1;
  }

  getProposalBond(): BigInt {
    return this.value2;
  }

  getSettled(): BigInt {
    return this.value3;
  }

  getRequestTimestamp(): BigInt {
    return this.value4;
  }

  getAdminResolutionTimestamp(): BigInt {
    return this.value5;
  }

  getEarlyResolutionEnabled(): boolean {
    return this.value6;
  }

  getResolved(): boolean {
    return this.value7;
  }

  getPaused(): boolean {
    return this.value8;
  }

  getRewardToken(): Address {
    return this.value9;
  }

  getAncillaryData(): Bytes {
    return this.value10;
  }
}

export class UmaCtfAdapterOld extends ethereum.SmartContract {
  static bind(address: Address): UmaCtfAdapterOld {
    return new UmaCtfAdapterOld("UmaCtfAdapterOld", address);
  }

  conditionalTokenContract(): Address {
    let result = super.call(
      "conditionalTokenContract",
      "conditionalTokenContract():(address)",
      []
    );

    return result[0].toAddress();
  }

  try_conditionalTokenContract(): ethereum.CallResult<Address> {
    let result = super.tryCall(
      "conditionalTokenContract",
      "conditionalTokenContract():(address)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  emergencySafetyPeriod(): BigInt {
    let result = super.call(
      "emergencySafetyPeriod",
      "emergencySafetyPeriod():(uint256)",
      []
    );

    return result[0].toBigInt();
  }

  try_emergencySafetyPeriod(): ethereum.CallResult<BigInt> {
    let result = super.tryCall(
      "emergencySafetyPeriod",
      "emergencySafetyPeriod():(uint256)",
      []
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  getExpectedPayouts(questionID: Bytes): Array<BigInt> {
    let result = super.call(
      "getExpectedPayouts",
      "getExpectedPayouts(bytes32):(uint256[])",
      [ethereum.Value.fromFixedBytes(questionID)]
    );

    return result[0].toBigIntArray();
  }

  try_getExpectedPayouts(
    questionID: Bytes
  ): ethereum.CallResult<Array<BigInt>> {
    let result = super.tryCall(
      "getExpectedPayouts",
      "getExpectedPayouts(bytes32):(uint256[])",
      [ethereum.Value.fromFixedBytes(questionID)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigIntArray());
  }

  identifier(): Bytes {
    let result = super.call("identifier", "identifier():(bytes32)", []);

    return result[0].toBytes();
  }

  try_identifier(): ethereum.CallResult<Bytes> {
    let result = super.tryCall("identifier", "identifier():(bytes32)", []);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBytes());
  }

  ignorePrice(): BigInt {
    let result = super.call("ignorePrice", "ignorePrice():(int256)", []);

    return result[0].toBigInt();
  }

  try_ignorePrice(): ethereum.CallResult<BigInt> {
    let result = super.tryCall("ignorePrice", "ignorePrice():(int256)", []);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }

  isQuestionFlaggedForEmergencyResolution(questionID: Bytes): boolean {
    let result = super.call(
      "isQuestionFlaggedForEmergencyResolution",
      "isQuestionFlaggedForEmergencyResolution(bytes32):(bool)",
      [ethereum.Value.fromFixedBytes(questionID)]
    );

    return result[0].toBoolean();
  }

  try_isQuestionFlaggedForEmergencyResolution(
    questionID: Bytes
  ): ethereum.CallResult<boolean> {
    let result = super.tryCall(
      "isQuestionFlaggedForEmergencyResolution",
      "isQuestionFlaggedForEmergencyResolution(bytes32):(bool)",
      [ethereum.Value.fromFixedBytes(questionID)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBoolean());
  }

  isQuestionInitialized(questionID: Bytes): boolean {
    let result = super.call(
      "isQuestionInitialized",
      "isQuestionInitialized(bytes32):(bool)",
      [ethereum.Value.fromFixedBytes(questionID)]
    );

    return result[0].toBoolean();
  }

  try_isQuestionInitialized(questionID: Bytes): ethereum.CallResult<boolean> {
    let result = super.tryCall(
      "isQuestionInitialized",
      "isQuestionInitialized(bytes32):(bool)",
      [ethereum.Value.fromFixedBytes(questionID)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBoolean());
  }

  questions(param0: Bytes): UmaCtfAdapterOld__questionsResult {
    let result = super.call(
      "questions",
      "questions(bytes32):(uint256,uint256,uint256,uint256,uint256,uint256,bool,bool,bool,address,bytes)",
      [ethereum.Value.fromFixedBytes(param0)]
    );

    return new UmaCtfAdapterOld__questionsResult(
      result[0].toBigInt(),
      result[1].toBigInt(),
      result[2].toBigInt(),
      result[3].toBigInt(),
      result[4].toBigInt(),
      result[5].toBigInt(),
      result[6].toBoolean(),
      result[7].toBoolean(),
      result[8].toBoolean(),
      result[9].toAddress(),
      result[10].toBytes()
    );
  }

  try_questions(
    param0: Bytes
  ): ethereum.CallResult<UmaCtfAdapterOld__questionsResult> {
    let result = super.tryCall(
      "questions",
      "questions(bytes32):(uint256,uint256,uint256,uint256,uint256,uint256,bool,bool,bool,address,bytes)",
      [ethereum.Value.fromFixedBytes(param0)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(
      new UmaCtfAdapterOld__questionsResult(
        value[0].toBigInt(),
        value[1].toBigInt(),
        value[2].toBigInt(),
        value[3].toBigInt(),
        value[4].toBigInt(),
        value[5].toBigInt(),
        value[6].toBoolean(),
        value[7].toBoolean(),
        value[8].toBoolean(),
        value[9].toAddress(),
        value[10].toBytes()
      )
    );
  }

  readyToRequestResolution(questionID: Bytes): boolean {
    let result = super.call(
      "readyToRequestResolution",
      "readyToRequestResolution(bytes32):(bool)",
      [ethereum.Value.fromFixedBytes(questionID)]
    );

    return result[0].toBoolean();
  }

  try_readyToRequestResolution(
    questionID: Bytes
  ): ethereum.CallResult<boolean> {
    let result = super.tryCall(
      "readyToRequestResolution",
      "readyToRequestResolution(bytes32):(bool)",
      [ethereum.Value.fromFixedBytes(questionID)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBoolean());
  }

  readyToSettle(questionID: Bytes): boolean {
    let result = super.call("readyToSettle", "readyToSettle(bytes32):(bool)", [
      ethereum.Value.fromFixedBytes(questionID)
    ]);

    return result[0].toBoolean();
  }

  try_readyToSettle(questionID: Bytes): ethereum.CallResult<boolean> {
    let result = super.tryCall(
      "readyToSettle",
      "readyToSettle(bytes32):(bool)",
      [ethereum.Value.fromFixedBytes(questionID)]
    );
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBoolean());
  }

  umaFinder(): Address {
    let result = super.call("umaFinder", "umaFinder():(address)", []);

    return result[0].toAddress();
  }

  try_umaFinder(): ethereum.CallResult<Address> {
    let result = super.tryCall("umaFinder", "umaFinder():(address)", []);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toAddress());
  }

  wards(param0: Address): BigInt {
    let result = super.call("wards", "wards(address):(uint256)", [
      ethereum.Value.fromAddress(param0)
    ]);

    return result[0].toBigInt();
  }

  try_wards(param0: Address): ethereum.CallResult<BigInt> {
    let result = super.tryCall("wards", "wards(address):(uint256)", [
      ethereum.Value.fromAddress(param0)
    ]);
    if (result.reverted) {
      return new ethereum.CallResult();
    }
    let value = result.value;
    return ethereum.CallResult.fromValue(value[0].toBigInt());
  }
}

export class ConstructorCall extends ethereum.Call {
  get inputs(): ConstructorCall__Inputs {
    return new ConstructorCall__Inputs(this);
  }

  get outputs(): ConstructorCall__Outputs {
    return new ConstructorCall__Outputs(this);
  }
}

export class ConstructorCall__Inputs {
  _call: ConstructorCall;

  constructor(call: ConstructorCall) {
    this._call = call;
  }

  get conditionalTokenAddress(): Address {
    return this._call.inputValues[0].value.toAddress();
  }

  get umaFinderAddress(): Address {
    return this._call.inputValues[1].value.toAddress();
  }
}

export class ConstructorCall__Outputs {
  _call: ConstructorCall;

  constructor(call: ConstructorCall) {
    this._call = call;
  }
}

export class DenyCall extends ethereum.Call {
  get inputs(): DenyCall__Inputs {
    return new DenyCall__Inputs(this);
  }

  get outputs(): DenyCall__Outputs {
    return new DenyCall__Outputs(this);
  }
}

export class DenyCall__Inputs {
  _call: DenyCall;

  constructor(call: DenyCall) {
    this._call = call;
  }

  get usr(): Address {
    return this._call.inputValues[0].value.toAddress();
  }
}

export class DenyCall__Outputs {
  _call: DenyCall;

  constructor(call: DenyCall) {
    this._call = call;
  }
}

export class EmergencyReportPayoutsCall extends ethereum.Call {
  get inputs(): EmergencyReportPayoutsCall__Inputs {
    return new EmergencyReportPayoutsCall__Inputs(this);
  }

  get outputs(): EmergencyReportPayoutsCall__Outputs {
    return new EmergencyReportPayoutsCall__Outputs(this);
  }
}

export class EmergencyReportPayoutsCall__Inputs {
  _call: EmergencyReportPayoutsCall;

  constructor(call: EmergencyReportPayoutsCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get payouts(): Array<BigInt> {
    return this._call.inputValues[1].value.toBigIntArray();
  }
}

export class EmergencyReportPayoutsCall__Outputs {
  _call: EmergencyReportPayoutsCall;

  constructor(call: EmergencyReportPayoutsCall) {
    this._call = call;
  }
}

export class FlagQuestionForEmergencyResolutionCall extends ethereum.Call {
  get inputs(): FlagQuestionForEmergencyResolutionCall__Inputs {
    return new FlagQuestionForEmergencyResolutionCall__Inputs(this);
  }

  get outputs(): FlagQuestionForEmergencyResolutionCall__Outputs {
    return new FlagQuestionForEmergencyResolutionCall__Outputs(this);
  }
}

export class FlagQuestionForEmergencyResolutionCall__Inputs {
  _call: FlagQuestionForEmergencyResolutionCall;

  constructor(call: FlagQuestionForEmergencyResolutionCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }
}

export class FlagQuestionForEmergencyResolutionCall__Outputs {
  _call: FlagQuestionForEmergencyResolutionCall;

  constructor(call: FlagQuestionForEmergencyResolutionCall) {
    this._call = call;
  }
}

export class InitializeQuestionCall extends ethereum.Call {
  get inputs(): InitializeQuestionCall__Inputs {
    return new InitializeQuestionCall__Inputs(this);
  }

  get outputs(): InitializeQuestionCall__Outputs {
    return new InitializeQuestionCall__Outputs(this);
  }
}

export class InitializeQuestionCall__Inputs {
  _call: InitializeQuestionCall;

  constructor(call: InitializeQuestionCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get resolutionTime(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get rewardToken(): Address {
    return this._call.inputValues[3].value.toAddress();
  }

  get reward(): BigInt {
    return this._call.inputValues[4].value.toBigInt();
  }

  get proposalBond(): BigInt {
    return this._call.inputValues[5].value.toBigInt();
  }

  get earlyResolutionEnabled(): boolean {
    return this._call.inputValues[6].value.toBoolean();
  }
}

export class InitializeQuestionCall__Outputs {
  _call: InitializeQuestionCall;

  constructor(call: InitializeQuestionCall) {
    this._call = call;
  }
}

export class PauseQuestionCall extends ethereum.Call {
  get inputs(): PauseQuestionCall__Inputs {
    return new PauseQuestionCall__Inputs(this);
  }

  get outputs(): PauseQuestionCall__Outputs {
    return new PauseQuestionCall__Outputs(this);
  }
}

export class PauseQuestionCall__Inputs {
  _call: PauseQuestionCall;

  constructor(call: PauseQuestionCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }
}

export class PauseQuestionCall__Outputs {
  _call: PauseQuestionCall;

  constructor(call: PauseQuestionCall) {
    this._call = call;
  }
}

export class PrepareAndInitializeCall extends ethereum.Call {
  get inputs(): PrepareAndInitializeCall__Inputs {
    return new PrepareAndInitializeCall__Inputs(this);
  }

  get outputs(): PrepareAndInitializeCall__Outputs {
    return new PrepareAndInitializeCall__Outputs(this);
  }
}

export class PrepareAndInitializeCall__Inputs {
  _call: PrepareAndInitializeCall;

  constructor(call: PrepareAndInitializeCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get resolutionTime(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get rewardToken(): Address {
    return this._call.inputValues[3].value.toAddress();
  }

  get reward(): BigInt {
    return this._call.inputValues[4].value.toBigInt();
  }

  get proposalBond(): BigInt {
    return this._call.inputValues[5].value.toBigInt();
  }

  get earlyResolutionEnabled(): boolean {
    return this._call.inputValues[6].value.toBoolean();
  }
}

export class PrepareAndInitializeCall__Outputs {
  _call: PrepareAndInitializeCall;

  constructor(call: PrepareAndInitializeCall) {
    this._call = call;
  }
}

export class RelyCall extends ethereum.Call {
  get inputs(): RelyCall__Inputs {
    return new RelyCall__Inputs(this);
  }

  get outputs(): RelyCall__Outputs {
    return new RelyCall__Outputs(this);
  }
}

export class RelyCall__Inputs {
  _call: RelyCall;

  constructor(call: RelyCall) {
    this._call = call;
  }

  get usr(): Address {
    return this._call.inputValues[0].value.toAddress();
  }
}

export class RelyCall__Outputs {
  _call: RelyCall;

  constructor(call: RelyCall) {
    this._call = call;
  }
}

export class ReportPayoutsCall extends ethereum.Call {
  get inputs(): ReportPayoutsCall__Inputs {
    return new ReportPayoutsCall__Inputs(this);
  }

  get outputs(): ReportPayoutsCall__Outputs {
    return new ReportPayoutsCall__Outputs(this);
  }
}

export class ReportPayoutsCall__Inputs {
  _call: ReportPayoutsCall;

  constructor(call: ReportPayoutsCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }
}

export class ReportPayoutsCall__Outputs {
  _call: ReportPayoutsCall;

  constructor(call: ReportPayoutsCall) {
    this._call = call;
  }
}

export class RequestResolutionDataCall extends ethereum.Call {
  get inputs(): RequestResolutionDataCall__Inputs {
    return new RequestResolutionDataCall__Inputs(this);
  }

  get outputs(): RequestResolutionDataCall__Outputs {
    return new RequestResolutionDataCall__Outputs(this);
  }
}

export class RequestResolutionDataCall__Inputs {
  _call: RequestResolutionDataCall;

  constructor(call: RequestResolutionDataCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }
}

export class RequestResolutionDataCall__Outputs {
  _call: RequestResolutionDataCall;

  constructor(call: RequestResolutionDataCall) {
    this._call = call;
  }
}

export class SetFinderAddressCall extends ethereum.Call {
  get inputs(): SetFinderAddressCall__Inputs {
    return new SetFinderAddressCall__Inputs(this);
  }

  get outputs(): SetFinderAddressCall__Outputs {
    return new SetFinderAddressCall__Outputs(this);
  }
}

export class SetFinderAddressCall__Inputs {
  _call: SetFinderAddressCall;

  constructor(call: SetFinderAddressCall) {
    this._call = call;
  }

  get newFinderAddress(): Address {
    return this._call.inputValues[0].value.toAddress();
  }
}

export class SetFinderAddressCall__Outputs {
  _call: SetFinderAddressCall;

  constructor(call: SetFinderAddressCall) {
    this._call = call;
  }
}

export class SettleCall extends ethereum.Call {
  get inputs(): SettleCall__Inputs {
    return new SettleCall__Inputs(this);
  }

  get outputs(): SettleCall__Outputs {
    return new SettleCall__Outputs(this);
  }
}

export class SettleCall__Inputs {
  _call: SettleCall;

  constructor(call: SettleCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }
}

export class SettleCall__Outputs {
  _call: SettleCall;

  constructor(call: SettleCall) {
    this._call = call;
  }
}

export class UnPauseQuestionCall extends ethereum.Call {
  get inputs(): UnPauseQuestionCall__Inputs {
    return new UnPauseQuestionCall__Inputs(this);
  }

  get outputs(): UnPauseQuestionCall__Outputs {
    return new UnPauseQuestionCall__Outputs(this);
  }
}

export class UnPauseQuestionCall__Inputs {
  _call: UnPauseQuestionCall;

  constructor(call: UnPauseQuestionCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }
}

export class UnPauseQuestionCall__Outputs {
  _call: UnPauseQuestionCall;

  constructor(call: UnPauseQuestionCall) {
    this._call = call;
  }
}

export class UpdateQuestionCall extends ethereum.Call {
  get inputs(): UpdateQuestionCall__Inputs {
    return new UpdateQuestionCall__Inputs(this);
  }

  get outputs(): UpdateQuestionCall__Outputs {
    return new UpdateQuestionCall__Outputs(this);
  }
}

export class UpdateQuestionCall__Inputs {
  _call: UpdateQuestionCall;

  constructor(call: UpdateQuestionCall) {
    this._call = call;
  }

  get questionID(): Bytes {
    return this._call.inputValues[0].value.toBytes();
  }

  get ancillaryData(): Bytes {
    return this._call.inputValues[1].value.toBytes();
  }

  get resolutionTime(): BigInt {
    return this._call.inputValues[2].value.toBigInt();
  }

  get rewardToken(): Address {
    return this._call.inputValues[3].value.toAddress();
  }

  get reward(): BigInt {
    return this._call.inputValues[4].value.toBigInt();
  }

  get proposalBond(): BigInt {
    return this._call.inputValues[5].value.toBigInt();
  }

  get earlyResolutionEnabled(): boolean {
    return this._call.inputValues[6].value.toBoolean();
  }
}

export class UpdateQuestionCall__Outputs {
  _call: UpdateQuestionCall;

  constructor(call: UpdateQuestionCall) {
    this._call = call;
  }
}



================================================
FILE: src/managed-oo-v2.ts
================================================
import {
  DisputePrice as DisputePriceEvent,
  ProposePrice as ProposePriceEvent,
  RequestPrice as RequestPriceEvent,
} from "../generated/ManagedOptimisticOracleV2/ManagedOptimisticOracleV2";
import { MarketResolution } from "../generated/schema";
import { UMA_CTF_ADAPTER_V4_ADDRESS } from "./utils/constants";
import { crypto, Address } from "@graphprotocol/graph-ts";

export function handleDisputePrice(event: DisputePriceEvent): void {
  let entity = MarketResolution.load(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  if (entity == null) {
    // might not be a polymarket request
    return;
  }
  // make sure from UMA CTF adapter
  if (
    event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_V4_ADDRESS)
  ) {
    return;
  }
  if (entity.status == "proposed") {
    // first challenge
    entity.status = "challenged";
    entity.lastUpdateTimestamp = event.block.timestamp;
  } else if (entity.status == "reproposed") {
    // second or further challenges
    entity.status = "disputed";
    entity.wasDisputed = true;
    entity.lastUpdateTimestamp = event.block.timestamp;
  }
  entity.save();
}

export function handleProposePrice(event: ProposePriceEvent): void {
  let entity = MarketResolution.load(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  if (entity == null) {
    // might not be a polymarket request
    return;
  }
  // make sure from UMA CTF adapter
  if (
    event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_V4_ADDRESS)
  ) {
    return;
  }
  if (entity.status == "posed") {
    // first proposal
    entity.status = "proposed";
    entity.proposedPrice = event.params.proposedPrice;
    entity.lastUpdateTimestamp = event.block.timestamp;
  } else if (entity.status == "challenged") {
    // second or further proposals
    entity.status = "reproposed";
    entity.reproposedPrice = event.params.proposedPrice;
    entity.lastUpdateTimestamp = event.block.timestamp;
  }
  entity.save();
}

export function handleRequestPrice(event: RequestPriceEvent): void {
  let entity = MarketResolution.load(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  if (entity == null) {
    // might not be a polymarket request
    return;
  }
  // make sure from UMA CTF adapter
  if (
    event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_V4_ADDRESS)
  ) {
    return;
  }
  entity.transactionHash = event.transaction.hash.toHexString();
  entity.logIndex = event.logIndex;
  entity.save();
}



================================================
FILE: src/mod-registry.ts
================================================
import { log } from "@graphprotocol/graph-ts";
import { ModAdded, ModRemoved } from "../generated/ModRegistry/ModRegistry";
import { Moderator } from "../generated/schema";

export function handleModAdded(event: ModAdded): void {
    log.info("mod added in transaction {}", [event.transaction.hash.toHexString()]);
    let modAddress = event.params.addedMod.toHexString();
    let mod = Moderator.load(modAddress);
    if(mod == null) {
        // first time mod has been added
        mod = new Moderator(modAddress);
        mod.canMod = true;
    } else {
        // Mod was added before but removed
        mod.canMod = true;
    }
    mod.save();
}

export function handleModRemoved(event: ModRemoved): void {
    log.info("mod removed in transaction {}", [event.transaction.hash.toHexString()]);
    let modAddress = event.params.removedMod.toHexString();
    let mod = Moderator.load(modAddress);
    if(mod == null) {
        // mod removed before it was added
        // should not be possible due to how events are emitted but handle that case
        mod = new Moderator(modAddress);
        mod.canMod = false;
    } else {
        // Mod removed
        mod.canMod = false;
    }
    mod.save();
}


================================================
FILE: src/optimistic-oracle-old.ts
================================================
import {
  DisputePrice as DisputePriceEvent,
  ProposePrice as ProposePriceEvent,
  RequestPrice as RequestPriceEvent,
} from "../generated/OptimisticOracleOld/OptimisticOracleOld";
import {
  AncillaryDataHashToQuestionId,
  MarketResolution,
} from "../generated/schema";
import { UMA_CTF_ADAPTER_OLD_ADDRESS } from "./utils/constants";
import { Address, crypto } from "@graphprotocol/graph-ts";

export function handleDisputePrice(event: DisputePriceEvent): void {
  if (event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_OLD_ADDRESS)) {
    return; // only consider requests from old UMA CTF adapter
  }
  let mapping = AncillaryDataHashToQuestionId.load(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  if (mapping == null) {
    return;
  }
  let entity = MarketResolution.load(mapping.questionId);
  if (entity == null) {
    return;
  }
  entity.status = "disputed";
  entity.wasDisputed = true;
  entity.lastUpdateTimestamp = event.block.timestamp;
  entity.save();
}

export function handleProposePrice(event: ProposePriceEvent): void {
  if (event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_OLD_ADDRESS)) {
    return; // only consider requests from old UMA CTF adapter
  }
  let mapping = AncillaryDataHashToQuestionId.load(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  if (mapping == null) {
    return;
  }
  let entity = MarketResolution.load(mapping.questionId);
  if (entity == null) {
    return;
  }
  entity.status = "proposed";
  entity.proposedPrice = event.params.proposedPrice;
  entity.lastUpdateTimestamp = event.block.timestamp;
  entity.save();
}

export function handleRequestPrice(event: RequestPriceEvent): void {
  if (event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_OLD_ADDRESS)) {
    return; // only consider requests from old UMA CTF adapter
  }
  let mapping = AncillaryDataHashToQuestionId.load(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  if (mapping == null) {
    return;
  }
  let entity = MarketResolution.load(mapping.questionId);
  if (entity == null) {
    return;
  }
  entity.status = "posed";
  entity.lastUpdateTimestamp = event.block.timestamp;
  entity.transactionHash = event.transaction.hash.toHexString();
  entity.logIndex = event.logIndex;
  entity.save();
}



================================================
FILE: src/optimistic-oracle-v-2.ts
================================================
import {
  DisputePrice as DisputePriceEvent,
  ProposePrice as ProposePriceEvent,
  RequestPrice as RequestPriceEvent,
} from "../generated/OptimisticOracleV2/OptimisticOracleV2";
import { MarketResolution } from "../generated/schema";
import { UMA_CTF_ADAPTER_V2_ADDRESS,UMA_CTF_ADAPTER_V3_ADDRESS, NEG_RISK_UMA_CTF_ADAPTER_ADDRESS } from "./utils/constants";
import { crypto, Address } from "@graphprotocol/graph-ts";

export function handleDisputePrice(event: DisputePriceEvent): void {
  let entity = MarketResolution.load(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  if (entity == null) {
    // might not be a polymarket request
    return;
  }
  // make sure from UMA CTF adapter
  if (
    event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_V2_ADDRESS) &&
    event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_V3_ADDRESS) &&
    event.params.requester != Address.fromHexString(NEG_RISK_UMA_CTF_ADAPTER_ADDRESS)
  ) {
    return;
  }
  if (entity.status == "proposed") {
    // first challenge
    entity.status = "challenged";
    entity.lastUpdateTimestamp = event.block.timestamp;
  } else if (entity.status == "reproposed") {
    // second or further challenges
    entity.status = "disputed";
    entity.wasDisputed = true;
    entity.lastUpdateTimestamp = event.block.timestamp;
  }
  entity.save();
}

export function handleProposePrice(event: ProposePriceEvent): void {
  let entity = MarketResolution.load(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  if (entity == null) {
    // might not be a polymarket request
    return;
  }
  // make sure from UMA CTF adapter
  if (
    event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_V2_ADDRESS) &&
    event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_V3_ADDRESS) &&
    event.params.requester != Address.fromHexString(NEG_RISK_UMA_CTF_ADAPTER_ADDRESS)
  ) {
    return;
  }
  if (entity.status == "posed") {
    // first proposal
    entity.status = "proposed";
    entity.proposedPrice = event.params.proposedPrice;
    entity.lastUpdateTimestamp = event.block.timestamp;
  } else if (entity.status == "challenged") {
    // second or further proposals
    entity.status = "reproposed";
    entity.reproposedPrice = event.params.proposedPrice;
    entity.lastUpdateTimestamp = event.block.timestamp;
  }
  entity.save();
}

export function handleRequestPrice(event: RequestPriceEvent): void {
  let entity = MarketResolution.load(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  if (entity == null) {
    // might not be a polymarket request
    return;
  }
  // make sure from UMA CTF adapter
  if (
    event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_V2_ADDRESS) &&
    event.params.requester != Address.fromHexString(UMA_CTF_ADAPTER_V3_ADDRESS) &&
    event.params.requester != Address.fromHexString(NEG_RISK_UMA_CTF_ADAPTER_ADDRESS)
  ) {
    return;
  }
  entity.transactionHash = event.transaction.hash.toHexString();
  entity.logIndex = event.logIndex;
  entity.save();
}



================================================
FILE: src/uma-ctf-adapter-old.ts
================================================
import { BigInt, log, crypto } from "@graphprotocol/graph-ts";
import {
  QuestionInitialized as QuestionInitializedEvent,
  QuestionReset as QuestionResetEvent,
  QuestionResolved as QuestionResolvedEvent,
  QuestionSettled as QuestionSettledEvent,
} from "../generated/UmaCtfAdapterOld/UmaCtfAdapterOld";
import {
  MarketResolution,
  AncillaryDataHashToQuestionId,
} from "../generated/schema";

export function handleQuestionInitialized(
  event: QuestionInitializedEvent
): void {
  log.info("initialize question {}", [event.params.questionID.toHexString()]);
  let mapping = new AncillaryDataHashToQuestionId(
    crypto.keccak256(event.params.ancillaryData).toHexString()
  );
  mapping.questionId = event.params.questionID.toHexString();

  mapping.save();

  let entity = new MarketResolution(event.params.questionID.toHexString());
  entity.newVersionQ = false;
  entity.author = event.transaction.from;
  entity.ancillaryData = event.params.ancillaryData;
  entity.lastUpdateTimestamp = event.block.timestamp;
  entity.status = "initialized";
  entity.wasDisputed = false;
  entity.proposedPrice = BigInt.fromI32(69); // we use 69 as the unproposed price
  entity.reproposedPrice = BigInt.fromI32(69);
  entity.price = BigInt.fromI32(69);
  entity.updates = "";
  entity.approved = false;
  entity.save();
}

export function handleQuestionReset(event: QuestionResetEvent): void {
  log.info("reset question {}", [event.params.questionID.toHexString()]);
  let entity = MarketResolution.load(
    event.params.questionID.toHexString()
  ) as MarketResolution;

  entity.status = "posed";
  entity.wasDisputed = false;
  entity.proposedPrice = BigInt.fromI32(69); // we use 69 as the unproposed price
  entity.reproposedPrice = BigInt.fromI32(69);
  entity.price = BigInt.fromI32(69);
  entity.updates = "";
  entity.save();
}

export function handleQuestionResolved(event: QuestionResolvedEvent): void {
  log.info("resolve question {}", [event.params.questionID.toHexString()]);
  let entity = MarketResolution.load(
    event.params.questionID.toHexString()
  ) as MarketResolution;
  entity.lastUpdateTimestamp = event.block.timestamp;
  entity.status = "resolved"; // reports price actually resolving question
  entity.save();
}

export function handleQuestionSettled(event: QuestionSettledEvent): void {
  log.info("resolve question {}", [event.params.questionID.toHexString()]);
  let entity = MarketResolution.load(
    event.params.questionID.toHexString()
  ) as MarketResolution;
  entity.lastUpdateTimestamp = event.block.timestamp;
  entity.price = event.params.settledPrice; // sets price then question needs to be resolved
  entity.save();
}



================================================
FILE: src/uma-ctf-adapter.ts
================================================
import { BigInt, log } from "@graphprotocol/graph-ts";
// the events we use have the same signatures across V2 and V3.1
import {
  QuestionInitialized,
  QuestionReset,
  QuestionResolved,
  PostUpdateCall,
} from "../generated/UmaCtfAdapterV2/UmaCtfAdapterV2";
import { MarketResolution, Moderator, Revision } from "../generated/schema";
import { isApprovalUpdate, isRevisionUpdate } from "./utils/qualifier";

export function handleQuestionInitialized(event: QuestionInitialized): void {
  log.info("initialize question {}", [event.params.questionID.toHexString()]);
  let entity = new MarketResolution(event.params.questionID.toHexString());
  entity.newVersionQ = true;
  entity.author = event.params.creator;
  entity.ancillaryData = event.params.ancillaryData;
  entity.lastUpdateTimestamp = event.params.requestTimestamp;
  entity.status = "posed";
  entity.wasDisputed = false;
  entity.proposedPrice = BigInt.fromI32(69); // we use 69 as the unproposed price
  entity.reproposedPrice = BigInt.fromI32(69);
  entity.price = BigInt.fromI32(69);
  entity.updates = "";
  entity.transactionHash = event.transaction.hash.toHexString();
  entity.logIndex = event.logIndex.minus(new BigInt(1)); // price request event is event before this one
  if (isModerator(event.params.creator.toHexString())) {
    entity.approved = true;
  } else {
    entity.approved = false;
  }
  entity.save();
}

export function handleQuestionReset(event: QuestionReset): void {
  log.info("reset question {}", [event.params.questionID.toHexString()]);
  let entity = MarketResolution.load(
    event.params.questionID.toHexString()
  ) as MarketResolution;

  if (entity.status == "disputed") {
    // too early after dispute case essentially throw away initial proposal
    entity.status = "challenged";
    entity.lastUpdateTimestamp = event.block.timestamp;
    entity.proposedPrice = entity.reproposedPrice;
    entity.reproposedPrice = BigInt.fromI32(69);
    entity.save();
  }
}

export function handleQuestionResolved(event: QuestionResolved): void {
  log.info("resolve question {}", [event.params.questionID.toHexString()]);
  let entity = MarketResolution.load(
    event.params.questionID.toHexString()
  ) as MarketResolution;

  // TODO: add checks on status?

  // mark as resolve and set price
  entity.status = "resolved";
  entity.lastUpdateTimestamp = event.block.timestamp;
  entity.price = event.params.settledPrice;
  entity.save();
}

function isModerator(modAddress: string): boolean {
  let mod = Moderator.load(modAddress);
  if (mod == null) {
    return false;
  }
  return true;
}

function handleRevisionPostUpdate(call: PostUpdateCall): void {
  let questionId = call.inputs.questionID.toHexString();
  log.info("handling revision postUpdate question {}", [questionId]);

  let modAddress = call.transaction.from.toHexString();

  // Ensure that the caller is a moderator
  if (!isModerator(modAddress)) {
    return;
  }

  // Revision entities only get created before a market is approved
  let mkt = MarketResolution.load(questionId);
  if (mkt == null || mkt.approved) {
    return;
  }

  // Revision key: questionId + transactionIndex + update hex
  let revision = new Revision(
    questionId +
      "-" +
      call.transaction.index.toString() +
      "-" +
      call.inputs.update.toHexString()
  );
  revision.questionId = questionId;
  revision.moderator = modAddress;
  revision.timestamp = call.block.timestamp;
  revision.update = call.inputs.update.toString();
  revision.transactionHash = call.transaction.hash.toHexString();
  revision.save();
  return;
}

function handleApprovalPostUpdate(call: PostUpdateCall): void {
  let questionID = call.inputs.questionID.toHexString();
  log.info("handling approval postUpdate question {}", [questionID]);
  let modAddress = call.from.toHexString();

  if (!isModerator(modAddress)) {
    return;
  }

  let mkt = MarketResolution.load(questionID);
  if (mkt == null) {
    return;
  }

  mkt.approved = true;
  mkt.save();
  return;
}

function handleClarificationsPostUpdate(call: PostUpdateCall): void {
  log.info("update question {}", [call.inputs.questionID.toHexString()]);
  let entity = MarketResolution.load(call.inputs.questionID.toHexString());
  if (entity == null) {
    return;
  }
  if (
    !isModerator(call.from.toHexString()) &&
    !(
      call.from.toHexString().toLowerCase() ==
      "0x91430cad2d3975766499717fa0d66a78d814e5c5"
    )
  ) {
    return;
  }
  entity.updates = entity.updates.concat(
    "," +
      call.block.timestamp.toString() +
      "-" +
      call.inputs.update.toHexString()
  );
  entity.save();
}

export function handleAncillaryDataUpdated(call: PostUpdateCall): void {
  log.info("update question {}", [call.inputs.questionID.toHexString()]);

  let update = call.inputs.update.toString();

  // Revision flow
  if (isRevisionUpdate(update)) {
    return handleRevisionPostUpdate(call);
  }

  // Approval flow
  if (isApprovalUpdate(update)) {
    return handleApprovalPostUpdate(call);
  }

  // Standard Post Approval Clarification Flow
  return handleClarificationsPostUpdate(call);
}



================================================
FILE: src/utils/constants.ts
================================================
export const UMA_CTF_ADAPTER_OLD_ADDRESS = "0xCB1822859cEF82Cd2Eb4E6276C7916e692995130";
export const UMA_CTF_ADAPTER_V2_ADDRESS = "0x6A9D222616C90FcA5754cd1333cFD9b7fb6a4F74";
export const UMA_CTF_ADAPTER_V3_ADDRESS = "0x157Ce2d672854c848c9b79C49a8Cc6cc89176a49";
export const UMA_CTF_ADAPTER_V4_ADDRESS = "0x65070BE91477460D8A7AeEb94ef92fe056C2f2A7";
export const NEG_RISK_UMA_CTF_ADAPTER_ADDRESS = "0x2F5e3684cb1F318ec51b00Edba38d79Ac2c0aA9d";



================================================
FILE: src/utils/qualifier.ts
================================================
import { RegExp } from "assemblyscript-regex";

const REVISION_IDENTIFIER_REGEX = new RegExp("^REVISION:");
const APPROVAL_IDENTIFIER_REGEX = new RegExp("^APPROVAL:");

function regexMatch(inputString: string, regex: RegExp): boolean {
    return regex.test(inputString);
}

export function isRevisionUpdate(updateString: string): boolean {
    return regexMatch(updateString, REVISION_IDENTIFIER_REGEX);
}


export function isApprovalUpdate(updateString: string): boolean {
    return regexMatch(updateString, APPROVAL_IDENTIFIER_REGEX);
}


================================================
FILE: tests/qualifier.test.ts
================================================
import { assert, describe, test } from 'matchstick-as/assembly/index';

import { isRevisionUpdate, isApprovalUpdate } from "../src/utils/qualifier";

describe('Qualifier functions', () => {
  test('Revision qualifier', () => {
    assert.assertTrue(isRevisionUpdate("REVISION: This is a revision update"));
    
    // No revision identifier
    assert.assertTrue(!isRevisionUpdate("This is not a revision update"));
    
    // Incorrectly located revision identifier
    assert.assertTrue(!isRevisionUpdate("This is not a REVISION: update"));

    // Attempt to use both qualifiers but only the first will be recognized
    assert.assertTrue(!isRevisionUpdate("APPROVAL:REVISION:This is a malicious update"));
  });

  test('Approval qualifier', () => {
    assert.assertTrue(isApprovalUpdate("APPROVAL: This is an approval update"));
    
    // No approval identifier
    assert.assertTrue(!isRevisionUpdate("This is not a approval update"));
    // Incorrectly located approval identifier
    assert.assertTrue(!isRevisionUpdate("This is not an APPROVAL: update"));
  });
});


