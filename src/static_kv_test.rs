use pretty_assertions::assert_eq;

use crate::bitmap::fmt::fmt_bitmap;
use crate::bitmap::index::RankIndex;
use crate::bitmap::index::SelectRankIndex;
use crate::static_kv::LabelBitmap;
use crate::static_kv::StaticKV;

fn absent_keys() -> Vec<String> {
    let x = vec!["AB", "abx", "bC", "bcX", "xyz"];

    x.iter().map(|x| x.to_string()).collect()
}

#[test]
fn test_kv() -> anyhow::Result<()> {
    #[derive(PartialEq, Debug)]
    struct WantType {
        leaves: String,
        label_bitmap: String,
        rank_index: Vec<i32>,
        select_index: Vec<i32>,
        labels: String,
    }

    struct Case {
        keys: Vec<&'static str>,
        want: WantType,
    }

    let cases = vec![
        Case {
            keys: vec!["", "a"],
            want: WantType {
                leaves: "11000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000"
                    .to_string(),
                label_bitmap:
                    "01100000 00000000 00000000 00000000 00000000 00000000 00000000 00000000"
                        .to_string(),
                rank_index: vec![0, 2],
                select_index: vec![1],
                labels: "a".to_string(),
            },
        },
        Case {
            keys: vec!["a", "b", "c"],
            want: WantType {
                leaves: "01110000 00000000 00000000 00000000 00000000 00000000 00000000 00000000"
                    .to_string(),
                label_bitmap:
                    "00011110 00000000 00000000 00000000 00000000 00000000 00000000 00000000"
                        .to_string(),
                rank_index: vec![0, 4],
                select_index: vec![3],
                labels: "abc".to_string(),
            },
        },
        Case {
            keys: vec!["a", "ab", "abc"],
            want: WantType {
                leaves: "01110000 00000000 00000000 00000000 00000000 00000000 00000000 00000000"
                    .to_string(),
                label_bitmap:
                    "01010110 00000000 00000000 00000000 00000000 00000000 00000000 00000000"
                        .to_string(),
                rank_index: vec![0, 4],
                select_index: vec![1],
                labels: "abc".to_string(),
            },
        },
        Case {
            keys: vec![
               "abc",
               "abcd",
               "abd",
               "abde",
               "bc",
               "bcd",
               "bcde",
               "cde",
            ],
            want: WantType {
               leaves: "00000101 11111100 00000000 00000000 00000000 00000000 00000000 00000000"
                    .to_string(),
               // 0 -a-> 1 -b-> 4 -c-> 7 -d-> $
               //                 -d-> 8 -e-> $
               //   -b-> 2 -c-> 5 -d-> 9 -e-> $
               //   -c-> 3 -d-> 6 -e-> $
               //
               //        1   2      3       4
               //        abc b c  d cd d e  d e e
                label_bitmap:
                    "00010101 01001010 10101011 11100000 00000000 00000000 00000000 00000000"
                        .to_string(),
                rank_index: vec![0, 14],
                select_index: vec![3],
                labels: "abcbcdcddedee".to_string(),
            },
        },
        Case {
            keys: vec![
               "A", "Aani", "Aaron", "Aaronic", "Aaronical", "Aaronite",
               "Aaronitic", "Aaru", "Ab", "Ababdeh", "Ababua", "Abadite",
               "Abama", "Abanic", "Abantes", "Abarambo", "Abaris", "Abasgi",
               "Abassin", "Abatua", "Abba", "Abbadide", "Abbasside", "Abbie",
               "Abby", "Abderian", "Abderite", "Abdiel", "Abdominales", "Abe",
               "Abel", "Abelia", "Abelian", "Abelicea", "Abelite",
               "Abelmoschus", "Abelonian", "Abencerrages", "Aberdeen",
               "Aberdonian", "Aberia", "Abhorson",
            ],
            want: WantType {
               leaves: concat!(
                   "01010000 01010100 00000101 00010001 00010000 00000100 00000000 00101001,", 
                   "10100010 10000000 10101110 10000010 10000000 10110110 10001011 11001000,", 
                   "00111010 00000000 00000000 00000000 00000000 00000000 00000000 00000000")
                    .to_string(),
                label_bitmap: concat!(
                   "01001001 00000101 00100000 00100010 00100010 11011001 01010010 01001010,", 
                   "01011010 10100010 10010101 01010110 10101010 10101010 11010101 00010101,", 
                   "01001010 10010110 11010111 01101010 01101010 10101010 10101101 01001111,", 
                   "01101010 10101101 10101010 10101011 01110111 01101010 11011111 01011010,", 
                   "10101011 11011000 00000000 00000000 00000000 00000000 00000000 00000000")
                        .to_string(),
                rank_index: vec![0, 21, 52, 87, 126, 135],
                select_index: vec![1, 89, 151, 208, 260],
                labels: "Aabnrabdehioubdmnrstaiyeiolnronduiaitaigsudseremimocdirieatcemsiiaisiliactoneeoascthesbndiatnneesirenoaeioedneaacarninlcelhnaaeugnsses".to_string(),
            },
        },
    ];

    for (_i, c) in cases.iter().enumerate() {
        let s = StaticKV::<LabelBitmap>::new(&to_vecs(&c.keys));

        let got = WantType {
            leaves: fmt_bitmap(&s.leaves),
            label_bitmap: fmt_bitmap(&s.label_bitmap.words),
            rank_index: RankIndex::get_rank_index(&s.label_bitmap.index).to_vec(),
            select_index: SelectRankIndex::get_select_index(&s.label_bitmap.index).to_vec(),
            labels: String::from_utf8(s.labels.clone()).unwrap(),
        };

        assert_eq!(c.want, got);

        for k in c.keys.iter() {
            let k = k.as_bytes().to_vec();
            let found = s.has(&k);
            assert!(found);
        }

        for k in absent_keys() {
            let bs = k.as_bytes().to_vec();
            let found = s.has(&bs);
            assert!(!found, "absent key: {}", k);
        }
    }

    Ok(())
}

fn to_vecs(v: &[&'static str]) -> Vec<Vec<u8>> {
    let mut res = vec![];
    for s in v {
        res.push(s.as_bytes().to_vec())
    }

    res
}
