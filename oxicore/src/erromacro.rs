# [macro_export]

macro_rules! erromacro {

	($saje:expr) => {{

		let archivo = file!() ;

		let linea = line!() ;

		let crono = :: chrono :: Local :: now().format("%Y-%m-%d %H:%M:%S").to_string() ;

		eprintln!("🔎 : {}:{}" , archivo , linea) ;

		eprintln!("🪛 : {}" , $saje) ;

		eprintln!("⌛ : {}" , crono) ;

	}};

}
