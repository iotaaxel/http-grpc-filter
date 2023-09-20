# http-grpc-filter

## Overview

```
The API analyzes an HTTP user agent and decides to block or allow it. If the user agent header is from a Safari browser, it should return a decision to block the request. If it is from a Firefox browser, it should allow the request.

A gRPC client and server implements a single endpoint that receives the user agent string, analyzes it, and returns the result.

The terminal CLI uses the client to call the API.

```
## TODO: documentation and tests
