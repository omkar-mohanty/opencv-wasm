export WASI_SDK_PATH="/opt/wasi-sdk"
export WASI_SYSROOT="${WASI_SDK_PATH}/share/sysroot"
export CXX="${WASI_SDK_PATH}/bin/clang++ --sysroot=${WASI_SYSROOT}"		
export AR="${WASI_SDK_PATH}/bin/llvm-ar"
export CCX_wasm32_wasi="${CXX}"
export CARGO_TARGET_WASM32_WASI_LINKER="${WASI_SDK_PATH}/bin/wasm-ld"
export PKG_CONFIG_PATH=
export PKG_CONFIG_SYSROOT_DIR=${WASI_SYSROOT}