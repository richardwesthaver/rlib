import init, { wasm_main } from "./pkg/ui.js";

async function run() {
  await init();
  wasm_main();
}

run();
