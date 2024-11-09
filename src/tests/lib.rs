#![allow(clippy::unusual_byte_groupings, reason = "Clearer to show weekdays vs weekends")]

//		Packages

use super::*;
use claims::assert_err;



//		Tests

mod constructors {
	use super::*;
	
	//		new																	
	#[test]
	fn new__valid() {
		assert_eq!(Weekdays::new(0b0000_0000).0, 0b0000_0000);
		assert_eq!(Weekdays::new(0b0000_0001).0, 0b0000_0001);
		assert_eq!(Weekdays::new(0b0000_1000).0, 0b0000_1000);
		assert_eq!(Weekdays::new(0b0100_0000).0, 0b0100_0000);
		assert_eq!(Weekdays::new(0b0111_1111).0, 0b0111_1111);
	}
	#[test]
	fn new__invalid() {
		assert_eq!(Weekdays::new(0b1000_0000).0, 0b0000_0000);
		assert_eq!(Weekdays::new(0b1000_0001).0, 0b0000_0001);
		assert_eq!(Weekdays::new(0b1111_1111).0, 0b0111_1111);
	}
}

mod public_methods {
	use super::*;
	
	//		contains															
	#[test]
	fn contains() {
		let weekdays = Weekdays::new(0b01010_10);
		assert!( weekdays.contains(Weekdays::new(0b00000_00)));
		assert!(!weekdays.contains(Weekdays::new(0b00000_01)));
		assert!( weekdays.contains(Weekdays::new(0b00000_10)));
		assert!( weekdays.contains(Weekdays::new(0b01010_00)));
		assert!( weekdays.contains(Weekdays::new(0b01010_10)));
		assert!(!weekdays.contains(Weekdays::new(0b01010_11)));
	}
	
	//		days																
	#[test]
	fn days() {
		assert_eq!(Weekdays::new(0b00000_00).days(), 0);
		assert_eq!(Weekdays::new(0b00000_01).days(), 1);
		assert_eq!(Weekdays::new(0b00010_00).days(), 1);
		assert_eq!(Weekdays::new(0b10001_00).days(), 2);
		assert_eq!(Weekdays::new(0b11111_11).days(), 7);
	}
	
	//		is_empty															
	#[test]
	fn is_empty() {
		assert!( Weekdays::new(0b00000_00).is_empty());
		assert!(!Weekdays::new(0b00000_01).is_empty());
		assert!(!Weekdays::new(0b00010_00).is_empty());
		assert!(!Weekdays::new(0b10001_00).is_empty());
		assert!(!Weekdays::new(0b11111_11).is_empty());
	}
	
	//		is_weekday															
	#[test]
	fn is_weekday() {
		assert!( Weekdays::MONDAY   .is_weekday());
		assert!( Weekdays::TUESDAY  .is_weekday());
		assert!( Weekdays::WEDNESDAY.is_weekday());
		assert!( Weekdays::THURSDAY .is_weekday());
		assert!( Weekdays::FRIDAY   .is_weekday());
		assert!(!Weekdays::SATURDAY .is_weekday());
		assert!(!Weekdays::SUNDAY   .is_weekday());
		assert!( Weekdays::WEEKDAYS .is_weekday());
		assert!(!Weekdays::WEEKENDS .is_weekday());
	}
	
	//		is_weekend															
	#[test]
	fn is_weekend() {
		assert!(!Weekdays::MONDAY   .is_weekend());
		assert!(!Weekdays::TUESDAY  .is_weekend());
		assert!(!Weekdays::WEDNESDAY.is_weekend());
		assert!(!Weekdays::THURSDAY .is_weekend());
		assert!(!Weekdays::FRIDAY   .is_weekend());
		assert!( Weekdays::SATURDAY .is_weekend());
		assert!( Weekdays::SUNDAY   .is_weekend());
		assert!(!Weekdays::WEEKDAYS .is_weekend());
		assert!( Weekdays::WEEKENDS .is_weekend());
	}
	
	//		iter																
	#[test]
	fn iter() {
		let weekdays = Weekdays::new(0b01010_10);
		let mut iter = weekdays.iter();
		assert_eq!(iter.next(), Some(Weekdays::TUESDAY));
		assert_eq!(iter.next(), Some(Weekdays::THURSDAY));
		assert_eq!(iter.next(), Some(Weekdays::SATURDAY));
		assert_eq!(iter.next(), None);
	}
	
