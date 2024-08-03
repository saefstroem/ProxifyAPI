# `ProxifyAPI` - Proxify API calls programmatically.

## Introduction

**ProxifyAPI** is an application written in Rust that allows you to make dynamic API calls to an API via a proxy server. This can be useful if you want to aggregate data from multiple sources and display them in a single application whilst avoiding leaking API keys to a client.

## Installation

To install the application, you can clone the repository and build the application using `cargo build --release`. This will create a binary in the `target/release` directory.

## Usage

Create a `apis.json` file in the root of the repository with the following format:

```json
{
    "apiname":{
        "identifier":"apiname",
        "api_key":"test_1234567890",
        "metadata":"any metadata"
    },
    "apiname2":{
        "identifier":"apiname2",
        "api_key":"test_1234567890",
        "metadata":"any metadata"
    }
}
``

To use the application, you can run the binary with the following command from the root of the repository:

```bash
./target/release/proxifyapi
```


You can override the default configuration by passing the following environment variables:

- `HOST`: The host of the proxy server. Default: `127.0.0.1`.
- `PORT`: The port of the proxy server. Default: `3000`.
- `APIS_PATH`: The path to the `apis.json` file. Default: `./apis.json`.
- `SSL_CERT_PATH`: The path to the SSL certificate.
- `SSL_KEY_PATH`: The path to the SSL key.

**If either SSL certificate or key is not provided, the application will run in HTTP mode.**

## License
This project is licensed under the MIT License.