import init, { run_app } from './pkg/tolp.js';
async function main() {
   await init('/pkg/tolp_bg.wasm');
   run_app();
}
main()
