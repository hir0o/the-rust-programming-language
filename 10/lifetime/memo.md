# ライフタイム

https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html

## とは

その参照が有効になるスコープのこと

以下はエラーになる

```rust
main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

以下はエラーにならない

```rust
main() {
    let r;

    {
        let x = 5;
        r = &x;
        println!("r: {}", r);
    }

}

```


