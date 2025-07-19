use std :: {

	env ,

	path :: Path ,

	path :: PathBuf

};

fn proto(proto : &[String] , salida : &Path , raiz : &Path) {

	let _ = tonic_build :: configure()

	.build_server(true)

	.out_dir(salida)

	.compile(proto , &[raiz])

	.unwrap() ;

}

fn protoxi() {

	let raiz = env :: current_dir().unwrap() ;

	let cuenta : PathBuf = raiz.join("src").join("cuenta") ;

	let inicio = vec![cuenta.join("inicio.proto").to_string_lossy().into_owned()] ;

	let registro = vec![cuenta.join("registro.proto").to_string_lossy().into_owned()] ;

	proto(&inicio , &cuenta , &raiz) ;

	proto(&registro , &cuenta , &raiz) ;

}

fn main() { protoxi() ; }
