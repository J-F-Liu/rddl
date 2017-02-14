extern crate rddl;
use rddl::parser::data_item;

extern crate pom;
use pom::DataInput;

fn main() {
	let text = br#"
	Person $chuck
	{
		Name {str "Charles"}
		Friends
		{
			ref {$alice, $bob}
		}
	}
	Person $alice { Name {str "Alice"} }
	Person $bob { Name {str "Bob"} }
	"#;
	let mut input = DataInput::new(text);
	let item = data_item().parse(&mut input);
	println!("{:?}", item);
}
