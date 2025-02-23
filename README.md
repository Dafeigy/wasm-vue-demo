![](https://s2.loli.net/2025/02/22/NwizZa3VO5IyM2L.png)

## 什么是WebAssembly

 WebAssembly（简称WASM）是一种可以在网页浏览器中运行的二进制指令格式。它由WebAssembly社区组（W3C的WebAssembly工作组）开发，旨在提供一种高效、安全且跨平台的计算方式，以支持现代Web应用的需求。

### WASM的工作原理

1. **二进制格式**：WASM是一种二进制格式，而不是像JavaScript那样的高级脚本语言。这种二进制格式使得编译器可以将高级语言（如C、C++、Rust等）编译成高效的机器代码，从而提高执行速度。

2. **模块化**：WASM模块是独立的、可重用的代码单元，可以包含函数、数据、表和内存等。这些模块可以通过WebAssembly的链接器进行组合，形成更复杂的程序。

3. **沙箱环境**：WASM运行在一个沙箱环境中，这意味着它只能访问特定的资源和功能，从而提高安全性。例如，WASM模块不能直接访问用户的文件系统或摄像头。

4. **跨平台**：WASM可以运行在任何支持WASM的平台上，包括浏览器、服务器和移动设备。这使得开发者可以使用一种语言编写代码，然后在不同的平台上运行。

### WASM的特点

WebAssembly具有以下几个显著的特点：

1. **高性能**：WASM是二进制格式，这意味着它比传统的脚本语言（如JavaScript）更高效。编译器可以将高级语言代码编译成高效的机器代码，从而提高执行速度。

2. **安全**：WASM运行在一个沙箱环境中，这意味着它只能访问特定的资源和功能。这种沙箱环境可以防止WASM模块执行恶意代码，从而提高安全性。

3. **跨平台**：WASM可以运行在任何支持WASM的平台上，包括浏览器、服务器和移动设备。这使得开发者可以使用一种语言编写代码，然后在不同的平台上运行。

4. **模块化**：WASM模块是独立的、可重用的代码单元，可以包含函数、数据、表和内存等。这些模块可以通过WebAssembly的链接器进行组合，形成更复杂的程序。

5. **低延迟**：由于WASM是二进制格式，因此它可以减少网络传输的开销，从而提高应用的响应速度。

6. **丰富的生态系统**：WASM有一个活跃的社区和丰富的生态系统，包括编译器、工具链、库和框架等。这使得开发者可以轻松地使用WASM进行开发。

7. **兼容性**：WASM可以与现有的Web技术（如HTML、CSS和JavaScript）无缝集成，从而使得开发者可以使用WASM来扩展和优化现有的Web应用。

### WASM的应用场景

1. **高性能计算**：WASM可以用于运行高性能计算任务，如游戏、视频编辑、数据分析等。由于WASM可以将高级语言编译成高效的机器代码，因此在这些场景中可以实现更好的性能。

2. **游戏开发**：WASM可以用于开发Web游戏，因为游戏通常需要高性能的计算能力和快速的响应速度。WASM可以将游戏逻辑和渲染代码编译成高效的机器代码，从而提高游戏的性能。

3. **桌面应用**：WASM可以用于开发桌面应用，因为桌面应用通常需要高性能的计算能力和丰富的用户界面。WASM可以将桌面应用的代码编译成高效的机器代码，从而提高应用的性能。

4. **服务器端应用**：WASM可以用于开发服务器端应用，因为服务器端应用通常需要处理大量的数据和计算任务。WASM可以将服务器端应用的代码编译成高效的机器代码，从而提高应用的性能。

5. **插件和扩展**：WASM可以用于开发浏览器插件和扩展，因为插件和扩展通常需要高性能的计算能力和丰富的用户界面。WASM可以将插件和扩展的代码编译成高效的机器代码，从而提高插件和扩展的性能。

总之，WASM是一种强大的工具，可以用于开发高性能、安全且跨平台的Web应用。随着WASM的不断发展和优化，它将在更多的场景中得到应用。

## Rust编写WASM模块

WASM生态中，Rust是一个比较活跃且稳定发展的WASM构建选项。Rust的开发环境推荐使用VSCode的`Rust-Analyzer`，有非常好的类型提示和方便的调试测试。

### 工具与开发环境配置

WASM的编译依赖工具：`wasm-pack` 和 `wasm-opt`。

```bash
cargo install wasm-pack
cargo install wasm-opt
```

随后在`cargo,.toml` 中添加`wasm-bindgen`依赖：

```toml
[package]
name = "wasm_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
wasm-bindgen = "0.2"

[lib]
crate-type = ["cdylib"]
```

### 代码调试与编译

尽管提供了一个用于测试的整数加法的函数，我们可以编写一个递归的斐波拉契的数列函数。

```rust
use wasm_bindgen::prelude::*; 

#[wasm_bindgen]
pub fn add(left: f32, right: f32) -> f32 {
    return left + right;
}
#[wasm_bindgen]
pub fn fib(n: i32) -> i32 {
    if n <= 0 {
        -1
    } else if (n == 1) || (n == 2) {
        1
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

```

需要注意的是，我们要对需要编译成wasm的函数前面加上`#[wasm_bindgen]`的宏才可以在前端中被调用。

随后通过`wasm-pack`将Rust编译成WebAssembly：

```bash
wasm-pack build --target web
```

![{93C2DAED-BAD6-4E64-944C-F996359B96CD}](https://s2.loli.net/2025/02/09/dBEP5SpYFostwi9.png)

成功的样子应该是这样的，你可以在目录找到`pkg`文件夹中的`.wasm`文件。

## 在前端项目使用WASM

直接在当前目录下新建一个Vue项目使用WASM，先按照下方的指令创建Vue项目：

```bash
npm create vue@latest
```

![{65877B01-D330-4CEC-8F47-6305BCB547AD}](https://s2.loli.net/2025/02/09/3mNHxr4uqiTLlAC.png)

```bash
cd vue-wasm
npm install
```

随后我们开始进行相关的配置。

### 前端项目的WASM配置

配置`vite.config.js`或`vite.config.ts`：

```typescript
export default defineConfig({
  assetsInclude: ["**/*.wasm"], // <= 添加这个以支持WASM
  plugins: [
    vue(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})
```

这样才能开启对`wasm`的文件加载支持。

### WASM加载器的使用

将编译好的文件夹复制到前端项目中，比如说新建一个`wasm`的文件夹，容纳其中的内容。随后新建一个js文件`wasmLoader.js`：

```javascript
import init, { fib } from './wasm_rust';

export async function loadWasm(){
    await init();
    return { fib }
}
```

这个组件将会暴露出我们要使用的函数，我们会在后面使用它。

### 组件使用WASM

给个简单的使用案例吧：

```vue
// App.vue
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

```

然后修改一下样式就好：

```
// src/assets/main.css:
*{
  padding: 0;
  margin: 0;
  color: darkseagreen;
}

#app{
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-shrink: 0;
  align-items: center;
  justify-content: center;
  background-color: rgb(29, 29, 29);
}

div{
  display: flex;
  align-items: center;
  flex-direction: column; 
}

h1{
  color: aliceblue;
  font-size: 3vw;
}

button{
  width: 20vw;
  height: 5vh;
  border: 2px solid rgb(114, 194, 170); 
  border-radius: 4px;
  cursor: pointer;
  margin-top: 5%;
  font-size: 4vw;
}
```

效果如图：

![{27DF79DE-19BB-4686-A2A3-349AB87CE2FD}](https://s2.loli.net/2025/02/22/qd5zKLb8TQXxyPo.png)

点击按钮会自增1，用于求斐波拉契的数列的第n项。大功告成~

