# http-grpc-filter

[![Build Status](https://github.com/iotaaxel/http-grpc-filter/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/iotaaxel/http-grpc-filter/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/iotaaxel/http-grpc-filter/branch/main/graph/badge.svg)](https://codecov.io/gh/iotaaxel/http-grpc-filter)

## Overview

The API analyzes an HTTP user agent and decides to block or allow it. If the user agent header is from a Safari browser, it should return a decision to block the request. If it is from a Firefox browser, it should allow the request.

A gRPC client and server implements a single endpoint that receives the user agent string, analyzes it, and returns the result.

The terminal CLI uses the client to call the API.

## Getting Started 
### Prerequisites
#### Install Protoc
  
The `Protoc` compiler is needed to generate the gRPC code.

If you are using MacOS, you can use Homebrew:
```console
$ brew install protobuf
```
#### Verify Installation of Protoc

The `protoc` compiler should be included in your `PATH`. Verify by running the following command:

```console
$ protoc --version
libprotoc 24.3 # this is an example version
```

## Running

Open two terminals or tabs in a terminal. 

Start the server in one terminal (or separate tab): 

```console
$ cargo run --bin echo-server
```
Start the client in another terminal (or separate tab):

The last part of the input string should be a HTTP User Agent. Mozilla has provided [examples](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent) of user agent strings for different browsers. 

```console
$ cargo run --bin echo-client -- "Hello World\!"
```

### Example 1: Firefox User Agent
Run client after starting the server: 

```console
$ cargo run --bin echo-client -- "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:101.0) Gecko/20100101 Firefox/101.0"

RESPONSE=EchoResponse { message: "Accepting the Request.", user_agent: "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:101.0) Gecko/20100101 Firefox/101.0" }"
```
### Example 2: Safari User Agent
Run client after starting the server: 
```console
$ cargo run --bin echo-client -- "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.1 Mobile/15E148 Safari/604.1"

RESPONSE=EchoResponse { message: "Blocking the Request.", user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.1 Mobile/15E148 Safari/604.1\n" }
```

## Testing
```console
$ cargo test

#################################### Sample output below ####################################
Finished test [unoptimized + debuginfo] target(s) in 0.04s
     Running unittests src/main.rs (target/debug/deps/echo_client-3d3566e115c08a41)

running 1 test
test tests::test_bad_client_connection ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/server.rs (target/debug/deps/echo_server-ffaaf4bd0ea8fe8a)

running 1 test
test tests::test_bad_address ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## Troubleshooting
- If you see a `dquote>` on the terminal when running the client, [history expansion](https://www.gnu.org/software/bash/manual/html_node/Double-Quotes.html) might be enabled in your terminal, so retry the input string and escape backspaces. 
- Use `CTRL+G` to escape `dquote>`
### Example
```console
$ cargo run --bin echo-client --"Hello World!"

dquote>
```

Click `CTRL+G` then retype the following:

```console
$ cargo run --bin echo-client --"Hello World\!"
```

## Future Work
- See the [issues](https://github.com/iotaaxel/http-grpc-filter/issues) for future feature and enchancement ideas.
