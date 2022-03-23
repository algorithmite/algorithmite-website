import init, { run_app } from '../pkg/algorithmite_website_client.js';
async function main() {
   await init('../pkg/algorithmite_website_client.wasm');
   run_app();
}
main()