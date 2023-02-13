# WebServer Using Quark API
A Rust API implementation for Quark NFT.

## About

The code is a Rust program using the `actix-web` library to create a web server. The main function sets environment variables for logging and starts the server with a logger middleware. The server has a single endpoint that is defined in the uri module. The uri module is a complex set of code with a lot of imports that defines a REST API endpoint for signing a URL. The endpoint takes some JSON data as input and returns a signed URL as well as some metadata about the URL. The metadata includes various information about a quark collection, such as its name, image, description, origins, and attributes. The metadata also includes information about templates, projects, and collections that the quark collection belongs to. The endpoint also has various error handling codes that return different HTTP status codes based on the type of error.

uri.rs is a Rust source file that implements the endpoint for creating and signing a unique resource identifier (URI) for a quark collection. This file uses actix_web library for implementing the web endpoint, serde library for serializing and deserializing data, and ethers-signers library for generating the digital signature for the URI.

In the file, several structs are defined to store the metadata information of the quark collection such as `QuarkCollectionMetadataStandard`, `Template`, `Project`, `Collection`, `Attribute`, and `AttributeValueOnly`. The metadata information includes the name, image, description, and attributes of the collection and its associated templates, projects, and variations.

The endpoint for creating and signing a URI is implemented using the `#[post]` annotation. The endpoint accepts the combination of ingridients for the quark collection and returns a JSON object with the signed URI, signature, and metadata information. The endpoint handles several possible errors using the `#[derive(Debug, Display)]` and `impl ResponseError` for `TaskError` to convert the errors into HTTP responses with appropriate status codes.

In the implementation of the endpoint, `dotenv` library is used to read environment variables, `PinataApi` and `PinByJson` libraries are used to interact with IPFS, and `LocalWallet` and `Signer` libraries are used to create a local wallet and sign the URI. The encoding of the URI and the metadata information is done using the `encode` function from the `ethers-core` library.

### Features

* generate metadata
* Create uri on IPFS
* Sign uri
* return `UriSignatureResponse`

### Prerequisites
* Rust v1.45 or later
* Pinata `API Key` and `Secret Key`
* Ethereum Wallet

## Modules
The API is composed of three main modules:

- `api` module: responsible for defining endpoints for creating signed URIs
- `model` module: responsible for handling the data structure used in API
- `repository` module: responsible for storing and retrieving data

## Endpoints
`uri.rs` implements the following endpoints:

- `POST` `/create_uri`: creates a signed URI using the provided combination and metadata

### example

To make a `POST` request to a server at `/uri` with the provided data using axios, you can use the following code:

```javascript
const axios = require('axios');

const data = {
    combination: "ingredient21_ingredient42_ingredient551"
};

axios.post('/uri', data)
  .then(response => {
    console.log(response.data);
  })
  .catch(error => {
    console.error(error);
  });
```

In this example, axios is used to send a `POST` request to the server at "/uri". The data to be sent is defined in the data object and is passed as the second argument to the post method. The response from the server is logged to the console using the then method. In case of an error, the error message is logged to the console using the catch method.


## Environment Variables
The following environment variables must be set to run the API:

`RUST_LOG`: sets the log level for the API
`RUST_BACKTRACE`: sets the backtrace for debugging purposes
`PINATA_API_KEY`:
`PINATA_SECRET_API_KEY`:
`PRIVATE_KEY`:

## Usage
To start the API, simply run `cargo run` in the terminal. The API will start on `localhost:80`.

## Errors
If an error occurs while creating the signed URI, the following errors can be returned:

`TaskError::SignatureFailed`: failed to sign the message
`TaskError::WalletFailed`: failed to create the wallet
`TaskError::MetadataFailed`: failed to retrieve the metadata
`TaskError::NftTaken`: NFT has already been taken
`TaskError::Forbidden`: access forbidden
`TaskError::Conflict`: conflict with existing data