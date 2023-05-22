use core::{
	mem,
	ops::{Add, Sub},
};

/// Distance between singletons.
#[allow(clippy::len_without_is_empty)]
pub trait Measure<U = Self> {
	type Len: Default + Add<Output = Self::Len> + Sub<Output = Self::Len> + PartialEq;

	/// Returns the length of the given element.
	fn len(&self) -> Self::Len;

	/// Returns the distance to the given other element.
	fn distance(&self, other: &U) -> Self::Len;
}

impl Measure for char {
	type Len = u64;

	fn len(&self) -> u64 {
		1
	}

	fn distance(&self, other: &char) -> u64 {
		let mut a = *self as u64;
		let mut b = *other as u64;

		if a > b {
			mem::swap(&mut a, &mut b);
		}

		if (..=0xd7ff).contains(&a) && (0xe000..).contains(&b) {
			(b - 0xd000 + 1) + (0xd7ff - a)
		} else {
			b - a
		}
	}
}

macro_rules! impl_measure {
	// Measure for type `$ty`.
	// `$cast` is a type that can handle the subtraction of two elements
	// without overflowing.
	// `$len` is a type that can handle the size of the entire domain of `$ty`.
	(@refl $ty:ty, $cast:ty, $len:ty) => {
		impl_measure!($ty, $ty, $cast, $len);
	};
	(@both $ty1:ty, $ty2:ty, $cast:ty, $len:ty) => {
		impl_measure!($ty1, $ty2, $cast, $len);
		impl_measure!($ty2, $ty1, $cast, $len);
	};
	($ty1:ty, $ty2:ty, $cast:ty, $len:ty) => {
		impl Measure<$ty2> for $ty1 {
			type Len = $len;

			fn len(&self) -> $len {
				1
			}

			fn distance(&self, other: &$ty2) -> $len {
				let a = *self as $cast;
				let b = *other as $cast;

				if a > b {
					(a - b) as $len
				} else {
					(b - a) as $len
				}
			}
		}
	};
}

