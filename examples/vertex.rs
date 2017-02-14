extern crate rddl;
use rddl::parser::data_item;

extern crate pom;
use pom::DataInput;

fn main() {
	let text = br"VertexArray
	{
		f32
		{
			{1.0, 2.0, 3.0},
			{0.5, 0.0, 0.5},
			{0.0, -1.0, 4.0}
		}
	}";
	let mut input = DataInput::new(text);
	let item = data_item().parse(&mut input);
	println!("{:?}", item);
}
