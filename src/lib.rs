//! The Weekdays crate provides days of the week bit-mapped in a single byte.



//		Global configuration

//	Customisations of the standard linting configuration
#![allow(clippy::items_after_test_module, reason = "Not needed with separated tests")]

//	Lints specifically disabled for unit tests
#![cfg_attr(test, allow(
	non_snake_case,
	clippy::arithmetic_side_effects,
	clippy::cast_lossless,
	clippy::cast_precision_loss,
	clippy::cognitive_complexity,
	clippy::default_numeric_fallback,
	clippy::exhaustive_enums,
	clippy::exhaustive_structs,
	clippy::expect_used,
	clippy::indexing_slicing,
	clippy::let_underscore_must_use,
	clippy::let_underscore_untyped,
	clippy::missing_assert_message,
	clippy::missing_panics_doc,
	clippy::must_use_candidate,
	clippy::panic,
	clippy::print_stdout,
	clippy::too_many_lines,
	clippy::unwrap_in_result,
	clippy::unwrap_used,
	reason = "Not useful in unit tests"
))]



//		Modules

#[cfg(test)]
#[path = "tests/lib.rs"]
mod tests;



//		Packages

use bytes::BytesMut;
use chrono::Weekday;
use core::{
	error::Error,
	fmt::{Debug, Display, Formatter},
	fmt,
	ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not, Sub, SubAssign},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::io::{Error as IoError, ErrorKind as IoErrorKind};
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type, to_sql_checked};



//		Structs

//		Weekdays																
/// A bit-mapped representation of the days of the week.
/// 
/// This struct uses a single byte, where each bit represents a day of the week.
/// The bits are ordered from most significant to least significant as Monday to
/// Sunday, with the least significant bit representing Sunday.
/// 
#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Weekdays(u8);

//󰭅		Weekdays																
impl Weekdays {
	//		Public constants													
	/// Monday.
	pub const MONDAY:    Self = Self(0b100_0000);
	
	/// Tuesday.
	pub const TUESDAY:   Self = Self(0b010_0000);
	
	/// Wednesday.
	pub const WEDNESDAY: Self = Self(0b001_0000);
	
	/// Thursday.
	pub const THURSDAY:  Self = Self(0b000_1000);
	
	/// Friday.
	pub const FRIDAY:    Self = Self(0b000_0100);
	
	/// Saturday.
	pub const SATURDAY:  Self = Self(0b000_0010);
	
	/// Sunday.
	pub const SUNDAY:    Self = Self(0b000_0001);
	
	/// Weekdays (Monday to Friday).
	pub const WEEKDAYS:  Self = Self(0b111_1100);
	
	/// Weekends (Saturday and Sunday).
	pub const WEEKENDS:  Self = Self(0b000_0011);
	
	/// All days of the week.
	pub const ALL_DAYS:  Self = Self(0b111_1111);
	
	/// No days of the week (empty).
	pub const NONE:      Self = Self(0b000_0000);
	
	//		Private constants													
	/// The mask for all days of the week.
	const ALL_DAYS_MASK: u8 = 0b111_1111;
	
	//		Constructors														
	
	//		new																	
	/// Creates a new [`Weekdays`] struct from the given number of days.
	/// 
	/// The number of days is expected to be a 7-bit number, where each bit
	/// represents a day of the week. The bits are ordered from most significant
	/// to least significant as Monday to Sunday, with the least significant bit
	/// representing Sunday.
	/// 
	/// # Parameters
	/// 
	/// * `days` - The days to represent, as a bit-mapped value.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use weekdays::Weekdays;
	/// 
	/// let weekdays = Weekdays::new(0b111_1100);
	/// assert_eq!(weekdays, Weekdays::WEEKDAYS);
	/// ```
	/// 
	#[must_use]
	pub const fn new(days: u8) -> Self {
		//	Ensure only 7 bits are used
		Self(days & Self::ALL_DAYS_MASK)
	}
	
	//		Public methods														
	
