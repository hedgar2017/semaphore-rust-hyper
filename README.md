# Semaphore demo CI pipeline using Rust

This is an example program and CI pipeline showing how to run a Rust project on Semaphore 2.0.

The program is a simple hello-world HTTP server implemented with the Hyper asynchronous framework.

## Local project setup

This sequence describes a setup on a newly installed Linux system. If some of the tools have already been installed, you may just skip those steps.

1. Install Rust with `curl https://sh.rustup.rs -sSf | sh`
2. Clone the project with `git clone ${REPOSITORY}`
3. Run the project with `cargo run --verbose --release -- --port 8080`

The release build of the Hyper HTTP server is now running in your terminal on port 8080.

## CI on Semaphore

The workflow executes the following actions:
1. Checkouts the source code from the Git repository
2. Gets the cargo package index from the cache
3. Checks the code with `rustfmt` and `clippy`
   1. Installs the formatter `rustfmt` and linter `clippy` using `rustup`
   2. Gets the linter build from the cache
   3. Checks formatting with the `rustfmt` formatter
   4. Checks the source code with the `clippy` linter
   5. Puts the linter build to the cache
4. Builds and tests the project
   1. Gets the default build from the cache
   2. Builds the project and runs the tests
   3. Puts the default build and ~/.cargo to the cache

Sequences **3** and **4** are executed in parallel.

Linter and default builds are different, so they are built and stored separately.

## License

Copyright (c) 2019 Rendered Text

Distributed under the MIT License. See the file LICENSE.
