use rand :: {

	distributions :: {

		Distribution ,

		Uniform

	}

};

pub fn token(size : usize , cadena : String) -> String {

	let mut rng = rand :: thread_rng() ;

    let distribution = Uniform :: from(0..cadena.len()) ;

	let datatoken : String = (0..size).map(

		|_| cadena

		.chars()

		.nth(distribution.sample(&mut rng))

		.unwrap()

	).collect() ;

	return datatoken ;

}

pub fn token_fixed(size : usize) -> String {

	let mut rng = rand :: thread_rng() ;

	const CADENA : &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#^&*()-_=+[]{}/?<>" ;

    let distribution = Uniform :: from(0..CADENA.len()) ;

	let datatoken : String = (0..size).map(

		|_| CADENA

		.chars()

		.nth(distribution.sample(&mut rng))

		.unwrap()

	).collect() ;

	return datatoken ;

}
