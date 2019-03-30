# Semaphore demo CI pipeline using Rust

This is an example program and CI pipeline showing how to run a Rust project on Semaphore 2.0.

The program is a simple hello-world HTTP server implemented with the Hyper asynchronous library.

## Local project setup

This sequence describes a setup on a newly installed Linux system. If some of the tools have already been installed, you may just skip those steps.

1. Install Rust with `curl https://sh.rustup.rs -sSf | sh`
2. Clone the project with `git clone ${REPOSITORY}`
3. Run the project with `cargo run --verbose --release -- --port 8080`

The release build of the Hyper HTTP server is now running in your terminal on port 8080.

## CI on Semaphore

The workflow executes the following sequence of actions:
1. Installs the formatter `rustfmt` and the linter `clippy` using `rustup`
2. * Gets the linter build and ~/.cargo from the cache
   * Checks formatting with the `rustfmt` formatter
   * Checks the source code with the `clippy` linter
   * Puts the linter build to the cache
3. * Gets the main build and ~/.cargo from the cache
   * Builds the project
   * Runs all the tests
   * Puts the main build and ~/.cargo to the cache

Steps **2** and **3** are executed in parallel.

Linter and main builds are different, so they are built and stored separately.

## License

Copyright (c) 2019 Rendered Text

Distributed under the MIT License. See the file LICENSE.
