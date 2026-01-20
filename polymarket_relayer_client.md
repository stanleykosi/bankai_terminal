Directory structure:
└── polymarket-builder-relayer-client/
    ├── README.md
    ├── Makefile
    ├── package.json
    ├── pnpm-lock.yaml
    ├── tsconfig.json
    ├── tsconfig.production.json
    ├── .env.example
    ├── .eslintignore
    ├── .eslintrc.js
    ├── .prettierignore
    ├── .prettierrc
    ├── examples/
    │   ├── approve.ts
    │   ├── approveProxy.ts
    │   ├── deploy.ts
    │   ├── getTransaction.ts
    │   ├── getTransactions.ts
    │   ├── poll.ts
    │   └── redeem.ts
    ├── src/
    │   ├── client.ts
    │   ├── endpoints.ts
    │   ├── errors.ts
    │   ├── index.ts
    │   ├── types.ts
    │   ├── abis/
    │   │   ├── erc20Abi.ts
    │   │   ├── index.ts
    │   │   ├── multisend.ts
    │   │   ├── proxyFactory.ts
    │   │   ├── safe.ts
    │   │   └── safeFactory.ts
    │   ├── builder/
    │   │   ├── create.ts
    │   │   ├── derive.ts
    │   │   ├── index.ts
    │   │   ├── proxy.ts
    │   │   └── safe.ts
    │   ├── config/
    │   │   └── index.ts
    │   ├── constants/
    │   │   └── index.ts
    │   ├── encode/
    │   │   ├── index.ts
    │   │   ├── proxy.ts
    │   │   └── safe.ts
    │   ├── http-helpers/
    │   │   └── index.ts
    │   ├── response/
    │   │   └── index.ts
    │   └── utils/
    │       └── index.ts
    ├── tests/
    │   └── signatures/
    │       └── index.test.ts
    └── .github/
        └── workflows/
            └── test.yaml


Files Content:

================================================
FILE: README.md
================================================
# builder-relayer-client

TypeScript client library for interacting with Polymarket relayer infrastructure

## Installation

```bash
pnpm install @polymarket/builder-relayer-client
```

## Quick Start

### Basic Setup

```typescript
import { createWalletClient, Hex, http } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { polygon } from "viem/chains";
import { RelayClient, RelayerTxType } from "@polymarket/builder-relayer-client";

const relayerUrl = process.env.POLYMARKET_RELAYER_URL;
const chainId = parseInt(process.env.CHAIN_ID);

const account = privateKeyToAccount(process.env.PRIVATE_KEY as Hex);
const wallet = createWalletClient({
  account,
  chain: polygon,
  transport: http(process.env.RPC_URL)
});

// Initialize the client with SAFE transaction type (default)
const client = new RelayClient(relayerUrl, chainId, wallet);

// Or initialize with PROXY transaction type
const proxyClient = new RelayClient(relayerUrl, chainId, wallet, undefined, RelayerTxType.PROXY);
```

### Transaction Types

The client supports two transaction types via the `RelayerTxType` enum:

- **`RelayerTxType.SAFE`** (default): Executes transactions through for a Gnosis Safe
- **`RelayerTxType.PROXY`**: Executes transactions for a Polymarket Proxy wallet

The transaction type is specified as the last parameter when creating a `RelayClient` instance. All examples use the `Transaction` type - the client automatically converts transactions to the appropriate format (`SafeTransaction` or `ProxyTransaction`) based on the `RelayerTxType` you've configured.

### With Local Builder Authentication

```typescript
import { BuilderApiKeyCreds, BuilderConfig } from "@polymarket/builder-signing-sdk";
import { RelayerTxType } from "@polymarket/builder-relayer-client";

const builderCreds: BuilderApiKeyCreds = {
  key: process.env.BUILDER_API_KEY,
  secret: process.env.BUILDER_SECRET,
  passphrase: process.env.BUILDER_PASS_PHRASE,
};

const builderConfig = new BuilderConfig({
  localBuilderCreds: builderCreds
});

// Initialize with SAFE transaction type (default)
const client = new RelayClient(relayerUrl, chainId, wallet, builderConfig);

// Or initialize with PROXY transaction type
const proxyClient = new RelayClient(relayerUrl, chainId, wallet, builderConfig, RelayerTxType.PROXY);
```

### With Remote Builder Authentication

```typescript
import { BuilderConfig } from "@polymarket/builder-signing-sdk";
import { RelayerTxType } from "@polymarket/builder-relayer-client";

const builderConfig = new BuilderConfig(
  {
    remoteBuilderConfig: {
      url: "http://localhost:3000/sign",
      token: `${process.env.MY_AUTH_TOKEN}`
    }
  },
);

// Initialize with SAFE transaction type (default)
const client = new RelayClient(relayerUrl, chainId, wallet, builderConfig);

// Or initialize with PROXY transaction type
const proxyClient = new RelayClient(relayerUrl, chainId, wallet, builderConfig, RelayerTxType.PROXY);
```

## Examples

### Execute ERC20 Approval Transaction

```typescript
import { encodeFunctionData, prepareEncodeFunctionData, maxUint256 } from "viem";
import { Transaction, RelayerTxType } from "@polymarket/builder-relayer-client";

const erc20Abi = [
  {
    "constant": false,
    "inputs": [
      {"name": "_spender", "type": "address"},
      {"name": "_value", "type": "uint256"}
    ],
    "name": "approve",
    "outputs": [{"name": "", "type": "bool"}],
    "payable": false,
    "stateMutability": "nonpayable",
    "type": "function"
  }
];

const erc20 = prepareEncodeFunctionData({
  abi: erc20Abi,
  functionName: "approve",
});

function createApprovalTransaction(
  tokenAddress: string,
  spenderAddress: string
): Transaction {
  const calldata = encodeFunctionData({
    ...erc20,
    args: [spenderAddress, maxUint256]
  });
  return {
    to: tokenAddress,
    data: calldata,
    value: "0"
  };
}

// Initialize client with SAFE transaction type (default)
const safeClient = new RelayClient(relayerUrl, chainId, wallet, builderConfig);

// Or initialize with PROXY transaction type
const proxyClient = new RelayClient(relayerUrl, chainId, wallet, builderConfig, RelayerTxType.PROXY);

// Execute the approval - works with both SAFE and PROXY
const approvalTx = createApprovalTransaction(
  "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174", // USDC
  "0x4d97dcd97ec945f40cf65f87097ace5ea0476045"  // CTF
);

// Using SAFE client
const safeResponse = await safeClient.execute([approvalTx], "usdc approval on the CTF");
const safeResult = await safeResponse.wait();
console.log("Safe approval completed:", safeResult.transactionHash);

// Using PROXY client
const proxyResponse = await proxyClient.execute([approvalTx], "usdc approval on the CTF");
const proxyResult = await proxyResponse.wait();
console.log("Proxy approval completed:", proxyResult.transactionHash);
```

### Deploy Safe Contract

> **Note:** Safe deployment is only available for `RelayerTxType.SAFE`. Proxy wallets are deployed automatically on its first transaction.

```typescript
// Initialize client with SAFE transaction type (default)
const client = new RelayClient(relayerUrl, chainId, wallet, builderConfig);

const response = await client.deploy();
const result = await response.wait();

if (result) {
  console.log("Safe deployed successfully!");
  console.log("Transaction Hash:", result.transactionHash);
  console.log("Safe Address:", result.proxyAddress);
} else {
  console.log("Safe deployment failed");
}
```

### Redeem Positions

#### CTF (ConditionalTokensFramework) Redeem

```typescript
import { encodeFunctionData, prepareEncodeFunctionData, zeroHash } from "viem";
import { Transaction, RelayerTxType } from "@polymarket/builder-relayer-client";

const ctfRedeemAbi = [
  {
    "constant": false,
    "inputs": [
      {"name": "collateralToken", "type": "address"},
      {"name": "parentCollectionId", "type": "bytes32"},
      {"name": "conditionId", "type": "bytes32"},
      {"name": "indexSets", "type": "uint256[]"}
    ],
    "name": "redeemPositions",
    "outputs": [],
    "payable": false,
    "stateMutability": "nonpayable",
    "type": "function"
  }
];

const ctf = prepareEncodeFunctionData({
  abi: ctfRedeemAbi,
  functionName: "redeemPositions",
});

function createCtfRedeemTransaction(
  ctfAddress: string,
  collateralToken: string,
  conditionId: string
): Transaction {
  const calldata = encodeFunctionData({
    ...ctf,
    args: [collateralToken, zeroHash, conditionId, [1, 2]]
  });
  return {
    to: ctfAddress,
    data: calldata,
    value: "0"
  };
}

// Initialize client with SAFE transaction type (default)
const safeClient = new RelayClient(relayerUrl, chainId, wallet, builderConfig);

// Or initialize with PROXY transaction type
const proxyClient = new RelayClient(relayerUrl, chainId, wallet, builderConfig, RelayerTxType.PROXY);

// Execute the redeem - works with both SAFE and PROXY
const ctfAddress = "0x4d97dcd97ec945f40cf65f87097ace5ea0476045";
const usdcAddress = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";
const conditionId = "0x..."; // Your condition ID

const redeemTx = createCtfRedeemTransaction(ctfAddress, usdcAddress, conditionId);

// Using SAFE client
const safeResponse = await safeClient.execute([redeemTx], "redeem positions");
const safeResult = await safeResponse.wait();
console.log("Safe redeem completed:", safeResult.transactionHash);

// Using PROXY client
const proxyResponse = await proxyClient.execute([redeemTx], "redeem positions");
const proxyResult = await proxyResponse.wait();
console.log("Proxy redeem completed:", proxyResult.transactionHash);
```

#### NegRisk Adapter Redeem

```typescript
import { encodeFunctionData, prepareEncodeFunctionData } from "viem";
import { Transaction, RelayerTxType } from "@polymarket/builder-relayer-client";

const nrAdapterRedeemAbi = [
  {
    "inputs": [
      {"internalType": "bytes32", "name": "_conditionId", "type": "bytes32"},
      {"internalType": "uint256[]", "name": "_amounts", "type": "uint256[]"}
    ],
    "name": "redeemPositions",
    "outputs": [],
    "stateMutability": "nonpayable",
    "type": "function"
  }
];

const nrAdapter = prepareEncodeFunctionData({
  abi: nrAdapterRedeemAbi,
  functionName: "redeemPositions",
});

function createNrAdapterRedeemTransaction(
  adapterAddress: string,
  conditionId: string,
  redeemAmounts: bigint[] // [yesAmount, noAmount]
): Transaction {
  const calldata = encodeFunctionData({
    ...nrAdapter,
    args: [conditionId, redeemAmounts]
  });
  return {
    to: adapterAddress,
    data: calldata,
    value: "0"
  };
}

// Initialize client with SAFE transaction type (default)
const safeClient = new RelayClient(relayerUrl, chainId, wallet, builderConfig);

// Or initialize with PROXY transaction type
const proxyClient = new RelayClient(relayerUrl, chainId, wallet, builderConfig, RelayerTxType.PROXY);

// Execute the redeem - works with both SAFE and PROXY
const negRiskAdapter = "0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296";
const conditionId = "0x..."; // Your condition ID
const redeemAmounts = [BigInt(111000000), BigInt(0)]; // [yes tokens, no tokens]

const redeemTx = createNrAdapterRedeemTransaction(negRiskAdapter, conditionId, redeemAmounts);

// Using SAFE client
const safeResponse = await safeClient.execute([redeemTx], "redeem positions");
const safeResult = await safeResponse.wait();
console.log("Safe redeem completed:", safeResult.transactionHash);

// Using PROXY client
const proxyResponse = await proxyClient.execute([redeemTx], "redeem positions");
const proxyResult = await proxyResponse.wait();
console.log("Proxy redeem completed:", proxyResult.transactionHash);
```



================================================
FILE: Makefile
================================================
.PHONY: build
build:
	@echo "Building ts code..."
	pnpm clean && tsc --project tsconfig.production.json

.PHONY: test
test:
	pnpm exec mocha --import=tsx 'tests/**/*.test.ts' --timeout 300000 --exit





================================================
FILE: package.json
================================================
{
    "name": "@polymarket/builder-relayer-client",
    "description": "Client for Polymarket relayers",
    "version": "0.0.8",
    "main": "dist/index.js",
    "types": "dist/index.d.ts",
    "files": [
        "/dist"
    ],
    "scripts": {
        "build": "make build",
        "clean": "rm -rf ./dist",
        "test": "make test"
    },
    "devDependencies": {
        "@types/chai": "5.2.2",
        "@types/mocha": "10.0.10",
        "@types/node": "^18.7.18",
        "@types/ws": "^8.5.3",
        "chai": "5.2.0",
        "dotenv": "^16.0.2",
        "esm": "^3.2.25",
        "jsdom": "^20.0.0",
        "jsdom-global": "^3.0.2",
        "mocha": "9.2.2",
        "path": "^0.12.7",
        "prettier": "^2.7.1",
        "ts-node": "^9.1.1",
        "tsconfig-paths": "^4.2.0",
        "tslib": "^2.8.1",
        "ws": "^8.11.0"
    },
    "dependencies": {
        "@polymarket/builder-abstract-signer": "0.0.1",
        "@polymarket/builder-signing-sdk": "^0.0.8",
        "axios": "^0.27.2",
        "browser-or-node": "^3.0.0",
        "ethers": "5.8.0",
        "tsx": "^4.20.3",
        "typescript": "^5.8.3",
        "viem": "^2.31.4"
    }
}



================================================
FILE: pnpm-lock.yaml
================================================
lockfileVersion: '9.0'

settings:
  autoInstallPeers: true
  excludeLinksFromLockfile: false

importers:

  .:
    dependencies:
      '@polymarket/builder-abstract-signer':
        specifier: 0.0.1
        version: 0.0.1
      '@polymarket/builder-signing-sdk':
        specifier: ^0.0.8
        version: 0.0.8
      axios:
        specifier: ^0.27.2
        version: 0.27.2
      browser-or-node:
        specifier: ^3.0.0
        version: 3.0.0
      ethers:
        specifier: 5.8.0
        version: 5.8.0
      tsx:
        specifier: ^4.20.3
        version: 4.20.6
      typescript:
        specifier: ^5.8.3
        version: 5.9.3
      viem:
        specifier: ^2.31.4
        version: 2.38.5(typescript@5.9.3)
    devDependencies:
      '@types/chai':
        specifier: 5.2.2
        version: 5.2.2
      '@types/mocha':
        specifier: 10.0.10
        version: 10.0.10
      '@types/node':
        specifier: ^18.7.18
        version: 18.19.130
      '@types/ws':
        specifier: ^8.5.3
        version: 8.18.1
      chai:
        specifier: 5.2.0
        version: 5.2.0
      dotenv:
        specifier: ^16.0.2
        version: 16.6.1
      esm:
        specifier: ^3.2.25
        version: 3.2.25
      jsdom:
        specifier: ^20.0.0
        version: 20.0.3
      jsdom-global:
        specifier: ^3.0.2
        version: 3.0.2(jsdom@20.0.3)
      mocha:
        specifier: 9.2.2
        version: 9.2.2
      path:
        specifier: ^0.12.7
        version: 0.12.7
      prettier:
        specifier: ^2.7.1
        version: 2.8.8
      ts-node:
        specifier: ^9.1.1
        version: 9.1.1(typescript@5.9.3)
      tsconfig-paths:
        specifier: ^4.2.0
        version: 4.2.0
      tslib:
        specifier: ^2.8.1
        version: 2.8.1
      ws:
        specifier: ^8.11.0
        version: 8.18.3

