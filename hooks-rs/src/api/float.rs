use core::{
    cmp::Ordering,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::c;

use super::*;

/// Abstraction of [XFL floating point numbers](https://github.com/XRPLF/XRPL-Standards/discussions/39).
///
/// The struct is overloaded with basic numeric operations, such as addition, subtraction, multiplication, division, and negation. Comparison operators are also implemented.
#[derive(Clone, Copy)]
pub struct XFL(pub i64);

/// Output an XFL as a serialized object
#[inline(always)]
pub fn float_sto(
    amount: &mut [u8],
    currency_code: &[u8],
    issuer_accid: &[u8],
    float: XFL,
    field_code: FieldId,
) -> Result<u64> {
    let res = unsafe {
        c::float_sto(
            amount.as_mut_ptr() as _,
            amount.len() as _,
            currency_code.as_ptr() as _,
            currency_code.len() as _,
            issuer_accid.as_ptr() as _,
            issuer_accid.len() as _,
            float.0,
            field_code as _,
        )
    };

    res.into()
}

impl XFL {
    /// Create a new XFL number from an exponent and mantissa
    ///
    /// # Example
    /// ```
    /// let plus_1000 = XFL::new(-12, 1000000000000000).unwrap_line_number();
    /// ```
    #[inline(always)]
    pub fn new(exponent: i32, mantissa: i64) -> Result<Self> {
        Self::from_verified_i64(unsafe { c::float_set(exponent, mantissa) })
    }

    /// Read a serialized XFL amount into an XFL
    #[inline(always)]
    pub fn from_sto(serialized_xfl: &[u8; XFL_LEN]) -> Result<Self> {
        Self::from_verified_i64(unsafe {
            c::float_sto_set(serialized_xfl.as_ptr() as _, XFL_LEN as _)
        })
    }

    /// Return the number 1 represented in an XFL enclosing number
    ///
    /// # Example
    /// ```
    /// let half = XFL::one().mulratio(false, 1, 2)
    /// ```
    #[inline(always)]
    pub fn one() -> Self {
        // Instead of using c::float_one, we use the computed enclosing
        // value directly.
        XFL(6089866696204910592)
    }

    /// Convert an XFL floating point into an integer (floor).
    /// The behavior is as follows:
    /// 1. Left shift (multiply by 10) the XFL by the number of specified decimal places
    /// 2. Convert the resulting XFL to an integer, discarding any remainder
    /// 3. Return the integer
    ///
    /// # Example
    /// ```
    /// let approx_pi = XFL::new(-15, 3140000000000000).unwrap_line_number();
    ///
    /// if 3 != approx_pi.to_int64(0, false).unwrap_line_number() {
    ///     rollback(b"incorect rounding", line!().into());
    /// }
    /// ```
    #[inline(always)]
    pub fn to_int64(&self, decimal_places: u32, is_absolute: bool) -> Result<i64> {
        let result = unsafe { c::float_int(self.0, decimal_places, is_absolute as _) };

        match result {
            res if res >= 0 => Ok(res),
            _ => Err(Error::from_code(result as _)),
        }
    }

    /// Get the exponent of an XFL enclosing number
    ///
    /// # Example
    /// ```
    /// let plus_998 = XFL::new(-13, 9980000000000000).unwrap_line_number();
    ///
    /// if plus_998.exponent() != -13 {
    ///     rollback(b"exponent incorrect", line!().into());
    /// }
    /// ```
    #[inline(always)]
    pub fn exponent(&self) -> i64 {
        ((((self.0 >> 54) & 0xFF) as i32) - 97).into()
    }

    /// Get the exponent of an XFL enclosing number
    ///
    /// # Example
    /// ```
    /// let plus_998 = XFL::new(-13, 9980000000000000).unwrap_line_number();
    ///
    /// if plus_998.mantissa() != 9980000000000000 {
    ///     rollback(b"mantissa incorrect", line!().into());
    /// }
    /// ```
    #[inline(always)]
    pub fn mantissa(&self) -> i64 {
        unsafe { c::float_mantissa(self.0) }
    }

    /// Multiply an XFL floating point by a non-XFL numerator and denominator
    ///
    /// # Example
    /// ```
    /// if XFL::one().mulratio(false, 1, 2).unwrap_line_number()
    ///     != XFL::one().mulratio(false, 5, 10).unwrap_line_number()
    /// {
    ///     rollback(b"", line!().into());
    /// }
    /// ```
    #[inline(always)]
    pub fn mulratio(&self, round_up: bool, numerator: u32, denominator: u32) -> Result<XFL> {
        Self::from_verified_i64(unsafe {
            c::float_mulratio(self.0, round_up as _, numerator, denominator)
        })
    }

    // Create a new XFL number from a verified i64, that is,
    // a number that is known to be a valid XFL number.
    //
    // Because it is too dangerous to be exposed to the user,
    // this function is only visible to `pub(crate)` level.
    //
    // For that reason, `From<i64> for Result<XFL>` is not implemented.
    //
    // Only use this function to create an XFL number from
    // C function calls.
    #[inline(always)]
    pub(crate) fn from_verified_i64(source: i64) -> Result<Self> {
        match source {
            source if source >= 0 => Ok(XFL(source)),
            _ => Err(Error::from_code(source as _)),
        }
    }
}

impl Add for XFL {
    type Output = Result<XFL>;

    #[inline(always)]
    fn add(self, other: XFL) -> Self::Output {
        Self::from_verified_i64(unsafe { c::float_sum(self.0, other.0) })
    }
}

impl Sub for XFL {
    type Output = Result<XFL>;

    #[inline(always)]
    fn sub(self, other: XFL) -> Self::Output {
        unsafe {
            let rhs = match Self::from_verified_i64(c::float_negate(other.0)) {
                Ok(rhs) => rhs,
                Err(e) => return Err(e),
            };

            Self::from_verified_i64(c::float_sum(self.0, rhs.0))
        }
    }
}

impl Mul for XFL {
    type Output = Result<XFL>;

    #[inline(always)]
    fn mul(self, other: XFL) -> Self::Output {
        Self::from_verified_i64(unsafe { c::float_multiply(self.0, other.0) })
    }
}

impl Div for XFL {
    type Output = Result<XFL>;

    #[inline(always)]
    fn div(self, other: XFL) -> Self::Output {
        Self::from_verified_i64(unsafe { c::float_divide(self.0, other.0) })
    }
}

impl Neg for XFL {
    type Output = Result<XFL>;

    #[inline(always)]
    fn neg(self) -> Self::Output {
        Self::from_verified_i64(unsafe { c::float_negate(self.0) })
    }
}

impl PartialEq for XFL {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        let res = unsafe { c::float_compare(self.0, other.0, c::COMPARE_EQUAL) };

        match res {
            1 => true,
            // This is based on the invariant that the arguments to eq function
            // are all valid XFL numbers.
            _ => false,
        }
    }
}

impl PartialOrd for XFL {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unsafe {
            match c::float_compare(self.0, other.0, c::COMPARE_EQUAL) {
                1 => Some(Ordering::Equal),
                0 => {
                    // This is because float_compare cannot return an ordering at one go.
                    match c::float_compare(self.0, other.0, c::COMPARE_LESS) {
                        1 => Some(Ordering::Less),
                        0 => Some(Ordering::Greater),
                        _ => None,
                    }
                }
                _ => None,
            }
        }
    }
}
