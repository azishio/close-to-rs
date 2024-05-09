use std::fmt::Display;

use num::Float;

pub fn close_to<T: Float, U: Float + Into<T>>(left: T, right: U, precision: i32) -> bool
{
    let expected_diff = T::from(10).unwrap().powi(-precision) / T::from(2).unwrap();

    let received_diff = (left - right.into()).abs();

    received_diff <= expected_diff
}

pub fn assert_close_to<T: Float + Display, U: Float + Display + Into<T>>(left: T, right: U, precision: i32)
{
    if !close_to(left, right, precision)
    {
        panic!("assertion 'left ≈ right` failed\n left: {}\nright: {}\nreceived_diff: {}\nexpected_diff: {}", left, right, (left - right.into()).abs(), T::from(10).unwrap().powi(-precision) / T::from(2).unwrap());
    }
}


pub fn far_from<T: Float, U: Float + Into<T>>(left: T, right: U, precision: i32) -> bool
{
    !close_to(left, right, precision)
}

pub fn assert_far_from<T: Float + Display, U: Float + Display + Into<T>>(left: T, right: U, precision: i32)
{
    if !far_from(left, right, precision)
    {
        panic!("assertion 'left != right` failed\n left: {}\nright: {}\nreceived_diff: {}\nexpected_diff: {}", left, right, (left - right.into()).abs(), T::from(10).unwrap().powi(-precision) / T::from(2).unwrap());
    }
}

pub trait CloseTo
{
    fn close_to(&self, other: Self, precision: i32) -> bool;
    fn far_from(&self, other: Self, precision: i32) -> bool;
}

pub trait AssertCloseTo
{
    fn assert_close_to(&self, other: Self, precision: i32);
    fn assert_far_from(&self, other: Self, precision: i32);
}


impl<T: Float> CloseTo for T
{
    fn close_to(&self, other: T, precision: i32) -> bool
    {
        close_to(*self, other, precision)
    }

    fn far_from(&self, other: T, precision: i32) -> bool
    {
        far_from(*self, other, precision)
    }
}

impl<T: Float + Display> AssertCloseTo for T
{
    fn assert_close_to(&self, other: T, precision: i32)
    {
        assert_close_to(*self, other, precision)
    }

    fn assert_far_from(&self, other: T, precision: i32)
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
        assert!(close_to(1.0, 1.0001, 3));
        assert!(!close_to(1.0, 1.0001, 4));
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
        assert!(1.0.close_to(1.0001, 3));
        assert!(!1.0.close_to(1.0001, 4));
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
