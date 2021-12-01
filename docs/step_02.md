Поддержка компиляции под наш микроконтроллер ([справочник](https://doc.rust-lang.org/nightly/rustc/platform-support.html)):
```
rustup target add thumbv7em-none-eabihf
```

Расширения для прошивки микроконтроллера и для управления проектом:
```
cargo install cargo-flash cargo-edit
```

Посмотреть что там у нас установлено можно так:
```
rustup show
cargo install --list
```