packages:

  '@adraffy/ens-normalize@1.11.1':
    resolution: {integrity: sha512-nhCBV3quEgesuf7c7KYfperqSS14T8bYuvJ8PcLJp6znkZpFc0AuW4qBtr8eKVyPPe/8RSr7sglCWPU5eaxwKQ==}

  '@esbuild/aix-ppc64@0.25.11':
    resolution: {integrity: sha512-Xt1dOL13m8u0WE8iplx9Ibbm+hFAO0GsU2P34UNoDGvZYkY8ifSiy6Zuc1lYxfG7svWE2fzqCUmFp5HCn51gJg==}
    engines: {node: '>=18'}
    cpu: [ppc64]
    os: [aix]

  '@esbuild/android-arm64@0.25.11':
    resolution: {integrity: sha512-9slpyFBc4FPPz48+f6jyiXOx/Y4v34TUeDDXJpZqAWQn/08lKGeD8aDp9TMn9jDz2CiEuHwfhRmGBvpnd/PWIQ==}
    engines: {node: '>=18'}
    cpu: [arm64]
    os: [android]

  '@esbuild/android-arm@0.25.11':
    resolution: {integrity: sha512-uoa7dU+Dt3HYsethkJ1k6Z9YdcHjTrSb5NUy66ZfZaSV8hEYGD5ZHbEMXnqLFlbBflLsl89Zke7CAdDJ4JI+Gg==}
    engines: {node: '>=18'}
    cpu: [arm]
    os: [android]

  '@esbuild/android-x64@0.25.11':
    resolution: {integrity: sha512-Sgiab4xBjPU1QoPEIqS3Xx+R2lezu0LKIEcYe6pftr56PqPygbB7+szVnzoShbx64MUupqoE0KyRlN7gezbl8g==}
    engines: {node: '>=18'}
    cpu: [x64]
    os: [android]

  '@esbuild/darwin-arm64@0.25.11':
    resolution: {integrity: sha512-VekY0PBCukppoQrycFxUqkCojnTQhdec0vevUL/EDOCnXd9LKWqD/bHwMPzigIJXPhC59Vd1WFIL57SKs2mg4w==}
    engines: {node: '>=18'}
    cpu: [arm64]
    os: [darwin]

  '@esbuild/darwin-x64@0.25.11':
    resolution: {integrity: sha512-+hfp3yfBalNEpTGp9loYgbknjR695HkqtY3d3/JjSRUyPg/xd6q+mQqIb5qdywnDxRZykIHs3axEqU6l1+oWEQ==}
    engines: {node: '>=18'}
    cpu: [x64]
    os: [darwin]

  '@esbuild/freebsd-arm64@0.25.11':
    resolution: {integrity: sha512-CmKjrnayyTJF2eVuO//uSjl/K3KsMIeYeyN7FyDBjsR3lnSJHaXlVoAK8DZa7lXWChbuOk7NjAc7ygAwrnPBhA==}
    engines: {node: '>=18'}
    cpu: [arm64]
    os: [freebsd]

  '@esbuild/freebsd-x64@0.25.11':
    resolution: {integrity: sha512-Dyq+5oscTJvMaYPvW3x3FLpi2+gSZTCE/1ffdwuM6G1ARang/mb3jvjxs0mw6n3Lsw84ocfo9CrNMqc5lTfGOw==}
    engines: {node: '>=18'}
    cpu: [x64]
    os: [freebsd]

  '@esbuild/linux-arm64@0.25.11':
    resolution: {integrity: sha512-Qr8AzcplUhGvdyUF08A1kHU3Vr2O88xxP0Tm8GcdVOUm25XYcMPp2YqSVHbLuXzYQMf9Bh/iKx7YPqECs6ffLA==}
    engines: {node: '>=18'}
    cpu: [arm64]
    os: [linux]

  '@esbuild/linux-arm@0.25.11':
    resolution: {integrity: sha512-TBMv6B4kCfrGJ8cUPo7vd6NECZH/8hPpBHHlYI3qzoYFvWu2AdTvZNuU/7hsbKWqu/COU7NIK12dHAAqBLLXgw==}
    engines: {node: '>=18'}
    cpu: [arm]
    os: [linux]

  '@esbuild/linux-ia32@0.25.11':
    resolution: {integrity: sha512-TmnJg8BMGPehs5JKrCLqyWTVAvielc615jbkOirATQvWWB1NMXY77oLMzsUjRLa0+ngecEmDGqt5jiDC6bfvOw==}
    engines: {node: '>=18'}
    cpu: [ia32]
    os: [linux]

  '@esbuild/linux-loong64@0.25.11':
    resolution: {integrity: sha512-DIGXL2+gvDaXlaq8xruNXUJdT5tF+SBbJQKbWy/0J7OhU8gOHOzKmGIlfTTl6nHaCOoipxQbuJi7O++ldrxgMw==}
    engines: {node: '>=18'}
    cpu: [loong64]
    os: [linux]

  '@esbuild/linux-mips64el@0.25.11':
    resolution: {integrity: sha512-Osx1nALUJu4pU43o9OyjSCXokFkFbyzjXb6VhGIJZQ5JZi8ylCQ9/LFagolPsHtgw6himDSyb5ETSfmp4rpiKQ==}
    engines: {node: '>=18'}
    cpu: [mips64el]
    os: [linux]

  '@esbuild/linux-ppc64@0.25.11':
    resolution: {integrity: sha512-nbLFgsQQEsBa8XSgSTSlrnBSrpoWh7ioFDUmwo158gIm5NNP+17IYmNWzaIzWmgCxq56vfr34xGkOcZ7jX6CPw==}
    engines: {node: '>=18'}
    cpu: [ppc64]
    os: [linux]

  '@esbuild/linux-riscv64@0.25.11':
    resolution: {integrity: sha512-HfyAmqZi9uBAbgKYP1yGuI7tSREXwIb438q0nqvlpxAOs3XnZ8RsisRfmVsgV486NdjD7Mw2UrFSw51lzUk1ww==}
    engines: {node: '>=18'}
    cpu: [riscv64]
    os: [linux]

  '@esbuild/linux-s390x@0.25.11':
    resolution: {integrity: sha512-HjLqVgSSYnVXRisyfmzsH6mXqyvj0SA7pG5g+9W7ESgwA70AXYNpfKBqh1KbTxmQVaYxpzA/SvlB9oclGPbApw==}
    engines: {node: '>=18'}
    cpu: [s390x]
    os: [linux]

  '@esbuild/linux-x64@0.25.11':
    resolution: {integrity: sha512-HSFAT4+WYjIhrHxKBwGmOOSpphjYkcswF449j6EjsjbinTZbp8PJtjsVK1XFJStdzXdy/jaddAep2FGY+wyFAQ==}
    engines: {node: '>=18'}
    cpu: [x64]
    os: [linux]

  '@esbuild/netbsd-arm64@0.25.11':
    resolution: {integrity: sha512-hr9Oxj1Fa4r04dNpWr3P8QKVVsjQhqrMSUzZzf+LZcYjZNqhA3IAfPQdEh1FLVUJSiu6sgAwp3OmwBfbFgG2Xg==}
    engines: {node: '>=18'}
    cpu: [arm64]
    os: [netbsd]

  '@esbuild/netbsd-x64@0.25.11':
    resolution: {integrity: sha512-u7tKA+qbzBydyj0vgpu+5h5AeudxOAGncb8N6C9Kh1N4n7wU1Xw1JDApsRjpShRpXRQlJLb9wY28ELpwdPcZ7A==}
    engines: {node: '>=18'}
    cpu: [x64]
    os: [netbsd]

  '@esbuild/openbsd-arm64@0.25.11':
    resolution: {integrity: sha512-Qq6YHhayieor3DxFOoYM1q0q1uMFYb7cSpLD2qzDSvK1NAvqFi8Xgivv0cFC6J+hWVw2teCYltyy9/m/14ryHg==}
    engines: {node: '>=18'}
    cpu: [arm64]
    os: [openbsd]

  '@esbuild/openbsd-x64@0.25.11':
    resolution: {integrity: sha512-CN+7c++kkbrckTOz5hrehxWN7uIhFFlmS/hqziSFVWpAzpWrQoAG4chH+nN3Be+Kzv/uuo7zhX716x3Sn2Jduw==}
    engines: {node: '>=18'}
    cpu: [x64]
    os: [openbsd]

  '@esbuild/openharmony-arm64@0.25.11':
    resolution: {integrity: sha512-rOREuNIQgaiR+9QuNkbkxubbp8MSO9rONmwP5nKncnWJ9v5jQ4JxFnLu4zDSRPf3x4u+2VN4pM4RdyIzDty/wQ==}
    engines: {node: '>=18'}
    cpu: [arm64]
    os: [openharmony]

  '@esbuild/sunos-x64@0.25.11':
    resolution: {integrity: sha512-nq2xdYaWxyg9DcIyXkZhcYulC6pQ2FuCgem3LI92IwMgIZ69KHeY8T4Y88pcwoLIjbed8n36CyKoYRDygNSGhA==}
    engines: {node: '>=18'}
    cpu: [x64]
    os: [sunos]

  '@esbuild/win32-arm64@0.25.11':
    resolution: {integrity: sha512-3XxECOWJq1qMZ3MN8srCJ/QfoLpL+VaxD/WfNRm1O3B4+AZ/BnLVgFbUV3eiRYDMXetciH16dwPbbHqwe1uU0Q==}
    engines: {node: '>=18'}
    cpu: [arm64]
    os: [win32]

  '@esbuild/win32-ia32@0.25.11':
    resolution: {integrity: sha512-3ukss6gb9XZ8TlRyJlgLn17ecsK4NSQTmdIXRASVsiS2sQ6zPPZklNJT5GR5tE/MUarymmy8kCEf5xPCNCqVOA==}
    engines: {node: '>=18'}
    cpu: [ia32]
    os: [win32]

  '@esbuild/win32-x64@0.25.11':
    resolution: {integrity: sha512-D7Hpz6A2L4hzsRpPaCYkQnGOotdUpDzSGRIv9I+1ITdHROSFUWW95ZPZWQmGka1Fg7W3zFJowyn9WGwMJ0+KPA==}
    engines: {node: '>=18'}
    cpu: [x64]
    os: [win32]

  '@ethersproject/abi@5.8.0':
    resolution: {integrity: sha512-b9YS/43ObplgyV6SlyQsG53/vkSal0MNA1fskSC4mbnCMi8R+NkcH8K9FPYNESf6jUefBUniE4SOKms0E/KK1Q==}

  '@ethersproject/abstract-provider@5.8.0':
    resolution: {integrity: sha512-wC9SFcmh4UK0oKuLJQItoQdzS/qZ51EJegK6EmAWlh+OptpQ/npECOR3QqECd8iGHC0RJb4WKbVdSfif4ammrg==}

  '@ethersproject/abstract-signer@5.8.0':
    resolution: {integrity: sha512-N0XhZTswXcmIZQdYtUnd79VJzvEwXQw6PK0dTl9VoYrEBxxCPXqS0Eod7q5TNKRxe1/5WUMuR0u0nqTF/avdCA==}

  '@ethersproject/address@5.8.0':
    resolution: {integrity: sha512-GhH/abcC46LJwshoN+uBNoKVFPxUuZm6dA257z0vZkKmU1+t8xTn8oK7B9qrj8W2rFRMch4gbJl6PmVxjxBEBA==}

  '@ethersproject/base64@5.8.0':
    resolution: {integrity: sha512-lN0oIwfkYj9LbPx4xEkie6rAMJtySbpOAFXSDVQaBnAzYfB4X2Qr+FXJGxMoc3Bxp2Sm8OwvzMrywxyw0gLjIQ==}

  '@ethersproject/basex@5.8.0':
    resolution: {integrity: sha512-PIgTszMlDRmNwW9nhS6iqtVfdTAKosA7llYXNmGPw4YAI1PUyMv28988wAb41/gHF/WqGdoLv0erHaRcHRKW2Q==}

  '@ethersproject/bignumber@5.8.0':
    resolution: {integrity: sha512-ZyaT24bHaSeJon2tGPKIiHszWjD/54Sz8t57Toch475lCLljC6MgPmxk7Gtzz+ddNN5LuHea9qhAe0x3D+uYPA==}

  '@ethersproject/bytes@5.8.0':
    resolution: {integrity: sha512-vTkeohgJVCPVHu5c25XWaWQOZ4v+DkGoC42/TS2ond+PARCxTJvgTFUNDZovyQ/uAQ4EcpqqowKydcdmRKjg7A==}

  '@ethersproject/constants@5.8.0':
    resolution: {integrity: sha512-wigX4lrf5Vu+axVTIvNsuL6YrV4O5AXl5ubcURKMEME5TnWBouUh0CDTWxZ2GpnRn1kcCgE7l8O5+VbV9QTTcg==}

  '@ethersproject/contracts@5.8.0':
    resolution: {integrity: sha512-0eFjGz9GtuAi6MZwhb4uvUM216F38xiuR0yYCjKJpNfSEy4HUM8hvqqBj9Jmm0IUz8l0xKEhWwLIhPgxNY0yvQ==}

  '@ethersproject/hash@5.8.0':
    resolution: {integrity: sha512-ac/lBcTbEWW/VGJij0CNSw/wPcw9bSRgCB0AIBz8CvED/jfvDoV9hsIIiWfvWmFEi8RcXtlNwp2jv6ozWOsooA==}

  '@ethersproject/hdnode@5.8.0':
    resolution: {integrity: sha512-4bK1VF6E83/3/Im0ERnnUeWOY3P1BZml4ZD3wcH8Ys0/d1h1xaFt6Zc+Dh9zXf9TapGro0T4wvO71UTCp3/uoA==}

  '@ethersproject/json-wallets@5.8.0':
    resolution: {integrity: sha512-HxblNck8FVUtNxS3VTEYJAcwiKYsBIF77W15HufqlBF9gGfhmYOJtYZp8fSDZtn9y5EaXTE87zDwzxRoTFk11w==}

  '@ethersproject/keccak256@5.8.0':
    resolution: {integrity: sha512-A1pkKLZSz8pDaQ1ftutZoaN46I6+jvuqugx5KYNeQOPqq+JZ0Txm7dlWesCHB5cndJSu5vP2VKptKf7cksERng==}

  '@ethersproject/logger@5.8.0':
    resolution: {integrity: sha512-Qe6knGmY+zPPWTC+wQrpitodgBfH7XoceCGL5bJVejmH+yCS3R8jJm8iiWuvWbG76RUmyEG53oqv6GMVWqunjA==}

  '@ethersproject/networks@5.8.0':
    resolution: {integrity: sha512-egPJh3aPVAzbHwq8DD7Po53J4OUSsA1MjQp8Vf/OZPav5rlmWUaFLiq8cvQiGK0Z5K6LYzm29+VA/p4RL1FzNg==}

  '@ethersproject/pbkdf2@5.8.0':
    resolution: {integrity: sha512-wuHiv97BrzCmfEaPbUFpMjlVg/IDkZThp9Ri88BpjRleg4iePJaj2SW8AIyE8cXn5V1tuAaMj6lzvsGJkGWskg==}

  '@ethersproject/properties@5.8.0':
    resolution: {integrity: sha512-PYuiEoQ+FMaZZNGrStmN7+lWjlsoufGIHdww7454FIaGdbe/p5rnaCXTr5MtBYl3NkeoVhHZuyzChPeGeKIpQw==}

  '@ethersproject/providers@5.8.0':
    resolution: {integrity: sha512-3Il3oTzEx3o6kzcg9ZzbE+oCZYyY+3Zh83sKkn4s1DZfTUjIegHnN2Cm0kbn9YFy45FDVcuCLLONhU7ny0SsCw==}

  '@ethersproject/random@5.8.0':
    resolution: {integrity: sha512-E4I5TDl7SVqyg4/kkA/qTfuLWAQGXmSOgYyO01So8hLfwgKvYK5snIlzxJMk72IFdG/7oh8yuSqY2KX7MMwg+A==}

  '@ethersproject/rlp@5.8.0':
    resolution: {integrity: sha512-LqZgAznqDbiEunaUvykH2JAoXTT9NV0Atqk8rQN9nx9SEgThA/WMx5DnW8a9FOufo//6FZOCHZ+XiClzgbqV9Q==}

  '@ethersproject/sha2@5.8.0':
    resolution: {integrity: sha512-dDOUrXr9wF/YFltgTBYS0tKslPEKr6AekjqDW2dbn1L1xmjGR+9GiKu4ajxovnrDbwxAKdHjW8jNcwfz8PAz4A==}

  '@ethersproject/signing-key@5.8.0':
    resolution: {integrity: sha512-LrPW2ZxoigFi6U6aVkFN/fa9Yx/+4AtIUe4/HACTvKJdhm0eeb107EVCIQcrLZkxaSIgc/eCrX8Q1GtbH+9n3w==}

  '@ethersproject/solidity@5.8.0':
    resolution: {integrity: sha512-4CxFeCgmIWamOHwYN9d+QWGxye9qQLilpgTU0XhYs1OahkclF+ewO+3V1U0mvpiuQxm5EHHmv8f7ClVII8EHsA==}

  '@ethersproject/strings@5.8.0':
    resolution: {integrity: sha512-qWEAk0MAvl0LszjdfnZ2uC8xbR2wdv4cDabyHiBh3Cldq/T8dPH3V4BbBsAYJUeonwD+8afVXld274Ls+Y1xXg==}

  '@ethersproject/transactions@5.8.0':
    resolution: {integrity: sha512-UglxSDjByHG0TuU17bDfCemZ3AnKO2vYrL5/2n2oXvKzvb7Cz+W9gOWXKARjp2URVwcWlQlPOEQyAviKwT4AHg==}

  '@ethersproject/units@5.8.0':
    resolution: {integrity: sha512-lxq0CAnc5kMGIiWW4Mr041VT8IhNM+Pn5T3haO74XZWFulk7wH1Gv64HqE96hT4a7iiNMdOCFEBgaxWuk8ETKQ==}

  '@ethersproject/wallet@5.8.0':
    resolution: {integrity: sha512-G+jnzmgg6UxurVKRKvw27h0kvG75YKXZKdlLYmAHeF32TGUzHkOFd7Zn6QHOTYRFWnfjtSSFjBowKo7vfrXzPA==}

  '@ethersproject/web@5.8.0':
    resolution: {integrity: sha512-j7+Ksi/9KfGviws6Qtf9Q7KCqRhpwrYKQPs+JBA/rKVFF/yaWLHJEH3zfVP2plVu+eys0d2DlFmhoQJayFewcw==}

  '@ethersproject/wordlists@5.8.0':
    resolution: {integrity: sha512-2df9bbXicZws2Sb5S6ET493uJ0Z84Fjr3pC4tu/qlnZERibZCeUVuqdtt+7Tv9xxhUxHoIekIA7avrKUWHrezg==}

  '@noble/ciphers@1.3.0':
    resolution: {integrity: sha512-2I0gnIVPtfnMw9ee9h1dJG7tp81+8Ob3OJb3Mv37rx5L40/b0i7djjCVvGOVqc9AEIQyvyu1i6ypKdFw8R8gQw==}
    engines: {node: ^14.21.3 || >=16}

  '@noble/curves@1.9.1':
    resolution: {integrity: sha512-k11yZxZg+t+gWvBbIswW0yoJlu8cHOC7dhunwOzoWH/mXGBiYyR4YY6hAEK/3EUs4UpB8la1RfdRpeGsFHkWsA==}
    engines: {node: ^14.21.3 || >=16}

  '@noble/hashes@1.8.0':
    resolution: {integrity: sha512-jCs9ldd7NwzpgXDIf6P3+NrHh9/sD6CQdxHyjQI+h/6rDNo88ypBxxz45UDuZHz9r3tNz7N/VInSVoVdtXEI4A==}
    engines: {node: ^14.21.3 || >=16}

  '@polymarket/builder-abstract-signer@0.0.1':
    resolution: {integrity: sha512-XuuxQQcXYtqQce8slhqiJQti1lVPr+xXC7M3lbetmNnsc9tlYdYRnojE+tcniCHg7VQ6dokcIu0eLYDtM5vdvQ==}

  '@polymarket/builder-signing-sdk@0.0.8':
    resolution: {integrity: sha512-rZLCFxEdYahl5FiJmhe22RDXysS1ibFJlWz4NT0s3itJRYq3XJzXXHXEZkAQplU+nIS1IlbbKjA4zDQaeCyYtg==}

  '@scure/base@1.2.6':
    resolution: {integrity: sha512-g/nm5FgUa//MCj1gV09zTJTaM6KBAHqLN907YVQqf7zC49+DcO4B1so4ZX07Ef10Twr6nuqYEH9GEggFXA4Fmg==}

  '@scure/bip32@1.7.0':
    resolution: {integrity: sha512-E4FFX/N3f4B80AKWp5dP6ow+flD1LQZo/w8UnLGYZO674jS6YnYeepycOOksv+vLPSpgN35wgKgy+ybfTb2SMw==}

  '@scure/bip39@1.6.0':
    resolution: {integrity: sha512-+lF0BbLiJNwVlev4eKelw1WWLaiKXw7sSl8T6FvBlWkdX+94aGJ4o8XjUdlyhTCjd8c+B3KT3JfS8P0bLRNU6A==}

  '@tootallnate/once@2.0.0':
    resolution: {integrity: sha512-XCuKFP5PS55gnMVu3dty8KPatLqUoy/ZYzDzAGCQ8JNFCkLXzmI7vNHCR+XpbZaMWQK/vQubr7PkYq8g470J/A==}
    engines: {node: '>= 10'}

  '@types/chai@5.2.2':
    resolution: {integrity: sha512-8kB30R7Hwqf40JPiKhVzodJs2Qc1ZJ5zuT3uzw5Hq/dhNCl3G3l83jfpdI1e20BP348+fV7VIL/+FxaXkqBmWg==}

  '@types/deep-eql@4.0.2':
    resolution: {integrity: sha512-c9h9dVVMigMPc4bwTvC5dxqtqJZwQPePsWjPlpSOnojbor6pGqdk541lfA7AqFQr5pB1BRdq0juY9db81BwyFw==}

  '@types/mocha@10.0.10':
    resolution: {integrity: sha512-xPyYSz1cMPnJQhl0CLMH68j3gprKZaTjG3s5Vi+fDgx+uhG9NOXwbVt52eFS8ECyXhyKcjDLCBEqBExKuiZb7Q==}

  '@types/node@18.19.130':
    resolution: {integrity: sha512-GRaXQx6jGfL8sKfaIDD6OupbIHBr9jv7Jnaml9tB7l4v068PAOXqfcujMMo5PhbIs6ggR1XODELqahT2R8v0fg==}

  '@types/ws@8.18.1':
    resolution: {integrity: sha512-ThVF6DCVhA8kUGy+aazFQ4kXQ7E1Ty7A3ypFOe0IcJV8O/M511G99AW24irKrW56Wt44yG9+ij8FaqoBGkuBXg==}

  '@ungap/promise-all-settled@1.1.2':
    resolution: {integrity: sha512-sL/cEvJWAnClXw0wHk85/2L0G6Sj8UB0Ctc1TEMbKSsmpRosqhwj9gWgFRZSrBr2f9tiXISwNhCPmlfqUqyb9Q==}

  abab@2.0.6:
    resolution: {integrity: sha512-j2afSsaIENvHZN2B8GOpF566vZ5WVk5opAiMTvWgaQT8DkbOqsTfvNAvHoRGU2zzP8cPoqys+xHTRDWW8L+/BA==}
    deprecated: Use your platform's native atob() and btoa() methods instead

  abitype@1.1.0:
    resolution: {integrity: sha512-6Vh4HcRxNMLA0puzPjM5GBgT4aAcFGKZzSgAXvuZ27shJP6NEpielTuqbBmZILR5/xd0PizkBGy5hReKz9jl5A==}
    peerDependencies:
      typescript: '>=5.0.4'
      zod: ^3.22.0 || ^4.0.0
    peerDependenciesMeta:
      typescript:
        optional: true
      zod:
        optional: true

  acorn-globals@7.0.1:
    resolution: {integrity: sha512-umOSDSDrfHbTNPuNpC2NSnnA3LUrqpevPb4T9jRx4MagXNS0rs+gwiTcAvqCRmsD6utzsrzNt+ebm00SNWiC3Q==}

  acorn-walk@8.3.4:
    resolution: {integrity: sha512-ueEepnujpqee2o5aIYnvHU6C0A42MNdsIDeqy5BydrkuC5R1ZuUFnm27EeFJGoEHJQgn3uleRvmTXaJgfXbt4g==}
    engines: {node: '>=0.4.0'}

  acorn@8.15.0:
    resolution: {integrity: sha512-NZyJarBfL7nWwIq+FDL6Zp/yHEhePMNnnJ0y3qfieCrmNvYct8uvtiV41UvlSe6apAfk0fY1FbWx+NwfmpvtTg==}
    engines: {node: '>=0.4.0'}
    hasBin: true

  aes-js@3.0.0:
    resolution: {integrity: sha512-H7wUZRn8WpTq9jocdxQ2c8x2sKo9ZVmzfRE13GiNJXfp7NcKYEdvl3vspKjXox6RIG2VtaRe4JFvxG4rqp2Zuw==}

  agent-base@6.0.2:
    resolution: {integrity: sha512-RZNwNclF7+MS/8bDg70amg32dyeZGZxiDuQmZxKLAlQjr3jGyLx+4Kkk58UO7D2QdgFIQCovuSuZESne6RG6XQ==}
    engines: {node: '>= 6.0.0'}

  ansi-colors@4.1.1:
    resolution: {integrity: sha512-JoX0apGbHaUJBNl6yF+p6JAFYZ666/hhCGKN5t9QFjbJQKUU/g8MNbFDbvfrgKXvI1QpZplPOnwIo99lX/AAmA==}
    engines: {node: '>=6'}

  ansi-regex@5.0.1:
    resolution: {integrity: sha512-quJQXlTSUGL2LH9SUXo8VwsY4soanhgo6LNSm84E1LBcE8s3O0wpdiRzyR9z/ZZJMlMWv37qOOb9pdJlMUEKFQ==}
    engines: {node: '>=8'}

  ansi-styles@4.3.0:
    resolution: {integrity: sha512-zbB9rCJAT1rbjiVDb2hqKFHNYLxgtk8NURxZ3IZwD3F6NtxbXZQCnnSi1Lkx+IDohdPlFp222wVALIheZJQSEg==}
    engines: {node: '>=8'}

  anymatch@3.1.3:
    resolution: {integrity: sha512-KMReFUr0B4t+D+OBkjR3KYqvocp2XaSzO55UcB6mgQMd3KbcE+mWTyvVV7D/zsdEbNnV6acZUutkiHQXvTr1Rw==}
    engines: {node: '>= 8'}

  arg@4.1.3:
    resolution: {integrity: sha512-58S9QDqG0Xx27YwPSt9fJxivjYl432YCwfDMfZ+71RAqUrZef7LrKQZ3LHLOwCS4FLNBplP533Zx895SeOCHvA==}

  argparse@2.0.1:
    resolution: {integrity: sha512-8+9WqebbFzpX9OR+Wa6O29asIogeRMzcGtAINdpMHHyAg10f05aSFVBbcEqGf/PXw1EjAZ+q2/bEBg3DvurK3Q==}

  assertion-error@2.0.1:
    resolution: {integrity: sha512-Izi8RQcffqCeNVgFigKli1ssklIbpHnCYc6AknXGYoB6grJqyeby7jv12JUQgmTAnIDnbck1uxksT4dzN3PWBA==}
    engines: {node: '>=12'}

  asynckit@0.4.0:
    resolution: {integrity: sha512-Oei9OH4tRh0YqU3GxhX79dM/mwVgvbZJaSNaRk+bshkj0S5cfHcgYakreBjrHwatXKbz+IoIdYLxrKim2MjW0Q==}

  axios@0.27.2:
    resolution: {integrity: sha512-t+yRIyySRTp/wua5xEr+z1q60QmLq8ABsS5O9Me1AsE5dfKqgnCFzwiCZZ/cGNd1lq4/7akDWMxdhVlucjmnOQ==}

  axios@1.13.0:
    resolution: {integrity: sha512-zt40Pz4zcRXra9CVV31KeyofwiNvAbJ5B6YPz9pMJ+yOSLikvPT4Yi5LjfgjRa9CawVYBaD1JQzIVcIvBejKeA==}

  balanced-match@1.0.2:
    resolution: {integrity: sha512-3oSeUO0TMV67hN1AmbXsK4yaqU7tjiHlbxRDZOpH0KW9+CeX4bRAaX0Anxt0tx2MrpRpWwQaPwIlISEJhYU5Pw==}

  bech32@1.1.4:
    resolution: {integrity: sha512-s0IrSOzLlbvX7yp4WBfPITzpAU8sqQcpsmwXDiKwrG4r491vwCO/XpejasRNl0piBMe/DvP4Tz0mIS/X1DPJBQ==}

  binary-extensions@2.3.0:
    resolution: {integrity: sha512-Ceh+7ox5qe7LJuLHoY0feh3pHuUDHAcRUeyL2VYghZwfpkNIy/+8Ocg0a3UuSoYzavmylwuLWQOf3hl0jjMMIw==}
    engines: {node: '>=8'}

  bn.js@4.12.2:
    resolution: {integrity: sha512-n4DSx829VRTRByMRGdjQ9iqsN0Bh4OolPsFnaZBLcbi8iXcB+kJ9s7EnRt4wILZNV3kPLHkRVfOc/HvhC3ovDw==}

  bn.js@5.2.2:
    resolution: {integrity: sha512-v2YAxEmKaBLahNwE1mjp4WON6huMNeuDvagFZW+ASCuA/ku0bXR9hSMw0XpiqMoA3+rmnyck/tPRSFQkoC9Cuw==}

  brace-expansion@1.1.12:
    resolution: {integrity: sha512-9T9UjW3r0UW5c1Q7GTwllptXwhvYmEzFhzMfZ9H7FQWt+uZePjZPjBP/W1ZEyZ1twGWom5/56TF4lPcqjnDHcg==}

  braces@3.0.3:
    resolution: {integrity: sha512-yQbXgO/OSZVD2IsiLlro+7Hf6Q18EJrKSEsdoMzKePKXct3gvD8oLcOQdIzGupr5Fj+EDe8gO/lxc1BzfMpxvA==}
    engines: {node: '>=8'}

  brorand@1.1.0:
    resolution: {integrity: sha512-cKV8tMCEpQs4hK/ik71d6LrPOnpkpGBR0wzxqr68g2m/LB2GxVYQroAjMJZRVM1Y4BCjCKc3vAamxSzOY2RP+w==}

  browser-or-node@3.0.0:
    resolution: {integrity: sha512-iczIdVJzGEYhP5DqQxYM9Hh7Ztpqqi+CXZpSmX8ALFs9ecXkQIeqRyM6TfxEfMVpwhl3dSuDvxdzzo9sUOIVBQ==}

  browser-stdout@1.3.1:
    resolution: {integrity: sha512-qhAVI1+Av2X7qelOfAIYwXONood6XlZE/fXaBSmW/T5SzLAmCgzi+eiWE7fUvbHaeNBQH13UftjpXxsfLkMpgw==}

  buffer-from@1.1.2:
    resolution: {integrity: sha512-E+XQCRwSbaaiChtv6k6Dwgc+bx+Bs6vuKJHHl5kox/BaKbhiXzqQOwK4cO22yElGp2OCmjwVhT3HmxgyPGnJfQ==}

  call-bind-apply-helpers@1.0.2:
    resolution: {integrity: sha512-Sp1ablJ0ivDkSzjcaJdxEunN5/XvksFJ2sMBFfq6x0ryhQV/2b/KwFe21cMpmHtPOSij8K99/wSfoEuTObmuMQ==}
    engines: {node: '>= 0.4'}

  camelcase@6.3.0:
    resolution: {integrity: sha512-Gmy6FhYlCY7uOElZUSbxo2UCDH8owEk996gkbrpsgGtrJLM3J7jGxl9Ic7Qwwj4ivOE5AWZWRMecDdF7hqGjFA==}
    engines: {node: '>=10'}

  chai@5.2.0:
    resolution: {integrity: sha512-mCuXncKXk5iCLhfhwTc0izo0gtEmpz5CtG2y8GiOINBlMVS6v8TMRc5TaLWKS6692m9+dVVfzgeVxR5UxWHTYw==}
    engines: {node: '>=12'}

  chalk@4.1.2:
    resolution: {integrity: sha512-oKnbhFyRIXpUuez8iBMmyEa4nbj4IOQyuhc/wy9kY7/WVPcwIO9VA668Pu8RkO7+0G76SLROeyw9CpQ061i4mA==}
    engines: {node: '>=10'}

  check-error@2.1.1:
    resolution: {integrity: sha512-OAlb+T7V4Op9OwdkjmguYRqncdlx5JiofwOAUkmTF+jNdHwzTaTs4sRAGpzLF3oOz5xAyDGrPgeIDFQmDOTiJw==}
    engines: {node: '>= 16'}

  chokidar@3.5.3:
    resolution: {integrity: sha512-Dr3sfKRP6oTcjf2JmUmFJfeVMvXBdegxB0iVQ5eb2V10uFJUCAS8OByZdVAyVb8xXNz3GjjTgj9kLWsZTqE6kw==}
    engines: {node: '>= 8.10.0'}

  cliui@7.0.4:
    resolution: {integrity: sha512-OcRE68cOsVMXp1Yvonl/fzkQOyjLSu/8bhPDfQt0e0/Eb283TKP20Fs2MqoPsr9SwA595rRCA+QMzYc9nBP+JQ==}

  color-convert@2.0.1:
    resolution: {integrity: sha512-RRECPsj7iu/xb5oKYcsFHSppFNnsj/52OVTRKb4zP5onXwVF3zVmmToNcOfGC+CRDpfK/U584fMg38ZHCaElKQ==}
    engines: {node: '>=7.0.0'}

  color-name@1.1.4:
    resolution: {integrity: sha512-dOy+3AuW3a2wNbZHIuMZpTcgjGuLU/uBL/ubcZF9OXbDo8ff4O8yVp5Bf0efS8uEoYo5q4Fx7dY9OgQGXgAsQA==}

  combined-stream@1.0.8:
    resolution: {integrity: sha512-FQN4MRfuJeHf7cBbBMJFXhKSDq+2kAArBlmRBvcvFE5BB1HZKXtSFASDhdlz9zOYwxh8lDdnvmMOe/+5cdoEdg==}
    engines: {node: '>= 0.8'}

  concat-map@0.0.1:
    resolution: {integrity: sha512-/Srv4dswyQNBfohGpz9o6Yb3Gz3SrUDqBH5rTuhGR7ahtlbYKnVxw2bCFMRljaA7EXHaXZ8wsHdodFvbkhKmqg==}

  create-require@1.1.1:
    resolution: {integrity: sha512-dcKFX3jn0MpIaXjisoRvexIJVEKzaq7z2rZKxf+MSr9TkdmHmsU4m2lcLojrj/FHl8mk5VxMmYA+ftRkP/3oKQ==}

  cssom@0.3.8:
    resolution: {integrity: sha512-b0tGHbfegbhPJpxpiBPU2sCkigAqtM9O121le6bbOlgyV+NyGyCmVfJ6QW9eRjz8CpNfWEOYBIMIGRYkLwsIYg==}

  cssom@0.5.0:
    resolution: {integrity: sha512-iKuQcq+NdHqlAcwUY0o/HL69XQrUaQdMjmStJ8JFmUaiiQErlhrmuigkg/CU4E2J0IyUKUrMAgl36TvN67MqTw==}

  cssstyle@2.3.0:
    resolution: {integrity: sha512-AZL67abkUzIuvcHqk7c09cezpGNcxUxU4Ioi/05xHk4DQeTkWmGYftIE6ctU6AEt+Gn4n1lDStOtj7FKycP71A==}
    engines: {node: '>=8'}

  data-urls@3.0.2:
    resolution: {integrity: sha512-Jy/tj3ldjZJo63sVAvg6LHt2mHvl4V6AgRAmNDtLdm7faqtsx+aJG42rsyCo9JCoRVKwPFzKlIPx3DIibwSIaQ==}
    engines: {node: '>=12'}

  debug@4.3.3:
    resolution: {integrity: sha512-/zxw5+vh1Tfv+4Qn7a5nsbcJKPaSvCDhojn6FEl9vupwK2VCSDtEiEtqr8DFtzYFOdz63LBkxec7DYuc2jon6Q==}
    engines: {node: '>=6.0'}
    peerDependencies:
      supports-color: '*'
    peerDependenciesMeta:
      supports-color:
        optional: true

  debug@4.4.3:
    resolution: {integrity: sha512-RGwwWnwQvkVfavKVt22FGLw+xYSdzARwm0ru6DhTVA3umU5hZc28V3kO4stgYryrTlLpuvgI9GiijltAjNbcqA==}
    engines: {node: '>=6.0'}
    peerDependencies:
      supports-color: '*'
    peerDependenciesMeta:
      supports-color:
        optional: true

  decamelize@4.0.0:
    resolution: {integrity: sha512-9iE1PgSik9HeIIw2JO94IidnE3eBoQrFJ3w7sFuzSX4DpmZ3v5sZpUiV5Swcf6mQEF+Y0ru8Neo+p+nyh2J+hQ==}
    engines: {node: '>=10'}

  decimal.js@10.6.0:
    resolution: {integrity: sha512-YpgQiITW3JXGntzdUmyUR1V812Hn8T1YVXhCu+wO3OpS4eU9l4YdD3qjyiKdV6mvV29zapkMeD390UVEf2lkUg==}

  deep-eql@5.0.2:
    resolution: {integrity: sha512-h5k/5U50IJJFpzfL6nO9jaaumfjO/f2NjK/oYB2Djzm4p9L+3T9qWpZqZ2hAbLPuuYq9wrU08WQyBTL5GbPk5Q==}
    engines: {node: '>=6'}

  delayed-stream@1.0.0:
    resolution: {integrity: sha512-ZySD7Nf91aLB0RxL4KGrKHBXl7Eds1DAmEdcoVawXnLD7SDhpNgtuII2aAkg7a7QS41jxPSZ17p4VdGnMHk3MQ==}
    engines: {node: '>=0.4.0'}

  diff@4.0.2:
    resolution: {integrity: sha512-58lmxKSA4BNyLz+HHMUzlOEpg09FV+ev6ZMe3vJihgdxzgcwZ8VoEEPmALCZG9LmqfVoNMMKpttIYTVG6uDY7A==}
    engines: {node: '>=0.3.1'}

  diff@5.0.0:
    resolution: {integrity: sha512-/VTCrvm5Z0JGty/BWHljh+BAiw3IK+2j87NGMu8Nwc/f48WoDAC395uomO9ZD117ZOBaHmkX1oyLvkVM/aIT3w==}
    engines: {node: '>=0.3.1'}

  domexception@4.0.0:
    resolution: {integrity: sha512-A2is4PLG+eeSfoTMA95/s4pvAoSo2mKtiM5jlHkAVewmiO8ISFTFKZjH7UAM1Atli/OT/7JHOrJRJiMKUZKYBw==}
    engines: {node: '>=12'}
    deprecated: Use your platform's native DOMException instead

  dotenv@16.6.1:
    resolution: {integrity: sha512-uBq4egWHTcTt33a72vpSG0z3HnPuIl6NqYcTrKEg2azoEyl2hpW0zqlxysq2pK9HlDIHyHyakeYaYnSAwd8bow==}
    engines: {node: '>=12'}

  dunder-proto@1.0.1:
    resolution: {integrity: sha512-KIN/nDJBQRcXw0MLVhZE9iQHmG68qAVIBg9CqmUYjmQIhgij9U5MFvrqkUL5FbtyyzZuOeOt0zdeRe4UY7ct+A==}
    engines: {node: '>= 0.4'}

  elliptic@6.6.1:
    resolution: {integrity: sha512-RaddvvMatK2LJHqFJ+YA4WysVN5Ita9E35botqIYspQ4TkRAlCicdzKOjlyv/1Za5RyTNn7di//eEV0uTAfe3g==}

  emoji-regex@8.0.0:
    resolution: {integrity: sha512-MSjYzcWNOA0ewAHpz0MxpYFvwg6yjy1NG3xteoqz644VCo/RPgnr1/GGt+ic3iJTzQ8Eu3TdM14SawnVUmGE6A==}

  entities@6.0.1:
    resolution: {integrity: sha512-aN97NXWF6AWBTahfVOIrB/NShkzi5H7F9r1s9mD3cDj4Ko5f2qhhVoYMibXF7GlLveb/D2ioWay8lxI97Ven3g==}
    engines: {node: '>=0.12'}

  es-define-property@1.0.1:
    resolution: {integrity: sha512-e3nRfgfUZ4rNGL232gUgX06QNyyez04KdjFrF+LTRoOXmrOgFKDg4BCdsjW8EnT69eqdYGmRpJwiPVYNrCaW3g==}
    engines: {node: '>= 0.4'}

  es-errors@1.3.0:
    resolution: {integrity: sha512-Zf5H2Kxt2xjTvbJvP2ZWLEICxA6j+hAmMzIlypy4xcBg1vKVnx89Wy0GbS+kf5cwCVFFzdCFh2XSCFNULS6csw==}
    engines: {node: '>= 0.4'}

  es-object-atoms@1.1.1:
    resolution: {integrity: sha512-FGgH2h8zKNim9ljj7dankFPcICIK9Cp5bm+c2gQSYePhpaG5+esrLODihIorn+Pe6FGJzWhXQotPv73jTaldXA==}
    engines: {node: '>= 0.4'}

  es-set-tostringtag@2.1.0:
    resolution: {integrity: sha512-j6vWzfrGVfyXxge+O0x5sh6cvxAog0a/4Rdd2K36zCMV5eJ+/+tOAngRO8cODMNWbVRdVlmGZQL2YS3yR8bIUA==}
    engines: {node: '>= 0.4'}

  esbuild@0.25.11:
    resolution: {integrity: sha512-KohQwyzrKTQmhXDW1PjCv3Tyspn9n5GcY2RTDqeORIdIJY8yKIF7sTSopFmn/wpMPW4rdPXI0UE5LJLuq3bx0Q==}
    engines: {node: '>=18'}
    hasBin: true

  escalade@3.2.0:
    resolution: {integrity: sha512-WUj2qlxaQtO4g6Pq5c29GTcWGDyd8itL8zTlipgECz3JesAiiOKotd8JU6otB3PACgG6xkJUyVhboMS+bje/jA==}
    engines: {node: '>=6'}

  escape-string-regexp@4.0.0:
    resolution: {integrity: sha512-TtpcNJ3XAzx3Gq8sWRzJaVajRs0uVxA2YAkdb1jm2YkPz4G6egUFAyA3n5vtEIZefPk5Wa4UXbKuS5fKkJWdgA==}
    engines: {node: '>=10'}

  escodegen@2.1.0:
    resolution: {integrity: sha512-2NlIDTwUWJN0mRPQOdtQBzbUHvdGY2P1VXSyU83Q3xKxM7WHX2Ql8dKq782Q9TgQUNOLEzEYu9bzLNj1q88I5w==}
    engines: {node: '>=6.0'}
    hasBin: true

  esm@3.2.25:
    resolution: {integrity: sha512-U1suiZ2oDVWv4zPO56S0NcR5QriEahGtdN2OR6FiOG4WJvcjBVFB0qI4+eKoWFH483PKGuLuu6V8Z4T5g63UVA==}
    engines: {node: '>=6'}

  esprima@4.0.1:
    resolution: {integrity: sha512-eGuFFw7Upda+g4p+QHvnW0RyTX/SVeJBDM/gCtMARO0cLuT2HcEKnTPvhjV6aGeqrCB/sbNop0Kszm0jsaWU4A==}
    engines: {node: '>=4'}
    hasBin: true

  estraverse@5.3.0:
    resolution: {integrity: sha512-MMdARuVEQziNTeJD8DgMqmhwR11BRQ/cBP+pLtYdSTnf3MIO8fFeiINEbX36ZdNlfU/7A9f3gUw49B3oQsvwBA==}
    engines: {node: '>=4.0'}

  esutils@2.0.3:
    resolution: {integrity: sha512-kVscqXk4OCp68SZ0dkgEKVi6/8ij300KBWTJq32P/dYeWTSwK41WyTxalN1eRmA5Z9UU/LX9D7FWSmV9SAYx6g==}
    engines: {node: '>=0.10.0'}

  ethers@5.8.0:
    resolution: {integrity: sha512-DUq+7fHrCg1aPDFCHx6UIPb3nmt2XMpM7Y/g2gLhsl3lIBqeAfOJIl1qEvRf2uq3BiKxmh6Fh5pfp2ieyek7Kg==}

  eventemitter3@5.0.1:
    resolution: {integrity: sha512-GWkBvjiSZK87ELrYOSESUYeVIc9mvLLf/nXalMOS5dYrgZq9o5OVkbZAVM06CVxYsCwH9BDZFPlQTlPA1j4ahA==}

  fill-range@7.1.1:
    resolution: {integrity: sha512-YsGpe3WHLK8ZYi4tWDg2Jy3ebRz2rXowDxnld4bkQB00cc/1Zw9AWnC0i9ztDJitivtQvaI9KaLyKrc+hBW0yg==}
    engines: {node: '>=8'}

  find-up@5.0.0:
    resolution: {integrity: sha512-78/PXT1wlLLDgTzDs7sjq9hzz0vXD+zn+7wypEe4fXQxCmdmqfGsEPQxmiCSQI3ajFV91bVSsvNtrJRiW6nGng==}
    engines: {node: '>=10'}

  flat@5.0.2:
    resolution: {integrity: sha512-b6suED+5/3rTpUBdG1gupIl8MPFCAMA0QXwmljLhvCUKcUvdE4gWky9zpuGCcXHOsz4J9wPGNWq6OKpmIzz3hQ==}
    hasBin: true

  follow-redirects@1.15.11:
    resolution: {integrity: sha512-deG2P0JfjrTxl50XGCDyfI97ZGVCxIpfKYmfyrQ54n5FO/0gfIES8C/Psl6kWVDolizcaaxZJnTS0QSMxvnsBQ==}
    engines: {node: '>=4.0'}
    peerDependencies:
      debug: '*'
    peerDependenciesMeta:
      debug:
        optional: true

  form-data@4.0.4:
    resolution: {integrity: sha512-KrGhL9Q4zjj0kiUt5OO4Mr/A/jlI2jDYs5eHBpYHPcBEVSiipAvn2Ko2HnPe20rmcuuvMHNdZFp+4IlGTMF0Ow==}
    engines: {node: '>= 6'}

  fs.realpath@1.0.0:
    resolution: {integrity: sha512-OO0pH2lK6a0hZnAdau5ItzHPI6pUlvI7jMVnxUQRtw4owF2wk8lOSabtGDCTP4Ggrg2MbGnWO9X8K1t4+fGMDw==}

  fsevents@2.3.3:
    resolution: {integrity: sha512-5xoDfX+fL7faATnagmWPpbFtwh/R77WmMMqqHGS65C3vvB0YHrgF+B1YmZ3441tMj5n63k0212XNoJwzlhffQw==}
    engines: {node: ^8.16.0 || ^10.6.0 || >=11.0.0}
    os: [darwin]

  function-bind@1.1.2:
    resolution: {integrity: sha512-7XHNxH7qX9xG5mIwxkhumTox/MIRNcOgDrxWsMt2pAr23WHp6MrRlN7FBSFpCpr+oVO0F744iUgR82nJMfG2SA==}

  get-caller-file@2.0.5:
    resolution: {integrity: sha512-DyFP3BM/3YHTQOCUL/w0OZHR0lpKeGrxotcHWcqNEdnltqFwXVfhEBQ94eIo34AfQpo0rGki4cyIiftY06h2Fg==}
    engines: {node: 6.* || 8.* || >= 10.*}

  get-intrinsic@1.3.0:
    resolution: {integrity: sha512-9fSjSaos/fRIVIp+xSJlE6lfwhES7LNtKaCBIamHsjr2na1BiABJPo0mOjjz8GJDURarmCPGqaiVg5mfjb98CQ==}
    engines: {node: '>= 0.4'}

  get-proto@1.0.1:
    resolution: {integrity: sha512-sTSfBjoXBp89JvIKIefqw7U2CCebsc74kiY6awiGogKtoSGbgjYE/G/+l9sF3MWFPNc9IcoOC4ODfKHfxFmp0g==}
    engines: {node: '>= 0.4'}

  get-tsconfig@4.13.0:
    resolution: {integrity: sha512-1VKTZJCwBrvbd+Wn3AOgQP/2Av+TfTCOlE4AcRJE72W1ksZXbAx8PPBR9RzgTeSPzlPMHrbANMH3LbltH73wxQ==}

  glob-parent@5.1.2:
    resolution: {integrity: sha512-AOIgSQCepiJYwP3ARnGx+5VnTu2HBYdzbGP45eLw1vr3zB3vZLeyed1sC9hnbcOc9/SrMyM5RPQrkGz4aS9Zow==}
    engines: {node: '>= 6'}

  glob@7.2.0:
    resolution: {integrity: sha512-lmLf6gtyrPq8tTjSmrO94wBeQbFR3HbLHbuyD69wuyQkImp2hWqMGB47OX65FBkPffO641IP9jWa1z4ivqG26Q==}
    deprecated: Glob versions prior to v9 are no longer supported

  gopd@1.2.0:
    resolution: {integrity: sha512-ZUKRh6/kUFoAiTAtTYPZJ3hw9wNxx+BIBOijnlG9PnrJsCcSjs1wyyD6vJpaYtgnzDrKYRSqf3OO6Rfa93xsRg==}
    engines: {node: '>= 0.4'}

  growl@1.10.5:
    resolution: {integrity: sha512-qBr4OuELkhPenW6goKVXiv47US3clb3/IbuWF9KNKEijAy9oeHxU9IgzjvJhHkUzhaj7rOUD7+YGWqUjLp5oSA==}
    engines: {node: '>=4.x'}

  has-flag@4.0.0:
    resolution: {integrity: sha512-EykJT/Q1KjTWctppgIAgfSO0tKVuZUjhgMr17kqTumMl6Afv3EISleU7qZUzoXDFTAHTDC4NOoG/ZxU3EvlMPQ==}
    engines: {node: '>=8'}

  has-symbols@1.1.0:
    resolution: {integrity: sha512-1cDNdwJ2Jaohmb3sg4OmKaMBwuC48sYni5HUw2DvsC8LjGTLK9h+eb1X6RyuOHe4hT0ULCW68iomhjUoKUqlPQ==}
    engines: {node: '>= 0.4'}

  has-tostringtag@1.0.2:
    resolution: {integrity: sha512-NqADB8VjPFLM2V0VvHUewwwsw0ZWBaIdgo+ieHtK3hasLz4qeCRjYcqfB6AQrBggRKppKF8L52/VqdVsO47Dlw==}
    engines: {node: '>= 0.4'}

  hash.js@1.1.7:
    resolution: {integrity: sha512-taOaskGt4z4SOANNseOviYDvjEJinIkRgmp7LbKP2YTTmVxWBl87s/uzK9r+44BclBSp2X7K1hqeNfz9JbBeXA==}

  hasown@2.0.2:
    resolution: {integrity: sha512-0hJU9SCPvmMzIBdZFqNPXWa6dqh7WdH0cII9y+CyS8rG3nL48Bclra9HmKhVVUHyPWNH5Y7xDwAB7bfgSjkUMQ==}
    engines: {node: '>= 0.4'}

  he@1.2.0:
    resolution: {integrity: sha512-F/1DnUGPopORZi0ni+CvrCgHQ5FyEAHRLSApuYWMmrbSwoN2Mn/7k+Gl38gJnR7yyDZk6WLXwiGod1JOWNDKGw==}
    hasBin: true

  hmac-drbg@1.0.1:
    resolution: {integrity: sha512-Tti3gMqLdZfhOQY1Mzf/AanLiqh1WTiJgEj26ZuYQ9fbkLomzGchCws4FyrSd4VkpBfiNhaE1On+lOz894jvXg==}

  html-encoding-sniffer@3.0.0:
    resolution: {integrity: sha512-oWv4T4yJ52iKrufjnyZPkrN0CH3QnrUqdB6In1g5Fe1mia8GmF36gnfNySxoZtxD5+NmYw1EElVXiBk93UeskA==}
    engines: {node: '>=12'}

  http-proxy-agent@5.0.0:
    resolution: {integrity: sha512-n2hY8YdoRE1i7r6M0w9DIw5GgZN0G25P8zLCRQ8rjXtTU3vsNFBI/vWK/UIeE6g5MUUz6avwAPXmL6Fy9D/90w==}
    engines: {node: '>= 6'}

  https-proxy-agent@5.0.1:
    resolution: {integrity: sha512-dFcAjpTQFgoLMzC2VwU+C/CbS7uRL0lWmxDITmqm7C+7F0Odmj6s9l6alZc6AELXhrnggM2CeWSXHGOdX2YtwA==}
    engines: {node: '>= 6'}

  iconv-lite@0.6.3:
    resolution: {integrity: sha512-4fCk79wshMdzMp2rH06qWrJE4iolqLhCUH+OiuIgU++RB0+94NlDL81atO7GX55uUKueo0txHNtvEyI6D7WdMw==}
    engines: {node: '>=0.10.0'}

  inflight@1.0.6:
    resolution: {integrity: sha512-k92I/b08q4wvFscXCLvqfsHCrjrF7yiXsQuIVvVE7N82W3+aqpzuUdBbfhWcy/FZR3/4IgflMgKLOsvPDrGCJA==}
    deprecated: This module is not supported, and leaks memory. Do not use it. Check out lru-cache if you want a good and tested way to coalesce async requests by a key value, which is much more comprehensive and powerful.

  inherits@2.0.3:
    resolution: {integrity: sha512-x00IRNXNy63jwGkJmzPigoySHbaqpNuzKbBOmzK+g2OdZpQ9w+sxCN+VSB3ja7IAge2OP2qpfxTjeNcyjmW1uw==}

  inherits@2.0.4:
    resolution: {integrity: sha512-k/vGaX4/Yla3WzyMCvTQOXYeIHvqOKtnqBduzTHpzpQZzAskKMhZ2K+EnBiSM9zGSoIFeMpXKxa4dYeZIQqewQ==}

  is-binary-path@2.1.0:
    resolution: {integrity: sha512-ZMERYes6pDydyuGidse7OsHxtbI7WVeUEozgR/g7rd0xUimYNlvZRE/K2MgZTjWy725IfelLeVcEM97mmtRGXw==}
    engines: {node: '>=8'}

  is-extglob@2.1.1:
    resolution: {integrity: sha512-SbKbANkN603Vi4jEZv49LeVJMn4yGwsbzZworEoyEiutsN3nJYdbO36zfhGJ6QEDpOZIFkDtnq5JRxmvl3jsoQ==}
    engines: {node: '>=0.10.0'}

  is-fullwidth-code-point@3.0.0:
    resolution: {integrity: sha512-zymm5+u+sCsSWyD9qNaejV3DFvhCKclKdizYaJUuHA83RLjb7nSuGnddCHGv0hk+KY7BMAlsWeK4Ueg6EV6XQg==}
    engines: {node: '>=8'}

  is-glob@4.0.3:
    resolution: {integrity: sha512-xelSayHH36ZgE7ZWhli7pW34hNbNl8Ojv5KVmkJD4hBdD3th8Tfk9vYasLM+mXWOZhFkgZfxhLSnrwRr4elSSg==}
    engines: {node: '>=0.10.0'}

  is-number@7.0.0:
    resolution: {integrity: sha512-41Cifkg6e8TylSpdtTpeLVMqvSBEVzTttHvERD741+pnZ8ANv0004MRL43QKPDlK9cGvNp6NZWZUBlbGXYxxng==}
    engines: {node: '>=0.12.0'}

  is-plain-obj@2.1.0:
    resolution: {integrity: sha512-YWnfyRwxL/+SsrWYfOpUtz5b3YD+nyfkHvjbcanzk8zgyO4ASD67uVMRt8k5bM4lLMDnXfriRhOpemw+NfT1eA==}
    engines: {node: '>=8'}

  is-potential-custom-element-name@1.0.1:
    resolution: {integrity: sha512-bCYeRA2rVibKZd+s2625gGnGF/t7DSqDs4dP7CrLA1m7jKWz6pps0LpYLJN8Q64HtmPKJ1hrN3nzPNKFEKOUiQ==}

  is-unicode-supported@0.1.0:
    resolution: {integrity: sha512-knxG2q4UC3u8stRGyAVJCOdxFmv5DZiRcdlIaAQXAbSfJya+OhopNotLQrstBhququ4ZpuKbDc/8S6mgXgPFPw==}
    engines: {node: '>=10'}

  isexe@2.0.0:
    resolution: {integrity: sha512-RHxMLp9lnKHGHRng9QFhRCMbYAcVpn69smSGcq3f36xjgVVWThj4qqLbTLlq7Ssj8B+fIQ1EuCEGI2lKsyQeIw==}

  isows@1.0.7:
    resolution: {integrity: sha512-I1fSfDCZL5P0v33sVqeTDSpcstAg/N+wF5HS033mogOVIp4B+oHC7oOCsA3axAbBSGTJ8QubbNmnIRN/h8U7hg==}
    peerDependencies:
      ws: '*'

  js-sha3@0.8.0:
    resolution: {integrity: sha512-gF1cRrHhIzNfToc802P800N8PpXS+evLLXfsVpowqmAFR9uwbi89WvXg2QspOmXL8QL86J4T1EpFu+yUkwJY3Q==}

  js-yaml@4.1.0:
    resolution: {integrity: sha512-wpxZs9NoxZaJESJGIZTyDEaYpl0FKSA+FB9aJiyemKhMwkxQg63h4T1KJgUGHpTqPDNRcmmYLugrRjJlBtWvRA==}
    hasBin: true

  jsdom-global@3.0.2:
    resolution: {integrity: sha512-t1KMcBkz/pT5JrvcJbpUR2u/w1kO9jXctaaGJ0vZDzwFnIvGWw9IDSRciT83kIs8Bnw4qpOl8bQK08V01YgMPg==}
    peerDependencies:
      jsdom: '>=10.0.0'

  jsdom@20.0.3:
    resolution: {integrity: sha512-SYhBvTh89tTfCD/CRdSOm13mOBa42iTaTyfyEWBdKcGdPxPtLFBXuHR8XHb33YNYaP+lLbmSvBTsnoesCNJEsQ==}
    engines: {node: '>=14'}
    peerDependencies:
      canvas: ^2.5.0
    peerDependenciesMeta:
      canvas:
        optional: true

  json5@2.2.3:
    resolution: {integrity: sha512-XmOWe7eyHYH14cLdVPoyg+GOH3rYX++KpzrylJwSW98t3Nk+U8XOl8FWKOgwtzdb8lXGf6zYwDUzeHMWfxasyg==}
    engines: {node: '>=6'}
    hasBin: true

  locate-path@6.0.0:
    resolution: {integrity: sha512-iPZK6eYjbxRu3uB4/WZ3EsEIMJFMqAoopl3R+zuq0UjcAm/MO6KCweDgPfP3elTztoKP3KtnVHxTn2NHBSDVUw==}
    engines: {node: '>=10'}

  log-symbols@4.1.0:
    resolution: {integrity: sha512-8XPvpAA8uyhfteu8pIvQxpJZ7SYYdpUivZpGy6sFsBuKRY/7rQGavedeB8aK+Zkyq6upMFVL/9AW6vOYzfRyLg==}
    engines: {node: '>=10'}

  loupe@3.2.1:
    resolution: {integrity: sha512-CdzqowRJCeLU72bHvWqwRBBlLcMEtIvGrlvef74kMnV2AolS9Y8xUv1I0U/MNAWMhBlKIoyuEgoJ0t/bbwHbLQ==}

  make-error@1.3.6:
    resolution: {integrity: sha512-s8UhlNe7vPKomQhC1qFelMokr/Sc3AgNbso3n74mVPA5LTZwkB9NlXf4XPamLxJE8h0gh73rM94xvwRT2CVInw==}

  math-intrinsics@1.1.0:
    resolution: {integrity: sha512-/IXtbwEk5HTPyEwyKX6hGkYXxM9nbj64B+ilVJnC/R6B0pH5G4V3b0pVbL7DBj4tkhBAppbQUlf6F6Xl9LHu1g==}
    engines: {node: '>= 0.4'}

  mime-db@1.52.0:
    resolution: {integrity: sha512-sPU4uV7dYlvtWJxwwxHD0PuihVNiE7TyAbQ5SWxDCB9mUYvOgroQOwYQQOKPJ8CIbE+1ETVlOoK1UC2nU3gYvg==}
    engines: {node: '>= 0.6'}

  mime-types@2.1.35:
    resolution: {integrity: sha512-ZDY+bPm5zTTF+YpCrAU9nK0UgICYPT0QtT1NZWFv4s++TNkcgVaT0g6+4R2uI4MjQjzysHB1zxuWL50hzaeXiw==}
    engines: {node: '>= 0.6'}

  minimalistic-assert@1.0.1:
    resolution: {integrity: sha512-UtJcAD4yEaGtjPezWuO9wC4nwUnVH/8/Im3yEHQP4b67cXlD/Qr9hdITCU1xDbSEXg2XKNaP8jsReV7vQd00/A==}

  minimalistic-crypto-utils@1.0.1:
    resolution: {integrity: sha512-JIYlbt6g8i5jKfJ3xz7rF0LXmv2TkDxBLUkiBeZ7bAx4GnnNMr8xFpGnOxn6GhTEHx3SjRrZEoU+j04prX1ktg==}

  minimatch@3.1.2:
    resolution: {integrity: sha512-J7p63hRiAjw1NDEww1W7i37+ByIrOWO5XQQAzZ3VOcL0PNybwpfmV/N05zFAzwQ9USyEcX6t3UO+K5aqBQOIHw==}

  minimatch@4.2.1:
    resolution: {integrity: sha512-9Uq1ChtSZO+Mxa/CL1eGizn2vRn3MlLgzhT0Iz8zaY8NdvxvB0d5QdPFmCKf7JKA9Lerx5vRrnwO03jsSfGG9g==}
    engines: {node: '>=10'}

  minimist@1.2.8:
    resolution: {integrity: sha512-2yyAR8qBkN3YuheJanUpWC5U3bb5osDywNB8RzDVlDwDHbocAJveqqj1u8+SVD7jkWT4yvsHCpWqqWqAxb0zCA==}

  mocha@9.2.2:
    resolution: {integrity: sha512-L6XC3EdwT6YrIk0yXpavvLkn8h+EU+Y5UcCHKECyMbdUIxyMuZj4bX4U9e1nvnvUUvQVsV2VHQr5zLdcUkhW/g==}
    engines: {node: '>= 12.0.0'}
    hasBin: true

  ms@2.1.2:
    resolution: {integrity: sha512-sGkPx+VjMtmA6MX27oA4FBFELFCZZ4S4XqeGOXCv68tT+jb3vk/RyaKWP0PTKyWtmLSM0b+adUTEvbs1PEaH2w==}

  ms@2.1.3:
    resolution: {integrity: sha512-6FlzubTLZG3J2a/NVCAleEhjzq5oxgHyaCU9yYXvcLsvoVaHJq/s5xXI6/XXP6tz7R9xAOtHnSO/tXtF3WRTlA==}

  nanoid@3.3.1:
    resolution: {integrity: sha512-n6Vs/3KGyxPQd6uO0eH4Bv0ojGSUvuLlIHtC3Y0kEO23YRge8H9x1GCzLn28YX0H66pMkxuaeESFq4tKISKwdw==}
    engines: {node: ^10 || ^12 || ^13.7 || ^14 || >=15.0.1}
    hasBin: true

  normalize-path@3.0.0:
    resolution: {integrity: sha512-6eZs5Ls3WtCisHWp9S2GUy8dqkpGi4BVSz3GaqiE6ezub0512ESztXUwUB6C6IKbQkY2Pnb/mD4WYojCRwcwLA==}
    engines: {node: '>=0.10.0'}

  nwsapi@2.2.22:
    resolution: {integrity: sha512-ujSMe1OWVn55euT1ihwCI1ZcAaAU3nxUiDwfDQldc51ZXaB9m2AyOn6/jh1BLe2t/G8xd6uKG1UBF2aZJeg2SQ==}

  once@1.4.0:
    resolution: {integrity: sha512-lNaJgI+2Q5URQBkccEKHTQOPaXdUxnZZElQTZY0MFUAuaEqe1E+Nyvgdz/aIyNi6Z9MzO5dv1H8n58/GELp3+w==}

  ox@0.9.6:
    resolution: {integrity: sha512-8SuCbHPvv2eZLYXrNmC0EC12rdzXQLdhnOMlHDW2wiCPLxBrOOJwX5L5E61by+UjTPOryqQiRSnjIKCI+GykKg==}
    peerDependencies:
      typescript: '>=5.4.0'
    peerDependenciesMeta:
      typescript:
        optional: true

  p-limit@3.1.0:
    resolution: {integrity: sha512-TYOanM3wGwNGsZN2cVTYPArw454xnXj5qmWF1bEoAc4+cU/ol7GVh7odevjp1FNHduHc3KZMcFduxU5Xc6uJRQ==}
    engines: {node: '>=10'}

  p-locate@5.0.0:
    resolution: {integrity: sha512-LaNjtRWUBY++zB5nE/NwcaoMylSPk+S+ZHNB1TzdbMJMny6dynpAGt7X/tl/QYq3TIeE6nxHppbo2LGymrG5Pw==}
    engines: {node: '>=10'}

  parse5@7.3.0:
    resolution: {integrity: sha512-IInvU7fabl34qmi9gY8XOVxhYyMyuH2xUNpb2q8/Y+7552KlejkRvqvD19nMoUW/uQGGbqNpA6Tufu5FL5BZgw==}

  path-exists@4.0.0:
    resolution: {integrity: sha512-ak9Qy5Q7jYb2Wwcey5Fpvg2KoAc/ZIhLSLOSBmRmygPsGwkVVt0fZa0qrtMz+m6tJTAHfZQ8FnmB4MG4LWy7/w==}
    engines: {node: '>=8'}

  path-is-absolute@1.0.1:
    resolution: {integrity: sha512-AVbw3UJ2e9bq64vSaS9Am0fje1Pa8pbGqTTsmXfaIiMpnr5DlDhfJOuLj9Sf95ZPVDAUerDfEk88MPmPe7UCQg==}
    engines: {node: '>=0.10.0'}

  path@0.12.7:
    resolution: {integrity: sha512-aXXC6s+1w7otVF9UletFkFcDsJeO7lSZBPUQhtb5O0xJe8LtYhj/GxldoL09bBj9+ZmE2hNoHqQSFMN5fikh4Q==}

  pathval@2.0.1:
    resolution: {integrity: sha512-//nshmD55c46FuFw26xV/xFAaB5HF9Xdap7HJBBnrKdAd6/GxDBaNA1870O79+9ueg61cZLSVc+OaFlfmObYVQ==}
    engines: {node: '>= 14.16'}

  picomatch@2.3.1:
    resolution: {integrity: sha512-JU3teHTNjmE2VCGFzuY8EXzCDVwEqB2a8fsIvwaStHhAWJEeVd1o1QD80CU6+ZdEXXSLbSsuLwJjkCBWqRQUVA==}
    engines: {node: '>=8.6'}

  prettier@2.8.8:
    resolution: {integrity: sha512-tdN8qQGvNjw4CHbY+XXk0JgCXn9QiF21a55rBe5LJAU+kDyC4WQn4+awm2Xfk2lQMk5fKup9XgzTZtGkjBdP9Q==}
    engines: {node: '>=10.13.0'}
    hasBin: true

  process@0.11.10:
    resolution: {integrity: sha512-cdGef/drWFoydD1JsMzuFf8100nZl+GT+yacc2bEced5f9Rjk4z+WtFUTBu9PhOi9j/jfmBPu0mMEY4wIdAF8A==}
    engines: {node: '>= 0.6.0'}

  proxy-from-env@1.1.0:
    resolution: {integrity: sha512-D+zkORCbA9f1tdWRK0RaCR3GPv50cMxcrz4X8k5LTSUD1Dkw47mKJEZQNunItRTkWwgtaUSo1RVFRIG9ZXiFYg==}

  psl@1.15.0:
    resolution: {integrity: sha512-JZd3gMVBAVQkSs6HdNZo9Sdo0LNcQeMNP3CozBJb3JYC/QUYZTnKxP+f8oWRX4rHP5EurWxqAHTSwUCjlNKa1w==}

  punycode@2.3.1:
    resolution: {integrity: sha512-vYt7UD1U9Wg6138shLtLOvdAu+8DsC/ilFtEVHcH+wydcSpNE20AfSOduf6MkRFahL5FY7X1oU7nKVZFtfq8Fg==}
    engines: {node: '>=6'}

  querystringify@2.2.0:
    resolution: {integrity: sha512-FIqgj2EUvTa7R50u0rGsyTftzjYmv/a3hO345bZNrqabNqjtgiDMgmo4mkUjd+nzU5oF3dClKqFIPUKybUyqoQ==}

  randombytes@2.1.0:
    resolution: {integrity: sha512-vYl3iOX+4CKUWuxGi9Ukhie6fsqXqS9FE2Zaic4tNFD2N2QQaXOMFbuKK4QmDHC0JO6B1Zp41J0LpT0oR68amQ==}

  readdirp@3.6.0:
    resolution: {integrity: sha512-hOS089on8RduqdbhvQ5Z37A0ESjsqz6qnRcffsMU3495FuTdqSm+7bhJ29JvIOsBDEEnan5DPu9t3To9VRlMzA==}
    engines: {node: '>=8.10.0'}

  require-directory@2.1.1:
    resolution: {integrity: sha512-fGxEI7+wsG9xrvdjsrlmL22OMTTiHRwAMroiEeMgq8gzoLC/PQr7RsRDSTLUg/bZAZtF+TVIkHc6/4RIKrui+Q==}
    engines: {node: '>=0.10.0'}

  requires-port@1.0.0:
    resolution: {integrity: sha512-KigOCHcocU3XODJxsu8i/j8T9tzT4adHiecwORRQ0ZZFcp7ahwXuRU1m+yuO90C5ZUyGeGfocHDI14M3L3yDAQ==}

  resolve-pkg-maps@1.0.0:
    resolution: {integrity: sha512-seS2Tj26TBVOC2NIc2rOe2y2ZO7efxITtLZcGSOnHHNOQ7CkiUBfw0Iw2ck6xkIhPwLhKNLS8BO+hEpngQlqzw==}

  safe-buffer@5.2.1:
    resolution: {integrity: sha512-rp3So07KcdmmKbGvgaNxQSJr7bGVSVk5S9Eq1F+ppbRo70+YeaDxkw5Dd8NPN+GD6bjnYm2VuPuCXmpuYvmCXQ==}

  safer-buffer@2.1.2:
    resolution: {integrity: sha512-YZo3K82SD7Riyi0E1EQPojLz7kpepnSQI9IyPbHHg1XXXevb5dJI7tpyN2ADxGcQbHG7vcyRHk0cbwqcQriUtg==}

  saxes@6.0.0:
    resolution: {integrity: sha512-xAg7SOnEhrm5zI3puOOKyy1OMcMlIJZYNJY7xLBwSze0UjhPLnWfj2GF2EpT0jmzaJKIWKHLsaSSajf35bcYnA==}
    engines: {node: '>=v12.22.7'}

  scrypt-js@3.0.1:
    resolution: {integrity: sha512-cdwTTnqPu0Hyvf5in5asVdZocVDTNRmR7XEcJuIzMjJeSHybHl7vpB66AzwTaIg6CLSbtjcxc8fqcySfnTkccA==}

  serialize-javascript@6.0.0:
    resolution: {integrity: sha512-Qr3TosvguFt8ePWqsvRfrKyQXIiW+nGbYpy8XK24NQHE83caxWt+mIymTT19DGFbNWNLfEwsrkSmN64lVWB9ag==}

  source-map-support@0.5.21:
    resolution: {integrity: sha512-uBHU3L3czsIyYXKX88fdrGovxdSCoTGDRZ6SYXtSRxLZUzHg5P/66Ht6uoUlHu9EZod+inXhKo3qQgwXUT/y1w==}

  source-map@0.6.1:
    resolution: {integrity: sha512-UjgapumWlbMhkBgzT7Ykc5YXUT46F0iKu8SGXq0bcwP5dz/h0Plj6enJqjz1Zbq2l5WaqYnrVbwWOWMyF3F47g==}
    engines: {node: '>=0.10.0'}

  string-width@4.2.3:
    resolution: {integrity: sha512-wKyQRQpjJ0sIp62ErSZdGsjMJWsap5oRNihHhu6G7JVO/9jIB6UyevL+tXuOqrng8j/cxKTWyWUwvSTriiZz/g==}
    engines: {node: '>=8'}

  strip-ansi@6.0.1:
    resolution: {integrity: sha512-Y38VPSHcqkFrCpFnQ9vuSXmquuv5oXOKpGeT6aGrr3o3Gc9AlVa6JBfUSOCnbxGGZF+/0ooI7KrPuUSztUdU5A==}
    engines: {node: '>=8'}

  strip-bom@3.0.0:
    resolution: {integrity: sha512-vavAMRXOgBVNF6nyEEmL3DBK19iRpDcoIwW+swQ+CbGiu7lju6t+JklA1MHweoWtadgt4ISVUsXLyDq34ddcwA==}
    engines: {node: '>=4'}

  strip-json-comments@3.1.1:
    resolution: {integrity: sha512-6fPc+R4ihwqP6N/aIv2f1gMH8lOVtWQHoqC4yK6oSDVVocumAsfCqjkXnqiYMhmMwS/mEHLp7Vehlt3ql6lEig==}
    engines: {node: '>=8'}

  supports-color@7.2.0:
    resolution: {integrity: sha512-qpCAvRl9stuOHveKsn7HncJRvv501qIacKzQlO/+Lwxc9+0q2wLyv4Dfvt80/DPn2pqOBsJdDiogXGR9+OvwRw==}
    engines: {node: '>=8'}

  supports-color@8.1.1:
    resolution: {integrity: sha512-MpUEN2OodtUzxvKQl72cUF7RQ5EiHsGvSsVG0ia9c5RbWGL2CI4C7EpPS8UTBIplnlzZiNuV56w+FuNxy3ty2Q==}
    engines: {node: '>=10'}

  symbol-tree@3.2.4:
    resolution: {integrity: sha512-9QNk5KwDF+Bvz+PyObkmSYjI5ksVUYtjW7AU22r2NKcfLJcXp96hkDWU3+XndOsUb+AQ9QhfzfCT2O+CNWT5Tw==}

  to-regex-range@5.0.1:
    resolution: {integrity: sha512-65P7iz6X5yEr1cwcgvQxbbIw7Uk3gOy5dIdtZ4rDveLqhrdJP+Li/Hx6tyK0NEb+2GCyneCMJiGqrADCSNk8sQ==}
    engines: {node: '>=8.0'}

  tough-cookie@4.1.4:
    resolution: {integrity: sha512-Loo5UUvLD9ScZ6jh8beX1T6sO1w2/MpCRpEP7V280GKMVUQ0Jzar2U3UJPsrdbziLEMMhu3Ujnq//rhiFuIeag==}
    engines: {node: '>=6'}

  tr46@3.0.0:
    resolution: {integrity: sha512-l7FvfAHlcmulp8kr+flpQZmVwtu7nfRV7NZujtN0OqES8EL4O4e0qqzL0DC5gAvx/ZC/9lk6rhcUwYvkBnBnYA==}
    engines: {node: '>=12'}

  ts-node@9.1.1:
    resolution: {integrity: sha512-hPlt7ZACERQGf03M253ytLY3dHbGNGrAq9qIHWUY9XHYl1z7wYngSr3OQ5xmui8o2AaxsONxIzjafLUiWBo1Fg==}
    engines: {node: '>=10.0.0'}
    hasBin: true
    peerDependencies:
      typescript: '>=2.7'

  tsconfig-paths@4.2.0:
    resolution: {integrity: sha512-NoZ4roiN7LnbKn9QqE1amc9DJfzvZXxF4xDavcOWt1BPkdx+m+0gJuPM+S0vCe7zTJMYUP0R8pO2XMr+Y8oLIg==}
    engines: {node: '>=6'}

  tslib@2.8.1:
    resolution: {integrity: sha512-oJFu94HQb+KVduSUQL7wnpmqnfmLsOA/nAh6b6EH0wCEoK0/mPeXU6c3wKDV83MkOuHPRHtSXKKU99IBazS/2w==}

  tsx@4.20.6:
    resolution: {integrity: sha512-ytQKuwgmrrkDTFP4LjR0ToE2nqgy886GpvRSpU0JAnrdBYppuY5rLkRUYPU1yCryb24SsKBTL/hlDQAEFVwtZg==}
    engines: {node: '>=18.0.0'}
    hasBin: true

  typescript@5.9.3:
    resolution: {integrity: sha512-jl1vZzPDinLr9eUt3J/t7V6FgNEw9QjvBPdysz9KfQDD41fQrC2Y4vKQdiaUpFT4bXlb1RHhLpp8wtm6M5TgSw==}
    engines: {node: '>=14.17'}
    hasBin: true

  undici-types@5.26.5:
    resolution: {integrity: sha512-JlCMO+ehdEIKqlFxk6IfVoAUVmgz7cU7zD/h9XZ0qzeosSHmUJVOzSQvvYSYWXkFXC+IfLKSIffhv0sVZup6pA==}

  universalify@0.2.0:
    resolution: {integrity: sha512-CJ1QgKmNg3CwvAv/kOFmtnEN05f0D/cn9QntgNOQlQF9dgvVTHj3t+8JPdjqawCHk7V/KA+fbUqzZ9XWhcqPUg==}
    engines: {node: '>= 4.0.0'}

  url-parse@1.5.10:
    resolution: {integrity: sha512-WypcfiRhfeUP9vvF0j6rw0J3hrWrw6iZv3+22h6iRMJ/8z1Tj6XfLP4DsUix5MhMPnXpiHDoKyoZ/bdCkwBCiQ==}

  util@0.10.4:
    resolution: {integrity: sha512-0Pm9hTQ3se5ll1XihRic3FDIku70C+iHUdT/W926rSgHV5QgXsYbKZN8MSC3tJtSkhuROzvsQjAaFENRXr+19A==}

  viem@2.38.5:
    resolution: {integrity: sha512-EU2olUnWd5kBK1t3BicwaamPHGUANRYetoDLSVzDy7XQ8o8UswItnkQbufe3xTcdRCtb2JYMwjlgHZZ7fUoLdA==}
    peerDependencies:
      typescript: '>=5.0.4'
    peerDependenciesMeta:
      typescript:
        optional: true

  w3c-xmlserializer@4.0.0:
    resolution: {integrity: sha512-d+BFHzbiCx6zGfz0HyQ6Rg69w9k19nviJspaj4yNscGjrHu94sVP+aRm75yEbCh+r2/yR+7q6hux9LVtbuTGBw==}
    engines: {node: '>=14'}

  webidl-conversions@7.0.0:
    resolution: {integrity: sha512-VwddBukDzu71offAQR975unBIGqfKZpM+8ZX6ySk8nYhVoo5CYaZyzt3YBvYtRtO+aoGlqxPg/B87NGVZ/fu6g==}
    engines: {node: '>=12'}

  whatwg-encoding@2.0.0:
    resolution: {integrity: sha512-p41ogyeMUrw3jWclHWTQg1k05DSVXPLcVxRTYsXUk+ZooOCZLcoYgPZ/HL/D/N+uQPOtcp1me1WhBEaX02mhWg==}
    engines: {node: '>=12'}

  whatwg-mimetype@3.0.0:
    resolution: {integrity: sha512-nt+N2dzIutVRxARx1nghPKGv1xHikU7HKdfafKkLNLindmPU/ch3U31NOCGGA/dmPcmb1VlofO0vnKAcsm0o/Q==}
    engines: {node: '>=12'}

  whatwg-url@11.0.0:
    resolution: {integrity: sha512-RKT8HExMpoYx4igMiVMY83lN6UeITKJlBQ+vR/8ZJ8OCdSiN3RwCq+9gH0+Xzj0+5IrM6i4j/6LuvzbZIQgEcQ==}
    engines: {node: '>=12'}

  which@2.0.2:
    resolution: {integrity: sha512-BLI3Tl1TW3Pvl70l3yq3Y64i+awpwXqsGBYWkkqMtnbXgrMD+yj7rhW0kuEDxzJaYXGjEW5ogapKNMEKNMjibA==}
    engines: {node: '>= 8'}
    hasBin: true

  workerpool@6.2.0:
    resolution: {integrity: sha512-Rsk5qQHJ9eowMH28Jwhe8HEbmdYDX4lwoMWshiCXugjtHqMD9ZbiqSDLxcsfdqsETPzVUtX5s1Z5kStiIM6l4A==}

  wrap-ansi@7.0.0:
    resolution: {integrity: sha512-YVGIj2kamLSTxw6NsZjoBxfSwsn0ycdesmc4p+Q21c5zPuZ1pl+NfxVdxPtdHvmNVOQ6XSYG4AUtyt/Fi7D16Q==}
    engines: {node: '>=10'}

  wrappy@1.0.2:
    resolution: {integrity: sha512-l4Sp/DRseor9wL6EvV2+TuQn63dMkPjZ/sp9XkghTEbV9KlPS1xUsZ3u7/IQO4wxtcFB4bgpQPRcR3QCvezPcQ==}

  ws@8.18.0:
    resolution: {integrity: sha512-8VbfWfHLbbwu3+N6OKsOMpBdT4kXPDDB9cJk2bJ6mh9ucxdlnNvH1e+roYkKmN9Nxw2yjz7VzeO9oOz2zJ04Pw==}
    engines: {node: '>=10.0.0'}
    peerDependencies:
      bufferutil: ^4.0.1
      utf-8-validate: '>=5.0.2'
    peerDependenciesMeta:
      bufferutil:
        optional: true
      utf-8-validate:
        optional: true

  ws@8.18.3:
    resolution: {integrity: sha512-PEIGCY5tSlUt50cqyMXfCzX+oOPqN0vuGqWzbcJ2xvnkzkq46oOpz7dQaTDBdfICb4N14+GARUDw2XV2N4tvzg==}
    engines: {node: '>=10.0.0'}
    peerDependencies:
      bufferutil: ^4.0.1
      utf-8-validate: '>=5.0.2'
    peerDependenciesMeta:
      bufferutil:
        optional: true
      utf-8-validate:
        optional: true

  xml-name-validator@4.0.0:
    resolution: {integrity: sha512-ICP2e+jsHvAj2E2lIHxa5tjXRlKDJo4IdvPvCXbXQGdzSfmSpNVyIKMvoZHjDY9DP0zV17iI85o90vRFXNccRw==}
    engines: {node: '>=12'}

  xmlchars@2.2.0:
    resolution: {integrity: sha512-JZnDKK8B0RCDw84FNdDAIpZK+JuJw+s7Lz8nksI7SIuU3UXJJslUthsi+uWBUYOwPFwW7W7PRLRfUKpxjtjFCw==}

  y18n@5.0.8:
    resolution: {integrity: sha512-0pfFzegeDWJHJIAmTLRP2DwHjdF5s7jo9tuztdQxAhINCdvS+3nGINqPd00AphqJR/0LhANUS6/+7SCb98YOfA==}
    engines: {node: '>=10'}

  yargs-parser@20.2.4:
    resolution: {integrity: sha512-WOkpgNhPTlE73h4VFAFsOnomJVaovO8VqLDzy5saChRBFQFBoMYirowyW+Q9HB4HFF4Z7VZTiG3iSzJJA29yRA==}
    engines: {node: '>=10'}

  yargs-unparser@2.0.0:
    resolution: {integrity: sha512-7pRTIA9Qc1caZ0bZ6RYRGbHJthJWuakf+WmHK0rVeLkNrrGhfoabBNdue6kdINI6r4if7ocq9aD/n7xwKOdzOA==}
    engines: {node: '>=10'}

  yargs@16.2.0:
    resolution: {integrity: sha512-D1mvvtDG0L5ft/jGWkLpG1+m0eQxOfaBvTNELraWj22wSVUMWxZUvYgJYcKh6jGGIkJFhH4IZPQhR4TKpc8mBw==}
    engines: {node: '>=10'}

  yn@3.1.1:
    resolution: {integrity: sha512-Ux4ygGWsu2c7isFWe8Yu1YluJmqVhxqK2cLXNQA5AcC3QfbGNpM7fu0Y8b/z16pXLnFxZYvWhd3fhBY9DLmC6Q==}
    engines: {node: '>=6'}

  yocto-queue@0.1.0:
    resolution: {integrity: sha512-rVksvsnNCdJ/ohGc6xgPwyN8eheCxsiLM8mxuE/t/mOVqJewPuO1miLpTHQiRgTKCLexL4MeAFVagts7HmNZ2Q==}
    engines: {node: '>=10'}

snapshots:

  '@adraffy/ens-normalize@1.11.1': {}

  '@esbuild/aix-ppc64@0.25.11':
    optional: true

  '@esbuild/android-arm64@0.25.11':
    optional: true

  '@esbuild/android-arm@0.25.11':
    optional: true

  '@esbuild/android-x64@0.25.11':
    optional: true

  '@esbuild/darwin-arm64@0.25.11':
    optional: true

  '@esbuild/darwin-x64@0.25.11':
    optional: true

  '@esbuild/freebsd-arm64@0.25.11':
    optional: true

  '@esbuild/freebsd-x64@0.25.11':
    optional: true

  '@esbuild/linux-arm64@0.25.11':
    optional: true

  '@esbuild/linux-arm@0.25.11':
    optional: true

  '@esbuild/linux-ia32@0.25.11':
    optional: true

  '@esbuild/linux-loong64@0.25.11':
    optional: true

  '@esbuild/linux-mips64el@0.25.11':
    optional: true

  '@esbuild/linux-ppc64@0.25.11':
    optional: true

  '@esbuild/linux-riscv64@0.25.11':
    optional: true

  '@esbuild/linux-s390x@0.25.11':
    optional: true

  '@esbuild/linux-x64@0.25.11':
    optional: true

  '@esbuild/netbsd-arm64@0.25.11':
    optional: true

  '@esbuild/netbsd-x64@0.25.11':
    optional: true

  '@esbuild/openbsd-arm64@0.25.11':
    optional: true

  '@esbuild/openbsd-x64@0.25.11':
    optional: true

  '@esbuild/openharmony-arm64@0.25.11':
    optional: true

  '@esbuild/sunos-x64@0.25.11':
    optional: true

  '@esbuild/win32-arm64@0.25.11':
    optional: true

  '@esbuild/win32-ia32@0.25.11':
    optional: true

  '@esbuild/win32-x64@0.25.11':
    optional: true

  '@ethersproject/abi@5.8.0':
    dependencies:
      '@ethersproject/address': 5.8.0
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/constants': 5.8.0
      '@ethersproject/hash': 5.8.0
      '@ethersproject/keccak256': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/strings': 5.8.0

  '@ethersproject/abstract-provider@5.8.0':
    dependencies:
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/networks': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/transactions': 5.8.0
      '@ethersproject/web': 5.8.0

  '@ethersproject/abstract-signer@5.8.0':
    dependencies:
      '@ethersproject/abstract-provider': 5.8.0
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/properties': 5.8.0

  '@ethersproject/address@5.8.0':
    dependencies:
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/keccak256': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/rlp': 5.8.0

  '@ethersproject/base64@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0

  '@ethersproject/basex@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/properties': 5.8.0

  '@ethersproject/bignumber@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/logger': 5.8.0
      bn.js: 5.2.2

  '@ethersproject/bytes@5.8.0':
    dependencies:
      '@ethersproject/logger': 5.8.0

  '@ethersproject/constants@5.8.0':
    dependencies:
      '@ethersproject/bignumber': 5.8.0

  '@ethersproject/contracts@5.8.0':
    dependencies:
      '@ethersproject/abi': 5.8.0
      '@ethersproject/abstract-provider': 5.8.0
      '@ethersproject/abstract-signer': 5.8.0
      '@ethersproject/address': 5.8.0
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/constants': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/transactions': 5.8.0

  '@ethersproject/hash@5.8.0':
    dependencies:
      '@ethersproject/abstract-signer': 5.8.0
      '@ethersproject/address': 5.8.0
      '@ethersproject/base64': 5.8.0
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/keccak256': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/strings': 5.8.0

  '@ethersproject/hdnode@5.8.0':
    dependencies:
      '@ethersproject/abstract-signer': 5.8.0
      '@ethersproject/basex': 5.8.0
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/pbkdf2': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/sha2': 5.8.0
      '@ethersproject/signing-key': 5.8.0
      '@ethersproject/strings': 5.8.0
      '@ethersproject/transactions': 5.8.0
      '@ethersproject/wordlists': 5.8.0

  '@ethersproject/json-wallets@5.8.0':
    dependencies:
      '@ethersproject/abstract-signer': 5.8.0
      '@ethersproject/address': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/hdnode': 5.8.0
      '@ethersproject/keccak256': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/pbkdf2': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/random': 5.8.0
      '@ethersproject/strings': 5.8.0
      '@ethersproject/transactions': 5.8.0
      aes-js: 3.0.0
      scrypt-js: 3.0.1

  '@ethersproject/keccak256@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      js-sha3: 0.8.0

  '@ethersproject/logger@5.8.0': {}

  '@ethersproject/networks@5.8.0':
    dependencies:
      '@ethersproject/logger': 5.8.0

  '@ethersproject/pbkdf2@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/sha2': 5.8.0

  '@ethersproject/properties@5.8.0':
    dependencies:
      '@ethersproject/logger': 5.8.0

  '@ethersproject/providers@5.8.0':
    dependencies:
      '@ethersproject/abstract-provider': 5.8.0
      '@ethersproject/abstract-signer': 5.8.0
      '@ethersproject/address': 5.8.0
      '@ethersproject/base64': 5.8.0
      '@ethersproject/basex': 5.8.0
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/constants': 5.8.0
      '@ethersproject/hash': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/networks': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/random': 5.8.0
      '@ethersproject/rlp': 5.8.0
      '@ethersproject/sha2': 5.8.0
      '@ethersproject/strings': 5.8.0
      '@ethersproject/transactions': 5.8.0
      '@ethersproject/web': 5.8.0
      bech32: 1.1.4
      ws: 8.18.0
    transitivePeerDependencies:
      - bufferutil
      - utf-8-validate

  '@ethersproject/random@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/logger': 5.8.0

  '@ethersproject/rlp@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/logger': 5.8.0

  '@ethersproject/sha2@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/logger': 5.8.0
      hash.js: 1.1.7

  '@ethersproject/signing-key@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/properties': 5.8.0
      bn.js: 5.2.2
      elliptic: 6.6.1
      hash.js: 1.1.7

  '@ethersproject/solidity@5.8.0':
    dependencies:
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/keccak256': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/sha2': 5.8.0
      '@ethersproject/strings': 5.8.0

  '@ethersproject/strings@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/constants': 5.8.0
      '@ethersproject/logger': 5.8.0

  '@ethersproject/transactions@5.8.0':
    dependencies:
      '@ethersproject/address': 5.8.0
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/constants': 5.8.0
      '@ethersproject/keccak256': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/rlp': 5.8.0
      '@ethersproject/signing-key': 5.8.0

  '@ethersproject/units@5.8.0':
    dependencies:
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/constants': 5.8.0
      '@ethersproject/logger': 5.8.0

  '@ethersproject/wallet@5.8.0':
    dependencies:
      '@ethersproject/abstract-provider': 5.8.0
      '@ethersproject/abstract-signer': 5.8.0
      '@ethersproject/address': 5.8.0
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/hash': 5.8.0
      '@ethersproject/hdnode': 5.8.0
      '@ethersproject/json-wallets': 5.8.0
      '@ethersproject/keccak256': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/random': 5.8.0
      '@ethersproject/signing-key': 5.8.0
      '@ethersproject/transactions': 5.8.0
      '@ethersproject/wordlists': 5.8.0

  '@ethersproject/web@5.8.0':
    dependencies:
      '@ethersproject/base64': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/strings': 5.8.0

  '@ethersproject/wordlists@5.8.0':
    dependencies:
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/hash': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/strings': 5.8.0

  '@noble/ciphers@1.3.0': {}

  '@noble/curves@1.9.1':
    dependencies:
      '@noble/hashes': 1.8.0

  '@noble/hashes@1.8.0': {}

  '@polymarket/builder-abstract-signer@0.0.1':
    dependencies:
      ethers: 5.8.0
      ts-node: 9.1.1(typescript@5.9.3)
      typescript: 5.9.3
      viem: 2.38.5(typescript@5.9.3)
    transitivePeerDependencies:
      - bufferutil
      - utf-8-validate
      - zod

  '@polymarket/builder-signing-sdk@0.0.8':
    dependencies:
      '@types/node': 18.19.130
      axios: 1.13.0
      tslib: 2.8.1
    transitivePeerDependencies:
      - debug

  '@scure/base@1.2.6': {}

  '@scure/bip32@1.7.0':
    dependencies:
      '@noble/curves': 1.9.1
      '@noble/hashes': 1.8.0
      '@scure/base': 1.2.6

  '@scure/bip39@1.6.0':
    dependencies:
      '@noble/hashes': 1.8.0
      '@scure/base': 1.2.6

  '@tootallnate/once@2.0.0': {}

  '@types/chai@5.2.2':
    dependencies:
      '@types/deep-eql': 4.0.2

  '@types/deep-eql@4.0.2': {}

  '@types/mocha@10.0.10': {}

  '@types/node@18.19.130':
    dependencies:
      undici-types: 5.26.5

  '@types/ws@8.18.1':
    dependencies:
      '@types/node': 18.19.130

  '@ungap/promise-all-settled@1.1.2': {}

  abab@2.0.6: {}

  abitype@1.1.0(typescript@5.9.3):
    optionalDependencies:
      typescript: 5.9.3

  acorn-globals@7.0.1:
    dependencies:
      acorn: 8.15.0
      acorn-walk: 8.3.4

  acorn-walk@8.3.4:
    dependencies:
      acorn: 8.15.0

  acorn@8.15.0: {}

  aes-js@3.0.0: {}

  agent-base@6.0.2:
    dependencies:
      debug: 4.4.3
    transitivePeerDependencies:
      - supports-color

  ansi-colors@4.1.1: {}

  ansi-regex@5.0.1: {}

  ansi-styles@4.3.0:
    dependencies:
      color-convert: 2.0.1

  anymatch@3.1.3:
    dependencies:
      normalize-path: 3.0.0
      picomatch: 2.3.1

  arg@4.1.3: {}

  argparse@2.0.1: {}

  assertion-error@2.0.1: {}

  asynckit@0.4.0: {}

  axios@0.27.2:
    dependencies:
      follow-redirects: 1.15.11
      form-data: 4.0.4
    transitivePeerDependencies:
      - debug

  axios@1.13.0:
    dependencies:
      follow-redirects: 1.15.11
      form-data: 4.0.4
      proxy-from-env: 1.1.0
    transitivePeerDependencies:
      - debug

  balanced-match@1.0.2: {}

  bech32@1.1.4: {}

  binary-extensions@2.3.0: {}

  bn.js@4.12.2: {}

  bn.js@5.2.2: {}

  brace-expansion@1.1.12:
    dependencies:
      balanced-match: 1.0.2
      concat-map: 0.0.1

  braces@3.0.3:
    dependencies:
      fill-range: 7.1.1

  brorand@1.1.0: {}

  browser-or-node@3.0.0: {}

  browser-stdout@1.3.1: {}

  buffer-from@1.1.2: {}

  call-bind-apply-helpers@1.0.2:
    dependencies:
      es-errors: 1.3.0
      function-bind: 1.1.2

  camelcase@6.3.0: {}

  chai@5.2.0:
    dependencies:
      assertion-error: 2.0.1
      check-error: 2.1.1
      deep-eql: 5.0.2
      loupe: 3.2.1
      pathval: 2.0.1

  chalk@4.1.2:
    dependencies:
      ansi-styles: 4.3.0
      supports-color: 7.2.0

  check-error@2.1.1: {}

  chokidar@3.5.3:
    dependencies:
      anymatch: 3.1.3
      braces: 3.0.3
      glob-parent: 5.1.2
      is-binary-path: 2.1.0
      is-glob: 4.0.3
      normalize-path: 3.0.0
      readdirp: 3.6.0
    optionalDependencies:
      fsevents: 2.3.3

  cliui@7.0.4:
    dependencies:
      string-width: 4.2.3
      strip-ansi: 6.0.1
      wrap-ansi: 7.0.0

  color-convert@2.0.1:
    dependencies:
      color-name: 1.1.4

  color-name@1.1.4: {}

  combined-stream@1.0.8:
    dependencies:
      delayed-stream: 1.0.0

  concat-map@0.0.1: {}

  create-require@1.1.1: {}

  cssom@0.3.8: {}

  cssom@0.5.0: {}

  cssstyle@2.3.0:
    dependencies:
      cssom: 0.3.8

  data-urls@3.0.2:
    dependencies:
      abab: 2.0.6
      whatwg-mimetype: 3.0.0
      whatwg-url: 11.0.0

  debug@4.3.3(supports-color@8.1.1):
    dependencies:
      ms: 2.1.2
    optionalDependencies:
      supports-color: 8.1.1

  debug@4.4.3:
    dependencies:
      ms: 2.1.3

  decamelize@4.0.0: {}

  decimal.js@10.6.0: {}

  deep-eql@5.0.2: {}

  delayed-stream@1.0.0: {}

  diff@4.0.2: {}

  diff@5.0.0: {}

  domexception@4.0.0:
    dependencies:
      webidl-conversions: 7.0.0

  dotenv@16.6.1: {}

  dunder-proto@1.0.1:
    dependencies:
      call-bind-apply-helpers: 1.0.2
      es-errors: 1.3.0
      gopd: 1.2.0

  elliptic@6.6.1:
    dependencies:
      bn.js: 4.12.2
      brorand: 1.1.0
      hash.js: 1.1.7
      hmac-drbg: 1.0.1
      inherits: 2.0.4
      minimalistic-assert: 1.0.1
      minimalistic-crypto-utils: 1.0.1

  emoji-regex@8.0.0: {}

  entities@6.0.1: {}

  es-define-property@1.0.1: {}

  es-errors@1.3.0: {}

  es-object-atoms@1.1.1:
    dependencies:
      es-errors: 1.3.0

  es-set-tostringtag@2.1.0:
    dependencies:
      es-errors: 1.3.0
      get-intrinsic: 1.3.0
      has-tostringtag: 1.0.2
      hasown: 2.0.2

  esbuild@0.25.11:
    optionalDependencies:
      '@esbuild/aix-ppc64': 0.25.11
      '@esbuild/android-arm': 0.25.11
      '@esbuild/android-arm64': 0.25.11
      '@esbuild/android-x64': 0.25.11
      '@esbuild/darwin-arm64': 0.25.11
      '@esbuild/darwin-x64': 0.25.11
      '@esbuild/freebsd-arm64': 0.25.11
      '@esbuild/freebsd-x64': 0.25.11
      '@esbuild/linux-arm': 0.25.11
      '@esbuild/linux-arm64': 0.25.11
      '@esbuild/linux-ia32': 0.25.11
      '@esbuild/linux-loong64': 0.25.11
      '@esbuild/linux-mips64el': 0.25.11
      '@esbuild/linux-ppc64': 0.25.11
      '@esbuild/linux-riscv64': 0.25.11
      '@esbuild/linux-s390x': 0.25.11
      '@esbuild/linux-x64': 0.25.11
      '@esbuild/netbsd-arm64': 0.25.11
      '@esbuild/netbsd-x64': 0.25.11
      '@esbuild/openbsd-arm64': 0.25.11
      '@esbuild/openbsd-x64': 0.25.11
      '@esbuild/openharmony-arm64': 0.25.11
      '@esbuild/sunos-x64': 0.25.11
      '@esbuild/win32-arm64': 0.25.11
      '@esbuild/win32-ia32': 0.25.11
      '@esbuild/win32-x64': 0.25.11

  escalade@3.2.0: {}

  escape-string-regexp@4.0.0: {}

  escodegen@2.1.0:
    dependencies:
      esprima: 4.0.1
      estraverse: 5.3.0
      esutils: 2.0.3
    optionalDependencies:
      source-map: 0.6.1

  esm@3.2.25: {}

  esprima@4.0.1: {}

  estraverse@5.3.0: {}

  esutils@2.0.3: {}

  ethers@5.8.0:
    dependencies:
      '@ethersproject/abi': 5.8.0
      '@ethersproject/abstract-provider': 5.8.0
      '@ethersproject/abstract-signer': 5.8.0
      '@ethersproject/address': 5.8.0
      '@ethersproject/base64': 5.8.0
      '@ethersproject/basex': 5.8.0
      '@ethersproject/bignumber': 5.8.0
      '@ethersproject/bytes': 5.8.0
      '@ethersproject/constants': 5.8.0
      '@ethersproject/contracts': 5.8.0
      '@ethersproject/hash': 5.8.0
      '@ethersproject/hdnode': 5.8.0
      '@ethersproject/json-wallets': 5.8.0
      '@ethersproject/keccak256': 5.8.0
      '@ethersproject/logger': 5.8.0
      '@ethersproject/networks': 5.8.0
      '@ethersproject/pbkdf2': 5.8.0
      '@ethersproject/properties': 5.8.0
      '@ethersproject/providers': 5.8.0
      '@ethersproject/random': 5.8.0
      '@ethersproject/rlp': 5.8.0
      '@ethersproject/sha2': 5.8.0
      '@ethersproject/signing-key': 5.8.0
      '@ethersproject/solidity': 5.8.0
      '@ethersproject/strings': 5.8.0
      '@ethersproject/transactions': 5.8.0
      '@ethersproject/units': 5.8.0
      '@ethersproject/wallet': 5.8.0
      '@ethersproject/web': 5.8.0
      '@ethersproject/wordlists': 5.8.0
    transitivePeerDependencies:
      - bufferutil
      - utf-8-validate

  eventemitter3@5.0.1: {}

  fill-range@7.1.1:
    dependencies:
      to-regex-range: 5.0.1

  find-up@5.0.0:
    dependencies:
      locate-path: 6.0.0
      path-exists: 4.0.0

  flat@5.0.2: {}

  follow-redirects@1.15.11: {}

  form-data@4.0.4:
    dependencies:
      asynckit: 0.4.0
      combined-stream: 1.0.8
      es-set-tostringtag: 2.1.0
      hasown: 2.0.2
      mime-types: 2.1.35

  fs.realpath@1.0.0: {}

  fsevents@2.3.3:
    optional: true

  function-bind@1.1.2: {}

  get-caller-file@2.0.5: {}

  get-intrinsic@1.3.0:
    dependencies:
      call-bind-apply-helpers: 1.0.2
      es-define-property: 1.0.1
      es-errors: 1.3.0
      es-object-atoms: 1.1.1
      function-bind: 1.1.2
      get-proto: 1.0.1
      gopd: 1.2.0
      has-symbols: 1.1.0
      hasown: 2.0.2
      math-intrinsics: 1.1.0

  get-proto@1.0.1:
    dependencies:
      dunder-proto: 1.0.1
      es-object-atoms: 1.1.1

  get-tsconfig@4.13.0:
    dependencies:
      resolve-pkg-maps: 1.0.0

  glob-parent@5.1.2:
    dependencies:
      is-glob: 4.0.3

  glob@7.2.0:
    dependencies:
      fs.realpath: 1.0.0
      inflight: 1.0.6
      inherits: 2.0.4
      minimatch: 3.1.2
      once: 1.4.0
      path-is-absolute: 1.0.1

  gopd@1.2.0: {}

  growl@1.10.5: {}

  has-flag@4.0.0: {}

  has-symbols@1.1.0: {}

  has-tostringtag@1.0.2:
    dependencies:
      has-symbols: 1.1.0

  hash.js@1.1.7:
    dependencies:
      inherits: 2.0.4
      minimalistic-assert: 1.0.1

  hasown@2.0.2:
    dependencies:
      function-bind: 1.1.2

  he@1.2.0: {}

  hmac-drbg@1.0.1:
    dependencies:
      hash.js: 1.1.7
      minimalistic-assert: 1.0.1
      minimalistic-crypto-utils: 1.0.1

  html-encoding-sniffer@3.0.0:
    dependencies:
      whatwg-encoding: 2.0.0

  http-proxy-agent@5.0.0:
    dependencies:
      '@tootallnate/once': 2.0.0
      agent-base: 6.0.2
      debug: 4.4.3
    transitivePeerDependencies:
      - supports-color

  https-proxy-agent@5.0.1:
    dependencies:
      agent-base: 6.0.2
      debug: 4.4.3
    transitivePeerDependencies:
      - supports-color

  iconv-lite@0.6.3:
    dependencies:
      safer-buffer: 2.1.2

  inflight@1.0.6:
    dependencies:
      once: 1.4.0
      wrappy: 1.0.2

  inherits@2.0.3: {}

  inherits@2.0.4: {}

  is-binary-path@2.1.0:
    dependencies:
      binary-extensions: 2.3.0

  is-extglob@2.1.1: {}

  is-fullwidth-code-point@3.0.0: {}

  is-glob@4.0.3:
    dependencies:
      is-extglob: 2.1.1

  is-number@7.0.0: {}

  is-plain-obj@2.1.0: {}

  is-potential-custom-element-name@1.0.1: {}

  is-unicode-supported@0.1.0: {}

  isexe@2.0.0: {}

  isows@1.0.7(ws@8.18.3):
    dependencies:
      ws: 8.18.3

  js-sha3@0.8.0: {}

  js-yaml@4.1.0:
    dependencies:
      argparse: 2.0.1

  jsdom-global@3.0.2(jsdom@20.0.3):
    dependencies:
      jsdom: 20.0.3

  jsdom@20.0.3:
    dependencies:
      abab: 2.0.6
      acorn: 8.15.0
      acorn-globals: 7.0.1
      cssom: 0.5.0
      cssstyle: 2.3.0
      data-urls: 3.0.2
      decimal.js: 10.6.0
      domexception: 4.0.0
      escodegen: 2.1.0
      form-data: 4.0.4
      html-encoding-sniffer: 3.0.0
      http-proxy-agent: 5.0.0
      https-proxy-agent: 5.0.1
      is-potential-custom-element-name: 1.0.1
      nwsapi: 2.2.22
      parse5: 7.3.0
      saxes: 6.0.0
      symbol-tree: 3.2.4
      tough-cookie: 4.1.4
      w3c-xmlserializer: 4.0.0
      webidl-conversions: 7.0.0
      whatwg-encoding: 2.0.0
      whatwg-mimetype: 3.0.0
      whatwg-url: 11.0.0
      ws: 8.18.3
      xml-name-validator: 4.0.0
    transitivePeerDependencies:
      - bufferutil
      - supports-color
      - utf-8-validate

  json5@2.2.3: {}

  locate-path@6.0.0:
    dependencies:
      p-locate: 5.0.0

  log-symbols@4.1.0:
    dependencies:
      chalk: 4.1.2
      is-unicode-supported: 0.1.0

  loupe@3.2.1: {}

  make-error@1.3.6: {}

  math-intrinsics@1.1.0: {}

  mime-db@1.52.0: {}

  mime-types@2.1.35:
    dependencies:
      mime-db: 1.52.0

  minimalistic-assert@1.0.1: {}

  minimalistic-crypto-utils@1.0.1: {}

  minimatch@3.1.2:
    dependencies:
      brace-expansion: 1.1.12

  minimatch@4.2.1:
    dependencies:
      brace-expansion: 1.1.12

  minimist@1.2.8: {}

  mocha@9.2.2:
    dependencies:
      '@ungap/promise-all-settled': 1.1.2
      ansi-colors: 4.1.1
      browser-stdout: 1.3.1
      chokidar: 3.5.3
      debug: 4.3.3(supports-color@8.1.1)
      diff: 5.0.0
      escape-string-regexp: 4.0.0
      find-up: 5.0.0
      glob: 7.2.0
      growl: 1.10.5
      he: 1.2.0
      js-yaml: 4.1.0
      log-symbols: 4.1.0
      minimatch: 4.2.1
      ms: 2.1.3
      nanoid: 3.3.1
      serialize-javascript: 6.0.0
      strip-json-comments: 3.1.1
      supports-color: 8.1.1
      which: 2.0.2
      workerpool: 6.2.0
      yargs: 16.2.0
      yargs-parser: 20.2.4
      yargs-unparser: 2.0.0

  ms@2.1.2: {}

  ms@2.1.3: {}

  nanoid@3.3.1: {}

  normalize-path@3.0.0: {}

  nwsapi@2.2.22: {}

  once@1.4.0:
    dependencies:
      wrappy: 1.0.2

  ox@0.9.6(typescript@5.9.3):
    dependencies:
      '@adraffy/ens-normalize': 1.11.1
      '@noble/ciphers': 1.3.0
      '@noble/curves': 1.9.1
      '@noble/hashes': 1.8.0
      '@scure/bip32': 1.7.0
      '@scure/bip39': 1.6.0
      abitype: 1.1.0(typescript@5.9.3)
      eventemitter3: 5.0.1
    optionalDependencies:
      typescript: 5.9.3
    transitivePeerDependencies:
      - zod

  p-limit@3.1.0:
    dependencies:
      yocto-queue: 0.1.0

  p-locate@5.0.0:
    dependencies:
      p-limit: 3.1.0

  parse5@7.3.0:
    dependencies:
      entities: 6.0.1

  path-exists@4.0.0: {}

  path-is-absolute@1.0.1: {}

  path@0.12.7:
    dependencies:
      process: 0.11.10
      util: 0.10.4

  pathval@2.0.1: {}

  picomatch@2.3.1: {}

  prettier@2.8.8: {}

  process@0.11.10: {}

  proxy-from-env@1.1.0: {}

  psl@1.15.0:
    dependencies:
      punycode: 2.3.1

  punycode@2.3.1: {}

  querystringify@2.2.0: {}

  randombytes@2.1.0:
    dependencies:
      safe-buffer: 5.2.1

  readdirp@3.6.0:
    dependencies:
      picomatch: 2.3.1

  require-directory@2.1.1: {}

  requires-port@1.0.0: {}

  resolve-pkg-maps@1.0.0: {}

  safe-buffer@5.2.1: {}

  safer-buffer@2.1.2: {}

  saxes@6.0.0:
    dependencies:
      xmlchars: 2.2.0

  scrypt-js@3.0.1: {}

  serialize-javascript@6.0.0:
    dependencies:
      randombytes: 2.1.0

  source-map-support@0.5.21:
    dependencies:
      buffer-from: 1.1.2
      source-map: 0.6.1

  source-map@0.6.1: {}

  string-width@4.2.3:
    dependencies:
      emoji-regex: 8.0.0
      is-fullwidth-code-point: 3.0.0
      strip-ansi: 6.0.1

  strip-ansi@6.0.1:
    dependencies:
      ansi-regex: 5.0.1

  strip-bom@3.0.0: {}

  strip-json-comments@3.1.1: {}

  supports-color@7.2.0:
    dependencies:
      has-flag: 4.0.0

  supports-color@8.1.1:
    dependencies:
      has-flag: 4.0.0

  symbol-tree@3.2.4: {}

  to-regex-range@5.0.1:
    dependencies:
      is-number: 7.0.0

  tough-cookie@4.1.4:
    dependencies:
      psl: 1.15.0
      punycode: 2.3.1
      universalify: 0.2.0
      url-parse: 1.5.10

  tr46@3.0.0:
    dependencies:
      punycode: 2.3.1

  ts-node@9.1.1(typescript@5.9.3):
    dependencies:
      arg: 4.1.3
      create-require: 1.1.1
      diff: 4.0.2
      make-error: 1.3.6
      source-map-support: 0.5.21
      typescript: 5.9.3
      yn: 3.1.1

  tsconfig-paths@4.2.0:
    dependencies:
      json5: 2.2.3
      minimist: 1.2.8
      strip-bom: 3.0.0

  tslib@2.8.1: {}

  tsx@4.20.6:
    dependencies:
      esbuild: 0.25.11
      get-tsconfig: 4.13.0
    optionalDependencies:
      fsevents: 2.3.3

  typescript@5.9.3: {}

  undici-types@5.26.5: {}

  universalify@0.2.0: {}

  url-parse@1.5.10:
    dependencies:
      querystringify: 2.2.0
      requires-port: 1.0.0

  util@0.10.4:
    dependencies:
      inherits: 2.0.3

  viem@2.38.5(typescript@5.9.3):
    dependencies:
      '@noble/curves': 1.9.1
      '@noble/hashes': 1.8.0
      '@scure/bip32': 1.7.0
      '@scure/bip39': 1.6.0
      abitype: 1.1.0(typescript@5.9.3)
      isows: 1.0.7(ws@8.18.3)
      ox: 0.9.6(typescript@5.9.3)
      ws: 8.18.3
    optionalDependencies:
      typescript: 5.9.3
    transitivePeerDependencies:
      - bufferutil
      - utf-8-validate
      - zod

  w3c-xmlserializer@4.0.0:
    dependencies:
      xml-name-validator: 4.0.0

  webidl-conversions@7.0.0: {}

  whatwg-encoding@2.0.0:
    dependencies:
      iconv-lite: 0.6.3

  whatwg-mimetype@3.0.0: {}

  whatwg-url@11.0.0:
    dependencies:
      tr46: 3.0.0
      webidl-conversions: 7.0.0

  which@2.0.2:
    dependencies:
      isexe: 2.0.0

  workerpool@6.2.0: {}

  wrap-ansi@7.0.0:
    dependencies:
      ansi-styles: 4.3.0
      string-width: 4.2.3
      strip-ansi: 6.0.1

  wrappy@1.0.2: {}

  ws@8.18.0: {}

  ws@8.18.3: {}

  xml-name-validator@4.0.0: {}

  xmlchars@2.2.0: {}

  y18n@5.0.8: {}

  yargs-parser@20.2.4: {}

  yargs-unparser@2.0.0:
    dependencies:
      camelcase: 6.3.0
      decamelize: 4.0.0
      flat: 5.0.2
      is-plain-obj: 2.1.0

  yargs@16.2.0:
    dependencies:
      cliui: 7.0.4
      escalade: 3.2.0
      get-caller-file: 2.0.5
      require-directory: 2.1.1
      string-width: 4.2.3
      y18n: 5.0.8
      yargs-parser: 20.2.4

  yn@3.1.1: {}

  yocto-queue@0.1.0: {}



================================================
FILE: tsconfig.json
================================================
{
    "compileOnSave": false,
    "include": ["src/**/*", "examples/*"],
    "exclude": [
        "src/**/*.test.ts",
        "tests/**",
        "./dist/*",
        "node_modules/",
        "./node_modules/",
        "./node_modules/@types/node/index.d.ts",
    ],
    "compilerOptions": {
        "baseUrl": "./",
        "module": "commonjs",
        "target": "esnext",
        "outDir": "./dist",
        "sourceMap": true,
        "declaration": true,
        "moduleResolution": "node",
        "esModuleInterop": true,
        "experimentalDecorators": true,
        "allowSyntheticDefaultImports": true,
        "importHelpers": true,
        "strict": true,
        "noFallthroughCasesInSwitch": true,
        "noUnusedLocals": true,
        "noImplicitReturns": false,
        "noUnusedParameters": true,
        "resolveJsonModule": true,
        "strictNullChecks": true,
        "typeRoots": ["node_modules/@types"],
        "lib": ["es2018", "dom"],
        "skipLibCheck": true
    }
}


================================================
FILE: tsconfig.production.json
================================================
{
  "extends": "./tsconfig.json",
  "exclude": [
    "examples/*",
    "./dist/*",
    "node_modules/",
    "./node_modules/",
    "./node_modules/@types/node/*",
  ],
  "typeRoots": ["node_modules/@types"],
  "compilerOptions": {
      "sourceMap": false
  }
}



================================================
FILE: .env.example
================================================
RELAYER_URL=
CHAIN_ID=
PK=
RPC_URL=



================================================
FILE: .eslintignore
================================================
# folders
artifacts/
build/
cache/
contracts/
coverage/
dist/
lib/
node_modules/
typechain/

# files
.solcover.js
coverage.json



================================================
FILE: .eslintrc.js
================================================
module.exports = {
  root: true,
  plugins: ["@typescript-eslint", "unused-imports"],
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",
    "prettier",
  ],
  parserOptions: {
      "project": "./tsconfig.json",
      "tsconfigRootDir": __dirname,
      "sourceType": "module"
  },
  rules: {
    "@typescript-eslint/member-ordering": "off",
    "lines-between-class-members": "off",
    "padding-line-between-statements": "off",
    "no-unused-vars": "off",
    "max-len": "off",
    "max-depth": ["error", 3],
    "max-lines-per-function": "off",
    "max-params": "off",
    "@typescript-eslint/no-explicit-any": "off",
    "@typescript-eslint/no-unused-vars": "off",
    "unused-imports/no-unused-imports": "off",
    "unused-imports/no-unused-vars": 0,
  },
};
  


================================================
FILE: .prettierignore
================================================
# folders
artifacts/
build/
cache/
coverage/
dist/
lib/
node_modules/
typechain/

forks/fx-portal

# files
coverage.json



================================================
FILE: .prettierrc
================================================
{
    "arrowParens": "avoid",
    "bracketSpacing": true,
    "endOfLine": "auto",
    "printWidth": 100,
    "singleQuote": false,
    "tabWidth": 4,
    "trailingComma": "all"
}



================================================
FILE: examples/approve.ts
================================================
import { config as dotenvConfig } from "dotenv";
import { resolve } from "path";
import { RelayClient, Transaction } from "../src";
import { encodeFunctionData, prepareEncodeFunctionData, createWalletClient, Hex, http, maxUint256 } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { polygon } from "viem/chains";
import { BuilderApiKeyCreds, BuilderConfig } from "@polymarket/builder-signing-sdk";

dotenvConfig({ path: resolve(__dirname, "../.env") });

const erc20Abi = [
    {
        "constant": false,"inputs": 
        [{"name": "_spender","type": "address"},{"name": "_value","type": "uint256"}],
        "name": "approve",
        "outputs": [{"name": "","type": "bool"}],
        "payable": false,
        "stateMutability": "nonpayable",
        "type": "function"
    }
];

const erc20 = prepareEncodeFunctionData({
    abi: erc20Abi,
    functionName: "approve",
});

function createUsdcApproveTxn(
    token: string,
    spender: string,
): Transaction {
    const calldata = encodeFunctionData({...erc20, args: [spender, maxUint256]});
    return {
        to: token,
        data: calldata,
        value: "0",
    }
}

async function main() {
    console.log(`Starting...`);
    
    const relayerUrl = `${process.env.RELAYER_URL}`;
    const chainId = parseInt(`${process.env.CHAIN_ID}`);

    const pk = privateKeyToAccount(`${process.env.PK}` as Hex);
    const wallet = createWalletClient({account: pk, chain: polygon, transport: http(`${process.env.RPC_URL}`)});

    const builderCreds: BuilderApiKeyCreds = {
        key: `${process.env.BUILDER_API_KEY}`,
        secret: `${process.env.BUILDER_SECRET}`,
        passphrase: `${process.env.BUILDER_PASS_PHRASE}`,
    };
    const builderConfig = new BuilderConfig({
        localBuilderCreds: builderCreds
    });
    const client = new RelayClient(relayerUrl, chainId, wallet, builderConfig);

    const usdc = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";
    const ctf = "0x4d97dcd97ec945f40cf65f87097ace5ea0476045";
    const txn = createUsdcApproveTxn(usdc, ctf);

    const resp = await client.execute([txn, txn], "approve USDC on CTF");
    const t = await resp.wait();
    console.log(t);

}

main();



================================================
FILE: examples/approveProxy.ts
================================================
import { config as dotenvConfig } from "dotenv";
import { resolve } from "path";
import { RelayClient, RelayerTxType, Transaction } from "../src";
import { encodeFunctionData, prepareEncodeFunctionData, createWalletClient, Hex, http, maxUint256 } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { polygon } from "viem/chains";
import { BuilderApiKeyCreds, BuilderConfig } from "@polymarket/builder-signing-sdk";

dotenvConfig({ path: resolve(__dirname, "../.env") });

const erc20Abi = [
    {
        "constant": false,"inputs": 
        [{"name": "_spender","type": "address"},{"name": "_value","type": "uint256"}],
        "name": "approve",
        "outputs": [{"name": "","type": "bool"}],
        "payable": false,
        "stateMutability": "nonpayable",
        "type": "function"
    }
];

const erc20 = prepareEncodeFunctionData({
    abi: erc20Abi,
    functionName: "approve",
});

function createUsdcApproveTxn(
    token: string,
    spender: string,
): Transaction {
    const calldata = encodeFunctionData({...erc20, args: [spender, maxUint256]});
    return {
        to: token,
        data: calldata,
        value: "0",
    }
}

async function main() {
    console.log(`Starting...`);
    
    const relayerUrl = `${process.env.RELAYER_URL}`;
    const chainId = parseInt(`${process.env.CHAIN_ID}`);

    const pk = privateKeyToAccount(`${process.env.PK}` as Hex);
    const wallet = createWalletClient({account: pk, chain: polygon, transport: http(`${process.env.RPC_URL}`)});

    const builderCreds: BuilderApiKeyCreds = {
        key: `${process.env.BUILDER_API_KEY}`,
        secret: `${process.env.BUILDER_SECRET}`,
        passphrase: `${process.env.BUILDER_PASS_PHRASE}`,
    };
    const builderConfig = new BuilderConfig({
        localBuilderCreds: builderCreds
    });
    // Set RelayerTxType to PROXY to create Proxy Transactions
    const client = new RelayClient(relayerUrl, chainId, wallet, builderConfig, RelayerTxType.PROXY);

    const usdc = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";
    const ctf = "0x4d97dcd97ec945f40cf65f87097ace5ea0476045";
    const txn = createUsdcApproveTxn(usdc, ctf);

    const resp = await client.execute([txn, txn], "approve USDC on CTF");
    const t = await resp.wait();
    console.log(t);

}

main();



================================================
FILE: examples/deploy.ts
================================================
import { config as dotenvConfig } from "dotenv";
import { resolve } from "path";
import { RelayClient } from "../src/client";
import { BuilderApiKeyCreds, BuilderConfig } from "@polymarket/builder-signing-sdk";
import { privateKeyToAccount } from "viem/accounts";
import { createWalletClient, Hex, http } from "viem";
import { polygon } from "viem/chains";

dotenvConfig({ path: resolve(__dirname, "../.env") });

async function main() {
    console.log(`Starting...`);
    
    const relayerUrl = `${process.env.RELAYER_URL}`;
    const chainId = parseInt(`${process.env.CHAIN_ID}`);
    const pk = privateKeyToAccount(`${process.env.PK}` as Hex);
    const wallet = createWalletClient({account: pk, chain: polygon, transport: http(`${process.env.RPC_URL}`)});

    const builderCreds: BuilderApiKeyCreds = {
        key: `${process.env.BUILDER_API_KEY}`,
        secret: `${process.env.BUILDER_SECRET}`,
        passphrase: `${process.env.BUILDER_PASS_PHRASE}`,
    };
    
    const builderConfig = new BuilderConfig({
        localBuilderCreds: builderCreds
    });

    const client = new RelayClient(relayerUrl, chainId, wallet, builderConfig);

    const resp = await client.deploy();
    const res = await resp.wait();
    
    console.log(res);

    console.log(`Done!`);

}

main();


================================================
FILE: examples/getTransaction.ts
================================================
import { config as dotenvConfig } from "dotenv";
import { resolve } from "path";
import { RelayClient } from "../src/client";

dotenvConfig({ path: resolve(__dirname, "../.env") });


async function main() {

    console.log(`Starting...`);
    
    const relayerUrl = `${process.env.RELAYER_URL}`;
    const chainId = parseInt(`${process.env.CHAIN_ID}`);
    const client = new RelayClient(relayerUrl, chainId);

    const resp = await client.getTransaction("0191580c-6472-7266-beda-4deaebe46705");
    console.log(resp);

}

main();


================================================
FILE: examples/getTransactions.ts
================================================
import { config as dotenvConfig } from "dotenv";
import { resolve } from "path";
import { RelayClient } from "../src/client";

dotenvConfig({ path: resolve(__dirname, "../.env") });


async function main() {

    console.log(`Starting...`);
    
    const relayerUrl = `${process.env.RELAYER_URL}`;
    const chainId = parseInt(`${process.env.CHAIN_ID}`);

    const client = new RelayClient(relayerUrl, chainId);

    const resp = await client.getTransactions();
    console.log(resp);

}

main();


================================================
FILE: examples/poll.ts
================================================
import { config as dotenvConfig } from "dotenv";
import { resolve } from "path";
import { RelayerTransactionState } from "../src/types";
import { RelayClient } from "../src/client";

dotenvConfig({ path: resolve(__dirname, "../.env") });


async function main() {

    console.log(`Starting...`);
    
    const relayerUrl = `${process.env.RELAYER_URL}`;
    const chainId = parseInt(`${process.env.CHAIN_ID}`);
    const client = new RelayClient(relayerUrl, chainId);

    // const states = [RelayerTransactionState.STATE_EXECUTED.valueOf(), RelayerTransactionState.STATE_CONFIRMED.valueOf()];
    const states = [RelayerTransactionState.STATE_CONFIRMED.valueOf()];
    const resp = await client.pollUntilState("0190e61a-bb93-7c3f-88e2-e29e1c569fb1", states);
    console.log(resp);

}

main();


================================================
FILE: examples/redeem.ts
================================================
import { config as dotenvConfig } from "dotenv";
import { resolve } from "path";
import { RelayClient, Transaction } from "../src";
import { encodeFunctionData, prepareEncodeFunctionData, createWalletClient, Hex, http, zeroHash } from "viem";
import { privateKeyToAccount } from "viem/accounts";
import { polygon } from "viem/chains";
import { BuilderApiKeyCreds, BuilderConfig } from "@polymarket/builder-signing-sdk";

dotenvConfig({ path: resolve(__dirname, "../.env") });


const ctfRedeemAbi = [
    {
        "constant":false,
        "inputs":
        [
            {"name":"collateralToken","type":"address"},
            {"name":"parentCollectionId","type":"bytes32"},
            {"name":"conditionId","type":"bytes32"},
            {"name":"indexSets","type":"uint256[]"}
        ],
        "name":"redeemPositions",
        "outputs":[],
        "payable":false,
        "stateMutability":"nonpayable",
        "type":"function"
    }
];

const nrAdapterRedeemAbi = [
    {
        "inputs":
        [
            {"internalType":"bytes32","name":"_conditionId","type":"bytes32"},
            {"internalType":"uint256[]","name":"_amounts","type":"uint256[]"}
        ],
        "name":"redeemPositions",
        "outputs":[],
        "stateMutability":"nonpayable",
        "type":"function"
    }
];

const ctf = prepareEncodeFunctionData({
    abi: ctfRedeemAbi,
    functionName: "redeemPositions",
});

const nrAdapter = prepareEncodeFunctionData({
    abi: nrAdapterRedeemAbi,
    functionName: "redeemPositions",
});


function createCtfRedeemTxn(
    contract: string,
    conditionId: string,
    collateral: string,
): Transaction {
    const calldata = encodeFunctionData({...ctf, args: [collateral, zeroHash, conditionId, [1, 2]]});
    return {
            to: contract,
            data: calldata,
            value: "0",
    }
}

function createNrAdapterRedeemTxn(
    contract: string,
    conditionId: string,
    redeemAmounts: bigint[],
): Transaction {
    const calldata = encodeFunctionData({...nrAdapter, args: [conditionId, redeemAmounts]});
    return {
            to: contract,
            data: calldata,
            value: "0",
        }
}

async function main() {
    console.log(`Starting...`);
    
    const relayerUrl = `${process.env.RELAYER_URL}`;
    const chainId = parseInt(`${process.env.CHAIN_ID}`);

    const pk = privateKeyToAccount(`${process.env.PK}` as Hex);
    const wallet = createWalletClient({account: pk, chain: polygon, transport: http(`${process.env.RPC_URL}`)});

    const builderCreds: BuilderApiKeyCreds = {
        key: `${process.env.BUILDER_API_KEY}`,
        secret: `${process.env.BUILDER_SECRET}`,
        passphrase: `${process.env.BUILDER_PASS_PHRASE}`,
    };
    const builderConfig = new BuilderConfig({
        localBuilderCreds: builderCreds
    });
    const client = new RelayClient(relayerUrl, chainId, wallet, builderConfig);

    // Set your values here
    const negRisk = false;
    const conditionId = "0x...."; // conditionId to redeem
    
    // amounts to redeem per outcome, only necessary for neg risk
    // Must be an array of length 2 with:
    // the first element being the amount of yes tokens to redeem and
    // the second element being the amount of no tokens to redeem
    const redeemAmounts = [BigInt(111000000), BigInt(0)];

    const usdc = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";
    const ctf = "0x4d97dcd97ec945f40cf65f87097ace5ea0476045";
    const negRiskAdapter = "0xd91E80cF2E7be2e162c6513ceD06f1dD0dA35296";

    const txn = negRisk ? createNrAdapterRedeemTxn(negRiskAdapter, conditionId, redeemAmounts) :
        createCtfRedeemTxn(ctf, conditionId, usdc);
    
    const resp = await client.execute([txn], "redeem");
    const t = await resp.wait();
    console.log(t);

}

main();



================================================
FILE: src/client.ts
================================================
import { Wallet } from "@ethersproject/wallet";
import { JsonRpcSigner } from "@ethersproject/providers";
import { WalletClient, zeroAddress } from "viem";
import { createAbstractSigner, IAbstractSigner } from "@polymarket/builder-abstract-signer";
import {
    GET,
    POST,
    HttpClient,
    RequestOptions,
} from "./http-helpers";
import { 
    CallType,
    GetDeployedResponse,
    NoncePayload,
    OperationType,
    ProxyTransaction,
    ProxyTransactionArgs,
    RelayerTransaction,
    RelayerTransactionResponse,
    RelayerTxType,
    RelayPayload,
    SafeCreateTransactionArgs,
    SafeTransaction,
    SafeTransactionArgs,
    Transaction,
    TransactionType
} from "./types";
import { 
    GET_DEPLOYED,
    GET_NONCE,
    GET_RELAY_PAYLOAD,
    GET_TRANSACTION,
    GET_TRANSACTIONS,
    SUBMIT_TRANSACTION,
} from "./endpoints";
import { 
    buildSafeTransactionRequest,
    buildSafeCreateTransactionRequest,
    buildProxyTransactionRequest,
    deriveSafe,
} from "./builder";
import { sleep } from "./utils";
import { ClientRelayerTransactionResponse } from "./response";
import { ContractConfig, getContractConfig, isProxyContractConfigValid, isSafeContractConfigValid } from "./config";
import { BuilderConfig, BuilderHeaderPayload } from "@polymarket/builder-signing-sdk";
import { CONFIG_UNSUPPORTED_ON_CHAIN, SAFE_DEPLOYED, SAFE_NOT_DEPLOYED, SIGNER_UNAVAILABLE } from "./errors";
import { encodeProxyTransactionData } from "./encode";


export class RelayClient {
    readonly relayerUrl: string;

    readonly chainId: number;

    readonly relayTxType: RelayerTxType;

    readonly contractConfig: ContractConfig;

    readonly httpClient: HttpClient;

    readonly signer?: IAbstractSigner;

    readonly builderConfig?: BuilderConfig;

    constructor(
        relayerUrl: string,
        chainId: number,
        signer?: Wallet | JsonRpcSigner | WalletClient,
        builderConfig?: BuilderConfig,
        relayTxType?: RelayerTxType,
    ) {
        this.relayerUrl = relayerUrl.endsWith("/") ? relayerUrl.slice(0, -1) : relayerUrl;
        this.chainId = chainId;
        if (relayTxType == undefined) {
            relayTxType = RelayerTxType.SAFE;
        }
        this.relayTxType = relayTxType;
        this.contractConfig = getContractConfig(chainId);
        this.httpClient = new HttpClient();
        
        if (signer != undefined) {
            this.signer = createAbstractSigner(chainId, signer);
        }

        if (builderConfig !== undefined) {
            this.builderConfig = builderConfig;
        }
    }

    public async getNonce(signerAddress: string, signerType: string): Promise<NoncePayload> {
        return this.send(
            `${GET_NONCE}`,
            GET,
            {params: { address: signerAddress, type: signerType }},
        );
    }

    public async getRelayPayload(signerAddress: string, signerType: string): Promise<RelayPayload> {
        return this.send(
            `${GET_RELAY_PAYLOAD}`,
            GET,
            {params: { address: signerAddress, type: signerType }}
        );
    }

    public async getTransaction(transactionId: string): Promise<RelayerTransaction[]> {
        return this.send(
            `${GET_TRANSACTION}`,
            GET,
            {params: { id: transactionId }},
        );
    }

    public async getTransactions(): Promise<RelayerTransaction[]> {
        return this.sendAuthedRequest(GET, GET_TRANSACTIONS);
    }

    /**
     * Executes a batch of transactions
     * @param txns 
     * @param metadata 
     * @returns 
     */
    public async execute(txns: Transaction[], metadata?: string): Promise<RelayerTransactionResponse> {
        this.signerNeeded();
        
        if (txns.length == 0) {
            throw new Error("no transactions to execute");
        }

        switch (this.relayTxType) {
            case RelayerTxType.SAFE:
                return this.executeSafeTransactions(
                    txns.map(txn => ({
                        to: txn.to,
                        operation: OperationType.Call,
                        data: txn.data,
                        value: "0",
                    })),
                    metadata
                );
            case RelayerTxType.PROXY:
                return this.executeProxyTransactions(
                    txns.map(txn => ({
                        to: txn.to,
                        typeCode: CallType.Call,
                        data: txn.data,
                        value: "0",
                    })),
                    metadata
                );
            default:
                throw new Error(`Unsupported relay transaction type: ${this.relayTxType}`);
        }
    }

    private async executeProxyTransactions(txns: ProxyTransaction[], metadata?: string): Promise<RelayerTransactionResponse> {
        this.signerNeeded();
        console.log(`Executing proxy transactions...`);
        const start = Date.now();
        const from = await this.signer!.getAddress();
        const rp = await this.getRelayPayload(from, TransactionType.PROXY);
        const args: ProxyTransactionArgs = {
            from: from,
            gasPrice: "0",
            data: encodeProxyTransactionData(txns),
            relay: rp.address,
            nonce: rp.nonce,
        }
        const proxyContractConfig = this.contractConfig.ProxyContracts;
        if (!isProxyContractConfigValid(proxyContractConfig)) {
            throw CONFIG_UNSUPPORTED_ON_CHAIN;
        }

        const request = await buildProxyTransactionRequest(this.signer!, args, proxyContractConfig, metadata);
        console.log(`Client side proxy request creation took: ${(Date.now() - start) / 1000} seconds`);
        
        const requestPayload = JSON.stringify(request);
        
        const resp: RelayerTransactionResponse = await this.sendAuthedRequest(POST, SUBMIT_TRANSACTION, requestPayload)
        return new ClientRelayerTransactionResponse(
            resp.transactionID,
            resp.state,
            resp.transactionHash,
            this,
        );
    }

    private async executeSafeTransactions(txns: SafeTransaction[], metadata?: string): Promise<RelayerTransactionResponse> {
        this.signerNeeded();
        console.log(`Executing safe transactions...`);
        const safe = await this.getExpectedSafe();

        const deployed = await this.getDeployed(safe);
        if (!deployed) {
            throw SAFE_NOT_DEPLOYED;
        }
        
        const start = Date.now();
        const from = await (this.signer as IAbstractSigner).getAddress();

        const noncePayload = await this.getNonce(from, TransactionType.SAFE);

        const args: SafeTransactionArgs = {
            transactions: txns,
            from,
            nonce: noncePayload.nonce,
            chainId: this.chainId,
        }

        const safeContractConfig = this.contractConfig.SafeContracts;
        if (!isSafeContractConfigValid(safeContractConfig)) {
            throw CONFIG_UNSUPPORTED_ON_CHAIN;
        }

        const request = await buildSafeTransactionRequest(
            this.signer as IAbstractSigner,
            args,
            safeContractConfig,
            metadata,
        );

        console.log(`Client side safe request creation took: ${(Date.now() - start) / 1000} seconds`);
        
        const requestPayload = JSON.stringify(request);
        
        const resp: RelayerTransactionResponse = await this.sendAuthedRequest(POST, SUBMIT_TRANSACTION, requestPayload);
        
        return new ClientRelayerTransactionResponse(
            resp.transactionID,
            resp.state,
            resp.transactionHash,
            this,
        );
    }

    /**
     * Deploys a safe 
     * @returns 
     */
    public async deploy(): Promise<RelayerTransactionResponse> {
        this.signerNeeded();
        const safe = await this.getExpectedSafe();

        const deployed = await this.getDeployed(safe);
        if (deployed) {
            throw SAFE_DEPLOYED;
        }
        console.log(`Deploying safe ${safe}...`);
        return this._deploy();
    }

    private async _deploy(): Promise<RelayerTransactionResponse> {
        const start = Date.now();
        const from = await (this.signer as IAbstractSigner).getAddress();
        const args: SafeCreateTransactionArgs = {
            from: from,
            chainId: this.chainId,
            paymentToken: zeroAddress,
            payment: "0",
            paymentReceiver: zeroAddress,
        };
        const safeContractConfig = this.contractConfig.SafeContracts;

        const request = await buildSafeCreateTransactionRequest(
            this.signer as IAbstractSigner,
            safeContractConfig,
            args
        );

        console.log(`Client side deploy request creation took: ${(Date.now() - start) / 1000} seconds`);
        
        const requestPayload = JSON.stringify(request);

        const resp: RelayerTransactionResponse = await this.sendAuthedRequest(POST, SUBMIT_TRANSACTION, requestPayload)
        
        return new ClientRelayerTransactionResponse(
            resp.transactionID,
            resp.state,
            resp.transactionHash,
            this,
        );
    }

    public async getDeployed(safe: string): Promise<boolean> {        
        const resp: GetDeployedResponse = await this.send(
            `${GET_DEPLOYED}`,
            GET,
            {params: { address: safe }},
        );
        return resp.deployed;
    }

    /**
     * Periodically polls the transaction id until it reaches a desired state
     * Returns the relayer transaction if it does each the desired state
     * Returns undefined if the transaction hits the failed state
     * Times out after maxPolls is reached
     * @param transactionId 
     * @param states 
     * @param failState
     * @param maxPolls 
     * @param pollFrequency 
     * @returns 
     */
    public async pollUntilState(transactionId: string, states: string[], failState?: string, maxPolls?: number, pollFrequency?: number): Promise<RelayerTransaction | undefined> {
        console.log(`Waiting for transaction ${transactionId} matching states: ${states}...`)
        const maxPollCount = maxPolls != undefined ? maxPolls : 10;
        let pollFreq = 2000; // Default to polling every 2 seconds
        if (pollFrequency != undefined) {
            if (pollFrequency >= 1000) {
                pollFreq = pollFrequency;
            }
        }
        let pollCount = 0;
        while(pollCount < maxPollCount) {
            const txns = await this.getTransaction(transactionId);
            if(txns.length > 0) {
                const txn = txns[0];
                if(states.includes(txn.state)) {
                    return txn;
                }
                if (failState != undefined && txn.state == failState) {
                    console.error(`txn ${transactionId} failed onchain! Transaction hash: ${txn.transactionHash}`);
                    return undefined;
                }
            }
            pollCount++
            await sleep(pollFreq);
        }
        console.log(`Transaction not found or not in given states, timing out!`);
    }

    private async sendAuthedRequest(
        method: string,
        path: string,
        body?: string
    ): Promise<any> {        
        // builders auth
        if (this.canBuilderAuth()) {
            const builderHeaders = await this._generateBuilderHeaders(method, path, body);
            if (builderHeaders !== undefined) {
                return this.send(
                    path,
                    method, 
                    { headers: builderHeaders, data: body }
                );    
            }
        }

        return this.send(
            path,
            method,
            {data: body}
        );
    }

    private async _generateBuilderHeaders(
        method: string,
        path: string,
        body?: string
    ): Promise<BuilderHeaderPayload | undefined> {
        if (this.builderConfig !== undefined) {
            const builderHeaders = await this.builderConfig.generateBuilderHeaders(
                method,
                path,
                body,
            );
            if (builderHeaders == undefined) {
                return undefined;
            }
            return builderHeaders;
        }

        return undefined;
    }

    private canBuilderAuth(): boolean {
        return (this.builderConfig != undefined && this.builderConfig.isValid());
    }

    private async send(
        endpoint: string,
        method: string,
        options?: RequestOptions
    ): Promise<any> {
        const resp = await this.httpClient.send(`${this.relayerUrl}${endpoint}`, method, options);
        return resp.data;
    }

    private signerNeeded(): void {
        if (this.signer === undefined) {
            throw SIGNER_UNAVAILABLE;
        }
    }

    private async getExpectedSafe(): Promise<string> {
        const address = await (this.signer as IAbstractSigner).getAddress();
        return deriveSafe(address, this.contractConfig.SafeContracts.SafeFactory);
    }
}



================================================
FILE: src/endpoints.ts
================================================
export const GET_NONCE = "/nonce";
export const GET_RELAY_PAYLOAD = "/relay-payload"
export const GET_TRANSACTION = "/transaction"
export const GET_TRANSACTIONS = "/transactions"
export const SUBMIT_TRANSACTION = "/submit";
export const GET_DEPLOYED = "/deployed";



================================================
FILE: src/errors.ts
================================================
export const SIGNER_UNAVAILABLE = new Error("signer is needed to interact with this endpoint!");

export const SAFE_DEPLOYED = new Error("safe already deployed!");

export const SAFE_NOT_DEPLOYED = new Error("safe not deployed!");

export const CONFIG_UNSUPPORTED_ON_CHAIN = new Error("config is not supported on the chainId");



================================================
FILE: src/index.ts
================================================
export * from "./client";
export * from "./encode";
export * from "./types";
export * from "./response";


================================================
FILE: src/types.ts
================================================

export enum RelayerTxType {
    SAFE = "SAFE",
    PROXY = "PROXY"
}

export enum TransactionType {
    SAFE = "SAFE",
    PROXY = "PROXY",
    SAFE_CREATE = "SAFE-CREATE"
}

export interface SignatureParams {
    gasPrice?: string;

    // Proxy RelayHub sig params
    relayerFee?: string;
    // gasPrice: string; // User supplied minimum gas price
    gasLimit?: string; // User supplied gas limit
    relayHub?: string; // Relay Hub Address
    relay?: string; // Relayer address

	// SAFE sig parameters
	operation?: string;
    safeTxnGas?: string;
    baseGas?: string;
    // gasPrice: string;
    gasToken?: string;
    refundReceiver?: string;

	// SAFE CREATE sig parameters
    paymentToken?: string;
    payment?: string;
    paymentReceiver?: string;
}

export interface AddressPayload {
    address: string;
}

export interface NoncePayload {
    nonce: string;
}

export interface RelayPayload {
    address: string;
    nonce: string;
}

export interface TransactionRequest {
    type:               string;
	from:               string;
    to:                 string;
    proxyWallet?:        string;
    data:               string;
    nonce?:              string;
    signature:          string;
    signatureParams:    SignatureParams;
    metadata?:          string;
}

export enum CallType {
    Invalid = "0",
    Call = "1",
    DelegateCall = "2",
}
  
export interface ProxyTransaction {
    to: string;
    typeCode: CallType;
    data: string;
    value: string;
}

// Safe Transactions
export enum OperationType {
    Call, // 0
    DelegateCall, // 1
}  

export interface SafeTransaction {
    to: string;
    operation: OperationType
    data: string;
    value: string;
}

export interface Transaction {
    to: string;
    data: string;
    value: string;
}

export interface SafeTransactionArgs {
    from: string;
    nonce: string;
    chainId: number;
    transactions: SafeTransaction[];
}

export interface SafeCreateTransactionArgs {
    from: string;
    chainId: number;
    paymentToken: string;
    payment: string;
    paymentReceiver: string;
}

export interface ProxyTransactionArgs {
    from: string;
    nonce: string;
    gasPrice: string;
    gasLimit?: string;
    data: string;
    relay: string;
}

export enum RelayerTransactionState {
    STATE_NEW       = "STATE_NEW",
	STATE_EXECUTED  = "STATE_EXECUTED",
    STATE_MINED     = "STATE_MINED",
	STATE_INVALID   = "STATE_INVALID",
	STATE_CONFIRMED = "STATE_CONFIRMED",
	STATE_FAILED    = "STATE_FAILED",
}

export interface RelayerTransaction {
    transactionID: string;
    transactionHash: string;
    from: string;
    to: string;
    proxyAddress: string;
    data: string;
    nonce: string;
    value: string;
    state: string;
    type: string;
    metadata: string;
    createdAt: Date;
    updatedAt: Date;
}


export interface RelayerTransactionResponse {
    transactionID: string;
    state: string;
    hash: string;
    transactionHash: string;
    getTransaction: () => Promise<RelayerTransaction[]>
    wait: () => Promise<RelayerTransaction | undefined>
}


export interface GetDeployedResponse {
    deployed: boolean;
}


================================================
FILE: src/abis/erc20Abi.ts
================================================
export const erc20Abi = [
    {
        "constant": true,
        "inputs": [],
        "name": "name",
        "outputs": [
            {
                "name": "",
                "type": "string"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": false,
        "inputs": [
            {
                "name": "_spender",
                "type": "address"
            },
            {
                "name": "_value",
                "type": "uint256"
            }
        ],
        "name": "approve",
        "outputs": [
            {
                "name": "",
                "type": "bool"
            }
        ],
        "payable": false,
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "totalSupply",
        "outputs": [
            {
                "name": "",
                "type": "uint256"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": false,
        "inputs": [
            {
                "name": "_from",
                "type": "address"
            },
            {
                "name": "_to",
                "type": "address"
            },
            {
                "name": "_value",
                "type": "uint256"
            }
        ],
        "name": "transferFrom",
        "outputs": [
            {
                "name": "",
                "type": "bool"
            }
        ],
        "payable": false,
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "decimals",
        "outputs": [
            {
                "name": "",
                "type": "uint8"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [
            {
                "name": "_owner",
                "type": "address"
            }
        ],
        "name": "balanceOf",
        "outputs": [
            {
                "name": "balance",
                "type": "uint256"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [],
        "name": "symbol",
        "outputs": [
            {
                "name": "",
                "type": "string"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "constant": false,
        "inputs": [
            {
                "name": "_to",
                "type": "address"
            },
            {
                "name": "_value",
                "type": "uint256"
            }
        ],
        "name": "transfer",
        "outputs": [
            {
                "name": "",
                "type": "bool"
            }
        ],
        "payable": false,
        "stateMutability": "nonpayable",
        "type": "function"
    },
    {
        "constant": true,
        "inputs": [
            {
                "name": "_owner",
                "type": "address"
            },
            {
                "name": "_spender",
                "type": "address"
            }
        ],
        "name": "allowance",
        "outputs": [
            {
                "name": "",
                "type": "uint256"
            }
        ],
        "payable": false,
        "stateMutability": "view",
        "type": "function"
    },
    {
        "payable": true,
        "stateMutability": "payable",
        "type": "fallback"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "name": "owner",
                "type": "address"
            },
            {
                "indexed": true,
                "name": "spender",
                "type": "address"
            },
            {
                "indexed": false,
                "name": "value",
                "type": "uint256"
            }
        ],
        "name": "Approval",
        "type": "event"
    },
    {
        "anonymous": false,
        "inputs": [
            {
                "indexed": true,
                "name": "from",
                "type": "address"
            },
            {
                "indexed": true,
                "name": "to",
                "type": "address"
            },
            {
                "indexed": false,
                "name": "value",
                "type": "uint256"
            }
        ],
        "name": "Transfer",
        "type": "event"
    }
];


================================================
FILE: src/abis/index.ts
================================================
export * from "./safeFactory";
export * from "./proxyFactory";
export  * from "./erc20Abi";
export * from "./safe";
export  * from "./multisend";



================================================
FILE: src/abis/multisend.ts
================================================
export const multisendAbi = [
    {
      "constant": false,
      "inputs": [
        {
          "internalType": "bytes",
          "name": "transactions",
          "type": "bytes"
        }
      ],
      "name": "multiSend",
      "outputs": [],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
    }
  ]
  


================================================
FILE: src/abis/proxyFactory.ts
================================================
export const proxyWalletFactory = [
    {
      "constant": false,
      "inputs": [
        {
          "components": [
            {
              "name": "typeCode",
              "type": "uint8"
            },
            {
              "name": "to",
              "type": "address"
            },
            {
              "name": "value",
              "type": "uint256"
            },
            {
              "name": "data",
              "type": "bytes"
            }
          ],
          "name": "calls",
          "type": "tuple[]"
        }
      ],
      "name": "proxy",
      "outputs": [
        {
          "name": "returnValues",
          "type": "bytes[]"
        }
      ],
      "payable": true,
      "stateMutability": "payable",
      "type": "function"
    },
    {
      "constant": false,
      "inputs": [
        {
          "name": "",
          "type": "bytes"
        }
      ],
      "name": "cloneConstructor",
      "outputs": [],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "constant": false,
      "inputs": [],
      "name": "renounceOwnership",
      "outputs": [],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "constant": true,
      "inputs": [],
      "name": "getHubAddr",
      "outputs": [
        {
          "name": "",
          "type": "address"
        }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
    },
    {
      "constant": false,
      "inputs": [
        {
          "name": "context",
          "type": "bytes"
        }
      ],
      "name": "preRelayedCall",
      "outputs": [
        {
          "name": "",
          "type": "bytes32"
        }
      ],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "constant": true,
      "inputs": [
        {
          "name": "",
          "type": "address"
        },
        {
          "name": "",
          "type": "address"
        },
        {
          "name": "",
          "type": "bytes"
        },
        {
          "name": "",
          "type": "uint256"
        },
        {
          "name": "",
          "type": "uint256"
        },
        {
          "name": "",
          "type": "uint256"
        },
        {
          "name": "",
          "type": "uint256"
        },
        {
          "name": "",
          "type": "bytes"
        },
        {
          "name": "",
          "type": "uint256"
        }
      ],
      "name": "acceptRelayedCall",
      "outputs": [
        {
          "name": "doCall",
          "type": "uint256"
        },
        {
          "name": "context",
          "type": "bytes"
        }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
    },
    {
      "constant": true,
      "inputs": [],
      "name": "owner",
      "outputs": [
        {
          "name": "",
          "type": "address"
        }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
    },
    {
      "constant": true,
      "inputs": [],
      "name": "isOwner",
      "outputs": [
        {
          "name": "",
          "type": "bool"
        }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
    },
    {
      "constant": true,
      "inputs": [],
      "name": "getImplementation",
      "outputs": [
        {
          "name": "",
          "type": "address"
        }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
    },
    {
      "constant": true,
      "inputs": [],
      "name": "relayHubVersion",
      "outputs": [
        {
          "name": "",
          "type": "string"
        }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
    },
    {
      "constant": false,
      "inputs": [
        {
          "name": "gsnModule",
          "type": "address"
        }
      ],
      "name": "setGSNModule",
      "outputs": [],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "constant": false,
      "inputs": [
        {
          "name": "context",
          "type": "bytes"
        },
        {
          "name": "success",
          "type": "bool"
        },
        {
          "name": "actualCharge",
          "type": "uint256"
        },
        {
          "name": "preRetVal",
          "type": "bytes32"
        }
      ],
      "name": "postRelayedCall",
      "outputs": [],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "constant": false,
      "inputs": [
        {
          "name": "newOwner",
          "type": "address"
        }
      ],
      "name": "transferOwnership",
      "outputs": [],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "constant": true,
      "inputs": [],
      "name": "getGSNModule",
      "outputs": [
        {
          "name": "",
          "type": "address"
        }
      ],
      "payable": false,
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "payable": false,
      "stateMutability": "nonpayable",
      "type": "constructor"
    },
    {
      "payable": true,
      "stateMutability": "payable",
      "type": "fallback"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "name": "oldRelayHub",
          "type": "address"
        },
        {
          "indexed": true,
          "name": "newRelayHub",
          "type": "address"
        }
      ],
      "name": "RelayHubChanged",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "name": "previousOwner",
          "type": "address"
        },
        {
          "indexed": true,
          "name": "newOwner",
          "type": "address"
        }
      ],
      "name": "OwnershipTransferred",
      "type": "event"
    }
  ];


================================================
FILE: src/abis/safe.ts
================================================
export const safeAbi = [
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "owner",
          "type": "address"
        }
      ],
      "name": "AddedOwner",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "approvedHash",
          "type": "bytes32"
        },
        {
          "indexed": true,
          "internalType": "address",
          "name": "owner",
          "type": "address"
        }
      ],
      "name": "ApproveHash",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "handler",
          "type": "address"
        }
      ],
      "name": "ChangedFallbackHandler",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "guard",
          "type": "address"
        }
      ],
      "name": "ChangedGuard",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "threshold",
          "type": "uint256"
        }
      ],
      "name": "ChangedThreshold",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "module",
          "type": "address"
        }
      ],
      "name": "DisabledModule",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "module",
          "type": "address"
        }
      ],
      "name": "EnabledModule",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "bytes32",
          "name": "txHash",
          "type": "bytes32"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "payment",
          "type": "uint256"
        }
      ],
      "name": "ExecutionFailure",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "address",
          "name": "module",
          "type": "address"
        }
      ],
      "name": "ExecutionFromModuleFailure",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "address",
          "name": "module",
          "type": "address"
        }
      ],
      "name": "ExecutionFromModuleSuccess",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "bytes32",
          "name": "txHash",
          "type": "bytes32"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "payment",
          "type": "uint256"
        }
      ],
      "name": "ExecutionSuccess",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "owner",
          "type": "address"
        }
      ],
      "name": "RemovedOwner",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "module",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "to",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "indexed": false,
          "internalType": "enum Enum.Operation",
          "name": "operation",
          "type": "uint8"
        }
      ],
      "name": "SafeModuleTransaction",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": false,
          "internalType": "address",
          "name": "to",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "indexed": false,
          "internalType": "enum Enum.Operation",
          "name": "operation",
          "type": "uint8"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "safeTxGas",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "baseGas",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "gasPrice",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "gasToken",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address payable",
          "name": "refundReceiver",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "bytes",
          "name": "signatures",
          "type": "bytes"
        },
        {
          "indexed": false,
          "internalType": "bytes",
          "name": "additionalInfo",
          "type": "bytes"
        }
      ],
      "name": "SafeMultiSigTransaction",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "address",
          "name": "sender",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        }
      ],
      "name": "SafeReceived",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "address",
          "name": "initiator",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address[]",
          "name": "owners",
          "type": "address[]"
        },
        {
          "indexed": false,
          "internalType": "uint256",
          "name": "threshold",
          "type": "uint256"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "initializer",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "fallbackHandler",
          "type": "address"
        }
      ],
      "name": "SafeSetup",
      "type": "event"
    },
    {
      "anonymous": false,
      "inputs": [
        {
          "indexed": true,
          "internalType": "bytes32",
          "name": "msgHash",
          "type": "bytes32"
        }
      ],
      "name": "SignMsg",
      "type": "event"
    },
    {
      "stateMutability": "nonpayable",
      "type": "fallback"
    },
    {
      "inputs": [],
      "name": "VERSION",
      "outputs": [
        {
          "internalType": "string",
          "name": "",
          "type": "string"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "owner",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "_threshold",
          "type": "uint256"
        }
      ],
      "name": "addOwnerWithThreshold",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bytes32",
          "name": "hashToApprove",
          "type": "bytes32"
        }
      ],
      "name": "approveHash",
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
        },
        {
          "internalType": "bytes32",
          "name": "",
          "type": "bytes32"
        }
      ],
      "name": "approvedHashes",
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
          "name": "_threshold",
          "type": "uint256"
        }
      ],
      "name": "changeThreshold",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bytes32",
          "name": "dataHash",
          "type": "bytes32"
        },
        {
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "internalType": "bytes",
          "name": "signatures",
          "type": "bytes"
        },
        {
          "internalType": "uint256",
          "name": "requiredSignatures",
          "type": "uint256"
        }
      ],
      "name": "checkNSignatures",
      "outputs": [],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "bytes32",
          "name": "dataHash",
          "type": "bytes32"
        },
        {
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "internalType": "bytes",
          "name": "signatures",
          "type": "bytes"
        }
      ],
      "name": "checkSignatures",
      "outputs": [],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "prevModule",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "module",
          "type": "address"
        }
      ],
      "name": "disableModule",
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
          "internalType": "address",
          "name": "module",
          "type": "address"
        }
      ],
      "name": "enableModule",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "to",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        },
        {
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "internalType": "enum Enum.Operation",
          "name": "operation",
          "type": "uint8"
        },
        {
          "internalType": "uint256",
          "name": "safeTxGas",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "baseGas",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "gasPrice",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "gasToken",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "refundReceiver",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "_nonce",
          "type": "uint256"
        }
      ],
      "name": "encodeTransactionData",
      "outputs": [
        {
          "internalType": "bytes",
          "name": "",
          "type": "bytes"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "to",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        },
        {
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "internalType": "enum Enum.Operation",
          "name": "operation",
          "type": "uint8"
        },
        {
          "internalType": "uint256",
          "name": "safeTxGas",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "baseGas",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "gasPrice",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "gasToken",
          "type": "address"
        },
        {
          "internalType": "address payable",
          "name": "refundReceiver",
          "type": "address"
        },
        {
          "internalType": "bytes",
          "name": "signatures",
          "type": "bytes"
        }
      ],
      "name": "execTransaction",
      "outputs": [
        {
          "internalType": "bool",
          "name": "",
          "type": "bool"
        }
      ],
      "stateMutability": "payable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "to",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        },
        {
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "internalType": "enum Enum.Operation",
          "name": "operation",
          "type": "uint8"
        }
      ],
      "name": "execTransactionFromModule",
      "outputs": [
        {
          "internalType": "bool",
          "name": "success",
          "type": "bool"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "to",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        },
        {
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "internalType": "enum Enum.Operation",
          "name": "operation",
          "type": "uint8"
        }
      ],
      "name": "execTransactionFromModuleReturnData",
      "outputs": [
        {
          "internalType": "bool",
          "name": "success",
          "type": "bool"
        },
        {
          "internalType": "bytes",
          "name": "returnData",
          "type": "bytes"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "getChainId",
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
          "name": "start",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "pageSize",
          "type": "uint256"
        }
      ],
      "name": "getModulesPaginated",
      "outputs": [
        {
          "internalType": "address[]",
          "name": "array",
          "type": "address[]"
        },
        {
          "internalType": "address",
          "name": "next",
          "type": "address"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "getOwners",
      "outputs": [
        {
          "internalType": "address[]",
          "name": "",
          "type": "address[]"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "uint256",
          "name": "offset",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "length",
          "type": "uint256"
        }
      ],
      "name": "getStorageAt",
      "outputs": [
        {
          "internalType": "bytes",
          "name": "",
          "type": "bytes"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "getThreshold",
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
          "name": "to",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        },
        {
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "internalType": "enum Enum.Operation",
          "name": "operation",
          "type": "uint8"
        },
        {
          "internalType": "uint256",
          "name": "safeTxGas",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "baseGas",
          "type": "uint256"
        },
        {
          "internalType": "uint256",
          "name": "gasPrice",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "gasToken",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "refundReceiver",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "_nonce",
          "type": "uint256"
        }
      ],
      "name": "getTransactionHash",
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
          "name": "module",
          "type": "address"
        }
      ],
      "name": "isModuleEnabled",
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
          "name": "owner",
          "type": "address"
        }
      ],
      "name": "isOwner",
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
      "name": "nonce",
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
          "name": "prevOwner",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "owner",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "_threshold",
          "type": "uint256"
        }
      ],
      "name": "removeOwner",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "to",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "value",
          "type": "uint256"
        },
        {
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "internalType": "enum Enum.Operation",
          "name": "operation",
          "type": "uint8"
        }
      ],
      "name": "requiredTxGas",
      "outputs": [
        {
          "internalType": "uint256",
          "name": "",
          "type": "uint256"
        }
      ],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "handler",
          "type": "address"
        }
      ],
      "name": "setFallbackHandler",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "guard",
          "type": "address"
        }
      ],
      "name": "setGuard",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address[]",
          "name": "_owners",
          "type": "address[]"
        },
        {
          "internalType": "uint256",
          "name": "_threshold",
          "type": "uint256"
        },
        {
          "internalType": "address",
          "name": "to",
          "type": "address"
        },
        {
          "internalType": "bytes",
          "name": "data",
          "type": "bytes"
        },
        {
          "internalType": "address",
          "name": "fallbackHandler",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "paymentToken",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "payment",
          "type": "uint256"
        },
        {
          "internalType": "address payable",
          "name": "paymentReceiver",
          "type": "address"
        }
      ],
      "name": "setup",
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
      "name": "signedMessages",
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
          "name": "targetContract",
          "type": "address"
        },
        {
          "internalType": "bytes",
          "name": "calldataPayload",
          "type": "bytes"
        }
      ],
      "name": "simulateAndRevert",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "prevOwner",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "oldOwner",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "newOwner",
          "type": "address"
        }
      ],
      "name": "swapOwner",
      "outputs": [],
      "stateMutability": "nonpayable",
      "type": "function"
    },
    {
      "stateMutability": "payable",
      "type": "receive"
    }
  ];


