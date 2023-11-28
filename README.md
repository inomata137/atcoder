## Rust template for AtCoder
### Usage
#### 1. Create a workspace
```sh
$ cargo generate --path template --name abc000
```

#### 2. Edit Cargo.toml
```diff Cargo.toml
[workspace]
resolver = "2"
members = [
+   "abc000",
]
```

#### 3. Write your code
Open `abc000/src/main.rs` and you're good to go!

#### 4. Run/test your code
To run your code, run
```sh
$ cargo run -p abc000
```
To test your code, edit `abc000/tests/tests.rs` and run
```sh
$ cargo test -p abc000
```
