# Journaly Backend
This repository contains the backend code for Journaly.

## Getting Started
The steps below will help you get started on contributing to Journaly's backend.
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