// All of those are generated by the `generate-measures.rb` script
// to avoid mistakes.
impl_measure!(@refl u8, u8, u16);
impl_measure!(@refl u16, u16, u32);
impl_measure!(@refl u32, u32, u64);
impl_measure!(@refl u64, u64, u128);
impl_measure!(@refl i8, i16, u8);
impl_measure!(@refl i16, i32, u16);
impl_measure!(@refl i32, i64, u32);
impl_measure!(@refl i64, i128, u64);
impl_measure!(@both u8, u16, u16, u32);
impl_measure!(@both u8, u32, u32, u64);
impl_measure!(@both u8, u64, u64, u128);
impl_measure!(@both u8, i8, i16, u16);
impl_measure!(@both u8, i16, i32, u16);
impl_measure!(@both u8, i32, i64, u32);
impl_measure!(@both u8, i64, i128, u64);
impl_measure!(@both u16, u32, u32, u64);
impl_measure!(@both u16, u64, u64, u128);
impl_measure!(@both u16, i8, i32, u32);
impl_measure!(@both u16, i16, i32, u32);
impl_measure!(@both u16, i32, i64, u32);
impl_measure!(@both u16, i64, i128, u64);
impl_measure!(@both u32, u64, u64, u128);
impl_measure!(@both u32, i8, i64, u64);
impl_measure!(@both u32, i16, i64, u64);
impl_measure!(@both u32, i32, i64, u64);
impl_measure!(@both u32, i64, i128, u64);
impl_measure!(@both u64, i8, i128, u128);
impl_measure!(@both u64, i16, i128, u128);
impl_measure!(@both u64, i32, i128, u128);
impl_measure!(@both u64, i64, i128, u128);
impl_measure!(@both i8, i16, i32, u16);
impl_measure!(@both i8, i32, i64, u32);
impl_measure!(@both i8, i64, i128, u64);
impl_measure!(@both i16, i32, i64, u32);
impl_measure!(@both i16, i64, i128, u64);
impl_measure!(@both i32, i64, i128, u64);
#[cfg(target_pointer_width = "8")]
impl_measure!(@refl usize, u8, u16);
#[cfg(target_pointer_width = "8")]
impl_measure!(@both usize, u8, u8, u16);
#[cfg(target_pointer_width = "8")]
impl_measure!(@both usize, u16, u16, u32);
#[cfg(target_pointer_width = "8")]
impl_measure!(@both usize, u32, u32, u64);
#[cfg(target_pointer_width = "8")]
impl_measure!(@both usize, u64, u64, u128);
#[cfg(target_pointer_width = "8")]
impl_measure!(@both usize, i8, i16, u16);
#[cfg(target_pointer_width = "8")]
impl_measure!(@both usize, i16, i32, u16);
#[cfg(target_pointer_width = "8")]
impl_measure!(@both usize, i32, i64, u32);
#[cfg(target_pointer_width = "8")]
impl_measure!(@both usize, i64, i128, u64);
#[cfg(target_pointer_width = "16")]
impl_measure!(@refl usize, u16, u32);
#[cfg(target_pointer_width = "16")]
impl_measure!(@both usize, u8, u16, u32);
#[cfg(target_pointer_width = "16")]
impl_measure!(@both usize, u16, u16, u32);
#[cfg(target_pointer_width = "16")]
impl_measure!(@both usize, u32, u32, u64);
#[cfg(target_pointer_width = "16")]
impl_measure!(@both usize, u64, u64, u128);
#[cfg(target_pointer_width = "16")]
impl_measure!(@both usize, i8, i32, u32);
#[cfg(target_pointer_width = "16")]
impl_measure!(@both usize, i16, i32, u32);
#[cfg(target_pointer_width = "16")]
impl_measure!(@both usize, i32, i64, u32);
#[cfg(target_pointer_width = "16")]
impl_measure!(@both usize, i64, i128, u64);
#[cfg(target_pointer_width = "32")]
impl_measure!(@refl usize, u32, u64);
#[cfg(target_pointer_width = "32")]
impl_measure!(@both usize, u8, u32, u64);
#[cfg(target_pointer_width = "32")]
impl_measure!(@both usize, u16, u32, u64);
#[cfg(target_pointer_width = "32")]
impl_measure!(@both usize, u32, u32, u64);
#[cfg(target_pointer_width = "32")]
impl_measure!(@both usize, u64, u64, u128);
#[cfg(target_pointer_width = "32")]
impl_measure!(@both usize, i8, i64, u64);
#[cfg(target_pointer_width = "32")]
impl_measure!(@both usize, i16, i64, u64);
#[cfg(target_pointer_width = "32")]
impl_measure!(@both usize, i32, i64, u64);
#[cfg(target_pointer_width = "32")]
impl_measure!(@both usize, i64, i128, u64);
#[cfg(target_pointer_width = "64")]
impl_measure!(@refl usize, u64, u128);
#[cfg(target_pointer_width = "64")]
impl_measure!(@both usize, u8, u64, u128);
#[cfg(target_pointer_width = "64")]
impl_measure!(@both usize, u16, u64, u128);
#[cfg(target_pointer_width = "64")]
impl_measure!(@both usize, u32, u64, u128);
#[cfg(target_pointer_width = "64")]
impl_measure!(@both usize, u64, u64, u128);
#[cfg(target_pointer_width = "64")]
impl_measure!(@both usize, i8, i128, u128);
#[cfg(target_pointer_width = "64")]
impl_measure!(@both usize, i16, i128, u128);
#[cfg(target_pointer_width = "64")]
impl_measure!(@both usize, i32, i128, u128);
#[cfg(target_pointer_width = "64")]
impl_measure!(@both usize, i64, i128, u128);

macro_rules! impl_f_measure {
	($ty:ty, $zero:expr, $min:expr, $max:expr) => {
		impl Measure<$ty> for $ty {
			type Len = $ty;

			fn len(&self) -> $ty {
				$zero
			}

			fn distance(&self, other: &$ty) -> $ty {
				if self.is_infinite() || other.is_infinite() {
					$max
				} else {
					let a = *self as $ty;
					let b = *other as $ty;

					if a > b {
						(a - b) as $ty
					} else {
						(b - a) as $ty
					}
				}
			}
		}
	};
}

impl_f_measure!(f32, 0.0f32, f32::NEG_INFINITY, f32::INFINITY);
impl_f_measure!(f64, 0.0f64, f64::NEG_INFINITY, f64::INFINITY);

#[cfg(feature = "ordered-float")]
mod ordered_float {
	use super::Measure;
	use ordered_float::NotNan;

	impl_f_measure!(
		NotNan<f32>,
		unsafe { NotNan::new_unchecked(0.0f32) },
		unsafe { NotNan::new_unchecked(f32::NEG_INFINITY) },
		unsafe { NotNan::new_unchecked(f32::INFINITY) }
	);

	impl_f_measure!(
		NotNan<f64>,
		unsafe { NotNan::new_unchecked(0.0f64) },
		unsafe { NotNan::new_unchecked(f64::NEG_INFINITY) },
		unsafe { NotNan::new_unchecked(f64::INFINITY) }
	);
}
