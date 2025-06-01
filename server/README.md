# Journly Backend
This repository contains the backend code for Journly.

## Getting Started
The steps below will help you get started on contributing to Journly's backend.
1. Firstly, ensure that you have the Rust programming language installed to your system. To check this, open up your terminal and enter the command: `rustc`. If there is an error, go to the official [Rust installation guide](https://www.rust-lang.org/tools/install) and follow the instructions to install Rust.
2. Next, clone the repository.
```shell
git clone https://github.com/journaly-app/journaly-backend.git <DIRECTORY>
```
2. Upon cloning, simply CD into the project directory and run `cargo build` and you are free to get coding!


## Testing
To run tests, you first need to build the Postgres container using the Makefile in the project root directory. To do that, run the following command:
```
make postgres-on-docker
```
> Docker is required to run this command.

After the container is up and running, you can run `cargo test` to run tests.

### Errors with testing
An error message that you may run into when running `cargo test` is:

```
thread 'actix-rt|system:13|arbiter:214' panicked at /home/renchie/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/actix-server-2.5.1/src/worker.rs:429:30:
called `Result::unwrap()` on an `Err` value: Os { code: 24, kind: Uncategorized, message: "Too many open files" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

This happens because the test binary creates more file descriptors than available on the processes file descriptor table.

#### Linux Solution
We can manually increase the number of file descriptors that a process can have by setting `ulimit`.

The below command should sufficiently increase the limit where the tests should be able to run without the issue:
```
ulimit -n 65535
```
Note that this limit will only be changed for the current shell session. So in future sessions, the command will need to be executed again.

