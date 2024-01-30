use ark_bls12_381::{g2::Config, Bls12_381, Fr, G1Affine, G1Projective, G2Affine, G2Projective};
use ark_ec::{
    hashing::{curve_maps::wb::WBMap, map_to_curve_hasher::MapToCurveBasedHasher, HashToCurve},
    pairing::Pairing,
    AffineRepr, CurveGroup,
};
use ark_ff::{field_hashers::DefaultFieldHasher, BigInt, Field};

use ark_serialize::{CanonicalDeserialize, Read};

use prompt::{welcome};

use sha2::Sha256;
use std::fs::File;
use std::io::Cursor;
use std::ops::{Mul, Neg};

use ark_std::{rand::SeedableRng, UniformRand, Zero};

use log::{debug, LevelFilter, warn};

fn derive_point_for_pok(i: usize) -> G2Affine {
    let rng = &mut ark_std::rand::rngs::StdRng::seed_from_u64(20399u64);
    G2Affine::rand(rng).mul(Fr::from(i as u64 + 1)).into()
}

#[allow(dead_code)]
fn pok_prove(sk: Fr, i: usize) -> G2Affine {
    derive_point_for_pok(i).mul(sk).into()
}

fn pok_verify(pk: G1Affine, i: usize, proof: G2Affine) {
    assert!(Bls12_381::multi_pairing(
        &[pk, G1Affine::generator()],
        &[derive_point_for_pok(i).neg(), proof]
    )
        .is_zero());
}

fn hasher() -> MapToCurveBasedHasher<G2Projective, DefaultFieldHasher<Sha256, 128>, WBMap<Config>> {
    let wb_to_curve_hasher =
        MapToCurveBasedHasher::<G2Projective, DefaultFieldHasher<Sha256, 128>, WBMap<Config>>::new(
            &[1, 3, 3, 7],
        )
            .unwrap();
    wb_to_curve_hasher
}

#[allow(dead_code)]
fn bls_sign(sk: Fr, msg: &[u8]) -> G2Affine {
    hasher().hash(msg).unwrap().mul(sk).into_affine()
}

fn bls_verify(pk: G1Affine, sig: G2Affine, msg: &[u8]) {
    assert!(Bls12_381::multi_pairing(
        &[pk, G1Affine::generator()],
        &[hasher().hash(msg).unwrap().neg(), sig]
    )
        .is_zero());
}

fn from_file<T: CanonicalDeserialize>(path: &str) -> T {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    T::deserialize_uncompressed_unchecked(Cursor::new(&buffer)).unwrap()
}

fn main() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();

    welcome();

    let public_keys: Vec<(G1Affine, G2Affine)> = from_file("public_keys.bin");

    public_keys
        .iter()
        .enumerate()
        .for_each(|(i, (pk, proof))| pok_verify(*pk, i, *proof));

    let new_key_index = public_keys.len();
    let message = b"intldds";

    /* Enter solution here */

    // 1. creating rogue key

    let secret = Fr::from(BigInt!("100"));

    let my_key = G1Affine::generator().mul(secret).into_affine();

    let new_key = public_keys
        .iter()
        .fold(G1Projective::from(my_key), |acc, (key, _)| acc + key.neg())
        .into_affine();


    // 2. generating proof of knowledge (PoK)

    let my_proof = pok_prove(secret, new_key_index);

    let new_proof = public_keys
        .iter()
        .enumerate()
        .fold(G2Projective::from(my_proof), |acc, (i, (_, proof))| {
            let rhs = Fr::from(new_key_index as u128 + 1) * Fr::from(i as u128 + 1).inverse().unwrap();
            acc + proof.mul(rhs).neg()
        })
        .into_affine();


    // 3. forge signatures

    let my_sig = bls_sign(secret, message);

    let fake_sig = public_keys
        .iter()
        .fold(G2Projective::from(my_sig), |acc, (_, proof)| acc + proof.neg())
        .into_affine();

    let aggregate_signature = public_keys
        .iter()
        .fold(G2Projective::from(fake_sig), |acc, (_, proof)| acc + proof)
        .into_affine();

    /* End of solution */

    pok_verify(new_key, new_key_index, new_proof);

    debug!("PoK verified");

    let aggregate_key = public_keys
        .iter()
        .fold(G1Projective::from(new_key), |acc, (pk, _)| acc + pk)
        .into_affine();

    debug!("aggregate key created");

    // perform BLS verification and check the result
    if let Err(err) = std::panic::catch_unwind(|| {
        bls_verify(aggregate_key, aggregate_signature, message);
    }) {
        // catch any panic and print an error message
        println!("BLS verification panicked: {:?}", err);
    } else {
        println!("BLS verification successful!");
        warn!("end");
    }

    println!("puzzle completed");
}
