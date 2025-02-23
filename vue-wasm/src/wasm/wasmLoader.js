import init, { fib } from './wasm_rust';

export async function loadWasm(){
    await init();
    return { fib }
}