================================================
FILE: src/abis/safeFactory.ts
================================================
export const safeFactoryAbi = [
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "_masterCopy",
          "type": "address"
        },
        {
          "internalType": "address",
          "name": "_fallbackHandler",
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
          "indexed": false,
          "internalType": "contract GnosisSafe",
          "name": "proxy",
          "type": "address"
        },
        {
          "indexed": false,
          "internalType": "address",
          "name": "owner",
          "type": "address"
        }
      ],
      "name": "ProxyCreation",
      "type": "event"
    },
    {
      "inputs": [],
      "name": "CREATE_PROXY_TYPEHASH",
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
      "name": "DOMAIN_TYPEHASH",
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
      "name": "NAME",
      "outputs": [
        {
          "internalType": "string",
          "name": "",
          "type": "string"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "user",
          "type": "address"
        }
      ],
      "name": "computeProxyAddress",
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
          "name": "paymentToken",
          "type": "address"
        },
        {
          "internalType": "uint256",
          "name": "payment",
          "type": "uint256"
        },
        {
          "internalType": "address payable",
          "name": "paymentReceiver",
          "type": "address"
        },
        {
          "components": [
            {
              "internalType": "uint8",
              "name": "v",
              "type": "uint8"
            },
            {
              "internalType": "bytes32",
              "name": "r",
              "type": "bytes32"
            },
            {
              "internalType": "bytes32",
              "name": "s",
              "type": "bytes32"
            }
          ],
          "internalType": "struct SafeProxyFactory.Sig",
          "name": "createSig",
          "type": "tuple"
        }
      ],
      "name": "createProxy",
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
      "inputs": [],
      "name": "fallbackHandler",
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
      "name": "getContractBytecode",
      "outputs": [
        {
          "internalType": "bytes",
          "name": "",
          "type": "bytes"
        }
      ],
      "stateMutability": "view",
      "type": "function"
    },
    {
      "inputs": [
        {
          "internalType": "address",
          "name": "user",
          "type": "address"
        }
      ],
      "name": "getSalt",
      "outputs": [
        {
          "internalType": "bytes32",
          "name": "",
          "type": "bytes32"
        }
      ],
      "stateMutability": "pure",
      "type": "function"
    },
    {
      "inputs": [],
      "name": "masterCopy",
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
      "name": "proxyCreationCode",
      "outputs": [
        {
          "internalType": "bytes",
          "name": "",
          "type": "bytes"
        }
      ],
      "stateMutability": "pure",
      "type": "function"
    }
  ];


