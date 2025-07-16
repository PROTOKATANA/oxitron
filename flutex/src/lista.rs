use nom :: {

	Parser ,

	multi :: many0 ,

	multi :: many1 ,

	branch :: alt ,

	bytes :: complete :: tag ,

	sequence :: terminated ,

	error :: Error ,

	character :: complete :: {

		char ,

		multispace0 ,

		one_of ,

		space1

	},

	combinator :: {

		map ,

		opt

	}

};

use crate :: {

	texto :: parse_rich_text ,

	lego :: {

		Lego ,

		ListItem

	}

};

pub fn parse_lista<'a>() -> impl Parser<&'a str, Output = Lego, Error = Error<&'a str>> {

	map(

		many1(terminated(parse_list_item(), multispace0)) ,

		|items| Lego :: Lista(nest_list(items))

	)

}

fn parse_list_item<'a>() -> impl Parser<&'a str, Output = ListItem, Error = Error<&'a str>> {

	map(

		(map(many0(tag("    ")), |spaces| spaces.len() as u8) , // cada 4 espacios = 1 nivel

		alt((char('-'), char('*'), char('+'), one_of("0123456789"))) ,

		opt(char('.')) ,

		space1 ,

		parse_rich_text()) ,

		|(depth, prefix, _, _, content)| ListItem {

			depth ,

			ordered: prefix.is_ascii_digit() ,

			content ,

			children: vec![]

		},

	)

}

fn nest_list(flat: Vec<ListItem>) -> Vec<ListItem> {

	let mut stack: Vec<(u8, ListItem)> = vec![];

	let mut root: Vec<ListItem> = vec![];

	for item in flat {

		while let Some((depth, _)) = stack.last() {

			if *depth >= item.depth { stack.pop() ; } else { break ; }

        }

		if let Some((_, parent)) = stack.last_mut() { parent.children.push(item.clone()); }

		else { root.push(item.clone()) ; }

		stack.push((item.depth, item));

	}

	root

}