	//		to_vec																
	#[test]
	fn to_vec() {
		assert_eq!(Weekdays::new(0b00000_00).to_vec(), vec![]);
		assert_eq!(Weekdays::new(0b00000_01).to_vec(), vec![Weekdays::SUNDAY]);
		assert_eq!(Weekdays::new(0b00010_00).to_vec(), vec![Weekdays::THURSDAY]);
		assert_eq!(Weekdays::new(0b10001_00).to_vec(), vec![Weekdays::MONDAY, Weekdays::FRIDAY]);
		assert_eq!(Weekdays::new(0b11111_11).to_vec(), vec![
			Weekdays::MONDAY,
			Weekdays::TUESDAY,
			Weekdays::WEDNESDAY,
			Weekdays::THURSDAY,
			Weekdays::FRIDAY,
			Weekdays::SATURDAY,
			Weekdays::SUNDAY,
		]);
	}
}

mod derived_traits {
	use super::*;
	
	//		Eq																	
	#[test]
	fn eq() {
		assert_eq!(Weekdays::new(0b00000_00), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_01), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00010_00), Weekdays::new(0b00010_00));
		assert_eq!(Weekdays::new(0b10000_00), Weekdays::new(0b10000_00));
		assert_eq!(Weekdays::new(0b11111_11), Weekdays::new(0b11111_11));
	}
	
	//		Ord																	
	#[test]
	fn ord() {
		assert!(Weekdays::new(0b00000_00) < Weekdays::new(0b00000_01));
		assert!(Weekdays::new(0b00000_01) < Weekdays::new(0b00010_00));
		assert!(Weekdays::new(0b00010_00) < Weekdays::new(0b10000_00));
		assert!(Weekdays::new(0b10000_00) < Weekdays::new(0b11111_11));
	}
}

mod traits {
	use super::*;
	
