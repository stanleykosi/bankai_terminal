Directory structure:
└── polymarket-builder-signing-sdk/
    ├── README.md
    ├── Makefile
    ├── package.json
    ├── tsconfig.json
    ├── tsconfig.production.json
    ├── .env.example
    ├── src/
    │   ├── config.ts
    │   ├── index.ts
    │   ├── signer.ts
    │   ├── types.ts
    │   ├── http-helpers/
    │   │   └── index.ts
    │   └── signing/
    │       ├── hmac.ts
    │       └── index.ts
    ├── tests/
    │   ├── config.test.ts
    │   ├── hmac.test.ts
    │   └── signer.test.ts
    └── .github/
        ├── CODEOWNERS
        └── workflows/
            └── ci.yaml


Files Content:

================================================
FILE: README.md
================================================
# builder-signing-sdk

A TypeScript SDK for creating authenticated builder headers


## Installation

```bash
pnpm install @polymarket/builder-signing-sdk
```

## Quick Start

```typescript
import { BuilderSigner } from '@polymarket/builder-signing-sdk';

// Create a builder config for signing

// Local
const builderConfig = new BuilderConfig(
  {
    localBuilderCreds: {
      key: "xxxxxxx-xxx-xxxx-xxx-xxxxxxxxx",
      secret: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
      passphrase: "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    },
  },
);

const headers = await builderConfig.generateBuilderHeaders(
  'POST'                   // HTTP method
  '/order',               // API endpoint path
  '{"marketId": "0x123"}' // Request body
);

// Remote
const builderConfig = new BuilderConfig(
  {
    remoteBuilderConfig: {
      url: remoteSignerUrl,
      token: `${process.env.MY_AUTH_TOKEN}`
    }
  },
);

const headers = await builderConfig.generateBuilderHeaders(
  'POST'                   // HTTP method
  '/order',               // API endpoint path
  '{"marketId": "0x123"}' // Request body
);
```


================================================
FILE: Makefile
================================================
.PHONY: build
build:
	@echo "Building ts code..."
	rm -rf dist
	pnpm exec tsc --module commonjs

.PHONY: test
test:
	pnpm exec nyc -a \
		--reporter=html \
		--reporter=text mocha './tests' \
		--require esm \
		--require jsdom-global/register \
		--require ts-node/register 'tests/**/*.test.ts' \
		--require tsconfig-paths/register \
		--timeout 300000 \
		--exit



================================================
FILE: package.json
================================================
{
    "name": "@polymarket/builder-signing-sdk",
    "description": "Polymarket SDK for creating builder headers",
    "version": "0.0.8",
    "contributors": [
        {
            "name": "Jonathan Amenechi",
            "url": "https://github.com/JonathanAmenechi"
        }
    ],
    "main": "dist/index.js",
    "types": "dist/index.d.ts",
    "files": [
        "/dist"
    ],
    "license": "MIT",
    "scripts": {
        "build": "make build",
        "deploy": "make build && npm publish",
        "test": "make test"
    },
    "dependencies": {
        "@types/node": "^18.7.18",
        "axios": "^1.12.2",
        "tslib": "^2.8.1"
    },
    "devDependencies": {
        "@types/chai": "^4.3.3",
        "@types/mocha": "^9.1.1",
        "chai": "^4.3.6",
        "dotenv": "^16.0.2",
        "esm": "^3.2.25",
        "jsdom": "^20.0.0",
        "jsdom-global": "^3.0.2",
        "mocha": "^10.0.0",
        "nyc": "^15.1.0",
        "path": "^0.12.7",
        "ts-mocha": "^10.0.0",
        "ts-node": "^10.9.1",
        "typescript": "^4.8.3",
        "@types/sinon": "^17.0.4",
        "sinon": "^21.0.0"
    }
}



