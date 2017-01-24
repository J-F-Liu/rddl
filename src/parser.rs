use pom::{parser, Parser};
use pom::char_class::{alpha, alphanum, digit, hex_digit, oct_digit};
use pom::parser::*;
use std::collections::HashMap;
use std::str::FromStr;
use std::char::{self, REPLACEMENT_CHARACTER};
use super::types::*;

pub fn data_file() -> Parser<u8, Vec<DataItem>> {
	space() * data_item().repeat(1..) - end()
}

fn data_item() -> Parser<u8, DataItem> {
	(primitive_item() | custom_item()) - space()
}

fn primitive_item() -> Parser<u8, DataItem> {
	primitive_type() - space() + name().opt() >> |(type_name, name):(String, Option<Name>)| {
		primitive_data(type_name).map(move |mut item| {
			item.set_name(name.clone());
			item
		})
	}
}

fn primitive_data(type_name: String) -> Parser<u8, DataItem> {
	vector(type_name.clone()).map(move |elems| {
		DataItem::Vector {
			name: None,
			elems: elems,
		}
	}).name("vector value")
	|
	matrix(type_name.clone()).map(move |rows| {
		DataItem::Matrix {
			name: None,
			rows: rows,
		}
	}).name("matrix value")
	|
	primitive_value(type_name).map(move |value| {
		DataItem::Primitive {
			name: None,
			value: value,
		}
	}).name("primitive value")
}

fn vector<'a>(type_name: String) -> parser::Parser<'a, u8, Vec<PrimitiveValue>> {
	sym(b'{') * space() * list(primitive_value(type_name) - space(), sym(b',') * space()) - sym(b'}')
}

fn matrix<'a>(type_name: String) -> parser::Parser<'a, u8, Vec<Vec<PrimitiveValue>>> {
	sym(b'{') * space() * list(vector(type_name) - space(), sym(b',') * space()) - sym(b'}')
}

fn custom_item() -> Parser<u8, DataItem> {
	let property = identifier() - space() - sym(b'=') - space() + property_value() - space();
	let properties = sym(b'(') * space() * list(property, sym(b',') * space()) - sym(b')') - space();
	let properties = properties.map(|properties| properties.into_iter().collect::<HashMap<_, _>>());
	let data_items = sym(b'{') * space() * call(data_item).repeat(0..) - sym(b'}');
	let custom = identifier() - space() + name().opt() + properties.opt() + data_items;
	custom.map(|(((structure, name), properties), items)| {
		DataItem::Custom {
			name: name,
			structure: structure,
			properties: properties.unwrap_or(HashMap::new()),
			items: items,
		}
	}).name("custom item")
}

fn name() -> Parser<u8, Name> {
	(sym(b'$').map(|_| Scope::Global) | sym(b'%').map(|_| Scope::Local)) + identifier() - space()
}

fn identifier() -> Parser<u8, String> {
	let identifier = (is_a(alpha) | sym(b'_')) + (is_a(alphanum) | sym(b'_')).repeat(0..);
	identifier.collect().convert(|bytes| String::from_utf8(bytes))
}

fn primitive_type() -> Parser<u8, String> {
	(seq(b"bool") | seq(b"u8") | seq(b"u16") | seq(b"u32") | seq(b"u64") | seq(b"i8") |
	 seq(b"i16") | seq(b"i32") | seq(b"i64") | seq(b"f32") | seq(b"f64") |
	 seq(b"str") | seq(b"ref") | seq(b"type"))
		.convert(|bytes| String::from_utf8(bytes))
}

fn primitive_value<'a>(type_name: String) -> parser::Parser<'a, u8, PrimitiveValue> {
	if type_name == "bool" {
		seq(b"true").map(|_| PrimitiveValue::Bool(true))
		| seq(b"false").map(|_| PrimitiveValue::Bool(false))
	} else if type_name.starts_with('u') {
		unsigned_integer(type_name)
	} else if type_name.starts_with('i') {
		signed_integer(type_name)
	} else if type_name.starts_with('f') {
		float_number(type_name)
	} else if type_name == "str" {
		string().map(|text| PrimitiveValue::Str(text))
	} else if type_name == "ref" {
		reference()
	} else if type_name == "type" {
		primitive_type().map(|name| PrimitiveValue::Type(name))
	} else {
		unreachable!()
	}
}

fn property_value() -> Parser<u8, PrimitiveValue> {
	( seq(b"true").map(|_| PrimitiveValue::Bool(true))
	| seq(b"false").map(|_| PrimitiveValue::Bool(false))
	| unsigned_integer("u64".to_string())
	| signed_integer("i64".to_string())
	| float_number("f64".to_string())
	| string().map(|text| PrimitiveValue::Str(text))
	| reference()
	| primitive_type().map(|name| PrimitiveValue::Type(name))
	)
}

