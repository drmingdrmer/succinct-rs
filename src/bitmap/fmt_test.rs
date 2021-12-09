use crate::bitmap::fmt::fmt_bitmap;
use crate::bitmap::fmt::format_bits;

#[test]
fn test_fmt_one_word() -> anyhow::Result<()> {
    assert_eq!("00000000", format_bits(0u8));
    assert_eq!("10000000", format_bits(1u8));
    assert_eq!("00001111", format_bits(0xf0u8));

    assert_eq!("00001111 10000000", format_bits(0x01f0u16));

    assert_eq!(
        "00001111 10000000 00001111 10000000",
        format_bits(0x01f001f0u32)
    );

    assert_eq!(
        "10001000 00000000 00000000 00000000 00000000 00000000 00000000 00001111",
        format_bits(0xf000000000000011u64)
    );

    Ok(())
}

#[test]
fn test_fmt_bitmap() -> anyhow::Result<()> {
    assert_eq!("", fmt_bitmap(&vec![]));
    assert_eq!(
        "00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000",
        fmt_bitmap(&vec![0])
    );
    assert_eq!(
        concat!(
            "10000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000,",
            "01000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000"
        ),
        fmt_bitmap(&vec![1, 2])
    );
    Ok(())
}
