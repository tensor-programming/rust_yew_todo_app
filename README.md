# Rust Yew Todo Application

# A simple Rust client side Todo App using the Yew Framework.  

#### Major Props to [icode-live](https://github.com/icode-live) for [updating](https://github.com/tensor-programming/rust_yew_todo_app/pull/1/commits/a1ea01f303224c26b54fbda717b7dfff06256f8f) this repository for the latest version of Yew. 

### Running it

Clone or download this repository.

You need to have cargo-web installed as well as a suitable target for the Rust compiler to generate web output. 
By default cargo-web uses asmjs-unknown-emscripten. 

#### Rust stable

Install cargo-web and the asmjs and wasm32 emscripten targets as follows:

```
$ cargo install cargo-web
$ rustup target add asmjs-unknown-emscripten
$ rustup target add wasm32-unknown-emscripten
```

For normal Debug build run 
```
$ cargo web start
```

To run an optimised build instead of a debug build use:

```
$ cargo web start --target-webasm-emscripten=wasm32-unknown-emscripten --release
```

#### Rust nightly
If you are using rust nightly you can use the brand new `wasm32-unknown-unknown` target

```
$ cargo install cargo-web
$ rustup target add wasm32-unknown-unknown
$ cargo web start --target-webasm=wasm32-unknown-unknown
```

### Check out the Youtube Tutorial for this [Rust Tutorial](https://youtu.be/j8EnB7gkygw).  Here is our [Youtube Channel](https://www.youtube.com/channel/UCYqCZOwHbnPwyjawKfE21wg) go ahead and subscribe for more content.

### Check out our blog at [tensor-programming.com](http://tensor-programming.com/).

### Our [Twitter](https://twitter.com/TensorProgram), our [facebook](https://www.facebook.com/Tensor-Programming-1197847143611799/) and our [Steemit](https://steemit.com/@tensor).


