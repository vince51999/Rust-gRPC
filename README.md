# Rust-gRPC

This project is a robust and efficient implementation of a shop system using Rust and gRPC. 
The project simulate an online shop that start when 3 clients are subscribed.

## Setup project

The project has been tested on OS Ubuntu 22.04 and Mac OSX.

To compile the project and install all the dependencies you need to install **Rust** on your OS:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Run project

To run the project, you need to execute the following command:

Run the **server**:

```
cd server
cargo run
```

Run the **client**:

```
cd client
cargo run
```

**N.B. the shop start after 3 clients subscription.**