	//		contains															
	/// Checks if the given day (or days) is contained within the set of days.
	/// 
	/// This method can be used to check for the presence of a single day, or
	/// multiple days such as a weekend or other combination.
	/// 
	/// It will return `true` if all the bits of the given day(s) are set in the
	/// current set of days.
	/// 
	/// # Parameters
	/// 
	/// * `day` - The day to check for.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use weekdays::Weekdays;
	/// 
	/// let weekdays = Weekdays::WEEKDAYS;
	/// assert!( weekdays.contains(Weekdays::MONDAY));
	/// assert!( weekdays.contains(Weekdays::FRIDAY));
	/// assert!(!weekdays.contains(Weekdays::SATURDAY));
	/// ```
	/// 
	#[must_use]
	pub const fn contains(&self, day: Self) -> bool {
		self.0 & day.0 == day.0
	}
	
	//		days																
	/// Returns the number of days set.
	/// 
	/// This method will count the number of bits set in the bit-mapped value,
	/// and return the count as the number of days.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use weekdays::Weekdays;
	/// 
	/// let weekdays = Weekdays::WEEKDAYS;
	/// assert_eq!(weekdays.days(), 5);
	/// ```
	/// 
	#[expect(clippy::cast_possible_truncation, reason = "Value is guaranteed to be <= 7")]
	#[must_use]
	pub const fn days(&self) -> u8 {
		self.0.count_ones() as u8
	}
	
	//		is_empty															
	/// Checks if the set of days is empty.
	/// 
	/// This method will return `true` if no days are set in the bit-mapped
	/// value.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use weekdays::Weekdays;
	/// 
	/// let weekdays = Weekdays::NONE;
	/// assert!(weekdays.is_empty());
	/// ```
	/// 
	#[must_use]
	pub const fn is_empty(&self) -> bool {
		self.0 == 0
	}
	
	//		is_weekday															
	/// Checks if the set of days represents a weekday.
	/// 
	/// This method will return `true` if the set of days contains only weekdays
	/// (Monday to Friday).
	/// 
	/// # Examples
	/// 
	/// ```
	/// use weekdays::Weekdays;
	/// 
	/// let weekdays = Weekdays::THURSDAY;
	/// assert!(weekdays.is_weekday());
	/// ```
	/// 
	#[must_use]
	pub const fn is_weekday(&self) -> bool {
		self.0 & Self::WEEKENDS.0 == 0
	}
	
	//		is_weekend															
	/// Checks if the set of days represents a weekend.
	/// 
	/// This method will return `true` if the set of days contains only weekends
	/// (Saturday and Sunday).
	/// 
	/// # Examples
	/// 
	/// ```
	/// use weekdays::Weekdays;
	/// 
	/// let weekdays = Weekdays::SATURDAY;
	/// assert!(weekdays.is_weekend());
	/// ```
	/// 
	#[must_use]
	pub const fn is_weekend(&self) -> bool {
		self.0 & Self::WEEKDAYS.0 == 0
	}
	
	//		iter																
	/// Returns an iterator over the days of the week.
	/// 
	/// This method will return an iterator that will yield each day of the week
	/// that is set in the bit-mapped value.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use weekdays::Weekdays;
	/// 
	/// let weekdays = Weekdays::WEEKDAYS;
	/// let mut iter = weekdays.iter();
	/// 
	/// assert_eq!(iter.next(), Some(Weekdays::MONDAY));
	/// assert_eq!(iter.next(), Some(Weekdays::TUESDAY));
	/// assert_eq!(iter.next(), Some(Weekdays::WEDNESDAY));
	/// assert_eq!(iter.next(), Some(Weekdays::THURSDAY));
	/// assert_eq!(iter.next(), Some(Weekdays::FRIDAY));
	/// assert_eq!(iter.next(), None);
	/// ```
	/// 
	/// # See also
	/// 
	/// * [`Iterator`]
	/// 
	#[must_use]
	pub const fn iter(&self) -> WeekdaysIter {
		WeekdaysIter {
			remaining: self.0,
			position:  0,
		}
	}
	
