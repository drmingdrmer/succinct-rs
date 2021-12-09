use std::fmt::Debug;

use crate::bitmap::context::Context;
use crate::bitmap::index::BuildIndex;
use crate::bitmap::index::RankIndex;
use crate::bitmap::index::SelectRankIndex;
use crate::bitmap::rank::RankIndex64;
use crate::static_kv::CTX;

// SelectLookup8 is a lookup table for "select" on 8-bit bitmap:
// It stores the result of select(b, ith) in
// select8Lookup[b*256+ith].
pub struct SelectLookup8 {
    pub lookup: [u8; 256 * 8],
}

impl SelectLookup8 {
    pub fn new() -> Self {
        let mut s = SelectLookup8 {
            lookup: [0; 256 * 8],
        };

        for i in 0..256usize {
            let mut word = i as u8;

            for j in 0..8 {
                let least_zeros = word.trailing_zeros();

                s.lookup[i * 8 + j] = least_zeros as u8;

                if word > 0 {
                    // remove that least significant 1
                    word = word & (word - 1);
                }
            }
        }

        s
    }
}

/// Index data to speed `select()`.
///
/// select(i) returns the position of the i-th "1".
/// E.g.:
///     bitmap = 100100..
///     select(bitmap, 0) = 0
///     select(bitmap, 1) = 3
///
/// It stores the value of select(i*32) for every i.
#[derive(Clone)]
pub struct SelectIndex32<RI>
where RI: RankIndex + Debug + Clone
{
    pub index: Vec<i32>,
    pub rank_index: RI,

    pub ctx: &'static Context,
}

impl<RI> BuildIndex for SelectIndex32<RI>
where RI: RankIndex + Debug + Clone
{
    fn build(words: &[u64]) -> Self {
        let index = build_select32_index(words);
        SelectIndex32 {
            index,
            rank_index: RI::build(words),
            ctx: &*CTX,
        }
    }
}

/// SelectIndex32 depends on a rank index.
impl RankIndex for SelectIndex32<RankIndex64> {
    fn count_ones(&self, words: &[u64], i: i32) -> (i32, i32) {
        self.rank_index.count_ones(words, i)
    }

    fn get_rank_index(&self) -> &[i32] {
        &self.rank_index.get_rank_index()
    }
}

impl SelectRankIndex for SelectIndex32<RankIndex64> {
    fn select_ith_one(&self, words: &[u64], i: i32) -> i32 {
        select_s32_r64(
            words,
            &self.get_select_index(),
            &self.rank_index.get_rank_index(),
            self.ctx,
            i,
        )
    }

    fn get_select_index(&self) -> &[i32] {
        &self.index
    }
}

/// Build a index to speed up select(i).
///
/// select(i) returns the position of the i-th "1".
/// E.g.:
///     bitmap = 100100..
///     select(bitmap, 0) = 1
///     select(bitmap, 1) = 3
///
/// It returns an index of Vec<i32>.
/// An element in it is the value of select(i*32)
#[allow(dead_code)]
pub fn build_select32_index(words: &[u64]) -> Vec<i32> {
    let bits_count = words.len() << 6;

    let mut select_index = Vec::with_capacity(words.len());

    let mut ith_bit = -1;

    for i in 0..bits_count {
        if words[i >> 6] & (1 << (i & 63)) != 0 {
            ith_bit += 1;
            if ith_bit & 31 == 0 {
                select_index.push(i as i32);
            }
        }
    }

    select_index.shrink_to_fit();
    select_index
}

