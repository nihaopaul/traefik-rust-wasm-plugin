## Ambition for Launching `traefik-wasm-api`

The ambition for launching `traefik-wasm-api` was driven by the lack of available documentation from Traefik Labs and the absence of an easy way for the community to get started. This was my first crate, but very little logic is handled by the library.

Initial reference code from: https://github.com/elisasre/http-wasm-rust/ (deprecated)

Updated to match the implementation ABI: https://http-wasm.io/http-handler-abi/ which Traefik uses for the wasm interface between host and plugin.

included make commands to get up and running with hooks into the traefik and a docker command to run local testing.

### key decisions taken:

1. test if the buffer of 0 bytes is big enough to then implement a dynamic buffer based on size
2. body_read uses a 1MB buffer by default

### TODO

1. standardise the interface responses. (V2)
2. implement collections for all headers