================================================
FILE: src/builder/create.ts
================================================
import { 
    SafeCreateTransactionArgs,
    SignatureParams,
    TransactionRequest,
    TransactionType
} from "../types";
import { SAFE_FACTORY_NAME } from "../constants";
import { deriveSafe } from "./derive";
import { IAbstractSigner } from "@polymarket/builder-abstract-signer";
import { SafeContractConfig } from "../config";
import { Hex } from "viem";


async function createSafeCreateSignature(
    signer: IAbstractSigner,
    safeFactory: string,
    chainId: number,
    paymentToken: string,
    payment: string,
    paymentReceiver: string
): Promise<string> {
    const domain = {
        name: SAFE_FACTORY_NAME,
        chainId: BigInt(chainId),
        verifyingContract: safeFactory as Hex,
    };
    const types = {
        CreateProxy: [
            { name: "paymentToken", type: "address" },
            { name: "payment", type: "uint256" },
            { name: "paymentReceiver", type: "address" },
        ],
    };

    const values = {
        paymentToken,
        payment: BigInt(payment),
        paymentReceiver,
    };
    const sig = await signer.signTypedData(domain, types, values, "CreateProxy");

    console.log(`Sig: ${sig}`);
    return sig;
}


export async function buildSafeCreateTransactionRequest(
    signer: IAbstractSigner,
    safeContractConfig: SafeContractConfig,
    args: SafeCreateTransactionArgs,
) :Promise<TransactionRequest> {
    const safeFactory = safeContractConfig.SafeFactory;
    const sig = await createSafeCreateSignature(
        signer,
        safeFactory,
        args.chainId,
        args.paymentToken,
        args.payment,
        args.paymentReceiver
    );

    const sigParams: SignatureParams = {
        paymentToken: args.paymentToken,
        payment: args.payment,
        paymentReceiver: args.paymentReceiver,
    };

    const safeAddress = deriveSafe(args.from, safeFactory);

    const request: TransactionRequest = {
        from: args.from,
        to: safeFactory,
        // Note: obviously the safe here does not exist yet but useful to have this data in the db
        proxyWallet: safeAddress, 
        data: "0x",
        signature: sig,
        signatureParams: sigParams,
        type: TransactionType.SAFE_CREATE,
    };

    console.log(`Created a SAFE-CREATE Transaction:`);
    console.log(request);
    return request;
}


