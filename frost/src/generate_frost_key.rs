use schnorr_fun::binonce::NonceKeyPair;
use schnorr_fun::fun::s;
use schnorr_fun::{
    frost,
    Message,
};
use std::collections::BTreeMap;
use rand_chacha::ChaCha20Rng;
use sha2::Sha256;
use secp256kfun::Point;

fn main() {
    // TODO: Handle parsing other public polys as command line arguments, and convert to a format
    // that can be used with the below code to generate a joint FROST key. Right now the current
    // code just generates random polys instead of using distributed runs of generate_polynomials.rs.
    // Also need to figure out how we want to handle starting transactions and signing.

    // let args: Vec<String> = env::args().collect();

    // if args.len() < 3 {
    //     eprintln!("Usage: {} <my_private_key> <number_of_public_key_pairs> <public_key1> <public_poly1> <public_key2> <public_poly2> ...", args[0]);
    //     std::process::exit(1);
    // }

    // // Parse the number of public keys
    // let num_public_keys: usize = match args[2].parse() {
    //     Ok(num) => num,
    //     Err(_) => {
    //         eprintln!("Invalid number of public keys");
    //         std::process::exit(1);
    //     }
    // };

    // // Check if the correct number of public keys are provided
    // if args.len() != num_public_keys + 3 {
    //     eprintln!("Invalid number of public keys provided");
    //     std::process::exit(1);
    // }

    // // Read public keys
    // let public_keys: Vec<String> = args[3..].to_vec();

    // Replace this portion with reading secret keys
    // Use randomness from ThreadRng to create synthetic nonces -- harder to make a mistake.
    let frost = frost::new_with_synthetic_nonces::<Sha256, rand::rngs::ThreadRng>();
    // We need an RNG for key generation -- don't use ThreadRng in practice see note below.
    let mut rng = rand::thread_rng();
    // we're doing a 2 out of 3
    let threshold = 2;
    // Generate our secret scalar polynomial we'll use in the key generation protocol
    let my_secret_poly = frost::generate_scalar_poly(threshold, &mut rng);
    let my_public_poly = frost::to_point_poly(&my_secret_poly);

    let mut rng2 = rand::thread_rng();
    let mut rng3 = rand::thread_rng();
    let secret_poly2 = frost::generate_scalar_poly(threshold, &mut rng2);
    let public_poly2 = frost::to_point_poly(&secret_poly2);
    let secret_poly3 = frost::generate_scalar_poly(threshold, &mut rng3);
    let public_poly3 = frost::to_point_poly(&secret_poly3);


    // Party indexes can be any non-zero scalar
    let my_index = s!(1).public();
    let party_index2 = s!(2).public();
    let party_index3 = s!(3).public();
    // share our public point poly, and receive the point polys from other participants
    let public_polys = BTreeMap::from_iter([
        (my_index, my_public_poly),
        (party_index2, public_poly2),
        (party_index3, public_poly3),
    ]);

    // Ends here

    let keygen = frost.new_keygen(public_polys).expect("something wrong with what was provided by other parties");

    // Generate secret shares for others and proof-of-possession to protect against rogue key attacks.
    // We need to pass a message to sign for the proof-of-possession. We choose the keygen
    // id here but anything works (you can even use the empty message).
    let keygen_id = frost.keygen_id(&keygen);
    let pop_message = Message::raw(&keygen_id);
    let (mut shares_i_generated, my_pop) = frost.create_shares_and_pop(&keygen, &my_secret_poly, pop_message);

    let (mut shares_generated2, my_pop2) = frost.create_shares_and_pop(&keygen, &secret_poly2, pop_message);
    let (mut shares_generated3, my_pop3) = frost.create_shares_and_pop(&keygen, &secret_poly3, pop_message);

    // Now we send the corresponding shares we generated to the other parties along with our proof-of-possession.
    // Eventually we'll receive shares from the others and combine them to create our secret key share:
    let share_i_generated_for_myself = (shares_i_generated.remove(&my_index).unwrap(), my_pop);
    let share_and_pop_from_2 = (shares_generated2.remove(&party_index2).unwrap(), my_pop2);
    let share_and_pop_from_3 = (shares_generated3.remove(&party_index3).unwrap(), my_pop3);
    let my_shares = BTreeMap::from_iter([
        (my_index, share_i_generated_for_myself),
        (party_index2, share_and_pop_from_2),
        (party_index3, share_and_pop_from_3)
    ]);
    println!("Shares: {:#?}", my_shares);
    // finish keygen by verifying the shares we received, verifying all proofs-of-possession,
    // and calculate our long-lived secret share of the joint FROST key.
    let (my_secret_share, frost_key) = frost
        .finish_keygen(
            keygen,
            my_index,
            my_shares,
            pop_message,
        )
        .expect("something was wrong with the shares we received");
    println!("my_secret_share: {:#?}", my_secret_share);
    println!("frost_key: {:#?}", frost_key);
    let xonly_frost_key = frost_key.into_xonly_key();
    println!("xonly_frost_key: {:#?}", xonly_frost_key);
    /*
    CODE BELOW IS SAMPLE FOR SIGNING (TODO: Align code with above + testing, but general steps should be there)
     */
    // ⚠️ At this point you probably want to check out of band that all the other parties received their
    // secret shares correctly. If they all give the OK then we're ready to use the key and do some signing!
    // let xonly_frost_key = frost_key.into_xonly_key();
    // let message =  Message::plain("my-app", b"chancellor on brink of second bailout for banks");
    // // Generate nonces for this signing session.
    // // ⚠️ session_id MUST be different for every signing attempt to avoid nonce reuse
    // let session_id = b"signing-ominous-message-about-banks-attempt-1".as_slice();
    // let mut nonce_rng: ChaCha20Rng = frost.seed_nonce_rng(&xonly_frost_key, &my_secret_share, session_id);
    // let my_nonce = frost.gen_nonce(&mut nonce_rng);
    // // share your public nonce with the other signing participant(s) receive public nonces
    // let nonces = BTreeMap::from_iter([(my_index, my_nonce.public()), (party_index3, received_nonce3)]);
    // // start a sign session with these nonces for a message
    // let session = frost.start_sign_session(&xonly_frost_key, nonces, message);
    // // create a partial signature using our secret share and secret nonce
    // let my_sig_share = frost.sign(&xonly_frost_key, &session, my_index, &my_secret_share, my_nonce);
    // // receive the partial signature(s) from the other participant(s) and verify
    // assert!(frost.verify_signature_share(&xonly_frost_key, &session, party_index3, sig_share3));
    // // combine signature shares into a single signature that is valid under the FROST key
    // let combined_sig = frost.combine_signature_shares(&xonly_frost_key, &session, vec![my_sig_share, sig_share3]);
    // assert!(frost.schnorr.verify(
    //     &xonly_frost_key.public_key(),
    //     message,
    //     &combined_sig
    // ));
}