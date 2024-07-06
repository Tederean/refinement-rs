#![cfg_attr(not(feature = "std"), no_std)]

//! Convenient creation of type-safe refinement types.
//!
//! This crate tries to capture the idea of a refinement type, which
//! is a type endowed with a predicate which is assumed to hold for
//! any element of the refined type.[^1]
//!
//! Refinement types are useful when only certain values of a type are expected at runtime.
//! As an example, suppose there's a function that only logically works on even integers.
//!
//! ```should_panic
//! fn takes_even(i: i32) {
//!     if i % 2 == 0 {
//!         println!("Received even number {}", i);
//!     } else {
//!         panic!("Received odd number");
//!     }
//! }
//!
//! takes_even(1);  // oops
//! ```
//!
//! Using a refinement type, this function may be defined in a way where it is impossible to supply
//! an odd number.
//!
//! ```
//! use refinement::{Refinement, Predicate};
//!
//! struct Even;
//!
//! impl Predicate<i32> for Even {
//!     fn test(x: &i32) -> bool {
//!         *x % 2 == 0
//!     }
//! }
//!
//! type EvenInt = Refinement<i32, Even>;
//!
//! fn takes_even(i: EvenInt) {
//!     println!("Received even number {}", i);
//! }
//!
//! match EvenInt::new(4) {
//!     Some(x) => takes_even(x),  // "Received even number 4"
//!     None => { /* ... */ }      // Handle logical error
//! }
//!
//! ```
//! [^1]: https://en.wikipedia.org/wiki/Refinement_type

#[cfg(feature = "serde")]
mod serde_support;

#[cfg(not(feature = "serde"))]
mod no_serde_support;

#[cfg(feature = "serde")]
pub use self::serde_support::*;

#[cfg(not(feature = "serde"))]
pub use self::no_serde_support::*;