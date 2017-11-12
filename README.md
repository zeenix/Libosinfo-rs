### Rust bindings for Libosinfo

These are Rust language bindings for [Libosinfo](https://libosinfo.org/). The goal of libosinfo is to provide a single place containing all the information about an operating system that is required in order to provision and manage it in a virtualized environment.

## Updating the bindings

We rely on [gtk-rs/gir tool](https://github.com/gtk-rs/gir) to generate the bindings and it's included as a submodule so the first thing you want to do is:

```
git submodule update
cd gir
cargo build
cd ..
```

Now you have gir tool ready for use. On to actual bindings...

# FFI (sys) bindings

FFI bindings are the low-level unsafe bindings (mostly) generated automatically. To ensure that we don't break API without noticing, we keep the generated bindings in git. To re-generate them against the latest libosinfo on your machine, you use the following command:

```
./gir/target/release/gir -c sys.toml -o sys
```

If you (for some reason) want to use the FFI bindings rather than the High-level ones, you can simply build them:

```
cd sys
cargo build
```

# High-level safe API

High-level docs are similar:

```
./gir/target/release/gir -c Gir.toml -o api
cd api
cargo build
cd ..
```
