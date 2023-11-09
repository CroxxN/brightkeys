# A quick tool to set or unset brightness values of unix /sys/class/*/leds files

## How to use?

- `cargo build --release`
- [**IMPORTANT**] Set bypasses fileaccess using `sudo setcap 'cap_dac_override=ep' ./target/release/brightkeys`. This allows brightkeys to modify your 
`.../leds/brightness` files. If your user already has the necessary permissions, you can skip this process. Check by `echo 1 > /sys/class/{device_to_control}/leds/brightness`. If you encounter error, the user doesn't have enough permission.
- Copy the binary from `$(DOWN_DIR)/target/release/brightkeys` to your path
