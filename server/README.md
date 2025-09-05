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

## Local development
1. Install `docker` and `docker-compose`
2. Run `docker-compose up -f docker-compose.yaml`

## Configuration
Configuration can be done through creating a `(CONFIG_FILE_NAME).toml` in the root directory of this project. In `main.rs`, the config struct is built using the `Server::build` function.
```rust
    let config = Server::build("config.toml");
```

This following are properties are required to be set in the coniguration file in order for the server to be runnable:
```toml
[base]
production=false
domain_name=""
ip_address=""
port=0000
allowed_origins=[""]

[postgres]
host=""
user=""
password=""
port=0000
db=""

[mailgun_smtp]
login=""
password=""
server=""

[google_oauth]
client_id=""
client_secret=""

[jwt_config]
access_secret=""
refresh_secret=""
algorithm=""
access_token_expiration=123
refresh_token_expiration=123
```

#### JWT Configuration
**Algorithm** specifies the algorithm used to sign the JWT tokens. Supported algorithms include:
- HS256 (Default)
- HS384
- HS512
- ES256
- ES384
- RS256
- RS384
- RS512
- PS256
- PS384
- PS512
- EdDSA

**Access/refresh token expiration** is in minutes. A good expiration time for access tokens is something short-lived like 5 minutes.


## Testing
### Writing Tests
Rust currently does not have a elgant solution for asynchronous clean up. Therefore, integration tests must be written following a very specific pattern as shown below.
```rust
#[actix_rt::test]
pub async fn test() {
  let test_app = spawn_app().await;

  let result = AssertUnwindSafe(async {
    // test logic goes here
  })
  .catch_unwind()
  .await;

  test_app.cleanup().await;

  if result.is_err() {
    panic!("Test failed due to panic.");
  }
}
```
This pattern allows us to catch any panics that occur in our test logic so that we can clean up before terminating the test thread.

A panic must be called in order for the test to be shown as a failed test case, which is why `panic!` is manually called at the end when we check if the test logic produced any errors.

### Running Tests
To run tests, you first need to build the Postgres container using the Makefile in the project root directory. To do that, run the following command:
```
make postgres-on-docker
```
> Docker is required to run this command.

After the container is up and running, you can run `cargo test` to run tests.

