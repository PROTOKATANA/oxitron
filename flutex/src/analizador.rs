use nom :: {

	Parser ,

	multi :: many0 ,

	branch :: alt ,

	error :: Error ,

	character :: complete :: {

		digit1 ,

		char ,

		multispace0

	},

	combinator :: {

		map ,

		verify

	},

	sequence :: {

		preceded ,

		delimited ,

		terminated

	},

	bytes :: {

		complete :: tag ,

		complete :: take_until ,

		complete :: take_while1

	}

};

use crate :: {

	lego :: Lego ,

	texto :: parse_rich_text ,

	lista :: parse_lista

};

fn parse_lego<'a>() -> impl Parser<&'a str, Output = Vec<Lego>, Error = Error<&'a str>> {

	many0(alt((

		parse_math_block() ,

		parse_code_block() ,

		parse_quote() ,

		parse_title() ,

		parse_parrafo() ,

		parse_lista()

	)))

}

fn parse_title<'a>() -> impl Parser<&'a str, Output = Lego, Error = Error<&'a str>> {

	map((

		preceded(

			tag("T") ,

			verify(

				digit1 ,

				|s : &str| { s.parse :: <u8>().map_or(false, |n| n >= 1 && n <= 6) }

			),

		),

		delimited(

			tag("(") ,

			parse_rich_text() ,

			tag(")") ,

		)),

		|(level, content)| Lego :: Title {

			level : level.parse().unwrap() ,

			content

		},

	)

}

fn parse_code_block<'a>() -> impl Parser<&'a str, Output = Lego, Error = Error<&'a str>> {

	map((

		tag("Code") ,

		delimited(

			char('(') ,

			take_while1(|c : char| c.is_alphanumeric()) ,

			char(')')

		),

		delimited(

			char('{') ,

			take_until("}") ,

			char('}')

		)),

		|(_, language, content) : (&str, &str, &str)| Lego :: CodeBlock {

			language : language.to_string() ,

			content : content.trim().to_string()

		},

	)

}

fn parse_math_block<'a>() -> impl Parser<&'a str, Output = Lego, Error = Error<&'a str>> {

	map(

		delimited(

			tag("MathBlock(") ,

			take_until(")") ,

			tag(")")

		),

		|content : &str| Lego :: MathBlock(content.trim().to_string())

    )

}

fn parse_quote<'a>() -> impl Parser<&'a str, Output = Lego, Error = Error<&'a str>> {

	map(

		delimited(

			tag("Quote(") ,

			parse_rich_text() ,

			tag(")")

		),

        Lego :: Quote

	)

}

fn parse_parrafo<'a>() -> impl Parser<&'a str, Output = Lego, Error = Error<&'a str>> {

	map(

		terminated(parse_rich_text(), multispace0) ,

		Lego :: Parrafo

    )

}
