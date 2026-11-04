pub(super) fn cyclic_sum(body: &str, start: u32, reset: u32) -> u32 {
    let mut sum = 0;
    let mut weight = start;
    for byte in body.bytes() {
        sum += u32::from(byte - b'0') * weight;
        weight = if weight == 2 { reset } else { weight - 1 };
    }
    sum
}

pub(super) fn descending_sum_8(body: &str) -> u32 {
    cyclic_sum(body, 9, 9)
}
