# SWGL

  <img align="center" src="https://github.com/JakubesP/swgl/blob/main/logo.jpg?raw=true">
 
#### Simple Web Graphics Library

SWGL is a simple library that makes it easy to work with WASM/WebGL and web_sys for the Rust language.

## About

The project is still in development, so there may be some minor bugs or performance issues. The library will appear on crates.io in the future.

Nearest goals:
- Rendering and computation on separate threads.
- Easy to use high-level API.
- More extensive documentation and usage examples. 

Feel free to send me pull requests, comments or bug reports, I'd really appreciate it :smiley:

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

