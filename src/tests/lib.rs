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
}

mod derived_traits {
	use super::*;
	
	//		Debug																
	#[test]
	fn debug() {
		assert_eq!(format!("{:?}", Weekdays::new(0b00000_00)), "Weekdays(0)");
		assert_eq!(format!("{:?}", Weekdays::new(0b00000_01)), "Weekdays(1)");
		assert_eq!(format!("{:?}", Weekdays::new(0b00010_00)), "Weekdays(8)");
		assert_eq!(format!("{:?}", Weekdays::new(0b10000_00)), "Weekdays(64)");
		assert_eq!(format!("{:?}", Weekdays::new(0b11111_11)), "Weekdays(127)");
	}
	
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
	
	//		Display																
	#[test]
	fn display() {
		assert_eq!(format!("{}", Weekdays::new(0b00000_00)), "0000000");
		assert_eq!(format!("{}", Weekdays::new(0b00000_01)), "0000001");
		assert_eq!(format!("{}", Weekdays::new(0b00010_00)), "0001000");
		assert_eq!(format!("{}", Weekdays::new(0b10000_00)), "1000000");
		assert_eq!(format!("{}", Weekdays::new(0b11111_11)), "1111111");
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