================================================
FILE: src/builder/derive.ts
================================================
import { keccak256, getCreate2Address, encodePacked, Hex, encodeAbiParameters } from 'viem'
import { SAFE_INIT_CODE_HASH, PROXY_INIT_CODE_HASH } from "../constants";

export const deriveProxyWallet = (address: string, proxyFactory: string): string => {
    return getCreate2Address({
        bytecodeHash: PROXY_INIT_CODE_HASH as Hex,
        from: proxyFactory as Hex,
        salt: keccak256(encodePacked(["address"], [address as Hex]))}
    );
}

export const deriveSafe = (address: string, safeFactory: string) : string => {
    return getCreate2Address({
        bytecodeHash: SAFE_INIT_CODE_HASH as Hex,
        from: safeFactory as Hex,
        salt: keccak256(encodeAbiParameters([{ name: 'address', type: 'address' }], [address as Hex]))}
    );
}



================================================
FILE: src/builder/index.ts
================================================
export * from "./safe";
export * from "./create";
export * from "./derive";
export * from "./proxy";



================================================
FILE: src/builder/proxy.ts
================================================
import { concat, Hex, keccak256, toHex } from "viem";
import { IAbstractSigner } from "@polymarket/builder-abstract-signer";

import {
    ProxyTransactionArgs,
    SignatureParams,
    TransactionRequest,
    TransactionType
} from "../types";
import { deriveProxyWallet } from "./derive";
import { ProxyContractConfig } from "../config";

