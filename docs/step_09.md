```
$ cargo install probe-run
$ cargo add rtt-target --features cortex-m
    Updating 'https://github.com/rust-lang/crates.io-index' index
      Adding rtt-target v0.3.1 to dependencies with features: ["cortex-m"]
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.11s
     Running `probe-run --chip STM32F303VCTx --connect-under-reset target/thumbv7em-none-eabihf/debug/rcre`
(HOST) INFO  flashing program (16 pages / 16.00 KiB)
(HOST) INFO  success!
────────────────────────────────────────────────────────────────────────────────
Starting
Started
Click
Click
^C────────────────────────────────────────────────────────────────────────────────
stack backtrace:
   0: rcre::idle
        at src/main.rs:59:13
   1: main
        at src/main.rs:19:1
   2: ResetTrampoline
        at /Users/tosha/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.15/src/lib.rs:547:26
   3: Reset
        at /Users/tosha/.cargo/registry/src/github.com-1ecc6299db9ec823/cortex-m-rt-0.6.15/src/lib.rs:550:13
(HOST) INFO  device halted by user
$ cargo embed --chip STM32F303VCTx
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
      Config default
      Target /Users/tosha/workspace/apatrushev/rustcon/workspace/rcre/target/thumbv7em-none-eabihf/debug/rcre
     Erasing sectors ✔ [00:00:00] [#######################################################################################################################################################################################] 16.00KiB/16.00KiB @ 46.72KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [#######################################################################################################################################################################################] 16.00KiB/16.00KiB @ 11.83KiB/s (eta 0s )
    Finished flashing in 0.998s
Shutting down.
```
