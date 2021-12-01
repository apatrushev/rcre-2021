Добавим поддержку ядра нашего процессора и нашей доски:
```
$ cargo add cortex-m-rt stm32f3-discovery
    Updating 'https://github.com/rust-lang/crates.io-index' index
      Adding cortex-m-rt v0.7.1 to dependencies
      Adding stm32f3-discovery v0.7.2 to dependencies
```

И скомпилируем:
```
$ cargo build --target thumbv7em-none-eabihf --release
    Updating crates.io index
error: failed to select a version for `cortex-m-rt`.
    ... required by package `stm32f3-discovery v0.7.2`
    ... which satisfies dependency `stm32f3-discovery = "^0.7.2"` of package `rcre v0.1.0 ([skipped]/workspace/rcre)`
versions that meet the requirements `^0.6.14` are: 0.6.15

the package `cortex-m-rt` links to the native library `cortex-m-rt`, but it conflicts with a previous package which links to `cortex-m-rt` as well:
package `cortex-m-rt v0.7.1`
    ... which satisfies dependency `cortex-m-rt = "^0.7.1"` of package `rcre v0.1.0 ([skipped]/workspace/rcre)`
Only one package in the dependency graph may specify the same links value. This helps ensure that only one copy of a native library is linked in the final binary. Try to adjust your dependencies so that only one package uses the links ='cortex-m-rt' value. For more information, see https://doc.rust-lang.org/cargo/reference/resolver.html#links.

failed to select a version for `cortex-m-rt` which could resolve this conflict
```

Поправим версию, чтоб починить проблему:
```
$ cargo build --target thumbv7em-none-eabihf --release
   Compiling rcre v0.1.0 ([skipped]/workspace/rcre)
    Finished release [optimized] target(s) in 0.45s
```

Но наша программа ещё ничего не делает! Добавим main:
```
$ cargo build --target thumbv7em-none-eabihf --release
   Compiling rcre v0.1.0 ([skipped]/workspace/rcre)
    Finished release [optimized] target(s) in 0.27s
```

Отлично, попробуем прошить:
```
$ cargo flash --target thumbv7em-none-eabihf
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
    Flashing [skipped]/workspace/rcre/target/thumbv7em-none-eabihf/debug/rcre
        WARN probe_rs::architecture::arm::memory::romtable > Component at 0xe0001000: CIDR0 has invalid preamble (expected 0xd, got 0x0)
        WARN probe_rs::architecture::arm::memory::romtable > Component at 0xe0001000: CIDR2 has invalid preamble (expected 0x5, got 0x0)
        WARN probe_rs::architecture::arm::memory::romtable > Component at 0xe0001000: CIDR3 has invalid preamble (expected 0xb1, got 0x0)
        WARN probe_rs::architecture::arm::memory::romtable > Component at 0xe0000000: CIDR0 has invalid preamble (expected 0xd, got 0xb1)
        WARN probe_rs::architecture::arm::memory::romtable > Component at 0xe0000000: CIDR1 has invalid preamble (expected 0x0, got 0x1)
        WARN probe_rs::architecture::arm::memory::romtable > Component at 0xe0000000: CIDR2 has invalid preamble (expected 0x5, got 0xb1)
        WARN probe_rs::architecture::arm::memory::romtable > Component at 0xe0040000: CIDR0 has invalid preamble (expected 0xd, got 0x0)
        WARN probe_rs::architecture::arm::memory::romtable > Component at 0xe0040000: CIDR2 has invalid preamble (expected 0x5, got 0x0)
        WARN probe_rs::architecture::arm::memory::romtable > Component at 0xe0040000: CIDR3 has invalid preamble (expected 0xb1, got 0x0)

       Error Connecting to the chip was unsuccessful.
        Hint Try specifying your chip with the `--chip` argument.
        Hint You can list all the available chips by passing the `--list-chips` argument.
```

Выбираем чип:
```
$ cargo flash --list-chips | grep VCT | grep 303
        STM32F303VCTx
$ cargo flash --target thumbv7em-none-eabihf --chip STM32F303VCTx
    Finished dev [unoptimized + debuginfo] target(s) in 0.04s
    Flashing [skipped]/workspace/rcre/target/thumbv7em-none-eabihf/debug/rcre
        WARN probe_rs::flashing::loader > No loadable segments were found in the ELF file.

       Error No loadable ELF sections were found.
        Hint Please make sure your linker script is correct and not missing at all.
        Hint If you are working with Rust, check your `.cargo/config.toml`? If you are new to the rust-embedded ecosystem, please head over to https://github.com/rust-embedded/cortex-m-quickstart.
```

Объясняем линкеру что мы от него хотим в `.cargo/config`, как он и просит:
```
$ cargo flash --target thumbv7em-none-eabihf --chip STM32F303VCTx
   Compiling rcre v0.1.0 ([skipped]/workspace/rcre)
error: could not compile `rcre` due to 2 previous errors
error: linking with `rust-lld` failed: exit status: 1
  |
  [skipped]
  = note: rust-lld: error:
          ERROR(cortex-m-rt): The interrupt vectors are missing.
          Possible solutions, from most likely to less likely:
          - Link to a svd2rust generated device crate
          - Disable the 'device' feature of cortex-m-rt to build a generic application (a dependency
          may be enabling it)
          - Supply the interrupt handlers yourself. Check the documentation for details.
  [skipped]

error: aborting due to previous error
       Error Failed to build the cargo project.
        Hint 'cargo build' was not successful. Have a look at the error output above.
        Hint Make sure the working directory you selected is indeed a cargo project with a Cargo.toml in it
```

Добавим процессорную крату ([список](https://github.com/stm32-rs/stm32-rs-nightlies)), (и сделаем use!!!):
```
$ cargo add stm32f3 --features stm32f303
    Updating 'https://github.com/rust-lang/crates.io-index' index
      Adding stm32f3 v0.14.0 to dependencies with features: ["stm32f303"]
$ cargo flash --target thumbv7em-none-eabihf --chip STM32F303VCTx
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
warning: unused import: `stm32f3`
 --> src/main.rs:4:5
  |
4 | use stm32f3;
  |     ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: 1 warning emitted

    Flashing [skipped]/workspace/rcre/target/thumbv7em-none-eabihf/debug/rcre
     Erasing sectors ✔ [00:00:00] [#######################################################################################################################################################################################]  2.00KiB/ 2.00KiB @ 31.22KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [#######################################################################################################################################################################################]  2.00KiB/ 2.00KiB @ 11.90KiB/s (eta 0s )
    Finished in 0.191s
```

Ну и угомоним компилятор, наконец-то:
```
$ cargo flash --target thumbv7em-none-eabihf --chip STM32F303VCTx
   Compiling rcre v0.1.0 ([skipped]/workspace/rcre)
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
    Flashing [skipped]/workspace/rcre/target/thumbv7em-none-eabihf/debug/rcre
     Erasing sectors ✔ [00:00:00] [#######################################################################################################################################################################################]  2.00KiB/ 2.00KiB @ 31.03KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [#######################################################################################################################################################################################]  2.00KiB/ 2.00KiB @ 11.96KiB/s (eta 0s )
    Finished in 0.189s
```