const DEFAULT_GAS_LIMIT = BigInt(10_000_000);

function createStructHash(
    from: string,
    to: string,
    data: string,
    txFee: string,
    gasPrice: string,
    gasLimit: string,
    nonce: string,
    relayHubAddress: string,
    relayAddress: string,
  ): Hex {
    const relayHubPrefix = toHex("rlx:");
    const encodedFrom = from as Hex;
    const encodedTo = to as Hex;
    const encodedData = data as Hex;
    const encodedTxFee = toHex(BigInt(txFee), { size: 32 });
    const encodedGasPrice = toHex(BigInt(gasPrice), { size: 32 });
    const encodedGasLimit = toHex(BigInt(gasLimit), { size: 32 });
    const encodedNonce = toHex(BigInt(nonce), { size: 32 });
    const encodedRelayHubAddress = relayHubAddress as Hex;
    const encodedRelayAddress = relayAddress as Hex;

    const dataToHash = concat([
        relayHubPrefix,
        encodedFrom,
        encodedTo,
        encodedData,
        encodedTxFee,
        encodedGasPrice,
        encodedGasLimit,
        encodedNonce,
        encodedRelayHubAddress,
        encodedRelayAddress,
    ]);
    return keccak256(dataToHash);
}

async function createProxySignature(
    signer: IAbstractSigner,
    structHash: string,
): Promise<string> {
    return signer.signMessage(structHash);
}

