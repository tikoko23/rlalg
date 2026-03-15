use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

pub trait Sqrt {
    fn sqrt(&self) -> Self;
}

pub trait Numeric {
    const ZERO: Self;
    const ONE: Self;
}

/// impl_sqrt! {}
#[doc(hidden)]
macro_rules! impl_sqrt {
    ($([$T:ty, $method:ident]),* $(,)?) => {
        $(
            impl Sqrt for $T {
                fn sqrt(&self) -> Self {
                    <$T>::$method(*self)
                }
            }
        )*
    };
}

impl_sqrt! {
    [i8, isqrt],
    [i16, isqrt],
    [i32, isqrt],
    [i64, isqrt],
    [i128, isqrt],
    [u8, isqrt],
    [u16, isqrt],
    [u32, isqrt],
    [u64, isqrt],
    [u128, isqrt],
    [f32, sqrt],
    [f64, sqrt],
}

/// impl_numeric! {}
#[doc(hidden)]
macro_rules! impl_numeric {
    ($($T:ty),* $(,)?) => {
        $(
            impl Numeric for $T {
                const ZERO: Self = 0 as $T;
                const ONE: Self = 0 as $T;
            }
        )*
    };
}

impl_numeric! {
    i8,
    i16,
    i32,
    i64,
    i128,
    u8,
    u16,
    u32,
    u64,
    u128,
    f32,
    f64,
}

pub trait Component:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Clone
    + Copy
    + Numeric
    + PartialEq
{
}

/// impl_component! {}
macro_rules! impl_component {
    ($($T:ty),* $(,)?) => {
        $(
            impl Component for $T {}
        )*
    };
}

impl_component! {
    u8,
    u16,
    u32,
    u64,
    u128,
    i8,
    i16,
    i32,
    i64,
    i128,
    f32,
    f64,
}

pub trait Vector<T: Component, const N: usize>:
    Clone
    + Copy
    + Index<usize, Output = T>
    + IndexMut<usize, Output = T>
    + PartialEq
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + Mul<T, Output = Self>
    + MulAssign
    + MulAssign<T>
    + Div<Output = Self>
    + Div<T, Output = Self>
    + DivAssign
    + DivAssign<T>
{
    const N: usize = N;

    fn mag_sq(&self) -> T {
        let mut v = T::ZERO;

        for i in 0..N {
            let c = self[i];
            v += c * c;
        }

        v
    }

    fn mag(&self) -> T
    where
        T: Sqrt,
    {
        self.mag_sq().sqrt()
    }

    /// Returns a normalized copy of `self`.
    ///
    /// # Panics
    /// Panics if one of the components is 0 and `T` panics when division by zero occurs
    fn norm(&self) -> Self
    where
        T: Sqrt,
    {
        *self / self.mag()
    }
}

/// impl_op! {}
macro_rules! impl_op {
    ({$T:tt, $($member:ident),*}, [$Trait:ty, $AssignTrait:ty, $method:ident, $assign_method:ident, $op:tt, $aop:tt] $(,)?) => {
        impl<V: Component> $Trait for $T<V> {
            type Output = Self;

            fn $method(self, other: Self) -> Self::Output {
                Self {
                    $(
                        $member: self.$member $op other.$member,
                    )*
                }
            }
        }

        impl<V: Component> $AssignTrait for $T<V> {
            fn $assign_method(&mut self, other: Self) {
                $(
                    self.$member $aop other.$member;
                )*
            }
        }
    };
}

/// impl_mul_div! {}
macro_rules! impl_mul_div {
    ($T:tt, $($member:ident),*) => {
        impl<V: Component> Mul<V> for $T<V> {
            type Output = Self;

            fn mul(self, other: V) -> Self::Output {
                Self {
                    $(
                        $member: self.$member * other,
                    )*
                }
            }
        }

        impl<V: Component> MulAssign<V> for $T<V> {
            fn mul_assign(&mut self, other: V) {
                $(
                    self.$member *= other;
                )*
            }
        }

        impl<V: Component> Div<V> for $T<V> {
            type Output = Self;

            fn div(self, other: V) -> Self::Output {
                Self {
                    $(
                        $member: self.$member / other,
                    )*
                }
            }
        }

        impl<V: Component> DivAssign<V> for $T<V> {
            fn div_assign(&mut self, other: V) {
                $(
                    self.$member /= other;
                )*
            }
        }
    };
}

