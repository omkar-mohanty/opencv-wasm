
# opencv-wasm

## Requirements
- [Wasi-SDK](https://github.com/WebAssembly/wasi-sdk) is assumed to be installed at `/opt`
- pkg-config
- opencv4 
## Build
1. Set the path for WASI-SDK in `cxx.sh` for example
```bash
export WASI_VERSION=14
export WASI_VERSION_FULL=${WASI_VERSION}.0
export WASI_SDK_PATH=`pwd`/wasi-sdk-${WASI_VERSION_FULL}
```
2. Then Build
 ```bash 
 source ./cxx.sh
 cargo build --target=wasm32-wasi
 ./opencv-wasm.sh
 ```


