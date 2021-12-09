use lazy_static::lazy_static;

use crate::bitmap::bitmap_ops::BitmapOps;
use crate::bitmap::context::Context;
use crate::bitmap::indexed_bitmap::IndexedBitmap;
use crate::bitmap::rank::RankIndex64;
use crate::bitmap::select::SelectIndex32;

lazy_static! {
    pub static ref CTX: Context = Context::new();
}

pub type LabelBitmap = IndexedBitmap<SelectIndex32<RankIndex64>>;

pub struct StaticKV<LBM>
where LBM: BitmapOps
{
    pub leaves: Vec<u64>,
    pub label_bitmap: LBM,
    pub labels: Vec<u8>,
}

/// The breadth first walking state node
#[derive(Debug)]
struct BFSNode {
    /// The index of the starting key reachable from a trie node.
    start: usize,

    /// The index of the ending key(exclusive) reachable from a trie node.
    end: usize,

    /// The index from which to scan the keys.
    col: usize,
}

impl BFSNode {
    #[allow(dead_code)]
    pub fn new(start: usize, end: usize, col: usize) -> Self {
        BFSNode { start, end, col }
    }
    pub fn unpack(&self) -> (usize, usize, usize) {
        (self.start, self.end, self.col)
    }
}

impl<LBM> StaticKV<LBM>
where LBM: BitmapOps
{
    /// Build a compacted trie from a sorted list of keys.
    #[allow(dead_code)]
    pub fn new(keys: &[Vec<u8>]) -> Self {
        let mut leaves = vec![];
        let mut label_bitmap = vec![];
        let mut labels = vec![];

        let mut queue = vec![BFSNode::new(0, keys.len(), 0)];

        let mut node_id = 0;
        let mut bitmap_index = 0;

        while node_id < queue.len() {
            let (mut start, end, col) = queue[node_id].unpack();

            if col == keys[start].len() {
                // leaf node
                start += 1;
                set_bit(&mut leaves, node_id, 1);
            }

            let mut j = start;

            while j < end {
                let frm = j;
                let label = keys[frm][col];

                while j < end && keys[j][col] == label {
                    j += 1;
                }

                // new label

                queue.push(BFSNode::new(frm, j, col + 1));
                labels.push(label);
                set_bit(&mut label_bitmap, bitmap_index, 0);
                bitmap_index += 1;
            }

            set_bit(&mut label_bitmap, bitmap_index, 1);
            bitmap_index += 1;

            node_id += 1;
        }

        StaticKV {
            leaves,
            label_bitmap: LBM::new(label_bitmap),
            labels,
        }
    }
}

impl StaticKV<LabelBitmap> {
    #[allow(dead_code)]
    pub fn has(&self, key: &[u8]) -> bool {
        let mut node_id: i32 = 0;
        let mut bitmap_index: i32 = 0;

        for c in key {
            loop {
                if get_bit(&self.label_bitmap.words, bitmap_index as usize) != 0 {
                    // no more labels in this node
                    return false;
                }

                if self.labels[(bitmap_index - node_id) as usize] == *c {
                    // matched
                    break;
                }

                bitmap_index += 1;
            }

            // go to next level
            node_id = self.label_bitmap.count_zeros(bitmap_index + 1).0;
            bitmap_index = self.label_bitmap.select_ith_one(node_id - 1) + 1;
        }

        get_bit(&self.leaves, node_id as usize) != 0
    }
}

fn set_bit(words: &mut Vec<u64>, i: usize, value: u64) {
    while i >> 6 >= words.len() {
        words.push(0);
    }

    words[i >> 6] |= value << (i & 63);
}

#[allow(dead_code)]
fn get_bit(words: &[u64], i: usize) -> u64 {
    words[i >> 6] & (1 << (i & 63))
}
