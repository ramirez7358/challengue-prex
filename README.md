# Actix Web Application

This repository contains the source code of a web application built with Actix Web in order to complete the [Prex](https://www.prexcard.com.ar/) technical challenge.

## Prerequisites

Before running or building the application, ensure you have the following installed:
- Rust: You can install Rust using [rustup](https://rustup.rs/). This will also install `cargo`, Rust's package manager and build tool.
- Git (optional): For cloning the repository.

## Running the Application Locally

To run the application on your local machine, follow these steps:

1. **Clone the Repository (Optional)**

   If you have Git installed, clone the repository using:
   ```sh
   git clone https://github.com/ramirez7358/challengue-prex
   cd challengue-prex
   ```

   If you don't have Git, you can download the source code directly from GitHub and extract it.

2. **Run the Application**

   Navigate to the root directory of the application and run:
   ```sh
   cargo run
   ```

   This command will compile the application and start the server. By default, the server will listen on `http://localhost:8080`.

3. **Accessing the Application**

   Once the server is running, you can access the application by opening `http://localhost:8080/app` in your web browser.

## Building the Application

To build the application for release:

1. Navigate to the root directory of the application.

2. Run the build command:
   ```sh
   cargo build --release
   ```

   This will compile the application and place the executable in the `target/release` directory.

3. The executable can be found at `target/release/store_balances`.