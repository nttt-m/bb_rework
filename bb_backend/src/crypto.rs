use crate::DH;
use openssl::{
    bn::{BigNum, BigNumContext, MsbOption},
    dh::Dh,
    hash::{Hasher, MessageDigest},
    pkey::Params,
};
use rust_embed::RustEmbed;
use std::ops::Deref;
use warp::Buf;

// N, g: group parameters (prime and generator)
// s: salt
// B, b: server's public and private values
// A, a: client's public and private values
// I: user name (aka "identity")
// P: password
// v: verifier
// k: SRP-6 multiplier

// | : String concat
// ^ : exponent
// % : integer remainder

// N, g, s, v = <read from password file>
// b = random()
// k = SHA1(N | PAD(g))
// B = k*v + g^b % N
// A = <read from client>
// u = SHA1(PAD(A) | PAD(B))
// <premaster secret> = (A * v^u) ^ b % N

#[derive(RustEmbed)]
#[folder = "crypto/"]
struct CryptoFiles;

pub fn load_dh() -> Dh<Params> {
    Dh::params_from_pem(&if let Some(file) = CryptoFiles::get("dhparam-4096.pem") {
        file.data.to_vec()
    } else {
        vec![]
    })
    .unwrap() // This can never fail in prod
}

pub fn hash_multiplier() -> BigNum {
    let mut hasher = Hasher::new(MessageDigest::sha3_256()).unwrap();
    hasher.update(&DH.prime_p().to_vec()).unwrap();
    hasher.update(&DH.generator().to_vec()).unwrap();
    let multiplier = hasher.finish().unwrap();
    BigNum::from_u32(multiplier.deref().get_u32()).unwrap()
}

// k*v + g^b % N
pub fn generate_keys(multiplier: BigNum, verifier: Vec<u8>) -> [BigNum; 2] {
    let ctx = &mut BigNumContext::new().unwrap();

    let mut server_private = BigNum::new().unwrap();
    server_private
        .rand(256, MsbOption::MAYBE_ZERO, true)
        .unwrap();

    // k*v
    let mut sp_mul = BigNum::new().unwrap();
    sp_mul
        .checked_mul(
            &multiplier,
            &BigNum::from_slice(verifier.deref()).unwrap(),
            ctx,
        )
        .unwrap();

    // g^b % N
    let mut sp_exp = BigNum::new().unwrap();
    sp_exp
        .mod_exp(DH.generator(), &server_private, DH.prime_p(), ctx)
        .unwrap();

    // sp_mul + sp_exp
    let mut server_public = BigNum::new().unwrap();
    server_public.checked_add(&sp_mul, &sp_exp).unwrap();

    [server_private, server_public]
}

pub fn hash_keys(client_public: &BigNum, server_public: &BigNum) -> BigNum {
    let mut hasher = Hasher::new(MessageDigest::sha3_256()).unwrap();
    hasher.update(&*client_public.to_vec()).unwrap();
    hasher.update(&*server_public.to_vec()).unwrap();
    let intermediate = hasher.finish().unwrap();
    BigNum::from_slice(intermediate.deref()).unwrap()
}

// (A * v^u) ^ b % N
pub fn generate_premaster(
    key_hash: BigNum,
    verifier: BigNum,
    client_public: BigNum,
    server_private: &BigNum,
) -> BigNum {
    let mut block1 = BigNum::new().unwrap();
    let mut block2 = BigNum::new().unwrap();
    let mut premaster = BigNum::new().unwrap();
    let ctx = &mut BigNumContext::new().unwrap();

    // v^u
    block1.exp(&verifier, &key_hash, ctx).unwrap();

    // (A * block1)
    block2.checked_mul(&client_public, &block1, ctx).unwrap();

    // block2 ^ b % N
    premaster
        .mod_exp(&block2, &server_private, DH.prime_p(), ctx)
        .unwrap();

    premaster
}
