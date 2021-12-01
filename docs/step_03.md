Пробуем собрать под целевую платформу:
```
$ cargo build --target thumbv7em-none-eabihf --release
   Compiling rcre v0.1.0 ([skipped]/workspace/rcre)
error[E0463]: can't find crate for `std`
  |
  = note: the `thumbv7em-none-eabihf` target may not be installed
  = help: consider downloading the target with `rustup target add thumbv7em-none-eabihf`

For more information about this error, try `rustc --explain E0463`.
error: could not compile `rcre` due to previous error
```

Добавляем `#![no_std]`:
```
$ cargo build --target thumbv7em-none-eabihf --release
   Compiling rcre v0.1.0 ([skipped]/workspace/rcre)
error: cannot find macro `println` in this scope
 --> src/main.rs:4:5
  |
4 |     println!("Hello, world!");
  |     ^^^^^^^

error: `#[panic_handler]` function required, but not found

error: could not compile `rcre` due to 2 previous errors
```

Заменим print на бесконечный цикл, и сделаем panic_handler:
```
$ cargo build --target thumbv7em-none-eabihf --release
   Compiling rcre v0.1.0 ([skipped]/workspace/rcre)
error: requires `start` lang_item

error: could not compile `rcre` due to previous error
```

Ну и сообщим что у нас нет main - `#![no_main]`:
```
$ cargo build --target thumbv7em-none-eabihf --release
   Compiling rcre v0.1.0 ([skipped]/workspace/rcre)
    Finished release [optimized] target(s) in 0.22s
```
