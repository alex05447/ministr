use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

/// Hashes the string literal `s` to a `u64` using the Rust's [`default hasher`](DefaultHasher) (i.e. one used in the [`HashMap`](std::collections::HashMap)).
pub fn str_hash_default(s: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

/// Hashes the string literal `s` to a `u32` using the FNV1a (32b) hash.
pub fn str_hash_fnv1a(s: &str) -> u32 {
    const FNV1A32_PRIME: u32 = 0x0100_0193;
    const FNV1A32_SEED: u32 = 0x811c_9dc5;

    let mut hash = FNV1A32_SEED;

    for byte in s.as_bytes() {
        hash = (hash ^ *byte as u32).wrapping_mul(FNV1A32_PRIME);
    }

    hash
}

/// Hashes the string literal `s` to a `u64` using the FNV1a (64b) hash.
pub fn str_hash_fnv1a_64(s: &str) -> u64 {
    const FNV1A64_PRIME: u64 = 0x0000_0100_0000_01B3;
    const FNV1A64_SEED: u64 = 0xcbf2_9ce4_8422_2325;

    let mut hash = FNV1A64_SEED;

    for byte in s.as_bytes() {
        hash = (hash ^ *byte as u64).wrapping_mul(FNV1A64_PRIME);
    }

    hash
}
