use crate::bitmap::index::BuildIndex;
use crate::bitmap::index::SelectRankIndex;
use crate::bitmap::select::SelectIndex32;
use crate::bitmap::select::SelectLookup8;

#[test]
fn test_select_lookup_8() -> anyhow::Result<()> {
    //
    let lookup = SelectLookup8::new();

    let cases = vec![
        // word, to find i-th 1, expected position
        (0b0101, 0, 0),
        (0b0101, 1, 2),
        (0b0101, 2, 8), // no 2-th 1
        (0b01010, 0, 1),
    ];

    for (word, ith, want) in cases {
        let got = lookup.lookup[word * 8 + ith];
        assert_eq!(want, got);
    }

    Ok(())
}

#[test]
fn test_select_32_r64() -> anyhow::Result<()> {
    //

    let cases = vec![
        vec![],
        vec![0],
        vec![1],
        vec![2],
        vec![3],
        vec![4, 0],
        vec![0xf, 0xf],
        vec![0xf, 0, 0xf],
        vec![0xfffffffffffffff0],
        vec![0xffffffffffffffff],
        vec![0xffffffff, 0xffffffff],
        vec![0xffffffff, 0xffffffff, 1],
        vec![0b000101100110011],
    ];

    for (_i, c) in cases.iter().enumerate() {
        let sidx = SelectIndex32::build(c);

        let all = to_array(c);

        if all.len() > 0 {
            for j in 0..(all.len() - 1) {
                // let (a, b) = select_2_s32_r64(c, &sidx.index, &sidx.rank_index.index, &ctx, j as i32);
                // assert_eq!(all[j], a, "select: first: ");
                // assert_eq!(all[j + 1], b, "select: second: ");

                let a = sidx.select_ith_one(c, j as i32);
                assert_eq!(all[j], a, "select: first: ");
            }
        }
    }

    Ok(())
}

fn to_array(words: &[u64]) -> Vec<i32> {
    let mut r = Vec::<i32>::new();
    let l = (words.len() * 64) as i32;

    for i in 0..l {
        if words[(i >> 6) as usize] & (1 << (i & 63)) != 0 {
            r.push(i);
        }
    }

    return r;
}
