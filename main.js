import init, { run_app } from './pkg/rust_gun_wasm_play.js';
async function main() {
   await init('/pkg/rust_gun_wasm_play_bg.wasm');
   run_app();
}
main()
