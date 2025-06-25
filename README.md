# 🦀 Rust Learning Playground

A modular Rust project demonstrating core language simple operations, features, async programming, file downloads, REST API calls, and simple web server implementation using Axum.


## 📁 Project Structure

This project is organized into small modules, each focusing on a specific Rust concept.


### 🧱 Core Modules

Each module contains a `run()` function that demonstrates Rust syntax and behavior for:

- ✅ `print` – Basic printing
- ✅ `vars` – Variable binding and shadowing
- ✅ `types` – Primitive types and type inference
- ✅ `strings` – String handling
- ✅ `tuples` – Working with tuples
- ✅ `arrays` – Arrays and fixed-size collections
- ✅ `vectors` – Dynamic collections
- ✅ `conditionals` – If/else logic
- ✅ `loops` – Loop, while, for
- ✅ `functions` – Defining and calling functions
- ✅ `structs` – Structs and method implementations
- ✅ `ownership` – Ownership, moves, and borrowing
- ✅ `borrowing` – Mutable and immutable references
- ✅ `references` – Reference and dereference
- ✅ `forenheat` – Temperature converter (mini project)
- ✅ `calculator` – Basic calculator (mini project)
- ✅ `bankaccountsystem` – Bank account system (error handling & enums)
- ✅ `todos_api` – Fetches JSONPlaceholder posts using reqwest; demonstrates async/await, error handling, and parallel HTTP requests with join_all
- ✅ `scheduler` – Runs a scheduled background task using tokio::spawn; demonstrates async time-based execution, concurrent task spawning, and multi-threaded behavior with Tokio runtime
- ✅ `basic_apis` – Sets up basic GET and POST HTTP endpoints using Axum; demonstrates JSON serialization with Serde and route handling with async functions in a minimal API server


Run all the files from `main.rs` and make sure to uncomment the commented lines in main.rs one by one:

```bash
cargo run