/// Select32R64 returns the indexes of the i-th "1".
/// It requires a rank64 index for speeding up and a select32 index
#[allow(dead_code)]
pub fn select_s32_r64(
    words: &[u64],
    select_index: &[i32],
    rank_index: &[i32],
    context: &Context,
    i: i32,
) -> i32 {
    // find the word that contains i/32-th `1`.
    let mut word_i: usize = (select_index[(i >> 5) as usize] >> 6) as usize;

    // find the word that contains i-th `1`.
    while rank_index[(word_i + 1) as usize] <= i {
        word_i += 1;
    }

    let w = words[word_i];
    let mut ww = w;

    let base = (word_i << 6) as i32;

    // To find the `find_ith` `1` in words[word_i],
    // with a expanded binary search.

    let mut find_ith: u32 = (i - rank_index[word_i]) as u32;

    let mut offset = 0;

    // count of `1` in the least significant 32 bits.
    let ones = (ww as u32).count_ones();
    if ones <= find_ith {
        find_ith -= ones;
        offset += 32;
        ww >>= 32;
    }

    // count of `1` in the [32, 32+16] bits.
    let ones = (ww as u16).count_ones();
    if ones <= find_ith {
        find_ith -= ones;
        offset |= 16;
        ww >>= 16;
    }

    let ones = (ww as u8).count_ones();

    if ones <= find_ith {
        // The `1` to find is in the second 8 bits.

        let x = (((ww as usize) >> 5) & 0x7f8) | ((find_ith - ones) as usize);
        base + context.select_lookup_8.lookup[x] as i32 + offset + 8
    } else {
        // The `1` to find is in the first 8 bits.

        let x = ((ww as usize) & 0xff) << 3 | (find_ith as usize);
        base + context.select_lookup_8.lookup[x] as i32 + offset
    }
}

/// Select32R64 returns the indexes of the i-th "1" and the (i+1)-th "1".
/// It requires a rank64 index for speeding up and a select32 index
#[allow(dead_code)]
pub fn select_2_s32_r64(
    words: &[u64],
    select_index: &[i32],
    rank_index: &[i32],
    context: &Context,
    i: i32,
) -> (i32, i32) {
    //

    let mut in_word_idx;
    let l = words.len();

    // find the word that contains i/32-th `1`.
    let mut word_i: usize = (select_index[(i >> 5) as usize] >> 6) as usize;

    // find the word that contains i-th `1`.
    while rank_index[(word_i + 1) as usize] <= i {
        word_i += 1;
    }

    let mut w = words[word_i];
    let mut ww = w;

    let base = word_i << 6;

    // To find the `find_ith` `1` in words[word_i]
    let mut find_ith: u32 = (i - rank_index[word_i]) as u32;

    let mut offset = 0;

    // count of `1` in the least significant 32 bits.
    let ones = (ww as u32).count_ones();
    if ones <= find_ith {
        find_ith -= ones;
        offset += 32;
        ww >>= 32;
    }

    // count of `1` in the [32, 32+16] bits.
    let ones = (ww as u16).count_ones();
    if ones <= find_ith {
        find_ith -= ones;
        offset |= 16;
        ww >>= 16;
    }

    let ones = (ww as u8).count_ones();

    if ones <= find_ith {
        // The `1` to find is in the second 8 bits.

        let x = (((ww as usize) >> 5) & 0x7f8) | ((find_ith - ones) as usize);
        in_word_idx = context.select_lookup_8.lookup[x as usize] as i32 + offset + 8;
    } else {
        // The `1` to find is in the first 8 bits.

        let x = ((ww as usize) & 0xff) << 3 | (find_ith as usize);
        in_word_idx = context.select_lookup_8.lookup[x] as i32 + offset;
    }

    in_word_idx += base as i32;

    // clear the bits upto in_word_idx, continue to find next `1`
    w &= context.masks.r_mask_upto[(in_word_idx & 63) as usize];

    if w != 0 {
        return (in_word_idx, (base + w.trailing_zeros() as usize) as i32);
    }

    // there is no other `1` in this word, find in subsequent words.

    word_i += 1;

    while word_i < l {
        let w = words[word_i];
        if w != 0 {
            return (
                in_word_idx,
                ((word_i << 6) + w.trailing_zeros() as usize) as i32,
            );
        }
        word_i += 1;
    }

    (in_word_idx, (l << 6) as i32)
}
