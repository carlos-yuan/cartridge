use std::panic;
use base64::Engine;
use gmsm::sm2::{Keypair, sm2_decrypt_c1c3c2, sm2_encrypt_c1c3c2, sm2_generate_key_hex};
use gmsm::sm3::sm3_hex;
use gmsm::sm4::{sm4_ecb_decrypt_hex, sm4_ecb_encrypt_hex};

pub fn sm2_keypair() ->Result<Keypair, String>{
    panic::catch_unwind(||{
        sm2_generate_key_hex()
    }).map_err(|err|"keypair generate panic".to_string())
}

pub fn sm2_encode(pk:String, msg:String) ->Result<String, String>{
    panic::catch_unwind(||{
        sm2_encrypt_c1c3c2(&msg, &pk)
    }).map_err(|err|"encode panic".to_string())
}

pub fn sm2_decode(sk:String, msg:String) ->Result<String, String>{
    panic::catch_unwind(||{
        sm2_decrypt_c1c3c2(&msg, &sk)
    }).map_err(|err|"decode panic".to_string())
}

pub fn sm4_ecb_encode<'a>(key:&'a str, msg:&'a str) ->Result<String, String>{
    panic::catch_unwind(||{
        sm4_ecb_encrypt_hex(&msg, &key)
    }).map_err(|err|"encode panic".to_string())
}

pub fn sm4_ecb_decode<'a>(key: &'a str, cipher: &'a str) ->Result<String, String>{
    panic::catch_unwind(||{
        sm4_ecb_decrypt_hex(cipher, key)
    }).map_err(|err|"decode panic".to_string())
}

pub fn sm3(val:String)->String{
    sm3_hex(&val)
}

#[cfg(test)]
mod tests {
    use crate::sm::{sm2_decode, sm2_encode, sm2_keypair};

    #[test]
    fn test_sm3() {
        let kp=sm2_keypair().unwrap();
        let txt="123";
        println!("pri---{}",kp.pri_hex);
        println!("pub---{}",kp.pub_hex);
        let c=sm2_encode(kp.pub_hex,txt.to_string()).unwrap();
        let plain=sm2_decode(kp.pri_hex,c).unwrap();
        assert_eq!(txt, plain);
    }
}