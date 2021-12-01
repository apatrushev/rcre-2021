```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add thumbv7em-none-eabihf
cargo install cargo-flash
git clone https://github.com/rubberduck203/stm32f3-discovery.git
cd stm32f3-discovery
cargo flash --example blinky --chip STM32F303VCTx
```
