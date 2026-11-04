use std::time::{SystemTime, UNIX_EPOCH};

pub(super) struct RandomSource {
    state: u32,
}

impl RandomSource {
    pub(super) fn new(seed: Option<u64>) -> Self {
        let [a, b, c, d, ..] = seed.unwrap_or_else(unseeded).to_le_bytes();
        Self {
            state: u32::from_le_bytes([a, b, c, d]),
        }
    }

    // Mulberry32 (BR-GENERATE-001). Operações módulo 2³² como no TypeScript.
    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_add(0x6d2b_79f5);
        let mut t = self.state;
        t = (t ^ (t >> 15)).wrapping_mul(t | 1);
        t ^= t.wrapping_add((t ^ (t >> 7)).wrapping_mul(t | 0x3d));
        t ^ (t >> 14)
    }

    pub(super) fn int(&mut self, min: u32, max: u32) -> u32 {
        let width = u64::from(max - min) + 1;
        let offset = (u64::from(self.next()) * width) >> 32;
        min + u32::try_from(offset).unwrap_or_default()
    }

    pub(super) fn digits(&mut self, length: usize) -> String {
        (0..length).map(|_| self.digit()).collect()
    }

    pub(super) fn digit(&mut self) -> char {
        char::from_digit(self.int(0, 9), 10).unwrap_or('0')
    }

    pub(super) fn letter(&mut self) -> char {
        self.choose_char(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ")
    }

    pub(super) fn choose_char(&mut self, chars: &[u8]) -> char {
        self.pick(chars).map_or('A', char::from)
    }

    pub(super) fn pick<T: Copy>(&mut self, items: &[T]) -> Option<T> {
        let max = u32::try_from(items.len().checked_sub(1)?).ok()?;
        let index = usize::try_from(self.int(0, max)).ok()?;
        items.get(index).copied()
    }
}

fn unseeded() -> u64 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let address = std::ptr::from_ref(&time) as usize;
    time.as_secs() ^ u64::from(time.subsec_nanos()) ^ u64::try_from(address).unwrap_or_default()
}