/// impl_index! {}
macro_rules! impl_index {
    ($T:tt {$([$index:expr, $member:ident]),* $(,)?}) => {
        impl<V: Component> Index<usize> for $T<V> {
            type Output = V;

            fn index(&self, idx: usize) -> &Self::Output {
                match idx {
                    $(
                        $index => &self.$member,
                    )*
                    _ => panic!("vector index out of bounds"),
                }
            }
        }

        impl<V: Component> IndexMut<usize> for $T<V> {
            fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
                match idx {
                    $(
                        $index => &mut self.$member,
                    )*
                    _ => panic!("vector index out of bounds"),
                }
            }
        }
    };
}

/// impl_vec! {}
macro_rules! impl_vec {
    ($T:tt, $([$index:expr, $member:ident]),* $(,)?) => {
        impl_op! {
            { $T, $($member),* },
            [Add, AddAssign, add, add_assign, +, +=],
        }

        impl_op! {
            { $T, $($member),* },
            [Sub, SubAssign, sub, sub_assign, -, -=],
        }

        impl_op! {
            { $T, $($member),* },
            [Mul, MulAssign, mul, mul_assign, *, *=],
        }

        impl_op! {
            { $T, $($member),* },
            [Div, DivAssign, div, div_assign, /, /=],
        }

        impl_mul_div! {
            $T, $($member),*
        }

        impl_index! {
            $T {
                $(
                    [$index, $member],
                )*
            }
        }

        impl<V: Component> $T<V> {
            pub const ZERO: Self = Self {
                $(
                    $member: V::ZERO,
                )*
            };

            pub const fn new($($member: V),*) -> Self {
                Self {
                    $(
                        $member,
                    )*
                }
            }

            pub fn cast<U: Component + From<V>>(&self) -> $T<U> {
                $T {
                    $(
                        $member: self.$member.into(),
                    )*
                }
            }
        }

        impl<V: Component> Neg for $T<V>
        where
            V: Neg<Output = V>
        {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self {
                    $(
                        $member: -self.$member,
                    )*
                }
            }
        }
    };
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct v2<T: Component> {
    pub x: T,
    pub y: T,
}

impl_vec! {
    v2,
    [0, x],
    [1, y],
}

