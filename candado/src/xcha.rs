use faster_hex :: hex_string ;

use faster_hex :: hex_decode ;

use oxicore :: erromacro ;

use chacha20poly1305 :: {

	aead :: { Aead , KeyInit } ,

	XChaCha20Poly1305 ,

	XNonce

};

pub fn cifrado(saje : &str , clave : &str , unico : &str) -> String {

	let mut size = vec![0u8 ; saje.len() / 2] ;

	let _ = match hex_decode(saje.as_bytes() , &mut size) {

		Ok(_) => {

			let _ = match XChaCha20Poly1305 :: new_from_slice(clave.as_bytes()) {

				Ok(secreto_desenvuelto) => {

					let nonce = XNonce :: from_slice(&unico.as_bytes()) ;

					let _ = match secreto_desenvuelto.encrypt(&nonce , size.as_slice()) {

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

				Err(erro) => {

					erromacro!(erro) ;

					return String :: new() ;

				}

			};

		}

		Err(erro) => {

			erromacro!(erro) ;

			return String :: new() ;

		}

	};

}

pub fn descifrado(saje : &str , clave : &str , unico : &str) -> String {

	let mut size = vec![0u8 ; saje.len() / 2] ;

	let _ = match hex_decode(saje.as_bytes() , &mut size) {

		Ok(_) => {

			let _ = match XChaCha20Poly1305 :: new_from_slice(clave.as_bytes()) {

				Ok(secreto_desenvuelto) => {

					let nonce = XNonce :: from_slice(&unico.as_bytes()) ;

					let _ = match secreto_desenvuelto.decrypt(&nonce , size.as_slice()) {

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

				Err(erro) => {

					erromacro!(erro) ;

					return String :: new() ;

				}

			};

		}

		Err(erro) => {

			erromacro!(erro) ;

			return String :: new() ;

		}

	};

}