================================================
FILE: tsconfig.json
================================================
{
    "compileOnSave": false,
    "include": ["src/*", "examples/*", "src/signing/hmac.ts"],
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
POLY_BUILDER_API_KEY=
POLY_BUILDER_PASSPHRASE=
POLY_BUILDER_SIGNATURE=
POLY_BUILDER_TIMESTAMP=



================================================
FILE: src/config.ts
================================================
import { BuilderSigner } from "./signer";
import { post } from "./http-helpers";
import {
    BuilderApiKeyCreds,
    BuilderHeaderPayload,
    BuilderType,
    RemoteBuilderConfig,
    RemoteSignerPayload 
} from "./types";
import { AxiosRequestHeaders } from 'axios';

export class BuilderConfig {
    readonly remoteBuilderConfig?: RemoteBuilderConfig;
    readonly localBuilderCreds?: BuilderApiKeyCreds;
    readonly signer?: BuilderSigner;

    constructor(config?: { 
        remoteBuilderConfig?: RemoteBuilderConfig;
        localBuilderCreds?: BuilderApiKeyCreds;
    }) {
        if (config) {
            if (config.remoteBuilderConfig !== undefined) {
                if (!BuilderConfig.hasValidRemoteUrl(config.remoteBuilderConfig.url)) {
                    throw new Error("invalid remote url!");
                }
                if (config.remoteBuilderConfig.token !== undefined) {
                    const tk = config.remoteBuilderConfig.token;
                    if (tk.length === 0) {
                        throw new Error("invalid auth token");
                    }
                }
                this.remoteBuilderConfig = config.remoteBuilderConfig;
            }
            if (config.localBuilderCreds !== undefined) {
                if (!BuilderConfig.hasValidLocalCreds(config.localBuilderCreds)) {
                    throw new Error("invalid local builder credentials!");
                } 
                this.localBuilderCreds = config.localBuilderCreds;
                this.signer = new BuilderSigner(config.localBuilderCreds);
            }
        }
    }

    /**
     * Helper function to generate builder headers using the configured credential method
     * @param method 
     * @param path 
     * @param body 
     */
    public async generateBuilderHeaders(
        method: string,
        path: string,
        body?: string,
        timestamp?: number,
    ): Promise<BuilderHeaderPayload | undefined> {
        this.ensureValid();

        const builderType = this.getBuilderType();

        if (builderType == BuilderType.LOCAL) {
            return Promise.resolve(this.signer?.createBuilderHeaderPayload(method, path, body, timestamp));
        } 
        
        if (builderType == BuilderType.REMOTE) {
            const url: string = (this.remoteBuilderConfig as RemoteBuilderConfig).url;
            // Execute a POST to the remote signer url with the header arguments
            const payload: RemoteSignerPayload = {
                method: method,
                path: path,
                body: body,
                timestamp: timestamp,
            };
            
            try {
                const token = (this.remoteBuilderConfig as RemoteBuilderConfig).token;
                return await post(url, {
                    data: payload,
                    headers: {
                      ...(token ? { Authorization: `Bearer ${token}` } : {}),
                    } as AxiosRequestHeaders,
                  });
            } catch (err) {
                console.error("error calling remote signer", err);
                return undefined;
            }
        }
        return undefined;
    }

    public isValid(): boolean {
        return this.getBuilderType() !== BuilderType.UNAVAILABLE;
    }

    public getBuilderType(): BuilderType {
        const local = this.localBuilderCreds;
        const remote = this.remoteBuilderConfig;
        if (local && remote) {
            // If both present, prefer local
            return BuilderType.LOCAL;
        }
        if (local) {
            return BuilderType.LOCAL;
        }
        if (remote) {
            return BuilderType.REMOTE;
        }
        return BuilderType.UNAVAILABLE;
    }

    private static hasValidLocalCreds(creds?: BuilderApiKeyCreds): boolean {
        if (!creds) return false;
        
        const { key, secret, passphrase } = creds;

        if (!key.trim()) return false;
        
        if (!secret.trim()) return false;
        
        if (!passphrase.trim()) return false;
        
        return true;
    }
    
      private static hasValidRemoteUrl(remoteUrl?: string): boolean {
          if (!remoteUrl?.trim()) return false;
          return remoteUrl.startsWith("http://") || remoteUrl.startsWith("https://");
      }
    
      private ensureValid(): void {
          if (this.getBuilderType() === BuilderType.UNAVAILABLE) {
              throw new Error("invalid builder creds configured!");
          }
      }
}



================================================
FILE: src/index.ts
================================================
export * from "./config";
export * from "./signing";
export * from "./signer";
export * from "./types";



================================================
FILE: src/signer.ts
================================================
import { buildHmacSignature } from "./signing";
import { BuilderApiKeyCreds, BuilderHeaderPayload } from "./types";


export class BuilderSigner {
    readonly creds: BuilderApiKeyCreds;

    constructor(creds: BuilderApiKeyCreds) {
        this.creds = creds;
    }

    public createBuilderHeaderPayload(
        method: string,
        path: string,
        body?: string,
        timestamp?: number,
    ): BuilderHeaderPayload {
        let ts = Math.floor(Date.now() / 1000);
        if (timestamp !== undefined) {
            ts = timestamp;
        }

        const builderSig = buildHmacSignature(
            this.creds.secret,
            ts,
            method,
            path,
            body,
        );

        return {
            POLY_BUILDER_API_KEY: this.creds.key,
            POLY_BUILDER_PASSPHRASE: this.creds.passphrase,
            POLY_BUILDER_SIGNATURE: builderSig,
            POLY_BUILDER_TIMESTAMP: `${ts}`,
        }
    }
}



================================================
FILE: src/types.ts
================================================

export enum BuilderType {
    UNAVAILABLE = "UNAVAILABLE",
    LOCAL = "LOCAL",
    REMOTE = "REMOTE",
}

export interface BuilderApiKeyCreds {
    key: string;
    secret: string;
    passphrase: string;
}

export interface RemoteBuilderConfig {
    url: string;
    token?: string; // Optional authorization token
}

export interface RemoteSignerPayload {
    method: string,
    path: string,
    body?: string,
    timestamp?: number,
}

export interface BuilderHeaderPayload {
    POLY_BUILDER_API_KEY: string;
    POLY_BUILDER_TIMESTAMP: string;
    POLY_BUILDER_PASSPHRASE: string;
    POLY_BUILDER_SIGNATURE: string;
    [key: string]: string;
}




================================================
FILE: src/http-helpers/index.ts
================================================
import axios, { AxiosRequestHeaders } from "axios";

type QueryParams = Record<string, any>;

interface RequestOptions {
    headers?: AxiosRequestHeaders;
    data?: any;
    params?: QueryParams;
}

const request = async (
    endpoint: string,
    method: string,
    headers?: any,
    data?: any,
    params?: any,
): Promise<any> => {
    return await axios({ method, url: endpoint, headers, data, params });
};

export const post = async (endpoint: string, options?: RequestOptions): Promise<any> => {
    const resp = await request(
        endpoint,
        "POST",
        options?.headers,
        options?.data,
        options?.params,
    );
    return resp.data;
};


================================================
FILE: src/signing/hmac.ts
================================================
import crypto from "crypto";

function replaceAll(s: string, search: string, replace: string) {
    return s.split(search).join(replace);
}

/**
 * Builds an hmac signature
 * @param signer
 * @param key
 * @param secret
 * @param passphrase
 * @returns string
 */
export const buildHmacSignature = (
    secret: string,
    timestamp: number,
    method: string,
    requestPath: string,
    body?: string,
): string => {
    let message = timestamp + method + requestPath;
    if (body !== undefined) {
        message += body;
    }

    const base64Secret = Buffer.from(secret, "base64");
    const hmac = crypto.createHmac("sha256", base64Secret);
    const sig = hmac.update(message).digest("base64");

    // NOTE: Must be url safe base64 encoding, but keep base64 "=" suffix
    // Convert '+' to '-'
    // Convert '/' to '_'
    const sigUrlSafe = replaceAll(replaceAll(sig, "+", "-"), "/", "_");
    return sigUrlSafe;
};



================================================
FILE: src/signing/index.ts
================================================
export * from "./hmac";



================================================
FILE: tests/config.test.ts
================================================
import * as httpHelpers from '../src/http-helpers';
import sinon from 'sinon';
import "mocha";
import { expect } from "chai";
import { BuilderApiKeyCreds, BuilderConfig, BuilderHeaderPayload, BuilderType } from "../src";

describe("builder config", () => {
    it("isValid", () => {
        let builderConfig: BuilderConfig;
        const creds: BuilderApiKeyCreds = {
            key: "019894b9-cb40-79c4-b2bd-6aecb6f8c6c5",
            secret: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
            passphrase: "1816e5ed89518467ffa78c65a2d6a62d240f6fd6d159cba7b2c4dc510800f75a",
        }
        // isValid false
        builderConfig = new BuilderConfig();
        
        // isValid true
        builderConfig = new BuilderConfig({localBuilderCreds: creds});
        expect(builderConfig.isValid()).true;
    });

    it("getBuilderType", async () => {
        let builderConfig: BuilderConfig;
        const creds: BuilderApiKeyCreds = {
            key: "019894b9-cb40-79c4-b2bd-6aecb6f8c6c5",
            secret: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
            passphrase: "1816e5ed89518467ffa78c65a2d6a62d240f6fd6d159cba7b2c4dc510800f75a",
        };
        builderConfig = new BuilderConfig({localBuilderCreds: creds});
        expect(builderConfig.getBuilderType()).equal(BuilderType.LOCAL);

        builderConfig = new BuilderConfig({remoteBuilderConfig: {url: "http://localhost:3000/sign"} })
        expect(builderConfig.getBuilderType()).equal(BuilderType.REMOTE);

        builderConfig = new BuilderConfig()
        expect(builderConfig.getBuilderType()).equal(BuilderType.UNAVAILABLE);

        // if both local is preferred
        builderConfig = new BuilderConfig({localBuilderCreds: creds, remoteBuilderConfig: {url: "http://localhost:3000/sign"}})
        expect(builderConfig.getBuilderType()).equal(BuilderType.LOCAL);
    });

    it("generateBuilderHeaders", async () => {
        const creds: BuilderApiKeyCreds = {
            key: "019894b9-cb40-79c4-b2bd-6aecb6f8c6c5",
            secret: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
            passphrase: "1816e5ed89518467ffa78c65a2d6a62d240f6fd6d159cba7b2c4dc510800f75a",
        };
        const builderConfig: BuilderConfig = new BuilderConfig({localBuilderCreds: creds});
        
        const requestPath = "/order";
        const requestBody = `{"deferExec":false,"order":{"salt":718139292476,"maker":"0x6e0c80c90ea6c15917308F820Eac91Ce2724B5b5","signer":"0x6e0c80c90ea6c15917308F820Eac91Ce2724B5b5","taker":"0x0000000000000000000000000000000000000000","tokenId":"15871154585880608648532107628464183779895785213830018178010423617714102767076","makerAmount":"5000000","takerAmount":"10000000","side":"BUY","expiration":"0","nonce":"0","feeRateBps":"1000","signatureType":0,"signature":"0x64a2b097cf14f9a24403748b4060bedf8f33f3dbe2a38e5f85bc2a5f2b841af633a2afcc9c4d57e60e4ff1d58df2756b2ca469f984ecfd46cb0c8baba8a0d6411b"},"owner":"5d1c266a-ed39-b9bd-c1f5-f24ae3e14a7b","orderType":"GTC"}`;
        const requestMethod = "POST";
        const timestamp = 1758744060;
        const headers: BuilderHeaderPayload = await builderConfig.generateBuilderHeaders(
            requestMethod,
            requestPath,
            requestBody,
            timestamp,
        ) as BuilderHeaderPayload;

        expect(headers).not.null;
        expect(headers).not.undefined;
        expect(headers).not.empty;
        expect(headers.POLY_BUILDER_API_KEY).equal("019894b9-cb40-79c4-b2bd-6aecb6f8c6c5");
        expect(headers.POLY_BUILDER_PASSPHRASE).equal("1816e5ed89518467ffa78c65a2d6a62d240f6fd6d159cba7b2c4dc510800f75a");
        expect(headers.POLY_BUILDER_TIMESTAMP).equal("1758744060");
        expect(headers.POLY_BUILDER_SIGNATURE).equal("8xh8d0qZHhBcLLYbsKNeiOW3Z0W2N5yNEq1kCVMe5QE=");
    });

    it("generateHeaders - remote", async () => {
        // Mock remote signer endpoint
        const remoteSignerUrl = "http://localhost:3000/sign";
        const mockResponse: BuilderHeaderPayload = {
            POLY_BUILDER_API_KEY: "test-api-key",
            POLY_BUILDER_TIMESTAMP: "1758744060",
            POLY_BUILDER_PASSPHRASE: "test-passphrase",
            POLY_BUILDER_SIGNATURE: "test-signature"
        };

        sinon.stub(httpHelpers, 'post').resolves(mockResponse);

        // Create config with remote signer URL
        const builderConfig = new BuilderConfig({
            remoteBuilderConfig: {url: remoteSignerUrl}
        });

        expect(builderConfig.getBuilderType()).equal(BuilderType.REMOTE);

        const requestMethod = "POST";
        const requestPath = "/order";
        const requestBody = `{"deferExec":false,"order":{"salt":718139292476,"maker":"0x6e0c80c90ea6c15917308F820Eac91Ce2724B5b5","signer":"0x6e0c80c90ea6c15917308F820Eac91Ce2724B5b5","taker":"0x0000000000000000000000000000000000000000","tokenId":"15871154585880608648532107628464183779895785213830018178010423617714102767076","makerAmount":"5000000","takerAmount":"10000000","side":"BUY","expiration":"0","nonce":"0","feeRateBps":"1000","signatureType":0,"signature":"0x64a2b097cf14f9a24403748b4060bedf8f33f3dbe2a38e5f85bc2a5f2b841af633a2afcc9c4d57e60e4ff1d58df2756b2ca469f984ecfd46cb0c8baba8a0d6411b"},"owner":"5d1c266a-ed39-b9bd-c1f5-f24ae3e14a7b","orderType":"GTC"}`;
        const timestamp = 1758744060;

        const headers = await builderConfig.generateBuilderHeaders(
            requestMethod,
            requestPath,
            requestBody,
            timestamp
        );

        // Verify the response
        expect(headers).not.null;
        expect(headers).not.undefined;
        expect(headers).to.deep.equal(mockResponse);
        expect(headers!.POLY_BUILDER_API_KEY).equal("test-api-key");
        expect(headers!.POLY_BUILDER_TIMESTAMP).equal("1758744060");
        expect(headers!.POLY_BUILDER_PASSPHRASE).equal("test-passphrase");
        expect(headers!.POLY_BUILDER_SIGNATURE).equal("test-signature");
    });

});



================================================
FILE: tests/hmac.test.ts
================================================
import "mocha";
import { expect } from "chai";
import { buildHmacSignature } from "../src/signing";

describe("hmac", () => {
    it("buildHmacSignature", () => {
        const signature = buildHmacSignature(
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
            1000000,
            "test-sign",
            "/orders",
            '{"hash": "0x123"}',
        );
        expect(signature).not.null;
        expect(signature).not.undefined;
        expect(signature).not.empty;
        expect(signature).equal("ZwAdJKvoYRlEKDkNMwd5BuwNNtg93kNaR_oU2HrfVvc=");
    });
});



================================================
FILE: tests/signer.test.ts
================================================
import "mocha";
import { expect } from "chai";
import { BuilderSigner, BuilderApiKeyCreds } from "../src";

describe("builderHeaderPayload", () => {
    it("createBuilderHeaderPayload", () => {
        const creds: BuilderApiKeyCreds = {
            key: "019894b9-cb40-79c4-b2bd-6aecb6f8c6c5",
            secret: "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
            passphrase: "1816e5ed89518467ffa78c65a2d6a62d240f6fd6d159cba7b2c4dc510800f75a",
        }
        const signer = new BuilderSigner(creds);
        const requestPath = "/order";
        const requestBody = `{"deferExec":false,"order":{"salt":718139292476,"maker":"0x6e0c80c90ea6c15917308F820Eac91Ce2724B5b5","signer":"0x6e0c80c90ea6c15917308F820Eac91Ce2724B5b5","taker":"0x0000000000000000000000000000000000000000","tokenId":"15871154585880608648532107628464183779895785213830018178010423617714102767076","makerAmount":"5000000","takerAmount":"10000000","side":"BUY","expiration":"0","nonce":"0","feeRateBps":"1000","signatureType":0,"signature":"0x64a2b097cf14f9a24403748b4060bedf8f33f3dbe2a38e5f85bc2a5f2b841af633a2afcc9c4d57e60e4ff1d58df2756b2ca469f984ecfd46cb0c8baba8a0d6411b"},"owner":"5d1c266a-ed39-b9bd-c1f5-f24ae3e14a7b","orderType":"GTC"}`;
        const requestMethod = "POST";
        const timestamp = 1758744060;
        
        const payload = signer.createBuilderHeaderPayload(requestMethod,requestPath, requestBody, timestamp);
        
        expect(payload).not.null;
        expect(payload).not.undefined;
        expect(payload).not.empty;
        expect(payload.POLY_BUILDER_API_KEY).equal("019894b9-cb40-79c4-b2bd-6aecb6f8c6c5");
        expect(payload.POLY_BUILDER_PASSPHRASE).equal("1816e5ed89518467ffa78c65a2d6a62d240f6fd6d159cba7b2c4dc510800f75a");
        expect(payload.POLY_BUILDER_TIMESTAMP).equal("1758744060");
        expect(payload.POLY_BUILDER_SIGNATURE).equal("8xh8d0qZHhBcLLYbsKNeiOW3Z0W2N5yNEq1kCVMe5QE=");

    });
});



================================================
FILE: .github/CODEOWNERS
================================================
# All PRs on any file must be reviewed by one of the following team members
# Wildcard (*) for all files
* @Polymarket/eng-platform



================================================
FILE: .github/workflows/ci.yaml
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
            
            - name: pnpm setup
              uses: pnpm/action-setup@v4
              with:
                version: 10

            - run: pnpm install --frozen-lockfile
            - run: pnpm test



