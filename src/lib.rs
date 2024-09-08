#![crate_name = "rasterization"]
#![no_std]
#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations
)]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/pic16f877ccs/image/pixelization/quadrant_parts.png"
)]
#![doc = include_str!("../README.md")]
//! <style>
//!     .rustdoc-hidden { display: none; }
//! </style>
use colorous::Gradient;
use core::fmt::{Debug, Display};
use core::iter::FusedIterator;
use core::ops::{Add, AddAssign, Mul, Neg, Range, Sub};
use num::{One, Unsigned, Zero};
use num_convert::FromAs;

/// An iterator of successive coordinates of a filled semicircle, using Bresenham's algorithm.
///
/// The semicircles are exactly equal in diameter to the circle.
#[derive(Default, Debug, Clone, PartialEq)]
pub struct SemicircleFilled<T> {
    x: T,
    y: T,
    err: T,
}

impl<T> SemicircleFilled<T> {
    /// Creates a new `SemicircleFilled` iterator that generates pixel coordinates.
    ///
    /// # Panics
    ///
    /// This function will panic if the radius cannot be converted to type `T` or
    ///
    /// if the radius exceeds the practical limit of 100,000,000.
    ///
    /// This limit exists to prevent overflow in the calculation of the error term.
    ///
    /// # Arguments
    ///
    /// * `radius` - A non-negative integer representing the radius of the semicircle.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::SemicircleFilled;
    ///
    /// let radius = 5_u32;
    /// let semicircle_iter = SemicircleFilled::<i32>::new(radius);
    /// let vec = semicircle_iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(-5..5, -1), (-5..5, -2), (-4..4, -3), (-3..3, -4), (-2..2, -5)]);
    /// ```
    #[inline]
    pub fn new<U>(radius: U) -> Self
    where
        U: Unsigned + Display + Copy,
        T: From<i8>
            + TryFrom<U>
            + Sub<Output = T>
            + Neg<Output = T>
            + Default
            + Mul<Output = T>
            + FromAs<isize>
            + PartialOrd
            + Copy,
        <T as TryFrom<U>>::Error: Debug,
    {
        let Ok(r) = <T as TryFrom<U>>::try_from(radius) else {
            panic!(
                "{}",
                format_args!("Not possible to convert radius to {}", radius)
            )
        };
        let m_radius = <T as FromAs<isize>>::from_as(100_000_000_isize);
        if r < m_radius {
            return Self {
                x: -r,
                y: 0.into(),
                err: <T as From<i8>>::from(2) - <T as From<i8>>::from(2) * r,
            };
        }

        panic!("Radius is too large")
    }
}

impl Iterator for SemicircleFilled<i32> {
    type Item = (Range<i32>, i32);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }

        let xy = (self.x..-self.x, -(self.y + 1));

        // FIXME: This loop creates redundant iterations!
        loop {
            let y_tmp = self.y;
            let err_tmp = self.err;

            if err_tmp <= self.y {
                self.y += 1;
                self.err += 2 * self.y + 2;
            };

            if err_tmp > self.x || self.err > self.y {
                self.x += 1;
                self.err += 2 * self.x + 2;
            }

            if y_tmp != self.y || self.x == 0 {
                break;
            }
        }

        Some(xy)
    }
}

impl Iterator for SemicircleFilled<isize> {
    type Item = (Range<isize>, isize);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }

        let xy = (self.x..-self.x, -(self.y + 1));

        // FIXME: This loop creates redundant iterations!
        loop {
            let y_tmp = self.y;
            let err_tmp = self.err;

            if err_tmp <= self.y {
                self.y += 1;
                self.err += 2 * self.y + 2;
            };

            if err_tmp > self.x || self.err > self.y {
                self.x += 1;
                self.err += 2 * self.x + 2;
            }

            if y_tmp != self.y || self.x == 0 {
                break;
            }
        }

        Some(xy)
    }
}

