#![crate_name="rust_bloom"]
#![crate_type = "rlib"]
#![allow(dead_code)]
#![allow(unused_imports)]

extern crate xxhash2;
extern crate bit_vec;

use bit_vec::BitVec;

pub struct Bloom {
    bitvec: BitVec,
    k: u32,
    bits: usize,
    hash: xxhash2::State64
}

impl Bloom {
    pub fn new(size: usize, k: u32) -> Bloom {
        assert!(size > 0);

        let bits = size * 8usize;
        let bitvec = BitVec::from_elem(bits as usize, false);
        let k = k;
        let hash = xxhash2::State64::new();;

        Bloom {
            bitvec: bitvec,
            k: k,
            bits: bits,
            hash : hash
        }
    }

    pub fn put(&mut self, item: &[u8]) {
        for i in 0..self.k {
            self.hash.reset(i as u64);
            self.hash.update(item);
            self.bitvec.set(self.hash.finish() as usize % self.bits, true);
        }
    }

    pub fn has(&mut self, item: &[u8]) -> bool {
        for i in 0..self.k {
            self.hash.reset(i as u64);
            self.hash.update(item);

            if self.bitvec.get(self.hash.finish() as usize % self.bits).unwrap() == false {
                return false;
            }
        }

        return true
    }

    pub fn clear(&mut self) {
        self.bitvec.clear()
    }
}

#[test]
fn bloom_test_bloom_size() {
    let bloom = Bloom::new(1024, 2);
    assert!(bloom.bitvec.len() == 8192);
}

#[test]
fn bloom_test_put() {
    let mut bloom = Bloom::new(1024, 2);
    let key: &[u8] = b"foo";

    bloom.put(key);;
}

#[test]
fn bloom_test_has() {
    let mut bloom = Bloom::new(1024, 2);
    let key: &[u8] = b"foo";

    bloom.put(key);
    assert!(bloom.has(key) == true);
    assert!(bloom.has(b"bar") == false);
}