	//		Add																	
	#[test]
	fn add() {
		assert_eq!(Weekdays::new(0b00000_00) + Weekdays::new(0b00000_00), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_01) + Weekdays::new(0b00000_00), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_00) + Weekdays::new(0b00000_01), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_01) + Weekdays::new(0b00000_01), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_01) + Weekdays::new(0b00000_10), Weekdays::new(0b00000_11));
	}
	
	//		AddAssign															
	#[test]
	fn add_assign() {
		let mut weekdays1 = Weekdays::new(0b00000_00);
		weekdays1        += Weekdays::new(0b00000_00);
		assert_eq!(weekdays1, Weekdays::new(0b00000_00));
		
		let mut weekdays2 = Weekdays::new(0b00000_01);
		weekdays2        += Weekdays::new(0b00000_00);
		assert_eq!(weekdays2, Weekdays::new(0b00000_01));
		
		let mut weekdays3 = Weekdays::new(0b00000_00);
		weekdays3        += Weekdays::new(0b00000_01);
		assert_eq!(weekdays3, Weekdays::new(0b00000_01));
		
		let mut weekdays4 = Weekdays::new(0b00000_01);
		weekdays4        += Weekdays::new(0b00000_01);
		assert_eq!(weekdays4, Weekdays::new(0b00000_01));
		
		let mut weekdays5 = Weekdays::new(0b00000_01);
		weekdays5        += Weekdays::new(0b00000_10);
		assert_eq!(weekdays5, Weekdays::new(0b00000_11));
	}
	
	//		BitAnd																
	#[test]
	fn bitand() {
		assert_eq!(Weekdays::new(0b00000_00) & Weekdays::new(0b00000_00), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_01) & Weekdays::new(0b00000_00), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_00) & Weekdays::new(0b00000_01), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_01) & Weekdays::new(0b00000_01), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_01) & Weekdays::new(0b00000_10), Weekdays::new(0b00000_00));
	}
	
	//		BitAndAssign														
	#[test]
	fn bitand_assign() {
		let mut weekdays1 = Weekdays::new(0b00000_00);
		weekdays1        &= Weekdays::new(0b00000_00);
		assert_eq!(weekdays1, Weekdays::new(0b00000_00));
		
		let mut weekdays2 = Weekdays::new(0b00000_01);
		weekdays2        &= Weekdays::new(0b00000_00);
		assert_eq!(weekdays2, Weekdays::new(0b00000_00));
		
		let mut weekdays3 = Weekdays::new(0b00000_00);
		weekdays3        &= Weekdays::new(0b00000_01);
		assert_eq!(weekdays3, Weekdays::new(0b00000_00));
		
		let mut weekdays4 = Weekdays::new(0b00000_01);
		weekdays4        &= Weekdays::new(0b00000_01);
		assert_eq!(weekdays4, Weekdays::new(0b00000_01));
		
		let mut weekdays5 = Weekdays::new(0b00000_01);
		weekdays5        &= Weekdays::new(0b00000_10);
		assert_eq!(weekdays5, Weekdays::new(0b00000_00));
	}
	
	//		BitOr																
	#[test]
	fn bitor() {
		assert_eq!(Weekdays::new(0b00000_00) | Weekdays::new(0b00000_00), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_01) | Weekdays::new(0b00000_00), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_00) | Weekdays::new(0b00000_01), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_01) | Weekdays::new(0b00000_01), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_01) | Weekdays::new(0b00000_10), Weekdays::new(0b00000_11));
	}
	
	//		BitOrAssign															
	#[test]
	fn bitor_assign() {
		let mut weekdays1 = Weekdays::new(0b00000_00);
		weekdays1        |= Weekdays::new(0b00000_00);
		assert_eq!(weekdays1, Weekdays::new(0b00000_00));
		
		let mut weekdays2 = Weekdays::new(0b00000_01);
		weekdays2        |= Weekdays::new(0b00000_00);
		assert_eq!(weekdays2, Weekdays::new(0b00000_01));
		
		let mut weekdays3 = Weekdays::new(0b00000_00);
		weekdays3        |= Weekdays::new(0b00000_01);
		assert_eq!(weekdays3, Weekdays::new(0b00000_01));
		
		let mut weekdays4 = Weekdays::new(0b00000_01);
		weekdays4        |= Weekdays::new(0b00000_01);
		assert_eq!(weekdays4, Weekdays::new(0b00000_01));
		
		let mut weekdays5 = Weekdays::new(0b00000_01);
		weekdays5        |= Weekdays::new(0b00000_10);
		assert_eq!(weekdays5, Weekdays::new(0b00000_11));
	}
	
	//		BitXor																
	#[test]
	fn bitxor() {
		assert_eq!(Weekdays::new(0b00000_00) ^ Weekdays::new(0b00000_00), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_01) ^ Weekdays::new(0b00000_00), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_00) ^ Weekdays::new(0b00000_01), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_01) ^ Weekdays::new(0b00000_01), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_01) ^ Weekdays::new(0b00000_10), Weekdays::new(0b00000_11));
	}
	
	//		BitXorAssign														
	#[test]
	fn bitxor_assign() {
		let mut weekdays1 = Weekdays::new(0b00000_00);
		weekdays1        ^= Weekdays::new(0b00000_00);
		assert_eq!(weekdays1, Weekdays::new(0b00000_00));
		
		let mut weekdays2 = Weekdays::new(0b00000_01);
		weekdays2        ^= Weekdays::new(0b00000_00);
		assert_eq!(weekdays2, Weekdays::new(0b00000_01));
		
		let mut weekdays3 = Weekdays::new(0b00000_00);
		weekdays3        ^= Weekdays::new(0b00000_01);
		assert_eq!(weekdays3, Weekdays::new(0b00000_01));
		
		let mut weekdays4 = Weekdays::new(0b00000_01);
		weekdays4        ^= Weekdays::new(0b00000_01);
		assert_eq!(weekdays4, Weekdays::new(0b00000_00));
		
		let mut weekdays5 = Weekdays::new(0b00000_01);
		weekdays5        ^= Weekdays::new(0b00000_10);
		assert_eq!(weekdays5, Weekdays::new(0b00000_11));
	}
	
	//		Debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", Weekdays::new(0b00000_00)), "Weekdays(000_0000)");
		assert_eq!(format!("{:?}", Weekdays::new(0b00000_01)), "Weekdays(000_0001)");
		assert_eq!(format!("{:?}", Weekdays::new(0b00010_00)), "Weekdays(000_1000)");
		assert_eq!(format!("{:?}", Weekdays::new(0b10000_00)), "Weekdays(100_0000)");
		assert_eq!(format!("{:?}", Weekdays::new(0b11111_11)), "Weekdays(111_1111)");
	}
	
	//		Display																
	#[test]
	fn display() {
		assert_eq!(format!("{}", Weekdays::new(0b00000_00)), "00000_00");
		assert_eq!(format!("{}", Weekdays::new(0b00000_01)), "00000_01");
		assert_eq!(format!("{}", Weekdays::new(0b00010_00)), "00010_00");
		assert_eq!(format!("{}", Weekdays::new(0b10000_00)), "10000_00");
		assert_eq!(format!("{}", Weekdays::new(0b11111_11)), "11111_11");
	}
	
	//		IntoIterator														
	#[test]
	fn into_iterator() {
		let weekdays = Weekdays::new(0b01010_10);
		let mut iter = weekdays.into_iter();
		assert_eq!(iter.next(), Some(Weekdays::TUESDAY));
		assert_eq!(iter.next(), Some(Weekdays::THURSDAY));
		assert_eq!(iter.next(), Some(Weekdays::SATURDAY));
		assert_eq!(iter.next(), None);
	}
	
	//		Not																	
	#[test]
	fn not() {
		assert_eq!(!Weekdays::new(0b00000_00), Weekdays::new(0b11111_11));
		assert_eq!(!Weekdays::new(0b00000_01), Weekdays::new(0b11111_10));
		assert_eq!(!Weekdays::new(0b00010_00), Weekdays::new(0b11101_11));
		assert_eq!(!Weekdays::new(0b10000_00), Weekdays::new(0b01111_11));
		assert_eq!(!Weekdays::new(0b11111_11), Weekdays::new(0b00000_00));
	}
	
	//		SubAssign															
	#[test]
	fn sub_assign() {
		let mut weekdays1 = Weekdays::new(0b00000_00);
		weekdays1        -= Weekdays::new(0b00000_00);
		assert_eq!(weekdays1, Weekdays::new(0b00000_00));
		
		let mut weekdays2 = Weekdays::new(0b00000_01);
		weekdays2        -= Weekdays::new(0b00000_00);
		assert_eq!(weekdays2, Weekdays::new(0b00000_01));
		
		let mut weekdays3 = Weekdays::new(0b00000_00);
		weekdays3        -= Weekdays::new(0b00000_01);
		assert_eq!(weekdays3, Weekdays::new(0b00000_00));
		
		let mut weekdays4 = Weekdays::new(0b00000_01);
		weekdays4        -= Weekdays::new(0b00000_01);
		assert_eq!(weekdays4, Weekdays::new(0b00000_00));
		
		let mut weekdays5 = Weekdays::new(0b00000_11);
		weekdays5        -= Weekdays::new(0b00000_01);
		assert_eq!(weekdays5, Weekdays::new(0b00000_10));
	}
	
	//		Sub																	
	#[test]
	fn sub() {
		assert_eq!(Weekdays::new(0b00000_00) - Weekdays::new(0b00000_00), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_01) - Weekdays::new(0b00000_00), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::new(0b00000_00) - Weekdays::new(0b00000_01), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_01) - Weekdays::new(0b00000_01), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::new(0b00000_11) - Weekdays::new(0b00000_01), Weekdays::new(0b00000_10));
	}
}

