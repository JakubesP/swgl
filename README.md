# SWGL

<p align="center">
  <img src="https://github.com/JakubesP/swgl/blob/main/logo.png?raw=true">
</p>
 
#### Simple Web Graphics Library

SWGL is a simple library that makes it easy to work with WASM/WebGL and web_sys for the Rust language.

## Install and running the examples

You will need wasm-pack and node & npm

```sh
git clone --recurse-submodules https://github.com/JakubesP/swgl
cd swgl/examples/[example dir]
wasm-pack build
cd www
npm i
npm run start
```

## Doc

```sh
cargo doc --no-deps --open
```

