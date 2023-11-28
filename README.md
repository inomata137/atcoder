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

#### 4. Test your code
Once you've edited `abc000/tests/tests.rs`, run
```sh
$ cargo test -p abc000
```