mod conversions {
	use super::*;
	
	//		FromSql																
	#[test]
	fn from_sql__valid() {
		assert_eq!(Weekdays::from_sql(&Type::BIT, &[0b00000_00]).unwrap(), Weekdays::new(0b00000_00));
		assert_eq!(Weekdays::from_sql(&Type::BIT, &[0b00000_01]).unwrap(), Weekdays::new(0b00000_01));
		assert_eq!(Weekdays::from_sql(&Type::BIT, &[0b00010_00]).unwrap(), Weekdays::new(0b00010_00));
		assert_eq!(Weekdays::from_sql(&Type::BIT, &[0b10000_00]).unwrap(), Weekdays::new(0b10000_00));
		assert_eq!(Weekdays::from_sql(&Type::BIT, &[0b11111_11]).unwrap(), Weekdays::new(0b11111_11));
	}
	#[test]
	fn from_sql__invalid_type() {
		let err = Weekdays::from_sql(&Type::FLOAT4, &42_i32.to_be_bytes());
		assert_err!(&err);
		assert_eq!(err.unwrap_err().to_string(), "Invalid type for Weekdays: float4");
	}
	#[test]
	fn from_sql__accepts() {
		assert!( <Weekdays as FromSql>::accepts(&Type::BIT));
		assert!(!<Weekdays as FromSql>::accepts(&Type::INT4));
		assert!(!<Weekdays as FromSql>::accepts(&Type::INT8));
		assert!(!<Weekdays as FromSql>::accepts(&Type::TEXT));
		assert!(!<Weekdays as FromSql>::accepts(&Type::FLOAT4));
	}
	
	//		ToSql																
	#[test]
	fn to_sql__valid() {
		let mut bytes = BytesMut::new();
		
		//	Match on IsNull variant
		match Weekdays::new(0b00101_01).to_sql(&Type::BIT, &mut bytes).unwrap() {
			IsNull::No  => (),  //  Expected case
			IsNull::Yes => panic!("Unexpected NULL value"),
		}
		
		//	Convert BytesMut to u8 and verify
		assert_eq!(u8::from_be_bytes(bytes.as_ref().try_into().unwrap()), 0b00101_01);
	}
	#[test]
	fn to_sql__accepts() {
		assert!( <Weekdays as FromSql>::accepts(&Type::BIT));
		assert!(!<Weekdays as FromSql>::accepts(&Type::INT4));
		assert!(!<Weekdays as FromSql>::accepts(&Type::INT8));
		assert!(!<Weekdays as FromSql>::accepts(&Type::TEXT));
		assert!(!<Weekdays as FromSql>::accepts(&Type::FLOAT4));
	}
}


