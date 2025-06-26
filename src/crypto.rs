use sodiumoxide::crypto::box_;

pub fn crypto_func() {
    let (alice_public_key, alice_private_key) = box_::gen_keypair();
    let (bob_public_key, bob_private_key) = box_::gen_keypair();

    let nonce = box_::gen_nonce();
    let plain_text = b"I am Saptakoshi";
    let cipher_text = box_::seal(plain_text, &nonce, &bob_public_key, &alice_private_key);
    let bob_plain_text =
        box_::open(&cipher_text, &nonce, &alice_public_key, &bob_private_key).unwrap();
    let string = String::from_utf8(bob_plain_text.clone()).unwrap();
    log::info!("Plaintext: {:?}", string);

    assert!(plain_text == &bob_plain_text[..]);
}
