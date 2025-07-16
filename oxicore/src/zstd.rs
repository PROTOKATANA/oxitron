use std :: str ;

use faster_hex :: hex_string ;

use faster_hex :: hex_decode ;

use crate :: erromacro ;

pub fn compresion(texto : &str , nivel : i32) -> String {

	let _ = match zstd :: encode_all(texto.as_bytes() , nivel) {

		Ok(desenvuelto) => {

			let hexa = hex_string(&desenvuelto) ;

			return hexa ;

		}

		Err(erro) => {

			erromacro!(erro) ;

			return String :: new() ;

		}

	};

}

pub fn descompresion(texto : &str) -> String {

	let mut size = vec![0u8 ; texto.len() / 2] ;

	let _ = match hex_decode(texto.as_bytes() , &mut size) {

		Ok(_) => {

			let _ = match zstd :: decode_all(size.as_slice()) {

				Ok(desenvuelto) => {

					let codex = str :: from_utf8(&desenvuelto).unwrap() ;

					return codex.to_string() ;

				},

				Err(erro) => {

					erromacro!(erro) ;

					return String :: new() ;

				}

			};

		} Err(erro) => {

			erromacro!(erro) ;

			return String :: new() ;

		}

	};

}