impl Iterator for SemicircleFilled<i64> {
    type Item = (Range<i64>, i64);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }

        let xy = (self.x..-self.x, -(self.y + 1));

        // FIXME: This loop creates redundant iterations!
        loop {
            let y_tmp = self.y;
            let err_tmp = self.err;

            if err_tmp <= self.y {
                self.y += 1;
                self.err += 2 * self.y + 2;
            };

            if err_tmp > self.x || self.err > self.y {
                self.x += 1;
                self.err += 2 * self.x + 2;
            }

            if y_tmp != self.y || self.x == 0 {
                break;
            }
        }

        Some(xy)
    }
}

impl<T> FusedIterator for SemicircleFilled<T> where SemicircleFilled<T>: Iterator {}

/// The trait for rasterization of given figures.
pub trait Rasterization: Iterator {
    /// An iterator adapter that creates (x, y) coordinates for the filled full circle.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<i32>::new(2_usize).circle();
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(-2, -1), (-2, 0), (-1, -1), (-1, 0), (0, -1), (0, 0),
    ///           (1, -1), (1, 0), (-1, -2), (-1, 1), (0, -2), (0, 1)]);
    /// ```
    #[inline]
    fn circle<T, R>(self) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (R, T)> + Clone + Debug,
        T: Add<Output = T> + One + Sub<Output = T> + Neg<Output = T> + Copy + Debug,
        R: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(|(range, y)| range.flat_map(move |x| [(x, y), (x, -y - T::one())]))
    }

    /// The iterator adapter adds an offset to a two-element tuple.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    ///
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let radius = 2_usize;
    /// let offset_x = radius as i32;
    /// let offset_y = radius as i32;
    /// let iter = SemicircleFilled::<i32>::new(radius).circle().offset(offset_x, offset_y);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(0, 1), (0, 2), (1, 1), (1, 2), (2, 1), (2, 2),
    ///           (3, 1), (3, 2), (1, 0), (1, 3), (2, 0), (2, 3)]);
    /// ```
    #[inline]
    fn offset<T>(self, offset_x: T, offset_y: T) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (T, T)> + Clone + Debug,
        T: Add<Output = T> + Copy + Debug,
    {
        self.map(move |(x, y)| (x + offset_x, y + offset_y))
    }

    /// An iterator adapter that creates (x, y) coordinates for the filled long circle.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<i32>::new(2_u64).circle_long(1, -1);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(-1, -1), (-1, 0), (0, -1), (0, 0)]);
    /// ```
    #[inline]
    fn circle_long<T>(self, start: T, end: T) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (Range<T>, T)> + Clone + Debug,
        T: Add<Output = T> + One + Sub<Output = T> + Neg<Output = T> + Copy + AddAssign + Debug,
        Range<T>: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(move |(mut range, y)| {
            range.start += start;
            range.end += end;
            range.flat_map(move |x| [(x, y), (x, -y - T::one())])
        })
    }

    /// An iterator adapter that creates (x, y) coordinates for the filled top semicircle.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<isize>::new(2_u8).semicircle_top();
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(-2, -1), (-1, -1), (0, -1), (1, -1), (-1, -2), (0, -2)]);
    /// ```
    #[inline]
    fn semicircle_top<T, R>(self) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (R, T)> + Clone + Debug,
        T: Add<Output = T> + One + Sub<Output = T> + Neg<Output = T> + Copy,
        R: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(|(range, y)| range.map(move |x| (x, y)))
    }

    /// An iterator adapter that creates (x, y) coordinates for the filled bottom semicircle.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<isize>::new(2_u16).semicircle_bottom();
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(-2, 0), (-1, 0), (0, 0), (1, 0), (-1, 1), (0, 1)]);
    /// ```
    #[inline]
    fn semicircle_bottom<T, R>(self) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (R, T)> + Clone + Debug,
        T: Add<Output = T> + One + Sub<Output = T> + Neg<Output = T> + Copy,
        R: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(|(range, y)| range.map(move |x| (x, -y - T::one())))
    }

    /// An iterator adapter that creates (x, y) coordinates for the filled top long semicircle.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<isize>::new(2_u32).semicircle_top_long(0, 1);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(-2, -1), (-1, -1), (0, -1), (1, -1), (2, -1), (-1, -2), (0, -2), (1, -2)]);
    /// ```
    #[inline]
    fn semicircle_top_long<T>(
        self,
        start: T,
        end: T,
    ) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (Range<T>, T)> + Clone + Debug,
        T: Add<Output = T> + One + Sub<Output = T> + Neg<Output = T> + Copy + AddAssign,
        Range<T>: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(move |(mut range, y)| {
            range.start += start;
            range.end += end;
            range.map(move |x| (x, y))
        })
    }

    /// An iterator adapter that creates (x, y) coordinates for the filled bottom long semicircle.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<isize>::new(2_usize).semicircle_bottom_long(0, 1);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(-2, 0), (-1, 0), (0, 0), (1, 0), (2, 0), (-1, 1), (0, 1), (1, 1)]);
    /// ```
    #[inline]
    fn semicircle_bottom_long<T>(
        self,
        start: T,
        end: T,
    ) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (Range<T>, T)> + Clone + Debug,
        T: Add<Output = T> + One + Sub<Output = T> + Neg<Output = T> + Copy + AddAssign,
        Range<T>: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(move |(mut range, y)| {
            range.start += start;
            range.end += end;
            range.map(move |x| (x, -y - T::one()))
        })
    }

    /// An iterator adapter that creates the (x, y) coordinates for the filled circle of the first quadrant.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<isize>::new(3_u64).first_quadrant(0);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(0, -1), (1, -1), (2, -1), (0, -2), (1, -2), (2, -2), (0, -3), (1, -3)]);
    /// ```
    #[inline]
    fn first_quadrant<T>(self, end: T) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (Range<T>, T)> + Clone + Debug,
        T: Add<Output = T> + Zero + Sub<Output = T> + Neg<Output = T> + Copy + AddAssign,
        Range<T>: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(move |(mut range, y)| {
            range.start = T::zero();
            range.end += end;
            range.map(move |x| (x, y))
        })
    }

    /// An iterator adapter that creates the (x, y) coordinates for the filled circle of the second quadrant.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<i64>::new(3_u8).second_quadrant(0);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(-3, -1), (-2, -1), (-1, -1), (-3, -2), (-2, -2), (-1, -2), (-2, -3), (-1, -3)]);
    /// ```
    #[inline]
    fn second_quadrant<T>(self, end: T) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (Range<T>, T)> + Clone + Debug,
        T: Add<Output = T> + Zero + Sub<Output = T> + Neg<Output = T> + Copy + AddAssign,
        Range<T>: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(move |(mut range, y)| {
            range.end = T::zero() + end;
            range.map(move |x| (x, y))
        })
    }

    /// An iterator adapter that creates the (x, y) coordinates for the filled circle of the third quadrant.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<i64>::new(3_u16).third_quadrant(0);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(-3, 0), (-2, 0), (-1, 0), (-3, 1), (-2, 1), (-1, 1), (-2, 2), (-1, 2)]);
    /// ```
    #[inline]
    fn third_quadrant<T>(self, end: T) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (Range<T>, T)> + Clone + Debug,
        T: Add<Output = T> + One + Zero + Sub<Output = T> + Neg<Output = T> + Copy + AddAssign,
        Range<T>: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(move |(mut range, y)| {
            range.end = T::zero() + end;
            range.map(move |x| (x, -y - T::one()))
        })
    }

    /// An iterator adapter that creates the (x, y) coordinates for the filled circle of the fourth quadrant.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    ///
    /// let iter = SemicircleFilled::<i64>::new(3_u32).fourth_quadrant(0);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(vec, vec![(0, 0), (1, 0), (2, 0), (0, 1), (1, 1), (2, 1), (0, 2), (1, 2)]);
    /// ```
    #[inline]
    fn fourth_quadrant<T>(self, end: T) -> impl Iterator<Item = (T, T)> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (Range<T>, T)> + Clone + Debug,
        T: Add<Output = T> + One + Zero + Sub<Output = T> + Neg<Output = T> + Copy + AddAssign,
        Range<T>: Iterator<Item = T> + Clone + Debug,
    {
        self.flat_map(move |(mut range, y)| {
            range.start = T::zero();
            range.end += end;
            range.map(move |x| (x, -y - T::one()))
        })
    }

    /// # Header
    /// The iterator adapter fills a circle or part of it with the color of the vertical gradient
    /// from crate [colorous].
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```rust
    /// use rasterization::{Rasterization, SemicircleFilled};
    /// use colorous::Gradient;
    ///
    /// let radius = 10_usize;
    /// let iter = SemicircleFilled::<i32>::new(radius).circle()
    ///     .gradient(colorous::BROWN_GREEN, radius);
    /// let vec = iter.collect::<Vec<_>>();
    /// assert_eq!(&vec[0..3], &vec![(-10, -1, [0, 59, 47]), (-10, 0, [84, 48, 5]), (-9, -1, [0, 59, 47])][..]);
    /// ```
    /// [colorous]: https://crates.io/crates/colorous
    #[inline]
    fn gradient(
        self,
        grad: Gradient,
        radius: usize,
    ) -> impl Iterator<Item = (i32, i32, [u8; 3])> + Clone + Debug
    where
        Self: Sized + Iterator<Item = (i32, i32)> + Clone + Debug,
    {
        self.map(move |(x, y)| (x, y, grad.eval_rational(y as usize, radius * 2).as_array()))
    }
}

