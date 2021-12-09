use std::fmt::Debug;

/// Define a convenient way to get a mask of 0xff in variant type
pub trait Mask8 {
    const MASK_8: Self;
}

impl Mask8 for u8 {
    const MASK_8: Self = 0xff;
}
impl Mask8 for u16 {
    const MASK_8: Self = 0xff;
}
impl Mask8 for u32 {
    const MASK_8: Self = 0xff;
}
impl Mask8 for u64 {
    const MASK_8: Self = 0xff;
}

#[allow(dead_code)]
pub fn format_bits<T>(word: T) -> String
where
    T: Copy + std::ops::Shr<usize, Output = T> + std::ops::BitAnd<Output = T> + TryInto<u8>,
    T: Mask8,
    <T as TryInto<u8>>::Error: Debug,
{
    let size = std::mem::size_of::<T>();

    let mut res = vec![];
    for i in 0..size {
        let x: u8 = ((word >> (i * 8)) & T::MASK_8)
            .try_into()
            .expect("must be error");
        let s = format!("{:08b}", x.reverse_bits());
        res.push(s);
    }

    res.join(" ")
}

#[allow(dead_code)]
pub fn fmt_bitmap(words: &[u64]) -> String {
    let mut v = Vec::with_capacity(words.len());
    for w in words {
        let s = format_bits(*w);
        v.push(s);
    }

    v.join(",")
}
