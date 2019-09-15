//! A `Scalar` represents an element of the finite field 
//! modulo `2^249 - 15145038707218910765482344729778085401`.
//! 
//! The `Scalar` type is an alias for one of the backend
//! implementations. 
//! 
//! `ConstantTimeEq` and `PartialEq` traits have been implemented 
//! here since they will be the samme across all of the different
//! backends.
//! 
//! Here it is also defined the `Ristretto255Scalar` type,
//! which is a type-alias for the curve25519-dalek Scalar Struct.
//! 
//! # Examples
//! ```rust
//! use zerocaf::scalar::Scalar;
//! use zerocaf::traits::ops::*;
//! 
//! // You can create a Scalar from a byte-array as follows:
//! let a = Scalar::from_bytes(&[0u8;32]); 
//! 
//! // You ca also create a Scalar from an uint type as follows:
//! let b = Scalar::from(&43325u128);
//! let c = Scalar::from(&86650u64);
//! 
//! // The last way of creating a Scalar it by calling the
//! // constructor. THIS IS NOT RECOMMENDED since ANY checks about
//! // the correctness of the input will be done. It can be done as
//! // follows: 
//! let d: Scalar = Scalar([0, 1, 0, 0, 0]); // d = 2^52.
//! assert!(d == Scalar::two_pow_k(&52u64));
//! 
//! // All of the basuc modular operations are implemented 
//! // for Scalar type:  
//! let mut res = a + b; // Performs a + b (mod l).
//! res = a - b; // Performs a - b (mod l).
//! res = c * d; // Performs c * d (mod l).
//! res = a.square(); // Performs a^2 (mod l).
//! res = -&a; // Performs Negation over the modulo l.
//! 
//! // Dividing even Scalars by two is recommended through the `Half`
//! // trait implmementation since it's much faster.
//! if a.is_even() {
//!     let half_c = c.half(); // This will panic if a isn't even.
//!     assert!(half_c == b);
//! }
//! 
//! // You can export your `Scalar` as an slice of 32 bytes in Little
//! // Endian encoding by:
//! let d_bytes: [u8; 32] = d.to_bytes();
//! ```
//! 
//! 
//! 
//! `PartialOrd`, `Ord`, `PartialEq` and `Eq` are also implemented for
//! `Scalar` type. 
//! 
//! All `std::core::ops traits -> (Add, Sub, Mul)` are implemented
//! for both, `&Scalar` and `Scalar`.

use crate::backend;

use subtle::Choice;
use subtle::ConstantTimeEq;

use rand::{Rng, CryptoRng};


#[cfg(feature = "u64_backend")]
pub use backend::u64::scalar::*;
/// A `Scalar` represents an element of the field generated by
/// the prime of the sub-group: `2^249 - 15145038707218910765482344729778085401`.
///
/// This is a type alias for one of the Scalar types in the `backend`
/// module.
#[cfg(feature = "u64_backend")]
pub type Scalar = backend::u64::scalar::Scalar;

impl PartialEq for Scalar {
    fn eq(&self, other: &Scalar) -> bool {
        self.ct_eq(other).unwrap_u8() == 1u8
    }
}

impl ConstantTimeEq for Scalar {
    /// Test equality between two `Scalar`s.  Since the
    /// internal representation is not canonical, the field elements
    /// are normalized to wire format before comparison.
    fn ct_eq(&self, other: &Scalar) -> Choice {
        self.to_bytes().ct_eq(&other.to_bytes())
    }
}

impl Eq for Scalar {}

impl Scalar {
    /// Generate a valid Scalar choosen uniformly using user-
    /// provided rng.
    /// 
    /// By `rng` we mean any Rng that implements: `Rng` + `CryptoRng`.
    pub fn random<T>(rand: &mut T) -> Scalar 
        where T: Rng + CryptoRng {
            let mut bytes = [0u8; 32];
            rand.fill_bytes(&mut bytes);
            Scalar::from_bytes(&bytes)
    }
}


/// This is a type alias for the Scalar type in the `curve25519-dalek` lib.
pub type Ristretto255Scalar = curve25519_dalek::scalar::Scalar;
