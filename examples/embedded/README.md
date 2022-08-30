Simple example showing how `easer` can be used in embedded `no_std` contexts.

This example is designed to be run on the [MDBT50Q-RX
dongle](https://www.adafruit.com/product/5199), but should be easily adapted to
other nrf52840-based boards by changing the LED pin number.

# Flashing/running

If your board has an `nrfdfu`-compatible bootloader installed,

```shell
cargo install nrfdfu
cargo run --release
```

If your board has a UF2-compatible bootloader installed (such as the stock
Adafruit bootloader),

```shell
cargo install cargo-binutils uf2conv
cargo objcopy --release -- -O binary flash.bin
uf2conv flash.bin --base 0x26000 --family 0xADA52840
cp flash.bin /Volumes/MDBT50QBOOT/
```
