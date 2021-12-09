use crate::bitmap::bitmap_ops::BitmapOps;
use crate::bitmap::index::BuildIndex;
use crate::bitmap::index::RankIndex;
use crate::bitmap::index::SelectRankIndex;
use crate::bitmap::rank::RankIndex64;
use crate::bitmap::select::SelectIndex32;

pub struct IndexedBitmap<SI>
where SI: SelectRankIndex
{
    pub words: Vec<u64>,
    pub index: SI,
}

impl BitmapOps for IndexedBitmap<SelectIndex32<RankIndex64>> {
    fn new(words: Vec<u64>) -> Self {
        let si = SelectIndex32::<RankIndex64>::build(&words);
        Self { words, index: si }
    }

    fn count_ones(&self, i: i32) -> (i32, i32) {
        self.index.count_ones(&self.words, i)
    }

    fn select_ith_one(&self, i: i32) -> i32 {
        self.index.select_ith_one(&self.words, i)
    }
}
