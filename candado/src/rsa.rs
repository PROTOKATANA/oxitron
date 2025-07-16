use rand :: prelude :: * ;

use sha2 :: Sha512 ; // modificar a Sha3

use faster_hex :: hex_string ;

use faster_hex :: hex_decode ;

use oxicore :: erromacro ;

use rsa :: {

	RsaPrivateKey ,

	RsaPublicKey ,

	Oaep ,

	pkcs1 :: DecodeRsaPublicKey ,

	pkcs1 :: DecodeRsaPrivateKey

};

pub fn cifrado(saje : &str , clave : &str) -> String {

	let mut clave_size = vec![0u8 ; clave.len() / 2] ;

	let _ = match hex_decode(clave.as_bytes() , &mut clave_size) {

		Ok(_) => {

			let parseo = RsaPublicKey :: from_pkcs1_der(clave_size.as_slice()) ;

			match parseo {

				Ok(parseo_desenvuelto) => {

					let mut size = vec![0u8 ; saje.len() / 2] ;

					let _ = match hex_decode(saje.as_bytes() , &mut size) {

						Ok(_) => {

							let mut aleatorio = StdRng :: from_entropy() ;

							let relleno = Oaep :: new :: <Sha512> () ;

							let ofuscado = parseo_desenvuelto.encrypt(

								&mut aleatorio ,

								relleno ,

								&size.as_slice()

							);

							match ofuscado {

								Ok(ofuscado_desenvuelto) => {

									let codex = hex_string(&ofuscado_desenvuelto) ;

									return codex ;

								}

								Err(erro) => {

									erromacro!(erro) ;

									return String :: new() ;

								}

							}

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

			}

		}

		Err(erro) => {

			erromacro!(erro) ;

			return String :: new() ;

		}

	};

}

pub fn descifrado(saje : &str , clave : &str) -> String {

	let mut clave_size = vec![0u8 ; clave.len() / 2] ;

	let _ = match hex_decode(clave.as_bytes() , &mut clave_size) {

		Ok(_) => {

			let _ = match RsaPrivateKey :: from_pkcs1_der(clave_size.as_slice()) {

				Ok(parseo_desenvuelto) => {

					let mut size = vec![0u8 ; saje.len() / 2] ;

					let _ = match hex_decode(saje.as_bytes() , &mut size) {

						Ok(_) => {

							let relleno = Oaep :: new :: <Sha512> () ;

							let plano = parseo_desenvuelto.decrypt(

								relleno ,

								&size.as_slice()

							);

							match plano {

								Ok(plano_desenvuelto) => {

									let codex = hex_string(&plano_desenvuelto) ;

									return codex ;

								}

								Err(erro) => {

									erromacro!(erro) ;

									return String :: new() ;

								}

							}

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
