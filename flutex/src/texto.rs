use nom :: {

	Parser ,

	multi :: many0 ,

	branch :: alt ,

	combinator :: map ,

	sequence :: delimited ,

	error :: Error ,

	bytes :: {

		complete :: tag ,

		complete :: take_until ,

		complete :: take_while1

	}

};

use crate :: lego :: RichText ;

pub fn parse_rich_text<'a>() -> impl Parser<&'a str, Output = Vec<RichText>, Error = Error<&'a str>> {

	many0(alt((

		parse_math_inline() ,

		parse_callout() ,

		parse_bold() ,

		parse_italic() ,

		parse_flatext()

	)))

}

fn parse_math_inline<'a>() -> impl Parser<&'a str, Output = RichText, Error = Error<&'a str>> {

	map(

		delimited(

			tag("Mathline(") ,

			take_until(")") ,

			tag(")")

		),

		|content : &str| RichText :: Mathline(content.trim().to_string())

	)

}

fn parse_callout<'a>() -> impl Parser<&'a str, Output = RichText, Error = Error<&'a str>> {

	map(

		delimited(

			tag("Callout(") ,

			take_until(")") ,

			tag(")")

		),

		|content : &str| RichText :: Callout(content.trim().to_string())

	)

}

fn parse_bold<'a>() -> impl Parser<&'a str, Output = RichText, Error = Error<&'a str>> {

	map(

		delimited(

			tag("Bold(") ,

			take_until(")") ,

			tag(")")

		),

		|content : &str| RichText :: Bold(content.trim().to_string())

	)

}

fn parse_italic<'a>() -> impl Parser<&'a str, Output = RichText, Error = Error<&'a str>> {

	map(

		delimited(

			tag("Italic(") ,

			take_until(")") ,

			tag(")")

		),

		|content : &str| RichText :: Italic(content.trim().to_string())

	)

}

fn parse_flatext<'a>() -> impl Parser<&'a str, Output = RichText, Error = Error<&'a str>> {

	map(

		take_while1(|c : char| !"{}*_".contains(c)) ,

		|content : &str| RichText :: Flat(content.to_string())

	)

}
