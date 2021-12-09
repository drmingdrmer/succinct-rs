#[derive(Clone, Debug)]
pub struct Masks {
    // mask are pre-calculated width-indexed bit masks.
    // E.g. Mask[1] is 63 "0" and 1 "1": 000..01 .
    pub mask: [u64; 65],

    // r_mask are pre-calculated reverse mask of Mask.
    pub r_mask: [u64; 65],

    // mask_upto are mask with bits set upto i-th bit(include i-th bit).
    // E.g. mask_upto[1] == Mask[2] == 000..011 .
    pub mask_upto: [u64; 64],

    // r_mask_upto are reverse of mask_upto.
    pub r_mask_upto: [u64; 64],

    // bit set i-th bit to 1.
    pub bit: [u64; 64],

    // r_bit are reverse of bit.
    pub r_bit: [u64; 64],
}

impl Masks {
    pub fn new() -> Masks {
        let mut m = Masks {
            mask: [0; 65],
            r_mask: [0; 65],
            mask_upto: [0; 64],
            r_mask_upto: [0; 64],
            bit: [0; 64],
            r_bit: [0; 64],
        };

        for i in 0..64 {
            m.mask[i] = (1 << i) - 1;
            m.r_mask[i] = !m.mask[i];
        }

        m.mask[64] = 0xffffffffffffffff;
        m.r_mask[64] = 0;

        for i in 0..63 {
            m.mask_upto[i] = (1 << (i + 1)) - 1;
            m.r_mask_upto[i] = !m.mask_upto[i];
        }

        m.mask_upto[63] = 0xffffffffffffffff;
        m.r_mask_upto[63] = 0;

        for i in 0..64 {
            m.bit[i] = 1 << i;
            m.r_bit[i] = !m.bit[i];
        }

        m
    }
}
