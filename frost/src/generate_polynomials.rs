use schnorr_fun::{frost, fun::{ Scalar, nonce, Tag, derive_nonce_rng }};
use std::env;
use sha2::Sha256;
use secp256kfun::Point;
use std::fs::OpenOptions;
use std::io::Write;
use hex;
// use arrayref::{array_ref, array_mut_ref};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <session_id> <threshold>", args[0].as_str());
        std::process::exit(1);
    }

    let session_id = &args[1];
    let threshold = args[2].parse::<usize>().expect("Invalid threshold format");

    // Generates a random secret scalar which it uses to generate your secret polynomial
    let secret_scalar = Scalar::random(&mut rand::thread_rng());

    let mut file = OpenOptions::new()
        .append(true)
        .open("output.txt")
        .expect("cannot open file");
    file.write(b"Your Secret Scalar (SAVE THIS, DO NOT SHARE):\n");
    file.write(hex::encode(secret_scalar.to_bytes()).as_bytes());
    file.write(b"\n");
    
    let nonce_gen = nonce::Deterministic::<Sha256>::default().tag(b"quarry-frost-keygen");
    let mut poly_rng = derive_nonce_rng!(
        nonce_gen => nonce_gen,
        secret => &secret_scalar,
        public => [session_id.as_str()],
        seedable_rng => rand::rngs::StdRng
    );

    // we can always reproduce my_secret_poly knowing the secret_key, threshold, and session id
    let my_secret_poly: Vec<Scalar> = frost::generate_scalar_poly(threshold, &mut poly_rng);
    let my_public_poly = frost::to_point_poly(&my_secret_poly);

    file.write(b"Public Poly (Share this with other FROST participants):\n");
    for point in my_public_poly {
        file.write(hex::encode(point.to_bytes()).as_bytes());
        file.write(b"\n");
    }

    // Sample code to turn public poly byte array back into point
    // let x = [3, 223, 223, 132, 59, 156, 232, 59, 106, 72, 38, 143, 164, 105, 1, 58, 17, 77, 2, 63, 158, 119, 162, 240, 106, 85, 173, 238, 99, 18, 131, 201, 127];
    // let point: Point<Normal, Public, NonZero> = Point::from_bytes(x).unwrap();
    // println!("Point: {:?}", point);
    // If it were hex strings, we can do:
    // let hex_string = "02ad8be805ce506245f2b1ac874763e34c9cc751f34f12bb04f72b69aae5b2c192";
    // let bytes = hex::decode(hex_string).unwrap();
    // let array_of_size_33: [u8; 33] = *array_ref![bytes, 0, 33];
}