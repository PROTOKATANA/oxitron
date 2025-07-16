#[derive(Debug, PartialEq, Clone)]

pub enum Lego {

	Title {

		level : u8 ,

		content : Vec<RichText>

	},

	Parrafo(Vec<RichText>) ,

	Lista(Vec<ListItem>) ,

	Quote(Vec<RichText>) ,

	MathBlock(String) ,

	CodeBlock {

		language : String ,

		content : String

	}

}

#[derive(Debug, PartialEq, Clone)]

pub enum RichText {

	Flat(String) ,

	Bold(String) ,

	Italic(String) ,

	Mathline(String) ,

	Callout(String)

}

#[derive(Debug, PartialEq, Clone)]

pub struct ListItem {

	pub depth : u8 ,

	pub ordered : bool ,

	pub content : Vec<RichText> ,

	pub children : Vec<ListItem>

}
