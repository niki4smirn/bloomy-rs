#![feature(generic_const_exprs)]
mod bitarray;

use ahash::AHasher;
use std::hash::Hash;
use std::hash::Hasher;

struct HashesIter {
    h1: u64,
    h2: u64,
    mod_mask: u64,
    cur_iter: usize,
    iters: usize,
}

impl HashesIter {
    fn new<T: Hash>(data: &T, iters: usize, mod_mask: u64) -> Self {
        let (mut h1, mut h2) = Self::hash(data);
        h1 &= mod_mask;
        h2 &= mod_mask;

        Self {
            h1,
            h2,
            mod_mask,
            cur_iter: 0,
            iters,
        }
    }

    fn hash<T: Hash>(data: &T) -> (u64, u64) {
        let mut hasher = AHasher::default();
        data.hash(&mut hasher);
        let hash1 = hasher.finish();
        hash1.hash(&mut hasher);
        let hash2 = hasher.finish();
        (hash1, hash2)
    }
}

impl std::iter::Iterator for HashesIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur_iter == self.iters {
            return None;
        }
        let h2i = ((self.cur_iter as u64 & self.mod_mask) * self.h2) & self.mod_mask;
        self.cur_iter += 1;
        Some((self.h1 + h2i) & self.mod_mask)
    }
}

pub struct BloomFilter<const N: usize>
where
    [u8; N / 8]: Sized,
{
    bitarray: bitarray::BitArray<N>,
    hash_functions_num: usize,
}

impl<const N: usize> BloomFilter<N>
where
    [u8; N / 8]: Sized,
{
    const N_BOUND_MASK: u64 = N as u64 - 1;
    pub fn new(expected_inserts_number: usize) -> Self {
        const LN_2: f64 = std::f64::consts::LN_2;
        let hash_functions_num =
            ((N as f64 / expected_inserts_number as f64) * LN_2).ceil() as usize;
        Self {
            bitarray: bitarray::BitArray::new(),
            hash_functions_num,
        }
    }

    pub fn insert<T: Hash>(&mut self, data: &T) {
        let hashes_iter = HashesIter::new(data, self.hash_functions_num, Self::N_BOUND_MASK);
        for hash in hashes_iter {
            self.bitarray.set(hash as usize);
        }
    }

    pub fn contains<T: Hash>(&self, data: &T) -> bool {
        let hashes_iter = HashesIter::new(data, self.hash_functions_num, Self::N_BOUND_MASK);
        for hash in hashes_iter {
            if !self.bitarray.get(hash as usize) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;
    use std::collections::HashSet;

    // e^(-filter_sz/inserts_num * (ln2)^2)
    fn false_pos_prob(filter_sz: usize, inserts_num: usize) -> f64 {
        let e = std::f64::consts::E;
        let ln2sq = std::f64::consts::LN_2.powi(2);
        let f_sz = filter_sz as f64;
        let ins_cnt = inserts_num as f64;
        let power = -(f_sz / ins_cnt as f64 * ln2sq);
        e.powf(power)
    }

    #[test]
    fn stress_test() {
        const INSERTS_COUNT: usize = 1000;
        const READS_COUNT: usize = 100000;
        const FILTER_SIZE: usize = 4096;
        let mut rng = rand::thread_rng();
        let mut bf = BloomFilter::<FILTER_SIZE>::new(INSERTS_COUNT);
        println!(
            "False positive probability: {}",
            false_pos_prob(FILTER_SIZE, INSERTS_COUNT)
        );
        let mut inserted = HashSet::new();
        for _ in 0..INSERTS_COUNT {
            let data = rng.gen::<u64>();
            bf.insert(&data);
            inserted.insert(data);
            assert!(bf.contains(&data));
        }
        let mut false_positives = 0;
        for _ in 0..READS_COUNT {
            let data = rng.gen::<u64>();
            if !inserted.contains(&data) && bf.contains(&data) {
                false_positives += 1;
            }
            if !bf.contains(&data) {
                assert!(!inserted.contains(&data));
            }
        }
        println!(
            "False positives: {} out of {} reads ({}%)",
            false_positives,
            READS_COUNT,
            false_positives as f64 / READS_COUNT as f64 * 100.0
        );
    }
}