	//		to_chrono_vec														
	/// Converts the set of days to a [`Vec`] of Chrono [`Weekday`]s.
	/// 
	/// This method will return a [`Vec`] containing each day of the week that
	/// is set in the bit-mapped value, converted to a Chrono [`Weekday`].
	/// 
	/// # Examples
	/// 
	/// ```
	/// use chrono::Weekday;
	/// use weekdays::Weekdays;
	///
	/// let weekdays = Weekdays::WEEKDAYS;
	/// let days     = weekdays.to_chrono_vec();
	///
	/// assert_eq!(days, vec![
	///     Weekday::Mon,
	///     Weekday::Tue,
	///     Weekday::Wed,
	///     Weekday::Thu,
	///     Weekday::Fri,
	/// ]);
	///
	/// assert_eq!(Weekdays::NONE.to_chrono_vec(), vec![]);
	/// ```
	/// 
	#[must_use]
	pub fn to_chrono_vec(&self) -> Vec<Weekday> {
		[
			Weekday::Mon,
			Weekday::Tue,
			Weekday::Wed,
			Weekday::Thu,
			Weekday::Fri,
			Weekday::Sat,
			Weekday::Sun,
		]
			.into_iter()
			.filter(|&day| self.contains(Self::from(day)))
			.collect()
	}
	
	//		to_vec																
	/// Converts the set of days to a [`Vec`] of days.
	/// 
	/// This method will return a [`Vec`] containing each day of the week that
	/// is set in the bit-mapped value.
	/// 
	/// # Examples
	/// 
	/// ```
	/// use weekdays::Weekdays;
	/// 
	/// let weekdays = Weekdays::WEEKDAYS;
	/// let days     = weekdays.to_vec();
	/// 
	/// assert_eq!(days, vec![
	///     Weekdays::MONDAY,
	///     Weekdays::TUESDAY,
	///     Weekdays::WEDNESDAY,
	///     Weekdays::THURSDAY,
	///     Weekdays::FRIDAY,
	/// ]);
	/// 
	/// assert_eq!(Weekdays::NONE.to_vec(), vec![]);
	/// ```
	/// 
	#[must_use]
	pub fn to_vec(&self) -> Vec<Self> {
		self.iter().collect()
	}
}

//󰭅		Add																		
impl Add for Weekdays {
	type Output = Self;
	
	//		add																	
	#[expect(clippy::suspicious_arithmetic_impl, reason = "Bitwise OR is the correct operation")]
	fn add(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}

//󰭅		AddAssign																
impl AddAssign for Weekdays {
	//		add_assign															
	#[expect(clippy::suspicious_op_assign_impl, reason = "Bitwise OR is the correct operation")]
	fn add_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0;
	}
}

//󰭅		BitAnd																	
impl BitAnd for Weekdays {
	type Output = Self;
	
	//		bitand																
	fn bitand(self, rhs: Self) -> Self::Output {
		Self(self.0 & rhs.0)
	}
}

//󰭅		BitAndAssign															
impl BitAndAssign for Weekdays {
	//		bitand_assign														
	fn bitand_assign(&mut self, rhs: Self) {
		self.0 &= rhs.0;
	}
}

//󰭅		BitOr																	
impl BitOr for Weekdays {
	type Output = Self;
	
	//		bitor																
	fn bitor(self, rhs: Self) -> Self::Output {
		Self(self.0 | rhs.0)
	}
}

//󰭅		BitOrAssign																
impl BitOrAssign for Weekdays {
	//		bitor_assign														
	fn bitor_assign(&mut self, rhs: Self) {
		self.0 |= rhs.0;
	}
}

//󰭅		BitXor																	
impl BitXor for Weekdays {
	type Output = Self;
	
	//		bitxor																
	fn bitxor(self, rhs: Self) -> Self::Output {
		Self(self.0 ^ rhs.0)
	}
}

//󰭅		BitXorAssign															
impl BitXorAssign for Weekdays {
	//		bitxor_assign														
	fn bitxor_assign(&mut self, rhs: Self) {
		self.0 ^= rhs.0;
	}
}

//󰭅		Debug																	
impl Debug for Weekdays {
	//		fmt																	
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Weekdays({:03b}_{:04b})", self.0 >> 4, self.0 & 0b1111)
	}
}

//󰭅		Deserialize																
impl<'de> Deserialize<'de> for Weekdays {
	//		deserialize															
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
		where D: Deserializer<'de>,
	{
		u8::deserialize(deserializer).map(Self::new)
	}
}

