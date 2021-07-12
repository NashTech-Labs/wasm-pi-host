# wasm-pi-host

This template will help you to build a host program in Rust to host a wasm module on a Raspberry Pi 2+ devices.
I am using -

- Rust to write the host code
- Wasm as compile target for demo battery indicator
- armv7 as a compile target for Raspberry Pi 2+ devices.

## Building the project

- Compile the battery_indicator module to wasm by executing the command.
  ```
  cargo build --release --target wasm32-unknown-unknown
  ```
- Copy the compiled wasm module to any folder.
- Change the MODULE_DIR and MODULE_FILE variables in the main.rs of the pihost module with the loaction of your compiled wasm module. 
- Now run the command in root folder of your project to build your project. 
  ```
  cargo build
  ```
## Run the project

- Run the command in root folder of your project.
  ```
  cargo run
  ```
  
Bingo, you have successfully executed a demo host program for Raspberry Pi. You can `scp` this program in your Raspberry Pi to see the LEDs light up.
