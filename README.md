# Rust driver for bq24196 battery controller

bq24196: I2C Controlled 2.5-A Single Cell USB/Adapter Charger with Narrow VDC Power Path Management and USB OTG.

This driver uses the platform-agnostics [`embedded-hal`] traits.

All registers are defined in [`src/registers.rs`](src/registers.rs), but only a fraction of functionality is currently conveniently exposed. I'm happy to accept PRs to extend this.

## Example

```rust
use bq24196::{BQ24196, InputCurrentLimit};

// Initialize bs24196 on a given i2c bus.
// This checks part number register to verify device.
let mut battery = BQ24196::new(i2c).expect("Failed to initialize bq24196");

// Set input current limit in REG00::IINLIM
battery.set_input_current_limit(InputCurrentLimit::IINLIM_1500mA).unwrap();

if battery.has_fault().unwrap() {
  println!("Battery has active faults!");
}
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   <http://opensource.org/licenses/MIT>)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
