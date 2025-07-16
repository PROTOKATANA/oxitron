use std :: env ;

use crate :: erromacro ;

pub fn resolutor(segmento : &str) -> String {

	match env :: current_dir() {

		Ok(ruta) => {

			for ancestros in ruta.ancestors() {

				if ancestros.ends_with(segmento) {

					return ancestros.display().to_string() ;

				}

			}

			return String :: new() ;

		}

		Err(erro) => {

			erromacro!(erro) ;

			return String :: new() ;

		}

	}

}
