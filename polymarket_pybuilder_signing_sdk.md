Directory structure:
└── polymarket-py-builder-signing-sdk/
    ├── README.md
    ├── Makefile
    ├── requirements.txt
    ├── setup.py
    ├── .editorconfig
    ├── .env.example
    ├── py_builder_signing_sdk/
    │   ├── __init__.py
    │   ├── http_helpers/
    │   │   ├── __init__.py
    │   │   └── helpers.py
    │   └── signing/
    │       ├── __init__.py
    │       └── hmac.py
    ├── tests/
    │   ├── __init__.py
    │   └── signing/
    │       ├── __init__.py
    │       └── test_hmac.py
    └── .github/
        └── workflows/
            └── workflow.yaml


Files Content:

================================================
FILE: README.md
================================================
# py-builder-signing-sdk

Python SDK for Polymarket builder authentication and signing.

## Installation

```bash
pip install py-builder-signing-sdk
```

## Usage

```python
from py_builder_signing_sdk import BuilderConfig, BuilderApiKeyCreds, RemoteBuilderConfig

# Local signing
creds = BuilderApiKeyCreds(
    key="your-api-key",
    secret="your-secret", 
    passphrase="your-passphrase"
)
config = BuilderConfig(local_builder_creds=creds)

# Remote signing
remote_config = RemoteBuilderConfig(
    url="http://localhost:3000/sign",
    token="your-auth-token"  # optional
)
config = BuilderConfig(remote_builder_config=remote_config)

# Generate signed headers
headers = config.generate_builder_headers("POST", "/order", '{"data": "example"}')
```



================================================
FILE: Makefile
================================================
init:
	pip install -r requirements.txt

test:
	pytest -s

fmt:
	black ./.


================================================
FILE: requirements.txt
================================================
black==24.4.2
pytest==8.2.2
python-dotenv==0.19.2
requests==2.32.3
responses==0.25.8


================================================
FILE: setup.py
================================================
import setuptools

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setuptools.setup(
    name="py_builder_signing_sdk",
    version="0.0.1",
    author="Polymarket Engineering",
    author_email="engineering@polymarket.com",
    maintainer="Polymarket Engineering",
    maintainer_email="engineering@polymarket.com",
    description="Python builder signing sdk",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/Polymarket/py-builder-signing-sdk",
    install_requires=[
        "python-dotenv",
        "requests",
    ],
    project_urls={
        "Bug Tracker": "https://github.com/Polymarket/py-builder-signing-sdk/issues",
    },
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    packages=setuptools.find_packages(),
    python_requires=">=3.11",
)



================================================
FILE: .editorconfig
================================================
root = true

[*]
charset = utf-8
end_of_line = lf
indent_style = space
trim_trailing_whitespace = true
insert_final_newline = true

[*.{py}]
indent_style = space
indent_size = 4


[*.json]
indent_style = space
indent_size = 2

[*.yaml]
indent_style = space
indent_size = 2
quote_type = single



================================================
FILE: .env.example
================================================
POLY_BUILDER_API_KEY=
POLY_BUILDER_SECRET=
POLY_BUILDER_PASSPHRASE=



================================================
FILE: py_builder_signing_sdk/__init__.py
================================================
[Empty file]


================================================
FILE: py_builder_signing_sdk/http_helpers/__init__.py
================================================
[Empty file]


================================================
FILE: py_builder_signing_sdk/http_helpers/helpers.py
================================================
import requests


GET = "GET"
POST = "POST"
DELETE = "DELETE"
PUT = "PUT"


def request(endpoint: str, method: str, headers=None, data=None):
    try:
        resp = requests.request(
            method=method, url=endpoint, headers=headers, json=data if data else None
        )
        if resp.status_code != 200:
            raise Exception(resp)

        try:
            return resp.json()
        except requests.JSONDecodeError:
            return resp.text

    except requests.RequestException:
        raise Exception(error_msg="Request exception!")


def post(endpoint, headers=None, data=None):
    return request(endpoint, POST, headers, data)



================================================
FILE: py_builder_signing_sdk/signing/__init__.py
================================================
[Empty file]


================================================
FILE: py_builder_signing_sdk/signing/hmac.py
================================================
import hmac
import hashlib
import base64


def build_hmac_signature(
    secret: str, timestamp: str, method: str, requestPath: str, body=None
):
    """
    Creates an HMAC signature by signing a payload with the secret
    """
    base64_secret = base64.urlsafe_b64decode(secret)
    message = str(timestamp) + str(method) + str(requestPath)
    if body:
        # NOTE: Necessary to replace single quotes with double quotes
        # to generate the same hmac message as go and typescript
        message += str(body).replace("'", '"')

    h = hmac.new(base64_secret, bytes(message, "utf-8"), hashlib.sha256)

    # ensure base64 encoded
    return (base64.urlsafe_b64encode(h.digest())).decode("utf-8")



================================================
FILE: tests/__init__.py
================================================
[Empty file]


================================================
FILE: tests/signing/__init__.py
================================================
[Empty file]


================================================
FILE: tests/signing/test_hmac.py
================================================
from unittest import TestCase

from py_builder_signing_sdk.signing.hmac import build_hmac_signature


class TestHMAC(TestCase):
    def test_build_hmac_signature(self):
        signature = build_hmac_signature(
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
            "1000000",
            "test-sign",
            "/orders",
            '{"hash": "0x123"}',
        )
        self.assertIsNotNone(signature)
        self.assertEqual(
            signature,
            "ZwAdJKvoYRlEKDkNMwd5BuwNNtg93kNaR_oU2HrfVvc=",
        )



================================================
FILE: .github/workflows/workflow.yaml
================================================
name: Test

on:
  push:
    branches: [main]
  pull_request:

jobs:
  build-lint-test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - name: Checkout Code
        uses: actions/checkout@v4.1.7
        with:
          persist-credentials: false

      - uses: actions/setup-python@v5
        with:
          python-version: 3.11

      - run: |
          python -m pip install --upgrade pip
          pip install -r requirements.txt
          pip install -e .

      - name: Run Tests
        run: make test


