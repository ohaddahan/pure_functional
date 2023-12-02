# Functional Macro

## Background
This `macro` is aimed to help enforce `pure functions`.

While it does have known limitations (see below), we still believe it provides value to help keep `mutation` in check.

## How does it work?
1. We parse each `function` arguments and check if they are `mutable` (`&mut`).
2. We rewrite the `function` as with an internal `module` to prevent `globals` usage.

*Original*:
```rust
fn foo(arg: i32) -> i32 {
    arg + 1
}
```

*Rewrite*:
```rust
fn foo(arg: i32) -> i32 {
    mod inner {
        pub fn foo(arg: i32) -> i32 {
            arg + 1
        }
    }
    inner::foo(arg)
}
```


## Known Limitations
Any `struct` that internally hides `mutability` will not be caught by this `macro`.

For example, the following `struct` will not be caught by this `macro`:
* `Arc<Mutex<T>>`.
* `Cell<T>`.
* `RefCell<T>`.
* `RwLock<T>`.
* `UnsafeCell<T>`.

All of these wrap `UnsafeCell<T>` internally, which is why they are not caught by this `macro`.




////// https://github.com/dtolnay/no-panic
////// https://github.com/dtolnay/cargo-expand/issues/40