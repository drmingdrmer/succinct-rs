use crate::bitmap::index::BuildIndex;
use crate::bitmap::index::RankIndex;

#[derive(Debug, Clone)]
pub struct RankIndex64 {
    pub index: Vec<i32>,
}

impl BuildIndex for RankIndex64 {
    fn build(words: &[u64]) -> Self {
        let index = build_rank64_index(words);
        RankIndex64 { index }
    }
}

impl RankIndex for RankIndex64 {
    fn count_ones(&self, words: &[u64], i: i32) -> (i32, i32) {
        rank64(words, &self.index, i)
    }

    fn get_rank_index(&self) -> &[i32] {
        &self.index
    }
}

/// Build a index to speed up rank() by counting the `1` in every 64 bits segment.
#[allow(dead_code)]
fn build_rank64_index(words: &[u64]) -> Vec<i32> {
    let l = words.len() + 1;

    let mut rank_index = vec![0; l];

    let mut n = 0;

    for i in 0..words.len() {
        rank_index[i] = n;
        n += words[i].count_ones() as i32;
    }

    rank_index[words.len()] = n;

    rank_index
}

/// Find the i-th `1` with the help of a pre-calc index.
#[allow(dead_code)]
fn rank64(words: &[u64], rank_index: &[i32], i: i32) -> (i32, i32) {
    let word_idx = (i >> 6) as usize;
    let in_word_idx = i & 63;

    let n = rank_index[word_idx];
    let w = words[word_idx];

    let count_of_1 = n + (w & ((1 << in_word_idx) - 1)).count_ones() as i32;

    (count_of_1, (w >> in_word_idx) as i32 & 1)
}
