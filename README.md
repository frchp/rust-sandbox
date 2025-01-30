## Dependencies :
[rustup](https://rustup.rs/) will install Rust, Cargo.
Install Vs Code [Rust-Analyzer extension](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
[Renode](https://builds.renode.io/renode-latest.msi)

## Misc :
- To create project : `cargo new --bin`
- To initialize already created folder : `cargo init --bin`
- To build and run : `cargo run`
- To install new crate : `cargo install __nameofyourcrate__`

## Embedded Rust
- Needed installation, see [stm32l552 repo](https://github.com/frchp/stm32-l552-sandbox)
- Get [standard library for your target](https://doc.rust-lang.org/rustc/platform-support/arm-none-eabi.html): `rustup target add thumbv7em-none-eabihf`

|Cortex M Type|Target|
|--|--|
| M0 M1 | thumbv6m-none-eabi |
| M3 | thumbv7m-none-eabi |
| M4 M7 | thumbv7em-none-eabi |
| M4F M7F | thumbv7em-none-eabihf |
| M23 | thumbv8m.base-none-eabi |
| M33 M35P M55 M85 | thumbv8m.main-none-eabi / thumbv8m.main-none-eabihf |

- Build `cargo build` or `cargo build --target thumbv8m.main-none-eabihf`
- Check exe `cargo readobj --target thumbv8m.main-none-eabihf --bin rust-sandbox -- --file-header`
- Check size `cargo size --target thumbv8m.main-none-eabihf --bin rust-sandbox -- -A`

## Run renode emulation :
Run `run_renode.bat`

OR

Run & Debug in VSCode : `Debug application in Renode`

## Doc :
[Embedded Rust](https://docs.rust-embedded.org/book/)
[Rust on ST Discovery](https://docs.rust-embedded.org/discovery/f3discovery/)
[Renode tutorial](https://interrupt.memfault.com/blog/intro-to-renode#automating-setup-with-a-resc-script)
[stm32-rs](https://github.com/stm32-rs/stm32-rs)