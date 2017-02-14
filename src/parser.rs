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
	vector(type_name.clone()).map(move |vector| {
		DataItem::Vector {
			name: None,
			vector: vector,
		}
	}).name("vector value")
	|
	matrix(type_name.clone()).map(move |matrix| {
		DataItem::Matrix {
			name: None,
			matrix: matrix,
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

fn vector<'a>(type_name: String) -> parser::Parser<'a, u8, PrimitiveVector> {
	let values = match type_name.as_str() {
		"bool" => list(parse_bool() - space(), sym(b',') * space()).map(|vals| PrimitiveVector::Bool(vals)),
		"i8" => list(parse_i8() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::I8(nums)),
		"i16" => list(parse_i16() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::I16(nums)),
		"i32" => list(parse_i32() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::I32(nums)),
		"i64" => list(parse_i64() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::I64(nums)),
		"u8" => list(parse_u8() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::U8(nums)),
		"u16" => list(parse_u16() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::U16(nums)),
		"u32" => list(parse_u32() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::U32(nums)),
		"u64" => list(parse_u64() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::U64(nums)),
		"f32" => list(parse_f32() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::F32(nums)),
		"f64" => list(parse_f64() - space(), sym(b',') * space()).map(|nums|PrimitiveVector::F64(nums)),
		"str" => list(string() - space(), sym(b',') * space()).map(|texts| PrimitiveVector::Str(texts)),
		"type" => list(primitive_type() - space(), sym(b',') * space()).map(|names| PrimitiveVector::Type(names)),
		_ => unreachable!()
	};
	sym(b'{') * space() * values - sym(b'}')
}

fn matrix<'a>(type_name: String) -> parser::Parser<'a, u8, PrimitiveMatrix> {
	let matrix = match type_name.as_str() {
		"bool" => list(sym(b'{') * space() * list(parse_bool() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|vals| PrimitiveMatrix::Bool(vals)),
		"i8" => list(sym(b'{') * space() * list(parse_i8() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::I8(nums)),
		"i16" => list(sym(b'{') * space() * list(parse_i16() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::I16(nums)),
		"i32" => list(sym(b'{') * space() * list(parse_i32() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::I32(nums)),
		"i64" => list(sym(b'{') * space() * list(parse_i64() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::I64(nums)),
		"u8" => list(sym(b'{') * space() * list(parse_u8() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::U8(nums)),
		"u16" => list(sym(b'{') * space() * list(parse_u16() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::U16(nums)),
		"u32" => list(sym(b'{') * space() * list(parse_u32() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::U32(nums)),
		"u64" => list(sym(b'{') * space() * list(parse_u64() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::U64(nums)),
		"f32" => list(sym(b'{') * space() * list(parse_f32() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::F32(nums)),
		"f64" => list(sym(b'{') * space() * list(parse_f64() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|nums|PrimitiveMatrix::F64(nums)),
		"str" => list(sym(b'{') * space() * list(string() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|texts| PrimitiveMatrix::Str(texts)),
		"type" => list(sym(b'{') * space() * list(primitive_type() - space(), sym(b',') * space()) - sym(b'}'), sym(b',') * space()).map(|names| PrimitiveMatrix::Type(names)),
		_ => unreachable!()
	};
	sym(b'{') * space() * matrix - sym(b'}')
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
	match type_name.as_str() {
		"bool" => parse_bool().map(|val| PrimitiveValue::Bool(val)),
		"i8" => parse_i8().map(|num|PrimitiveValue::I8(num)),
		"i16" => parse_i16().map(|num|PrimitiveValue::I16(num)),
		"i32" => parse_i32().map(|num|PrimitiveValue::I32(num)),
		"i64" => parse_i64().map(|num|PrimitiveValue::I64(num)),
		"u8" => parse_u8().map(|num|PrimitiveValue::U8(num)),
		"u16" => parse_u16().map(|num|PrimitiveValue::U16(num)),
		"u32" => parse_u32().map(|num|PrimitiveValue::U32(num)),
		"u64" => parse_u64().map(|num|PrimitiveValue::U64(num)),
		"f32" => parse_f32().map(|num|PrimitiveValue::F32(num)),
		"f64" => parse_f64().map(|num|PrimitiveValue::F64(num)),
		"str" => string().map(|text| PrimitiveValue::Str(text)),
		"ref" => reference(),
		"type" => primitive_type().map(|name| PrimitiveValue::Type(name)),
		_ => unreachable!()
	}
}

fn property_value() -> Parser<u8, PrimitiveValue> {
	( seq(b"true").map(|_| PrimitiveValue::Bool(true))
	| seq(b"false").map(|_| PrimitiveValue::Bool(false))
	| parse_u64().map(|num|PrimitiveValue::U64(num))
	| parse_i64().map(|num|PrimitiveValue::I64(num))
	| parse_f64().map(|num|PrimitiveValue::F64(num))
	| string().map(|text| PrimitiveValue::Str(text))
	| reference()
	| primitive_type().map(|name| PrimitiveValue::Type(name))
	)
}

fn parse_bool() -> Parser<u8, bool> {
	seq(b"true").map(|_| true)
	| seq(b"false").map(|_| false)
}

fn integer() -> Parser<u8, (String, u32)> {
	let hexical = seq(b"0x") * list(is_a(hex_digit).repeat(1..), sym(b'_')).map(|digits|(String::from_utf8(digits.concat()).unwrap(), 16));
	let octal = seq(b"0o") * list(is_a(oct_digit).repeat(1..), sym(b'_')).map(|digits|(String::from_utf8(digits.concat()).unwrap(), 8));
	let binary = seq(b"0b") * list(one_of(b"01").repeat(1..), sym(b'_')).map(|digits|(String::from_utf8(digits.concat()).unwrap(), 2));
	let decimal = list(is_a(digit).repeat(1..), sym(b'_')).map(|digits|(String::from_utf8(digits.concat()).unwrap(), 10));
	hexical | octal | binary | decimal
}

fn float_number<'a>() -> parser::Parser<'a, u8, String> {
	let integer = one_of(b"123456789") - one_of(b"0123456789").repeat(0..) | sym(b'0');
	let frac = sym(b'.') + one_of(b"0123456789").repeat(1..);
	let exp = one_of(b"eE") + one_of(b"+-").opt() + one_of(b"0123456789").repeat(1..);
	let number = sym(b'-').opt() + integer + frac.opt() + exp.opt();
	number.collect().convert(|v|String::from_utf8(v))
}

fn parse_u8() -> Parser<u8, u8> {
	integer().convert(move |(digits, radix)|u8::from_str_radix(&digits, radix))
}

fn parse_u16() -> Parser<u8, u16> {
	integer().convert(move |(digits, radix)|u16::from_str_radix(&digits, radix))
}

fn parse_u32() -> Parser<u8, u32> {
	integer().convert(move |(digits, radix)|u32::from_str_radix(&digits, radix))
}

fn parse_u64() -> Parser<u8, u64> {
	integer().convert(move |(digits, radix)|u64::from_str_radix(&digits, radix))
}

fn parse_i8() -> Parser<u8, i8> {
	(sym(b'-').opt() + integer()).convert(move |(neg, (digits, radix))|i8::from_str_radix(&digits, radix).map(|num|if neg.is_some() {-1} else {1} * num))
}

fn parse_i16() -> Parser<u8, i16> {
	(sym(b'-').opt() + integer()).convert(move |(neg, (digits, radix))|i16::from_str_radix(&digits, radix).map(|num|if neg.is_some() {-1} else {1} * num))
}

fn parse_i32() -> Parser<u8, i32> {
	(sym(b'-').opt() + integer()).convert(move |(neg, (digits, radix))|i32::from_str_radix(&digits, radix).map(|num|if neg.is_some() {-1} else {1} * num))
}

fn parse_i64() -> Parser<u8, i64> {
	(sym(b'-').opt() + integer()).convert(move |(neg, (digits, radix))|i64::from_str_radix(&digits, radix).map(|num|if neg.is_some() {-1} else {1} * num))
}

fn parse_f32() -> Parser<u8, f32> {
	float_number().convert(move |digits|f32::from_str(&digits))
}

fn parse_f64() -> Parser<u8, f64> {
	float_number().convert(move |digits|f64::from_str(&digits))
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
			vector: PrimitiveVector::U32(vec![
				1094861636,
				0x41424344,
				0o10120441504,
				0b0100_0001_0100_0010_0100_0011_0100_0100,
				23_889,
			])
		}));
	}
}
