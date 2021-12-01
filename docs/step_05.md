Добавим зависимость от cortex-m и получим доступ к переферии:
```
$ cargo add cortex-m
    Updating 'https://github.com/rust-lang/crates.io-index' index
      Adding cortex-m v0.7.3 to dependencies
$ cargo flash --target thumbv7em-none-eabihf --chip STM32F303VCTx
   Compiling rcre v0.1.0 (/Users/tosha/workspace/apatrushev/rustcon/workspace/rcre)
    Finished dev [unoptimized + debuginfo] target(s) in 0.67s
    Flashing /Users/tosha/workspace/apatrushev/rustcon/workspace/rcre/target/thumbv7em-none-eabihf/debug/rcre
     Erasing sectors ✔ [00:00:00] [#######################################################################################################################################################################################]  8.00KiB/ 8.00KiB @ 42.91KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [#######################################################################################################################################################################################]  7.00KiB/ 7.00KiB @ 10.68KiB/s (eta 0s )
    Finished in 0.478s
```

Добавим ещё немного красоты (она как известно - вещь субъективная, у меня вот такая, и хороша она только по ночам):
```
$ rustup install nightly
info: syncing channel updates for 'nightly-aarch64-apple-darwin'

  nightly-aarch64-apple-darwin unchanged - rustc 1.59.0-nightly (207c80f10 2021-11-30)

info: checking for self-updates
$ cargo +nightly fmt
```
