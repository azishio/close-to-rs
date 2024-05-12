# close-to-rs

Provides a function, Tolerate, to verify the equivalence of floating-point numbers at arbitrary precision.

任意の精度で浮動小数点数の等価性を検証するための関数、トレイトを提供します。

An alias also exists for compatibility with [assert-be-close](https://crates.io/crates/assert-be-close), so changing the
crate does not require changing the code.

[assert-be-close](https://crates.io/crates/assert-be-close)との互換性を保つためのエイリアスも存在するため、クレートを変更してもコードを変更する必要がありません。

## 使い方(Usage)

Compare values with arbitrary precision.

任意の精度で数値を比較する。

```rust
use close_to::{close_to, far_from};

fn example() {
    let (is_close, expected_diff, received_diff) = close_to(1.0, 1.0001, 3);
    assert!(is_close);

    let (is_close, expected_diff, received_diff) = close_to(1.0, 1.0001, 4);
    assert!(is_close); // panic!


    let (is_close, expected_diff, received_diff) = far_from(1.0, 1.0001, 4);
    assert!(is_close);

    let (is_close, expected_diff, received_diff) = far_from(1.0, 1.0001, 3);
    assert!(is_close); // panic!
}
```

Different types can be compared if the first argument `T` is a floating-point number and the second argument `U`
implements `Into<T>`.

異なる型同士でも、第1引数`T`が浮動小数点数かつ、第２引数`U`が`Into<T>`を実装していれば比較できます。

```rust
use close_to::{close_to, far_from};

fn example() {
    close_to(1.0_f64, 1.0001_f32, 3);
    close_to(1.0001_f64, 1_u8, 3);

    close_to(1.0_f32, 1.0001_f64, 3); // the trait bound `f32: std::convert::From<f64>` is not satisfied [E0277]
}
```

There is also a function that panics if the result of the comparison is false, along with a debug expression.

また、比較した結果がfalseである場合、デバック表現とともにpanicする関数もあります。

```rust
use close_to::{assert_close_to, assert_far_from};

fn example() {
    assert_close_to(1.0, 1.0001, 4); // panic with the following message

    // assertion 'left ≈ right` failed
    // left:  1
    // right: 1.0001
    // received_diff: 0.00009999999999998899
    // expected_diff: 0.00005

    assert_far_from(1.0, 1.0001, 3); // panic with the following message

    // assertion 'left != right` failed
    // left:  1
    // right: 1.0001
    // received_diff: 0.00009999999999998899
    // expected_diff: 0.0005
}
```

Traits are also provided to implement these functions.
By default, they are implemented for `f32` and `f64`.
`T.close_to(U, precision)` and `close_to(T, U, precision)` have the same behavior.

これらの機能を実装するためのトレイトも提供しています。
デフォルトでは、`f32`と`f64`に対して実装されています。
`T.close_to(U, precision)`と`close_to(T, U, precision)`は同じ挙動をします。

```rust
use close_to::{AssertCloseTo, CloseTo};

fn example() {
    let is_close = 1_f64.close_to(1.0001, 3).0;
    assert!(is_close);

    let is_close = 1_f64.close_to(1.0001, 4).0;
    assert!(is_close); // panic!

    1_f64.assert_close_to(1.0001, 4); // panic with the same message as `assert_close_to`
    1_f64.assert_far_from(1.0001, 3); // panic with the same message as `assert_far_from`
}
```

## ライセンス(License)

Licensed under either of

+ Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
+ MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
