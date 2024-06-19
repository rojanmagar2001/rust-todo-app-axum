pub fn encode_password(password: &str) -> String {
    bcrypt::hash(password, 10).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap()
}
