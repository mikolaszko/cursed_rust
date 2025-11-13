fn divide_by_five(x: u64) -> u64 {
    ((x as u128 * 0xCCCCCCCCCCCCCCCDu128) >> 66) as u64
}
