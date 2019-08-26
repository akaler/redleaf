### Setup
```
curl https://sh.rustup.rs -sSf | sh
rustup override add nightly
rustup component add llvm-tools-preview
cargo install cargo-xbuild
cargo xbuild --target x86_64-redleaf.json 
rustup component add rust-src
cargo xbuild --target x86_64-redleaf.json
```
### Prerequisites
* qemu

### Run
```
make iso
```

Install `xorriso` package, if `grub-mkrescue` complains
```
grub-mkrescue: warning: Your xorriso doesn't support `--grub2-boot-info'. Some features are disabled. Please use xorriso 1.2.9 or later..                                                 
```

```
make run
```

### Notes

The baremetal Rust setup (features, linking, etc. is best describe in https://os.phil-opp.com/set-up-rust/)
