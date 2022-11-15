# opencv-wasm  
  
## Requirements  
- [Wasi-SDK](https://github.com/WebAssembly/wasi-sdk) is assumed to be installed at `/opt`  

### Build  
```bash
cargo build --target=wasm32-wasi  --release
```  
###  Run an example  
```bash
cp test/relax.jp2 target/wasm32-wasi/release
cd target/wasm32-wasi/release
wasmedge --dir=. read.wasm ./relax.jp2  
```
Write example
```bash
cp test/relax.jp2 target/wasm32-wasi/release
cd target/wasm32-wasi/release
wasmedge --dir=. write.wasm ./relax.jp2  
```

