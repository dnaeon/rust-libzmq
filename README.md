## rust-libzmq

Rust low-level bindings to [libzmq](https://github.com/zeromq/libzmq).

Current version of `rust-libzmq` is built against libzmq version 4.2.0.

If you are looking for the high-level Rust ZeroMQ bindings, please
check [rust-zmq](https://github.com/erickt/rust-zmq).

## Using the library

In order to make use of `rust-libzmq` in your Rust project, first
create a new Rust project and add `rust-libzmq` as a dependency to
your project.

Here is an example project using `rust-libzmq`, which will get the
version of the libzmq library that you have installed on your
system.

```bash
$ cargo new --bin libzmq-version
```

This is how my `Cargo.toml` file looks like.

```toml
[package]
name = "libzmq-version"
version = "0.1.0"
authors = ["Marin Atanasov Nikolov <dnaeon@gmail.com>"]

[dependencies.libc]
version = "*"

[dependencies.libzmq]
git = "https://github.com/dnaeon/rust-libzmq.git"
```

And here is the code for our little project.

```rust
extern crate libc;
extern crate libzmq;

unsafe fn print_version() {
    let mut major = 0;
    let mut minor = 0;
    let mut patch = 0;

    libzmq::zmq_version(&mut major, &mut minor, &mut patch);
    println!("Installed ZeroMQ version is {}.{}.{}", major, minor, patch);
}

fn main() {
    unsafe { print_version(); }
}
```

Once ready, simply build and run the project.

```bash
$ cargo run
```

`rust-libzmq` is Open Source and licensed under the
[BSD License](http://opensource.org/licenses/BSD-2-Clause).

Contributions
=============

`rust-libzmq` is hosted on
[Github](https://github.com/dnaeon/rust-libzmq).
Please contribute by reporting issues, suggesting features or by
sending patches using pull requests.

Bugs
====

Probably. If you experience a bug issue, please report it to the
[rust-libzmq issue tracker on Github](https://github.com/dnaeon/rust-libzmq/issues>).