impl<T: Component> Vector<T, 2> for v2<T> {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct v3<T: Component> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl_vec! {
    v3,
    [0, x],
    [1, y],
    [2, z],
}

impl<T: Component> Vector<T, 3> for v3<T> {}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct v4<T: Component> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl_vec! {
    v4,
    [0, x],
    [1, y],
    [2, z],
    [3, w],
}

impl<T: Component> Vector<T, 4> for v4<T> {}

#[macro_export]
macro_rules! v {
    ($x:expr, $y:expr $(,)?) => {
        $crate::v2::new($x, $y)
    };

    ($x:expr, $y:expr, $z:expr $(,)?) => {
        $crate::v3::new($x, $y, $z)
    };

    ($x:expr, $y:expr, $z:expr, $w:expr $(,)?) => {
        $crate::v4::new($x, $y, $z, $w)
    };
}

impl<T: Component> From<v2<T>> for (T, T) {
    fn from(value: v2<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T: Component> From<(T, T)> for v2<T> {
    fn from(value: (T, T)) -> Self {
        v!(value.0, value.1)
    }
}

impl<T: Component> From<v3<T>> for (T, T, T) {
    fn from(value: v3<T>) -> Self {
        (value.x, value.y, value.z)
    }
}

impl<T: Component> From<(T, T, T)> for v3<T> {
    fn from(value: (T, T, T)) -> Self {
        v!(value.0, value.1, value.2)
    }
}

impl<T: Component> From<v4<T>> for (T, T, T, T) {
    fn from(value: v4<T>) -> Self {
        (value.x, value.y, value.z, value.w)
    }
}

impl<T: Component> From<(T, T, T, T)> for v4<T> {
    fn from(value: (T, T, T, T)) -> Self {
        v!(value.0, value.1, value.2, value.3)
    }
}

#[doc(hidden)]
mod alias {
    #![allow(non_camel_case_types)]

    use super::*;

    pub type v2i = v2<i32>;
    pub type v2u = v2<u32>;
    pub type v2f = v2<f32>;
    pub type v2d = v2<f64>;

    pub type v3i = v3<i32>;
    pub type v3u = v3<u32>;
    pub type v3f = v3<f32>;
    pub type v3d = v3<f64>;

    pub type v4i = v4<i32>;
    pub type v4u = v4<u32>;
    pub type v4f = v4<f32>;
    pub type v4d = v4<f64>;
}

pub use alias::*;

pub fn dot<T: Component, const N: usize>(a: impl Vector<T, N>, b: impl Vector<T, N>) -> T {
    let mut s = T::ZERO;

    for i in 0..N {
        s += a[i] * b[i];
    }

    s
}

pub fn cross<T: Component + Neg>(a: impl Vector<T, 3>, b: impl Vector<T, 3>) -> v3<T> {
    v3 {
        x: a[1] * b[2] - a[2] * b[1],
        y: a[2] * b[0] - a[0] * b[2],
        z: a[0] * b[1] - a[1] * b[0],
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn arith() {
        let a = v!(1, 2);
        let b = v!(2, 3);

        assert_eq!(a + b, v!(3, 5));
        assert_eq!(b - a, v!(1, 1));
        assert_eq!(a * b, v!(2, 6));
        assert_eq!(a / b, v!(0, 0));
        assert_eq!(b / a, v!(2, 1));
        assert_eq!(a * 2, v!(2, 4));
        assert_eq!(b / 2, v!(1, 1));
        assert_eq!(-a, v!(-1, -2));
    }

    #[test]
    fn mag() {
        let a = v!(3, 4);

        assert_eq!(a.mag_sq(), 25);
        assert_eq!(a.mag(), 5);
        assert_eq!(v!(2, 2).norm(), v!(1, 1));
    }

    #[test]
    fn iter() {
        let av = v!(1, 2, 3, 4);
        let aa = [1, 2, 3, 4];

        for i in 0..4 {
            assert_eq!(av[i], aa[i]);
        }
    }

    #[test]
    fn cast() {
        let a = v!(1, 2, 3);
        let b = v!(1.0, 2.0, 3.0);

        assert_eq!(a.cast(), b);
    }

    #[test]
    fn _dot() {
        let a = v!(1, 2, 3);
        let b = v!(4, 5, 6);

        assert_eq!(dot(a, b), 32);
        assert_eq!(dot(-a, -b), 32);

        let a = v!(1, 2);
        let b = v!(3, 4);

        assert_eq!(dot(a, b), 11);

        let a = v!(1, 0, 0);
        let b = v!(0, 1, 0);

        assert_eq!(dot(a, b), 0);

        let a = v!(2, 0, 0);
        let b = v!(2, 0, 0);

        assert_eq!(dot(a, b), 4);
        assert_eq!(dot(a, -b), -4);
    }

    #[test]
    fn _cross() {
        let a = v!(1, 2, 3);
        let b = v!(4, 5, 6);

        assert_eq!(cross(a, b), v!(-3, 6, -3));
        assert_eq!(cross(a, -b), v!(3, -6, 3));

        let a = v!(1, 0, 0);
        let b = v!(0, 1, 0);

        assert_eq!(cross(a, b), v!(0, 0, 1));

        let a = v!(1, 2, 3);
        let b = a * 2;

        assert_eq!(cross(a, b), v!(0, 0, 0));
        assert_eq!(cross(a, a), v!(0, 0, 0));
    }

    #[test]
    fn pack_unpack() {
        let (x, y, z) = v!(1, 2, 3).into();
        assert_eq!(x, 1);
        assert_eq!(y, 2);
        assert_eq!(z, 3);

        assert_eq!(Into::<v3i>::into((1, 2, 3)), v!(1, 2, 3));
    }
}
