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

