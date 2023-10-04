# http-grpc-filter

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
$ cargo run --bin echo-server"
```
Start the client in another terminal (or separate tab):

The last part of the input string should be a HTTP User Agent. Mozilla has provided [examples](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent) of user agent strings for different browsers. 

```console
$ cargo run --bin echo-client -- "Hello World\!"
```

### Example 1: Firefox User Agent
Run client after starting the server: 

```console
$ cargo run --bin echo-client -- "Mozilla/5.0 (Macintosh; Intel Mac OS X x.y; rv:42.0) Gecko/20100101 Firefox/42.0"
```
### Example 2: Safari User Agent
Run client after starting the server: 
```console
$ cargo run --bin echo-client -- "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.1 Mobile/15E148 Safari/604.1"

RESPONSE=EchoResponse { message: "Blocking the Request.", user_agent: "Mozilla/5.0 (iPhone; CPU iPhone OS 13_5_1 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.1.1 Mobile/15E148 Safari/604.1\n" }
```


## Testing

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
