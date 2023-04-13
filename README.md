# random password generator
A command-line tool to generate random passwords with a specified length and set of characters.

## Build

```
cargo build --release
./target/release/random_password_generator 12 "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()"
```
This command-line tool takes two arguments: the desired length of the password and the set of characters to choose from. It generates a random password by selecting characters from the provided set. The example above generates a 12-character password using uppercase and lowercase letters, digits, and special characters.
