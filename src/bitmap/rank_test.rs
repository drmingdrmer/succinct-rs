use crate::bitmap::index::BuildIndex;
use crate::bitmap::index::RankIndex;
use crate::bitmap::rank::RankIndex64;

#[test]
fn test_rank64() -> anyhow::Result<()> {
    #[derive(Debug)]
    struct T {
        bm: Vec<u64>,
        want64: Vec<i32>,
    }

    let cases = vec![
        T {
            bm: vec![],
            want64: vec![0],
        },
        T {
            bm: vec![0],
            want64: vec![0, 0],
        },
        T {
            bm: vec![1],
            want64: vec![0, 1],
        },
        T {
            bm: vec![0xffffffffffffffff],
            want64: vec![0, 64],
        },
        T {
            bm: vec![0xffffffffffffffff, 1],
            want64: vec![0, 64, 65],
        },
        T {
            bm: vec![0xffffffffffffffff, 1, 1],
            want64: vec![0, 64, 65, 66],
        },
        T {
            bm: vec![0xffffffffffffffff, 1, 1, 3],
            want64: vec![0, 64, 65, 66, 68],
        },
        T {
            bm: vec![0xffffffffffffffff, 1, 1, 3, 4],
            want64: vec![0, 64, 65, 66, 68, 69],
        },
    ];

    for (_i, c) in cases.iter().enumerate() {
        // test building index

        let index = RankIndex64::build(&c.bm);
        assert_eq!(c.want64, index.index);

        // test rank 64

        let mut cnt = 0;
        let mut cnt_exclude_i = 0;

        for j in 0..c.bm.len() * 64 {
            if c.bm[j >> 6] & (1 << (j & 63)) != 0 {
                cnt += 1;
            }

            let (r_exc, is_set) = index.count_ones(&c.bm, j as i32);
            assert_eq!(cnt_exclude_i, r_exc);
            assert_eq!(cnt - cnt_exclude_i, is_set);

            cnt_exclude_i = cnt;
        }
    }
    Ok(())
}
