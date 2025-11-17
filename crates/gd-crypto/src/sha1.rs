use sha1::{Digest, Sha1};

#[macro_export]
macro_rules! hex_digest {
    ($item: expr) => {
        format!("{:x}", $item)
    };
}

pub fn hash<D: AsRef<[u8]>>(data: D) -> String {
    let digest = Sha1::digest(data);

    hex_digest!(digest)
}

pub fn hash_with_salt<D: AsRef<[u8]>, S: AsRef<[u8]>>(data: D, salt: S) -> String {
    let mut hasher = Sha1::new();

    hasher.update(data);
    hasher.update(salt);

    let digest = hasher.finalize();

    hex_digest!(digest)
}
