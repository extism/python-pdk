const createPlugin = require("@extism/extism").default

const wasm = {
  url: '../wasm-wrapper-c/target/wasm32-wasi/wasm-wrapper-c.wasm'
}

const opts = {
  allowedPaths: {
    "/plugin": "../py-plugin",
    "/usr": "../wasm-wrapper-c/target/wasm32-wasi/deps/usr"
  }
}

createPlugin(wasm, {
  useWasi: true,
  options: opts,
}).then((plugin) => {
  console.log(plugin);
  plugin.call('run_it', '').then(console.log)
})

