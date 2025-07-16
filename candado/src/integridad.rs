use sha2 :: Sha512 ;

use hmac :: { Hmac , Mac } ;

use faster_hex :: hex_string ;

use faster_hex :: hex_decode ;

use oxicore :: erromacro ;

pub fn integridad(saje : &str , clave : &str) -> String {

	type Mac512 = Hmac <Sha512> ;

	let _ = match Mac512 :: new_from_slice(clave.as_bytes()) {

		Ok(mut mac_desenvuelto) => {

			let mut size = vec![0u8 ; saje.len() / 2] ;

			let _ = hex_decode(saje.as_bytes() , &mut size) ;

			let _ = mac_desenvuelto.update(size.as_slice()) ;

			let finalizacion = mac_desenvuelto.finalize().into_bytes() ;

			let hexa = hex_string(&finalizacion) ;

			return hexa ;

		}

		Err(erro) => {

			erromacro!(erro) ;

			return String :: new() ;

		}

	};

}
