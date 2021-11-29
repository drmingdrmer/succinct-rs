pub trait BuildIndex {
    fn build(words: &[u64]) -> Self;
}

pub trait RankIndex: BuildIndex {
    /// Calculate the rank at index `i`:
    /// returns the count of `1` upto `i`, exclusive, and the value at `i`(0 or 1).
    fn count_ones(&self, words: &[u64], i: i32) -> (i32, i32);

    fn get_rank_data(&self) -> &[i32];
}

pub trait SelectRankIndex: RankIndex + BuildIndex {
    /// Returns the index of the i-th "1".
    /// It requires a rank64 index for speeding up and a select32 index
    fn select_ith_one(&self, words: &[u64], i: i32) -> i32;

    fn get_select_data(&self) -> &[i32];
}
