
pub enum Mode {
    Sparse,
    Dense,
}

pub struct Hll {
    m: u32,
    p: u8,
    registers: Vec<u8>,
}
