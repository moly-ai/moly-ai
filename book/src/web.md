# Web Support

Moly Kit supports WebAssembly out of the box. To run your app in the browser:

```shell
cargo makepad wasm --bindgen run -p your_application_package
```

```admonish info
You will need to have the `cargo makepad` CLI installed. Check Makepad's
documentation for more information.
```

```admonish warning
By default, Makepad uses its own glue code to work in a web browser and doesn't
work with `wasm-bindgen` out of the box.

The `--bindgen` argument passed to `cargo makepad` is important as it enables
`wasm-bindgen` interoperability in Makepad.

However, if you pass `--bindgen` but don't actually use `wasm-bindgen` anywhere
in your app, you may see errors on the browser console about missing values.
```
