WASI_SDK_VERSION=20.0
WASI_CMD="docker run --platform=linux/amd64 -t -v$(pwd):/workdir -w /workdir ghcr.io/vmware-labs/wasmlabs/wasi-builder:${WASI_SDK_VERSION}"
#cd wasm-wrapper-c/; ./build-wasm.sh --clean; cd ..
echo -e "$(date ) | Building wasm-wrapper-c ${WASI_CMD:+with '$WASI_CMD' }(logs silenced to build.log)..."  | tee -a build.log
${WASI_CMD} bash -c "cd wasm-wrapper-c; ./build-wasm.sh --clean >>../build.log 2>&1" || fail

echo $(extism --version)

extism call wasm-wrapper-c/target/wasm32-wasi/wasm-wrapper-c.wasm run_it --wasi --allow-path ./py-plugin:/plugin --allow-path ./wasm-wrapper-c/target/wasm32-wasi/deps/usr:/usr --input "hello" -v --log-level=info
