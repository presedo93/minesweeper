# Minesweeper

This project consists on the minesweeper game. It is done using Rust and Web Aseembly and can be played in the browser. I used this project as a guide [minesweeper](https://github.com/yishn/lets-code/tree/main/minesweeper), and it has been a great way to start with WA.

## Compile

To compile the code we need to use the wasm toolkit, so we need to install [wasm-pack](https://rustwasm.github.io/wasm-pack/):

    $ wasm-pack build --target web

## Deploy

To run the project, we need a static file server, we can install:

    $ npm install serve -g

Then, we only need to serve the `index.html` file:

    $ serve

## References

All the steps followed to do this project have been taken from [this video](https://www.youtube.com/watch?v=0ywizYLPV00). Special thanks to [yishn](https://github.com/yishn)!
