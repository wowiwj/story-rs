use argon2::{self, Config, ThreadMode, Variant, Version};
use base64::encode;
use rand::{distributions::Alphanumeric, thread_rng, Rng};


pub trait Hasher {
    fn default() -> Self;
    fn hash_password(&self, password: &str) -> String;
    fn password_verify(&self, password_hash: &str, password: &str) -> bool;
}


pub fn rand_str(length: usize) -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}


pub struct PasswordHasher {
    hash_length: u32,
    lanes: u32,
    mem_cost: u32,
    time_cost: u32,
    salt_length: usize,
}

impl Hasher for PasswordHasher {
    fn default() -> Self {
        Self {
            mem_cost: 4096,
            hash_length: 64,
            lanes: 4,
            time_cost: 10,
            salt_length: 12,
        }
    }

    fn hash_password(&self, password: &str) -> String {
        let salt = rand_str(self.salt_length);
        let pwd_byte = password.as_bytes();
        let salt_byte = salt.as_bytes();
        let config = Config {
            ad: &[],
            hash_length: self.hash_length,
            lanes: self.lanes,
            mem_cost: self.mem_cost,
            secret: &[],
            thread_mode: ThreadMode::Parallel,
            time_cost: self.time_cost,
            variant: Variant::Argon2id,
            version: Version::Version13,
        };
        let hash = argon2::hash_encoded(pwd_byte, salt_byte, &config).expect("Password Hash Error");
        let hash = hash.split("$").last().expect("Password Not get Hash Value");
        format!("{}:{}", salt, hash)
    }

    fn password_verify(&self, password_hash: &str, password: &str) -> bool {
        let pwd_byte = password.as_bytes();
        let hash_pwd = password_hash.split(':').collect::<Vec<&str>>();
        let salt = match hash_pwd.get(0) {
            None => return false,
            Some(s) => {
                if s.len() != self.salt_length {
                    return false;
                } else {
                    encode(s).replace("==", "")
                }
            }
        };
        let hash = match hash_pwd.get(1) {
            None => return false,
            Some(s) => s,
        };
        let pwd_hash = format!(
            "${}$v={}$m={},t={},p={}${}${}",
            Variant::Argon2id.as_lowercase_str(),
            Version::Version13.as_u32(),
            self.mem_cost,
            self.time_cost,
            self.lanes,
            salt,
            hash
        );
        argon2::verify_encoded(&pwd_hash, pwd_byte).unwrap_or(false)
    }
}


#[cfg(test)]
mod test {
    use super::{PasswordHasher, Hasher};

    #[test]
    fn test_password() {
        let hasher = PasswordHasher::default();
        let pwd_hash = hasher.hash_password("123456");
        assert_eq!(hasher.password_verify(&pwd_hash, "123456"), true);
    }
}