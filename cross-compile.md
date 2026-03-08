# How to Cross Compile for Windows from Ubuntu

```console
sudo apt install clang lld llvm
rustup target add x86_64-pc-windows-msvc
cargo install xwin
cargo install cargo-xwin
cargo xwin build --target x86_64-pc-windows-msvc
```