impl<T: ?Sized> Rasterization for T where T: Iterator {}

impl DoubleEndedIterator for SemicircleFilled<i32> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }

        loop {
            let x_tmp = self.x;
            let y_tmp = self.y;
            let err_tmp = self.err;

            if err_tmp <= self.y {
                self.y += 1;
                self.err += 2 * self.y + 2;
            };

            if err_tmp > self.x || self.err > self.y {
                self.x += 1;
                self.err += 2 * self.x + 2;
                if self.y == y_tmp {
                    return Some((-(self.y + 1)..(self.y + 1), (self.x - 1)));
                }
            }

            if x_tmp != self.x {
                let xy = (-self.y..self.y, (self.x - 1));

                return Some(xy);
            }
        }
    }
}

impl DoubleEndedIterator for SemicircleFilled<i64> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }

        loop {
            let x_tmp = self.x;
            let y_tmp = self.y;
            let err_tmp = self.err;

            if err_tmp <= self.y {
                self.y += 1;
                self.err += 2 * self.y + 2;
            };

            if err_tmp > self.x || self.err > self.y {
                self.x += 1;
                self.err += 2 * self.x + 2;
                if self.y == y_tmp {
                    return Some((-(self.y + 1)..(self.y + 1), (self.x - 1)));
                }
            }

            if x_tmp != self.x {
                let xy = (-self.y..self.y, (self.x - 1));

                return Some(xy);
            }
        }
    }
}

impl DoubleEndedIterator for SemicircleFilled<isize> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.x == 0 {
            return None;
        }

        loop {
            let x_tmp = self.x;
            let y_tmp = self.y;
            let err_tmp = self.err;

            if err_tmp <= self.y {
                self.y += 1;
                self.err += 2 * self.y + 2;
            };

            if err_tmp > self.x || self.err > self.y {
                self.x += 1;
                self.err += 2 * self.x + 2;
                if self.y == y_tmp {
                    return Some((-(self.y + 1)..(self.y + 1), (self.x - 1)));
                }
            }

            if x_tmp != self.x {
                let xy = (-self.y..self.y, (self.x - 1));

                return Some(xy);
            }
        }
    }
}
