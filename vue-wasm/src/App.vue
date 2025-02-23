<script setup>
import { ref } from 'vue';
import { loadWasm } from './wasm/wasmLoader';
const result = ref(0);
const count = ref(0);

const calculate_fib = async() =>{
  const wasm = await loadWasm();
  count.value ++;
  result.value = wasm.fib(count.value)
}
</script>

<template>
  
  <div>
    <h1>WASM DEMO</h1>
    <h2>Rust + Vue3 + WASM</h2>
    <button @click="calculate_fib">斐波拉契数列第{{ count }}项：{{ result }}</button>  
  </div>
  
  

</template>

<style scoped>
  button{
    background-color: #444;
  }
</style>
