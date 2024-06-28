# Chance-Encounters
Uses Google Location History to find out when people were unknowingly near each other in both a spatial and temporal sense

## Project Setup

This guide will help you set up the project by installing necessary dependencies and tools.

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js and npm](https://nodejs.org/)

### Steps

1. **Add WASM as a target to rustup**

    ```sh
    rustup target add wasm32-unknown-unknown
    ```

2. **Install Trunk**

    Trunk is a WASM web application bundler for Rust.
    
    ```sh
    cargo install trunk
    ```

3. **Install cargo-make**

    cargo-make is a task runner for Rust.
    
    ```sh
    cargo install cargo-make
    ```

4. **Download Node.js dependencies**
    
    ```sh
    npm install
    ```

    This will install the dependencies listed in the `package.json` file.

5. **Run locally**

    Navigate to the project directory and run:
    
    ```sh
    trunk serve --open
    ```

    This will install the dependencies listed in the `Cargo.toml` file and serve the application locally

6. **More Options**

    Take a look at Makefile.toml for cargo-make targets. Here is one example

    ```sh
    cargo make run-dev
    ```

    This runs the same command as Step 5.