fn signed_integer<'a>(type_name: String) -> parser::Parser<'a, u8, PrimitiveValue> {
	(sym(b'-').opt() + integer()).convert(move |(neg, (digits, radix))|{
		match type_name.as_str() {
			"i8" => i8::from_str_radix(&digits, radix).map(|num|PrimitiveValue::I8(if neg.is_some() {-1} else {1} * num)),
			"i16" => i16::from_str_radix(&digits, radix).map(|num|PrimitiveValue::I16(if neg.is_some() {-1} else {1} * num)),
			"i32" => i32::from_str_radix(&digits, radix).map(|num|PrimitiveValue::I32(if neg.is_some() {-1} else {1} * num)),
			"i64" => i64::from_str_radix(&digits, radix).map(|num|PrimitiveValue::I64(if neg.is_some() {-1} else {1} * num)),
			_ => unreachable!()
		}
	})
}

fn unsigned_integer<'a>(type_name: String) -> parser::Parser<'a, u8, PrimitiveValue> {
	integer().convert(move |(digits, radix)|{
		match type_name.as_str() {
			"u8" => u8::from_str_radix(&digits, radix).map(|num|PrimitiveValue::U8(num)),
			"u16" => u16::from_str_radix(&digits, radix).map(|num|PrimitiveValue::U16(num)),
			"u32" => u32::from_str_radix(&digits, radix).map(|num|PrimitiveValue::U32(num)),
			"u64" => u64::from_str_radix(&digits, radix).map(|num|PrimitiveValue::U64(num)),
			_ => unreachable!()
		}
	})
}

fn integer() -> Parser<u8, (String, u32)> {
	let hexical = seq(b"0x") * list(is_a(hex_digit).repeat(1..), sym(b'_')).map(|digits|(String::from_utf8(digits.concat()).unwrap(), 16));
	let octal = seq(b"0o") * list(is_a(oct_digit).repeat(1..), sym(b'_')).map(|digits|(String::from_utf8(digits.concat()).unwrap(), 8));
	let binary = seq(b"0b") * list(one_of(b"01").repeat(1..), sym(b'_')).map(|digits|(String::from_utf8(digits.concat()).unwrap(), 2));
	let decimal = list(is_a(digit).repeat(1..), sym(b'_')).map(|digits|(String::from_utf8(digits.concat()).unwrap(), 10));
	hexical | octal | binary | decimal
}

fn float_number<'a>(type_name: String) -> parser::Parser<'a, u8, PrimitiveValue> {
	let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
	let frac = sym(b'.') + one_of(b"0123456789").repeat(1..);
	let exp = one_of(b"eE") + one_of(b"+-").opt() + one_of(b"0123456789").repeat(1..);
	let number = sym(b'-').opt() + integer + frac.opt() + exp.opt();
	number.collect().convert(|v|String::from_utf8(v)).convert(move |digits|{
		match type_name.as_str() {
			"f32" => f32::from_str(&digits).map(|num|PrimitiveValue::F32(num)),
			"f64" => f64::from_str(&digits).map(|num|PrimitiveValue::F64(num)),
			_ => unreachable!()
		}
	})
}

fn hex_digits_to_char(digits: Vec<u8>) -> char {
	let text = String::from_utf8(digits).unwrap();
	let code = u32::from_str_radix(&text, 16).unwrap();
	char::from_u32(code).unwrap_or(REPLACEMENT_CHARACTER)
}

fn string() -> Parser<u8, String> {
	let special_char = sym(b'\\') | sym(b'"') | sym(b'0').map(|_|b'\0')
		| sym(b'n').map(|_|b'\n') | sym(b'r').map(|_|b'\r') | sym(b't').map(|_|b'\t');
	let escape_sequence = sym(b'\\') * special_char;
	let utf8_string = (none_of(b"\\\"") | escape_sequence).repeat(1..).convert(|bytes|String::from_utf8(bytes));
	let hex_char = seq(b"\\x") * is_a(hex_digit).repeat(2..3).map(hex_digits_to_char);
	let unicode_char = seq(b"\\u{") * is_a(hex_digit).repeat(1..7).map(hex_digits_to_char) - sym(b'}');
	let unicode_string = (hex_char | unicode_char).repeat(1..).map(|chars|chars.into_iter().collect::<String>());
	let string = sym(b'"') * (utf8_string | unicode_string).repeat(0..) - sym(b'"');
	string.map(|strings|strings.concat())
}

fn reference() -> Parser<u8, PrimitiveValue> {
	let path = name() + (sym(b'%') * identifier() - space()).repeat(0..);
	path.map(|(name, items)| PrimitiveValue::Ref(name, items))
}

fn space() -> Parser<u8, ()> {
	one_of(b" \t\r\n").repeat(0..).discard()
}

#[cfg(test)]
mod tests {
	use super::*;
	use pom::DataInput;

	#[test]
	fn parser_works() {
		let mut input = DataInput::new(b"u32 $num {
			1094861636,
			0x41424344,
			0o10120441504,
			0b0100_0001_0100_0010_0100_0011_0100_0100,
			23_889
		}");
		assert_eq!(primitive_item().parse(&mut input), Ok(DataItem::Vector {
			name: Some((Scope::Global, "num".to_string())),
			elems: vec![
				PrimitiveValue::U32(1094861636),
				PrimitiveValue::U32(0x41424344),
				PrimitiveValue::U32(0o10120441504),
				PrimitiveValue::U32(0b0100_0001_0100_0010_0100_0011_0100_0100),
				PrimitiveValue::U32(23_889),
			]
		}));
	}
}
