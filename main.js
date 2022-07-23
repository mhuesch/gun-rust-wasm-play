import init, { run_app } from './pkg/gun_rust_wasm_play.js';
async function main() {
   await init('/pkg/gun_rust_wasm_play_bg.wasm');
   run_app();
}
main()
