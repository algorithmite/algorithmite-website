use argon2::{Config, ThreadMode};

const DEFAULT_SALT: &[u8] = b"fDzMWxV9RYwZ60ZzG0b4AejOEho/mVeFmnzwswpmUnEw";

pub fn encode_password(input_password: String) -> String {
    let mut config = Config::default();
    config.lanes = 4;
    config.thread_mode = ThreadMode::Parallel;
    argon2::hash_encoded(input_password.as_bytes(), DEFAULT_SALT, &config).unwrap()
}

pub fn test_hash(current_hash: String, test_password: String) -> bool {
    argon2::verify_encoded(&current_hash, test_password.as_bytes()).unwrap_or(false)
}