export async function buildProxyTransactionRequest(
    signer: IAbstractSigner,
    args: ProxyTransactionArgs,
    proxyContractConfig: ProxyContractConfig,
    metadata?: string,
) :Promise<TransactionRequest> {
    const proxyWalletFactory = proxyContractConfig.ProxyFactory;
    const to = proxyWalletFactory;
    const proxy = deriveProxyWallet(args.from, proxyWalletFactory);
    const relayerFee = "0";
    const relayHub = proxyContractConfig.RelayHub;
    const gasLimitStr = await getGasLimit(signer, to, args);

    const sigParams: SignatureParams = {
        gasPrice: args.gasPrice,
        gasLimit: gasLimitStr,
        relayerFee: relayerFee,
        relayHub: relayHub,
        relay: args.relay,
    };

    const txHash = createStructHash(
        args.from,
        to,
        args.data,
        relayerFee,
        args.gasPrice,
        gasLimitStr,
        args.nonce,
        relayHub,
        args.relay
    );

    const sig = await createProxySignature(signer, txHash);

    if(metadata == undefined){
        metadata = "";
    }

    const req = {
        from: args.from,
        to: to,
        proxyWallet: proxy,
        data: args.data,
        nonce: args.nonce,
        signature: sig,
        signatureParams: sigParams,
        type: TransactionType.PROXY,
        metadata: metadata,
    };

    console.log(`Created Proxy Transaction Request:`);
    console.log(req);
    return req;
}

