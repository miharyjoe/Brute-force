## Web Directory Brute Forcing
Web Directory Brute Forcing is a Rust program designed to brute force directories on a website using a provided wordlist.

## Requierement

Before you begin, ensure you have met the following requirements:
- Rust installed on your system. If not, you can install it from [here](https://www.rust-lang.org/tools/install).
- A wordlist file containing the directory names you want to brute force.

## How to launch

Follow these steps to build, compile, and run the program:

1. Clone the repository:
   ```bash
   git clone https://github.com/miharyjoe/Brute-force.git
    ```
2. Go to the project directory :
    ```bash
    cd Brute_force
    ```
3. Build the project :
    ```bash
   cargo build --release
    ```
4. Go to the release build and lanch the command :
   ```bash
   cd target/release

   ./web_directory_brute_forcing -u <url target> -w <path to the worldlist>
   ```

## Contributor

- Full Name : ANDRIMILANTO Mihary Joël
- Reference: STD21004
- mail: hei.mihary.2@gmail.com
