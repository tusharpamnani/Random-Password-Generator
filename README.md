# Random Password Generator

## Overview

This Rust program generates a random password of a specified length. It utilizes Rust's random number generation and character conversion capabilities to create a password consisting of printable ASCII characters.

## Features

- **Random Password Generation:** Generates a password with random characters.
- **Customizable Length:** The length of the generated password is customizable.

## Getting Started

1. Clone the repository:

    ```bash
    git clone https://github.com/tusharpamnani/Random-Password-Generator.git
    cd Random-Password-Generator
    ```

2. Build and run the application:

    ```bash
    cargo run
    ```

3. Adjust the password length:

    - Open `main.rs`.
    - Locate the `password_length` variable and change its value to set the desired password length.

4. Run the application again:

    ```bash
    cargo run
    ```

5. The generated random password will be displayed in the console.

## Project Structure

- **src/main.rs:** Main entry point for the Rust program.
- **Cargo.toml:** Configuration file for Rust dependencies and settings.

## Usage

- Customize the password length by modifying the `password_length` variable in `main.rs`.
- Run the application to generate a new random password each time.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to enhance the functionality or add new features.

## License

This project is licensed under the [MIT License](LICENSE).
