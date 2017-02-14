use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum DataItem {
	Primitive {
		name: Option<Name>,
		value: PrimitiveValue,
	},
	Vector {
		name: Option<Name>,
		elems: PrimitiveVector,
	},
	Matrix {
		name: Option<Name>,
		rows: Vec<PrimitiveVector>,
	},
	Custom {
		name: Option<Name>,
		structure: String,
		properties: HashMap<String, PrimitiveValue>,
		items: Vec<DataItem>,
	},
}

impl DataItem {
	pub fn set_name(&mut self, new_name: Option<Name>) {
		match self {
			&mut DataItem::Primitive{ref mut name, ..} => *name = new_name,
			&mut DataItem::Vector{ref mut name, ..} => *name = new_name,
			&mut DataItem::Matrix{ref mut name, ..} => *name = new_name,
			&mut DataItem::Custom{ref mut name, ..} => *name = new_name,
		}
	}
}

pub type Name = (Scope, String);

#[derive(Clone, Debug, PartialEq)]
pub enum Scope {
	Global,
	Local,
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveValue {
	Bool(bool),
	I8(i8),
	I16(i16),
	I32(i32),
	I64(i64),
	U8(u8),
	U16(u16),
	U32(u32),
	U64(u64),
	F32(f32),
	F64(f64),
	Str(String),
	Ref(Name, Vec<String>),
	Type(String),
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveVector {
	Bool(Vec<bool>),
	I8(Vec<i8>),
	I16(Vec<i16>),
	I32(Vec<i32>),
	I64(Vec<i64>),
	U8(Vec<u8>),
	U16(Vec<u16>),
	U32(Vec<u32>),
	U64(Vec<u64>),
	F32(Vec<f32>),
	F64(Vec<f64>),
	Str(Vec<String>),
	Type(Vec<String>),
}
