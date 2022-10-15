# Chapter 1

You can use Rust without `cargo` CLI, using `rustc` compiler CLI. It would create an executable, that you can execute

```zsh
rustc main.rs;
./main;
```

Whereas `cargo` CLI stands as package manager and utility tool. Using `cargo` you can:
1. Create new package using `cargo new <pkg_name>`
2. Build a development executable (`target/debug/<pkg_name>`) using `cargo build` or `cargo b`
3. Build an optimized production executable in (`target/release/<pkg_name>`) using `cargo build --release` or `cargo b -r`
4. Build as `build` command and executes it immediately `cargo run` or `cargo r`
5. Check if code is compilable using `cargo check` or `cargo c`



> You should add `target` to ignored vcs folder 