async function getGasLimit(signer: IAbstractSigner, to: string, args: ProxyTransactionArgs): Promise<string> {
    if (args.gasLimit && args.gasLimit !== "0") {
        return args.gasLimit;
    }

    let gasLimitBigInt: bigint;
    try {
        gasLimitBigInt = await signer.estimateGas({
            from: args.from,
            to: to,
            data: args.data,
        }
        );
    }  catch (e) {
        console.log("Error estimating gas for proxy transaction, using default gas limit:", e);
        gasLimitBigInt = DEFAULT_GAS_LIMIT;
    }
    return gasLimitBigInt.toString();
}


================================================
FILE: src/builder/safe.ts
================================================

import { IAbstractSigner } from "@polymarket/builder-abstract-signer";
import { hashTypedData, Hex, zeroAddress } from "viem";

import { 
    OperationType,
    SafeTransaction,
    SafeTransactionArgs,
    SignatureParams,
    TransactionRequest,
    TransactionType 
} from "../types";
import { deriveSafe } from "./derive";
import { createSafeMultisendTransaction } from "../encode/safe";
import { SafeContractConfig } from "../config";
import { splitAndPackSig } from "../utils";


async function createSafeSignature(signer: IAbstractSigner, structHash: string) : Promise<string> {
    return signer.signMessage(structHash);
}

function createStructHash(
    chainId: number,
    safe: string,
    to: string,
    value: string,
    data: string,
    operation: OperationType,
    safeTxGas: string,
    baseGas: string,
    gasPrice: string,
    gasToken: string,
    refundReceiver: string,
    nonce: string
) : string {
    const domain = {
        chainId: chainId,
        verifyingContract: safe as Hex,
    };

    const types = {
        // keccak256(
        //     "SafeTx(address to,uint256 value,bytes data,uint8 operation,uint256 safeTxGas,uint256 baseGas,uint256 gasPrice,address gasToken,address refundReceiver,uint256 nonce)"
        // );
        SafeTx: [
            { name: 'to', type: 'address' },
            { name: 'value', type: 'uint256' },
            { name: 'data', type: 'bytes' },
            { name: 'operation', type: 'uint8' },
            { name: 'safeTxGas', type: 'uint256' },
            { name: 'baseGas', type: 'uint256' },
            { name: 'gasPrice', type: 'uint256' },
            { name: 'gasToken', type: 'address' },
            { name: 'refundReceiver', type: 'address' },
            { name: 'nonce', type: 'uint256' },
        ],
    };
    const values = {
        to: to,
        value: value,
        data: data,
        operation: operation,
        safeTxGas: safeTxGas,
        baseGas: baseGas,
        gasPrice: gasPrice,
        gasToken: gasToken,
        refundReceiver: refundReceiver,
        nonce: nonce,
    };

    // // viem hashTypedData
    // const structHash = _TypedDataEncoder.hash(domain, types, values);

    const structHash = hashTypedData({primaryType: "SafeTx", domain: domain, types: types, message: values});
    return structHash;
}

export function aggregateTransaction(txns: SafeTransaction[], safeMultisend: string): SafeTransaction {
    let transaction: SafeTransaction;
    if(txns.length == 1) {
        transaction = txns[0];
    } else {
        transaction = createSafeMultisendTransaction(txns, safeMultisend);
    }
    return transaction;
}

export async function buildSafeTransactionRequest(
    signer: IAbstractSigner,
    args: SafeTransactionArgs,
    safeContractConfig: SafeContractConfig,
    metadata?: string,
) :Promise<TransactionRequest> {
    const safeFactory = safeContractConfig.SafeFactory;
    const safeMultisend = safeContractConfig.SafeMultisend;
    const transaction = aggregateTransaction(args.transactions, safeMultisend);
    const safeTxnGas = "0";
    const baseGas = "0";
    const gasPrice = "0";
    const gasToken = zeroAddress;
    const refundReceiver = zeroAddress;

    const safeAddress = deriveSafe(args.from, safeFactory);

    // Generate the struct hash
    const structHash = createStructHash(
        args.chainId,
        safeAddress,
        transaction.to,
        transaction.value,
        transaction.data,
        transaction.operation,
        safeTxnGas,
        baseGas,
        gasPrice,
        gasToken,
        refundReceiver,
        args.nonce,
    );

    const sig = await createSafeSignature(signer, structHash);

    // Split the sig then pack it into Gnosis accepted rsv format
    const packedSig = splitAndPackSig(sig)

    const sigParams: SignatureParams = {
        gasPrice,
        operation: `${transaction.operation}`,
        safeTxnGas,
        baseGas,
        gasToken,
        refundReceiver,
    }

    if(metadata == undefined){
        metadata = "";
    }

    const req =  {
        from: args.from,
        to: transaction.to,
        proxyWallet: safeAddress,
        data: transaction.data,
        nonce: args.nonce,
        signature: packedSig,
        signatureParams: sigParams,
        type: TransactionType.SAFE,
        metadata: metadata,
    }

    console.log(`Created Safe Transaction Request: `);
    console.log(req);
    return req;
}



================================================
FILE: src/config/index.ts
================================================
export interface ProxyContractConfig {
    RelayHub: string;
    ProxyFactory: string;
}

export interface SafeContractConfig {
    SafeFactory: string;
    SafeMultisend: string;
}

export interface ContractConfig {
    ProxyContracts: ProxyContractConfig;
    SafeContracts: SafeContractConfig;
};

const AMOY: ContractConfig = {
    ProxyContracts: {
        // Proxy factory unsupported on Amoy testnet
        RelayHub: "",
        ProxyFactory: "",
    },
    SafeContracts: {
        SafeFactory: "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b",
        SafeMultisend: "0xA238CBeb142c10Ef7Ad8442C6D1f9E89e07e7761",
    }
};

const POL: ContractConfig = {
    ProxyContracts: {
        ProxyFactory: "0xaB45c5A4B0c941a2F231C04C3f49182e1A254052",
        RelayHub: "0xD216153c06E857cD7f72665E0aF1d7D82172F494"
    },
    SafeContracts: {
        SafeFactory: "0xaacFeEa03eb1561C4e67d661e40682Bd20E3541b",
        SafeMultisend: "0xA238CBeb142c10Ef7Ad8442C6D1f9E89e07e7761",
    }
};

export function isProxyContractConfigValid(
    config: ProxyContractConfig
): boolean {
    return !!config.RelayHub && !!config.ProxyFactory;
}

export function isSafeContractConfigValid(
    config: SafeContractConfig
): boolean {
    return !!config.SafeFactory && !!config.SafeMultisend;
}

export const getContractConfig = (chainId: number): ContractConfig => {
    switch (chainId) {
        case 137:
            return POL;
        case 80002:
            return AMOY;
        default:
            throw new Error("Invalid network");
    }
};


================================================
FILE: src/constants/index.ts
================================================
export const SAFE_INIT_CODE_HASH = "0x2bce2127ff07fb632d16c8347c4ebf501f4841168bed00d9e6ef715ddb6fcecf";

export const PROXY_INIT_CODE_HASH = "0xd21df8dc65880a8606f09fe0ce3df9b8869287ab0b058be05aa9e8af6330a00b";

export const SAFE_FACTORY_NAME = "Polymarket Contract Proxy Factory";



================================================
FILE: src/encode/index.ts
================================================
export * from "./safe";
export * from "./proxy";



================================================
FILE: src/encode/proxy.ts
================================================
import { proxyWalletFactory } from "../abis";
import { ProxyTransaction } from "../types";
import { encodeFunctionData, prepareEncodeFunctionData } from "viem";

const proxy = prepareEncodeFunctionData({
    abi: proxyWalletFactory,
    functionName: 'proxy',
});


export function encodeProxyTransactionData(txns: ProxyTransaction[]) : string {
    return encodeFunctionData({
        ...proxy,
        args: [txns],
      });
}



================================================
FILE: src/encode/safe.ts
================================================
import { 
    concatHex,
    encodeFunctionData, 
    encodePacked, 
    Hex, 
    prepareEncodeFunctionData, 
    size 
} from "viem";

import { multisendAbi } from "../abis";
import { OperationType, SafeTransaction } from "../types";

const multisend = prepareEncodeFunctionData({
    abi: multisendAbi,
    functionName: "multiSend",
});

export const createSafeMultisendTransaction = (txns: SafeTransaction[], safeMultisendAddress: string): SafeTransaction => {
    const args = [
        concatHex(
            txns.map(tx =>
                encodePacked(
                    ["uint8", "address", "uint256", "uint256", "bytes"], 
                    [tx.operation, tx.to as Hex, BigInt(tx.value), BigInt(size(tx.data as Hex)), tx.data as Hex]
                ),
            ),
        ),
    ];
    const data = encodeFunctionData({...multisend, args: args});
    return {
        to: safeMultisendAddress,
        value: "0",
        data: data,
        operation: OperationType.DelegateCall,
    }
}



================================================
FILE: src/http-helpers/index.ts
================================================
import axios, { AxiosInstance, AxiosRequestHeaders, AxiosResponse } from "axios";

export const GET = "GET";
export const POST = "POST";
export const DELETE = "DELETE";
export const PUT = "PUT";


export type QueryParams = Record<string, any>;

export interface RequestOptions {
    headers?: AxiosRequestHeaders;
    data?: any;
    params?: QueryParams;
}

export class HttpClient {

    readonly instance: AxiosInstance;

    constructor() {
        this.instance = axios.create({withCredentials: true});
    }

    public async send(
        endpoint: string,
        method: string,
        options?: RequestOptions,
    ): Promise<AxiosResponse> {
        if (options !== undefined) {
            if (options.headers != undefined) {
                options.headers["Access-Control-Allow-Credentials"] = true;
            }
        }

        try {
            const resp = await this.instance.request(
                {
                    url: endpoint,
                    method: method,
                    headers: options?.headers,
                    data: options?.data,
                    params: options?.params,
                }
            );
            return resp;
        } catch (err) {
            if (axios.isAxiosError(err)) {
                if (err.response) {
                    const errPayload = {
                        error: "request error",
                        status: err.response?.status,
                        statusText: err.response?.statusText,
                        data: err.response?.data,
                    };
                    console.error("request error", errPayload);
                    throw new Error(JSON.stringify(errPayload));
                } else {
                    const errPayload = { error: "connection error" };
                    console.error("connection error", errPayload);
                    throw new Error(JSON.stringify(errPayload));
                }
            }
            throw new Error(JSON.stringify({ error: err }));
        }
    }
}



================================================
FILE: src/response/index.ts
================================================
import { RelayClient } from "../client";
import { RelayerTransaction, RelayerTransactionResponse, RelayerTransactionState } from "../types";


export class ClientRelayerTransactionResponse implements RelayerTransactionResponse {

    readonly client: RelayClient;
    readonly transactionID: string;
    readonly transactionHash: string;
    readonly hash: string;
    readonly state: string;

    constructor(transactionID: string, state: string, transactionHash: string, client: RelayClient) {
        this.transactionID = transactionID;
        this.state = state;
        this.transactionHash = transactionHash;
        this.hash = transactionHash;
        this.client = client;
    }

    public async getTransaction(): Promise<RelayerTransaction[]> {
        return this.client.getTransaction(this.transactionID);
    }

    public async wait(): Promise<RelayerTransaction | undefined> {
        return this.client.pollUntilState(
            this.transactionID,
            [
                RelayerTransactionState.STATE_MINED,
                RelayerTransactionState.STATE_CONFIRMED,
            ],
            RelayerTransactionState.STATE_FAILED,
            100, // max polls
        );
    }
}



================================================
FILE: src/utils/index.ts
================================================
import { encodePacked, Hex, hexToBigInt } from "viem";

export interface SplitSig {
    r: string;
    s: string;
    v: string;
}

export function splitAndPackSig(sig: string): string {
    const splitSig = splitSignature(sig);
    
    const packedSig = encodePacked(
        ["uint256", "uint256", "uint8"],
        [BigInt(splitSig.r), BigInt(splitSig.s), parseInt(splitSig.v)],
    );
    return packedSig;
}

function splitSignature(sig: string) : SplitSig {
    let sigV = parseInt(sig.slice(-2), 16);
    switch (sigV) {
        case 0:
        case 1:
            sigV += 31;
            break;
        case 27:
        case 28:
            sigV += 4;
            break;
        default:
            throw new Error("Invalid signature");
    }

    sig = sig.slice(0, -2) + sigV.toString(16);

    return {
        r: hexToBigInt('0x' + sig.slice(2, 66) as Hex).toString(),
        s: hexToBigInt('0x' + sig.slice(66, 130) as Hex).toString(),
        v: hexToBigInt('0x' + sig.slice(130, 132) as Hex).toString(),
    };
}


export function sleep(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
}



================================================
FILE: tests/signatures/index.test.ts
================================================
import { expect } from "chai";
import { createAbstractSigner, IAbstractSigner } from "@polymarket/builder-abstract-signer";

import { Wallet } from "ethers";
import { JsonRpcProvider } from "@ethersproject/providers";

import { createWalletClient, http, WalletClient, zeroAddress } from "viem";
import { polygon } from "viem/chains";
import { privateKeyToAccount } from "viem/accounts";
import { encodeProxyTransactionData } from "../../src/encode";
import { buildProxyTransactionRequest, buildSafeCreateTransactionRequest, buildSafeTransactionRequest } from "../../src/builder";
import {
    CallType,
    OperationType,
    ProxyTransaction,
    ProxyTransactionArgs,
    SafeCreateTransactionArgs, 
    SafeTransaction, 
    SafeTransactionArgs, 
    TransactionRequest 
} from "../../src/types";
import { getContractConfig } from "../../src/config";



describe("setup", () => {
    const chainId = 137;
    const contractConfig = getContractConfig(chainId);
    let signer: IAbstractSigner;
    // publicly known private key
    const privateKey = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    const address = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";

    // publicly known RPC url
    const rpcUrl = "https://polygon-rpc.com";

    // Calldata to approve CTF as spender on USDC
    const usdc = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174";
    const approveCalldata = "0x095ea7b30000000000000000000000004d97dcd97ec945f40cf65f87097ace5ea0476045ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

    // ethers signer
    const w = new Wallet(privateKey);
    const ethersWallet: Wallet = w.connect(new JsonRpcProvider(rpcUrl));

    // viem signer
    const viemAccount = privateKeyToAccount(privateKey);
    const viemWalletClient: WalletClient = createWalletClient({
        account: viemAccount,
        chain: polygon,
        transport: http(rpcUrl),
    });

    const proxyTransaction: ProxyTransaction = {
        to: usdc,
        typeCode: CallType.Call,
        value: "0",
        data: approveCalldata,
    };

    const safeTransaction: SafeTransaction= {
        to: usdc,
        operation: OperationType.Call,
        value: "0",
        data: approveCalldata
    }

    describe("build proxy transaction request", async () => {
        let req: TransactionRequest;
        const expectedProxyTxnSig = "0x4c18e2d2294a00d686714aff8e7936ab657cb4655dfccb2b556efadcb7e835f800dc2fecec69c501e29bb36ecb54b4da6b7c410c4dc740a33af2afde2b77297e1b";
        const args: ProxyTransactionArgs = {
            from: address,
            gasLimit: "85338",
            gasPrice: "0",
            nonce: "0",
            relay: "0xae700edfd9ab986395f3999fe11177b9903a52f1",
            data: encodeProxyTransactionData([proxyTransaction]),
        };

        it("ethers creates a valid proxy signature", async () => {
            signer = createAbstractSigner(chainId, ethersWallet);
            req = await buildProxyTransactionRequest(
                signer,
                args,
                contractConfig.ProxyContracts,
            );
            expect(req.signature).equal(expectedProxyTxnSig);
        });

        it("viem creates a valid proxy signature", async () => {
            signer = createAbstractSigner(chainId, viemWalletClient);
            req = await buildProxyTransactionRequest(
                signer,
                args,
                contractConfig.ProxyContracts,
            );
            expect(req.signature).equal(expectedProxyTxnSig);
        });
    });

    describe("build safe transaction request", async () => {
        let req: TransactionRequest;
        const expectedSafeTxnSig = "0xf368488355b0566e99eff3bccc35e98b77d8f3a6e6866176188488c34f0305b07e4a4c600c7a1592e4ac1e96b5887ebff2cb26987a3ad501006b39944df098c21f";
        const args: SafeTransactionArgs = {
            from: address,
            nonce: "0",
            chainId,
            transactions: [safeTransaction],
        };

        it("ethers creates a valid safe signature", async () => {
            signer = createAbstractSigner(chainId, ethersWallet);
            req = await buildSafeTransactionRequest(
                signer,
                args,
                contractConfig.SafeContracts,
            );
            expect(req.signature).equal(expectedSafeTxnSig);
        });

        it("viem creates a valid safe signature", async () => {
            signer = createAbstractSigner(chainId, viemWalletClient);
            req = await buildSafeTransactionRequest(
                signer,
                args,
                contractConfig.SafeContracts,
            );
            expect(req.signature).equal(expectedSafeTxnSig);
        });
    });

    describe("build safe create transaction request", async () => {
        let req: TransactionRequest;
        const expectedSafeCreateTxnSig = "0xe3e791c24134b7bebe93b4771bd07c7fe7bbe115eeb0bf629ac3b7a435e7ac8d05f979729d873f7d0e16205becf48ee450aa382bc28c65eedcd6454e81d81f921b";
        const args: SafeCreateTransactionArgs = {
            from: address,
            chainId,
            paymentToken: zeroAddress,
            payment: "0",
            paymentReceiver: zeroAddress,
        };

        it("ethers creates a valid safe-create signature", async () => {
            signer = createAbstractSigner(chainId, ethersWallet);
            req = await buildSafeCreateTransactionRequest(
                signer,
                contractConfig.SafeContracts,
                args,
            );
            expect(req.signature).equal(expectedSafeCreateTxnSig);
        });

        it("viem creates a valid safe-create signature", async () => {
            signer = createAbstractSigner(chainId, viemWalletClient);
            req = await buildSafeCreateTransactionRequest(
                signer,
                contractConfig.SafeContracts,
                args,
            );
            expect(req.signature).equal(expectedSafeCreateTxnSig);

        });
    });
});


================================================
FILE: .github/workflows/test.yaml
================================================
name: Test

on:
    push:
        branches: [main]
    pull_request:

jobs:
  test:
    name: Test
    runs-on: ubuntu-22.04
    steps:
      - name: Checkout Code
        uses: actions/checkout@v4.1.4

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: pnpm setup
        uses: pnpm/action-setup@v4
        with:
          version: 10

      - name: Configure .npmrc
        run: |
          echo "//registry.npmjs.org/:_authToken=${{ secrets.NPM_TOKEN }}" > ~/.npmrc

      - name: Install dependencies
        run: pnpm install --frozen-lockfile

      - name: Run tests
        run: pnpm test


