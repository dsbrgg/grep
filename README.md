# Globally Search a Regular Expression and Print

I/O tutorial for practicing Rust language

- https://doc.rust-lang.org/book/ch12-00-an-io-project.html

Our grep project will combine a number of concepts you’ve learned so far:

- Organizing code (using what you learned about modules in [Chapter 7](https://doc.rust-lang.org/book/ch07-00-packages-crates-and-modules.html))
- Using vectors and strings (collections, [Chapter 8](https://doc.rust-lang.org/book/ch08-00-common-collections.html))
- Handling errors ([Chapter 9](https://doc.rust-lang.org/book/ch09-00-error-handling.html))
- Using traits and lifetimes where appropriate ([Chapter 10](https://doc.rust-lang.org/book/ch10-00-generics.html))
- Writing tests ([Chapter 11](https://doc.rust-lang.org/book/ch11-00-testing.html))

## Separation of Concerns for Binary Projects

The organizational problem of allocating responsibility for multiple tasks to the main function is common to many binary projects. As a result, the Rust community has developed a process to use as a guideline for splitting the separate concerns of a binary program when main starts getting large. The process has the following steps:

Split your program into a `main.rs` and a `lib.rs` and move your program’s logic to `lib.rs`.

As long as your command line parsing logic is small, it can remain in `main.rs`.

When the command line parsing logic starts getting complicated, extract it from `main.rs` and move it to `lib.rs`.

The responsibilities that remain in the main function after this process should be limited to the following:

- Calling the command line parsing logic with the argument values
- Setting up any other configuration
- Calling a run function in `lib.rs`
- Handling the error if run returns an error
