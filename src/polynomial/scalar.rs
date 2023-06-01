//! Interactions between scalar numbers and polynomials. Here are defined the operations:
//! - Polynomial + Scalar
//! - Polynomial - Scalar
//! - Polynomial * Scalar
//! - Polynomial / Scalar

use super::Polynomial;

/// Supertrait that represents something that is operable with a scalar number.
///
/// This means that the four basic operations (addition, substraction, multiplication and division)
/// can be performed for the type with a scalar number.
pub trait OperableWithScalars<T>:
    AddScalar<T> + SubScalar<T> + MulScalar<T> + DivScalar<T>
{
}

pub trait AddScalar<T> {
    type Output;
    fn add_scalar(&self, other: T) -> Self::Output;
}

pub trait SubScalar<T> {
    type Output;
    fn sub_scalar(&self, other: T) -> Self::Output;
}

pub trait MulScalar<T> {
    type Output;
    fn mul_scalar(&self, other: T) -> Self::Output;
}

pub trait DivScalar<T> {
    type Output;
    fn div_scalar(&self, other: T) -> Self::Output;
}

macro_rules! impl_add_scalar {
    ($($scalar:ty),*) => {
        $(impl AddScalar<$scalar> for Polynomial {
            type Output = Polynomial;
            fn add_scalar(&self, other: $scalar) -> Polynomial {
                let mut result = self.clone();
                result.coefficients[0] += other as f64;
                result
            }
        })*


    };
}

macro_rules! impl_sub_scalar {
    ($($scalar:ty), *) => {
        $(impl SubScalar<$scalar> for Polynomial {
            type Output = Polynomial;
            fn sub_scalar(&self, other: $scalar) -> Self {
                let mut result = self.clone();
                result.coefficients[0] -= other as f64;
                result
            }
        })*
    };
}

macro_rules! impl_mul_scalar {
    ($($scalar:ty),*) => {
        $(impl MulScalar<$scalar> for Polynomial {
            type Output = Polynomial;
            fn mul_scalar(&self, other: $scalar) -> Self {
                let mut result = self.clone();
                result.coefficients.iter_mut().for_each(|x| *x *= other as f64);
                result
            }
        })*
    };
}

macro_rules! impl_div_scalar {
    ($($scalar:ty),*) => {
        $(impl DivScalar<$scalar> for Polynomial {
            type Output = Self;
            fn div_scalar(&self, other: $scalar) -> Self {
                let mut result = self.clone();
                result.coefficients.iter_mut().for_each(|x| *x /= other as f64);
                result
            }
        })*
    };
}

impl_add_scalar!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);
impl_sub_scalar!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);
impl_mul_scalar!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);
impl_div_scalar!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_scalar() {
        let p = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let r: Polynomial = p.add_scalar(1);
        println!("{:?}", p);
        pretty_assertions::assert_eq!(r.coefficients, vec![2.0, 2.0, 3.0]);
    }

    #[test]
    fn test_sub_scalar() {
        let p = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p: Polynomial = p.sub_scalar(1);
        pretty_assertions::assert_eq!(p.coefficients, vec![0.0, 2.0, 3.0]);
    }

    #[test]
    fn test_mul_scalar() {
        let p = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p: Polynomial = p.mul_scalar(2);
        pretty_assertions::assert_eq!(p.coefficients, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_div_scalar() {
        let p = Polynomial::new(vec![1.0, 2.0, 3.0], 0.0);
        let p: Polynomial = p.div_scalar(2);
        pretty_assertions::assert_eq!(p.coefficients, vec![0.5, 1.0, 1.5]);
    }
}
