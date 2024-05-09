use std::fmt::Display;

use num::Float;

/// ２つの小数が指定した精度で等しいかどうかを判定する。
///
/// # Examples
///
/// ```
/// use close_to::close_to;
///
/// let (is_close, expected_diff, received_diff) = close_to(1.0, 1.0001, 3);
/// assert!(is_close);
///```
/// ```should_panic
/// use close_to::close_to;
///
/// let (is_close, expected_diff, received_diff) = close_to(1.0, 1.0001, 4);
/// assert!(is_close);
/// ```
/// ```
pub fn close_to<T: Float, U: Into<T>>(left: T, right: U, precision: i32) -> (bool, T, T)
{
    let expected_diff = T::from(10).unwrap().powi(-precision) / T::from(2).unwrap();

    let received_diff = (left - right.into()).abs();

    (received_diff <= expected_diff, expected_diff, received_diff)
}

/// ２つの小数が指定した精度で等しいことを保証する。
/// パニックになると、この関数は小数の値をデバッグ表現とともに出力する。
///
/// # Examples
///
/// ```
/// use close_to::assert_close_to;
///
/// assert_close_to(1.0, 1.0001, 3);
/// ```
/// ```should_panic
/// use close_to::assert_close_to;
///
/// assert_close_to(1.0, 1.0001, 4); // panic with the following message
///
/// // assertion 'left ≈ right` failed
/// // left:  1
/// // right: 1.0001
/// // received_diff: 0.00009999999999998899
/// // expected_diff: 0.00005
/// ```
/// ```
pub fn assert_close_to<T: Float + Display + Copy, U: Display + Into<T> + Copy>(left: T, right: U, precision: i32)
{
    let (is_close, expected_diff, received_diff) = close_to(left, right, precision);

    if !is_close
    {
        panic!("assertion 'left ≈ right` failed\n left:  {}\nright: {}\nreceived_diff: {}\nexpected_diff: {}", left, right, received_diff, expected_diff);
    }
}


/// 2つの小数が指定した精度で等しくないかどうかを判定する。
///
/// # Examples
///
/// ```
/// use close_to::far_from;
///
/// let (is_far, expected_diff, received_diff) = far_from(1.0, 1.0001, 4);
/// assert!(is_far);
/// ```
/// ```should_panic
/// use close_to::far_from;
///
/// let (is_far, expected_diff, received_diff) = far_from(1.0, 1.0001, 3);
/// assert!(is_far);
/// ```
pub fn far_from<T: Float, U: Into<T>>(left: T, right: U, precision: i32) -> (bool, T, T)
{
    let (is_close, expected_diff, received_diff) = close_to(left, right, precision);

    (!is_close, expected_diff, received_diff)
}


/// 2つの小数が指定した精度で等しくないことを保証する。
/// パニックになると、この関数は小数の値をデバッグ表現とともに出力する。
///
/// # Examples
///
/// ```
/// use close_to::assert_far_from;
///
/// assert_far_from(1.0, 1.0001, 4);
/// ```
/// ```should_panic
/// use close_to::assert_far_from;
///
/// assert_far_from(1.0, 1.0001, 3); // panic with the following message
///
/// // assertion 'left != right` failed
/// // left:  1
/// // right: 1.0001
/// // received_diff: 0.00009999999999998899
/// // expected_diff: 0.0005
/// ```
pub fn assert_far_from<T: Float + Display + Copy, U: Display + Into<T> + Copy>(left: T, right: U, precision: i32)
{
    let (is_far, expected_diff, received_diff) = far_from(left, right, precision);

    if !is_far
    {
        panic!("assertion 'left != right` failed\n left:  {}\nright: {}\nreceived_diff: {}\nexpected_diff: {}", left, right, received_diff, expected_diff);
    }
}

/// 2つの小数が指定した精度で等しいかどうかを判定するトレイト。
/// デフォルトでfloat型に実装されている。
///
/// # Examples
///
/// ```
/// use close_to::CloseTo;
///
/// let (is_close, ..) = 1.0.close_to(1.0001, 3);
/// assert!(is_close);
///
/// assert!(1.0.far_from(1.0001, 4).0);
/// ```
pub trait CloseTo<T, U: Into<T>>
{
    /// 2つの小数が指定した精度で等しいかどうかを判定する。
    fn close_to(&self, other: U, precision: i32) -> (bool, T, T);
    /// 2つの小数が指定した精度で等しくないかどうかを判定する。
    fn far_from(&self, other: U, precision: i32) -> (bool, T, T);
}

/// 2つの小数が指定した精度で等しいことを保証するトレイト。
/// デフォルトでfloat型に実装されている。
///
/// # Examples
///
/// ```
/// use close_to::{assert_far_from, AssertCloseTo};
///
/// 1.0.assert_close_to(1.0001, 3);
/// 1.0.assert_far_from(1.0001, 4);
/// ```
pub trait AssertCloseTo<T>
{
    /// 2つの小数が指定した精度で等しいことを保証する。
    fn assert_close_to(&self, other: T, precision: i32);
    /// 2つの小数が指定した精度で等しくないことを保証する。
    fn assert_far_from(&self, other: T, precision: i32);
}


impl<T: Float, U: Into<T>> CloseTo<T, U> for T
{
    fn close_to(&self, other: U, precision: i32) -> (bool, T, T)
    {
        close_to(*self, other, precision)
    }

    fn far_from(&self, other: U, precision: i32) -> (bool, T, T)
    {
        far_from(*self, other, precision)
    }
}

impl<T: Float + Display, U: Into<T> + Display + Copy> AssertCloseTo<U> for T
{
    fn assert_close_to(&self, other: U, precision: i32)
    {
        assert_close_to(*self, other, precision)
    }

    fn assert_far_from(&self, other: U, precision: i32)
    {
        assert_far_from(*self, other, precision)
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_close_to()
    {
        let (is_close, ..) = close_to(1.0, 1.0001, 3);
        assert!(is_close);

        let (is_close, ..) = close_to(1.0, 1.0001, 4);
        assert!(!is_close);
    }

    #[test]
    fn test_assert_close_to()
    {
        assert_close_to(1.0, 1.0001, 3);
        assert_far_from(1.0, 1.0001, 4);
    }

    #[test]
    #[should_panic]
    fn test_assert_close_to_panic()
    {
        assert_close_to(1.0, 1.0001, 4);
    }

    #[test]
    #[should_panic]
    fn test_assert_far_from_panic()
    {
        assert_far_from(1.0, 1.0001, 3);
    }

    #[test]
    fn test_trait()
    {
        let (is_close, ..) = 1.0.close_to(1.0001, 3);
        assert!(is_close);

        let (is_close, ..) = 1.0.close_to(1.0001, 4);
        assert!(!is_close);
    }

    #[test]
    fn test_trait_assert()
    {
        1.0.assert_close_to(1.0001, 3);
        1.0.assert_far_from(1.0001, 4);
    }

    #[test]
    #[should_panic]
    fn test_trait_assert_panic()
    {
        1.0.assert_close_to(1.0001, 4);
    }

    #[test]
    #[should_panic]
    fn test_trait_assert_far_from_panic()
    {
        1.0.assert_far_from(1.0001, 3);
    }
}
