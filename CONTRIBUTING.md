# Contributing to weather-rs

Thank you for considering contributing to the weather-rs project! We appreciate your interest in making this project even better. To ensure a smooth and collaborative contribution process, please follow the guidelines below.

## Code Conventions

### Rust Code Convention

Please follow the [Rust Code Convention](https://rust-lang.github.io/api-guidelines/naming.html) while writing code. This helps maintain consistency and readability throughout the codebase.

### Clippy Tool

We recommend using the [Clippy](https://github.com/rust-lang/rust-clippy) tool to ensure code quality. Running Clippy checks can help catch potential issues and improve code clarity.

### KISS (Keep It Simple, Stupid) and SRP (Single Responsibility Principle)

When contributing to the codebase, adhere to the KISS and SRP principles. Keep code simple and ensure that each module, function, or method has a single responsibility, making the codebase more maintainable and understandable.

### Initialization Order in Files

To enhance navigation within the codebase, please follow this recommended initialization order when writing code in Rust files:

1. Module export and reexport using the `mod` keyword.
2. Import external crates.
3. Import internal crates or modules.
4. Define constants and static values.
5. Define error enums.
6. Define enums.
7. Define traits.
8. Define structs along with their associated functions, methods, and trait implementations.
9. Define functions.
10. Define test functions.

This suggested order is not a strict rule but helps make the code more organized and easier to navigate.

## Testing

To maintain code quality and ensure that new contributions do not introduce regressions, please follow these guidelines for testing:

- Run `cargo test` for all packages in the workspace to check for regressions across the entire project.
- If you are working on a specific package within the workspace, run `cargo test -p <package_name>` to focus on testing that package.

For testing, the codebase uses the following conventions:

- Use `rstest` for test cases.
- Use `tokio::test` for asynchronous testing.
- Submodules used for testing specific parts of the program should have names like `tests_<name_of_module_or_struct`.
- Test function, associative function, or method names should follow the pattern `test_<name_of_function_or_method>_<expected_result_or_condition>`.

## Future Rule Changes

Please note that while there are not many strict rules currently, they may be subject to change in the future. It's essential to stay updated with the project's documentation and discussions for any potential rule changes or additions.

We appreciate your contributions and adherence to these guidelines, as they help maintain a high-quality and well-organized codebase. Thank you for being a part of the weather-rs project! 🌦️