//󰭅		Display																	
impl Display for Weekdays {
	//		fmt																	
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{:05b}_{:02b}", self.0 >> 2, self.0 & 0b11)
	}
}

//󰭅		From: Weekday -> Weekdays												
impl From<Weekday> for Weekdays {
	//		from																
	fn from(day: Weekday) -> Self {
		match day {
			Weekday::Mon => Self::MONDAY,
			Weekday::Tue => Self::TUESDAY,
			Weekday::Wed => Self::WEDNESDAY,
			Weekday::Thu => Self::THURSDAY,
			Weekday::Fri => Self::FRIDAY,
			Weekday::Sat => Self::SATURDAY,
			Weekday::Sun => Self::SUNDAY,
		}
	}
}

//󰭅		FromSql																	
impl FromSql<'_> for Weekdays {
	//		from_sql															
	fn from_sql(ty: &Type, raw: &[u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
		match ty {
			&Type::BIT => Ok(
				//	PostgreSQL gives us a byte that represents the 7 bits
				match raw.first() {
					Some(&byte) => Self(byte & Self::ALL_DAYS_MASK),
					None        => Self(0)
				}
			),
			unknown    => Err(Box::new(IoError::new(
				IoErrorKind::InvalidData,
				format!("Invalid type for Weekdays: {unknown}"),
			))),
		}
	}
	
	//		accepts																
	fn accepts(ty: &Type) -> bool {
		ty.name() == "bit"
	}
}

//󰭅		IntoIterator															
impl IntoIterator for Weekdays {
	type Item     = Self;
	type IntoIter = WeekdaysIter;
	
	//		into_iter															
	fn into_iter(self) -> Self::IntoIter {
		WeekdaysIter {
			remaining: self.0,
			position:  0,
		}
	}
}

//󰭅		IntoIterator															
impl IntoIterator for &Weekdays {
	type Item     = Weekdays;
	type IntoIter = WeekdaysIter;
	
	//		into_iter															
	fn into_iter(self) -> Self::IntoIter {
		WeekdaysIter {
			remaining: self.0,
			position:  0,
		}
	}
}

//󰭅		Not																		
impl Not for Weekdays {
	type Output = Self;
	
	//		not																	
	fn not(self) -> Self::Output {
		Self(!self.0 & Self::ALL_DAYS_MASK)
	}
}

//󰭅		Serialize																
impl Serialize for Weekdays {
	//		serialize															
	fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_u8(self.0)
	}
}

//󰭅		Sub																		
impl Sub for Weekdays {
	type Output = Self;
	
	//		sub																	
	fn sub(self, rhs: Self) -> Self::Output {
		Self(self.0 & !rhs.0)
	}
}

//󰭅		SubAssign																
impl SubAssign for Weekdays {
	//		sub_assign															
	fn sub_assign(&mut self, rhs: Self) {
		self.0 &= !rhs.0;
	}
}

//󰭅		ToSql																	
impl ToSql for Weekdays {
	//		to_sql																
	fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
		match ty {
			&Type::BIT => {
				//	PostgreSQL expects 7 bits in the same format
				out.extend_from_slice(&[self.0]);
				Ok(IsNull::No)
			}
			unknown    => Err(Box::new(IoError::new(
				IoErrorKind::InvalidData,
				format!("Invalid type for Weekdays: {unknown}"),
			))),
		}
	}
	
	//		accepts																
	fn accepts(ty: &Type) -> bool {
		ty.name() == "bit"
	}
	
	to_sql_checked!();
}

//		WeekdaysIter															
/// An iterator over the days of the week.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WeekdaysIter {
	/// The remaining days to iterate over.
	remaining: u8,
	
	/// The current position in the iteration.
	position:  u8,
}

//󰭅		Iterator																
impl Iterator for WeekdaysIter {
	type Item = Weekdays;
	
	//		next																
	fn next(&mut self) -> Option<Self::Item> {
		while self.position < 7 {
			let current      = 0b100_0000 >> self.position;
			#[expect(clippy::arithmetic_side_effects, reason = "This is checked by the while loop")]
			{ self.position += 1; }
			
			if self.remaining & current != 0 {
				return Some(Weekdays(current));
			}
		}
		None
	}
}
