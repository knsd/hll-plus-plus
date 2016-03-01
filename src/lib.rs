
use std::collections::{HashSet};


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
}
