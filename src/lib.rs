
use std::collections::{HashSet};
use std::hash::{Hash, Hasher, SipHasher};

pub struct CompressedList {
    count: u32,
    last: u32,
    bytes: Vec<u8>,
}

impl CompressedList {
    fn new(size: usize) -> CompressedList {
        CompressedList {
            count: 0,
            last: 0,
            bytes: vec![0; size]
        }
    }
}

pub enum Hll {
    Sparse {
        m: u32,
        p: u8,
        set: HashSet<u32>,
        list: CompressedList,
    },
    Dense {
        m: u32,
        p: u8,
        registers: Vec<u8>
    },
}

impl Hll {
    fn new(precision: u8) -> Option<Hll> {
        if precision > 18 || precision < 4 {
            return None
        }

        let m = 1 << precision as u32;
        Some(Hll::Sparse {
            m: m,
            p: precision,
            set: HashSet::new(),
            list: CompressedList::new(m as usize)
        })
    }

    fn add<T: Hash>(&mut self, value: &T) {
        let mut hasher = SipHasher::new();
        value.hash(&mut hasher);
        let hash = hasher.finish();
        match self {
            &mut Hll::Sparse {..} => (),
            &mut Hll::Dense {p, ref mut registers, ..} => {
                let idx = eb64(hash, 64, 64 - p) as usize;
                let w = hash << p | 1 << (p - 1);
                let zero_bits = (w.leading_zeros() + 1) as u8;
                if zero_bits > registers[idx] {
                    registers[idx] = zero_bits
                }
            },
        }
    }
}


fn eb64(bits: u64, hi: u8, lo: u8) -> u64 {  // FIXME: something wrong with lo = 0, p = 64
    let diff = hi - lo;
    let m = if diff >= 64 {
        0 - 1
    } else {
        ((1u64 << diff) - 1) << lo

    };
    (bits & m) >> lo
}

#[cfg(test)]
mod tests {

    use super::{eb64};

    #[test]
    fn test_eb64() {
        assert_eq!(eb64(0xffffffffffffffff, 3, 1), 3);
        assert_eq!(eb64(0xffffffffffffffff, 64, 0), 0xffffffffffffffff);
        assert_eq!(eb64(0xffffffffffffffff, 68, 0), 0xffffffffffffffff);
        assert_eq!(eb64(0xffffffffffffffff, 64, 10), 0x3fffffffffffff);
        assert_eq!(eb64(0xf001, 64, 16), 0);
        assert_eq!(eb64(0xf001, 16, 0), 0xf001);
        assert_eq!(eb64(0xf001, 12, 0), 1);
        assert_eq!(eb64(0xf001, 16, 1), 0x7800);
        assert_eq!(eb64(0x1211, 13, 2), 0x484);
        assert_eq!(eb64(0x100000000000, 64, 1), 0x80000000000);
    }
}
