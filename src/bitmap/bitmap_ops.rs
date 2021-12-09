/// Bitmap API
pub trait BitmapOps {
    fn new(words: Vec<u64>) -> Self;

    /// Calculate the rank at index `i`:
    /// returns the count of `1` upto `i`, exclusive, and the value at `i`(0 or 1).
    fn count_ones(&self, i: i32) -> (i32, i32);

    /// Returns the index of the i-th "1".
    /// It requires a rank64 index for speeding up and a select32 index
    fn select_ith_one(&self, i: i32) -> i32;

    fn count_zeros(&self, i: i32) -> (i32, i32) {
        let (a, v) = self.count_ones(i);

        (i as i32 - a, v)
    }
}
