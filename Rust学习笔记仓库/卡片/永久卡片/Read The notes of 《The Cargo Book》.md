whe I use rustc to complie my sourse code I need to specify the file name explicitly.
```bash
rustc .\src\main.rs
main.exe
```

```rust
fn main() {
	println!("hello_world");
}
```

Obtaining the correct versions of all the necessary dependencies and keeping them up to date.

Cargo, a higher-level [“_package_”](https://doc.rust-lang.org/cargo/appendix/glossary.html#package "\"package\" (glossary entry)") abstraction, a [_package manager_](https://doc.rust-lang.org/cargo/appendix/glossary.html#package-manager "\"package manager\" (glossary entry)").

Cargo does four things:
- Introduces two metadata files with various bits of package information.
- Fetches and builds your package’s dependencies.
- Invokes `rustc` or another build tool with the correct parameters to build your package.
- Introduces conventions to make working with Rust packages easier.

# Cargo Guide
## Creating a New Package
