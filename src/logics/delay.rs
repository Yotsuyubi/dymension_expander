pub fn delay(index: usize, mut size: f32) -> isize {
    const SIZE_OFFSET: f32 = 0.06;
    const SIZE_MULT: f32 = 1_000.0;

    size += SIZE_OFFSET;

    // Spread ratio between delays
    const SPREAD: f32 = 0.3;

    let base = size * SIZE_MULT;
    let mult = (index as f32 * SPREAD) + 1.0;
    let offset = if index > 2 { base * SPREAD / 2.0 } else { 0.0 };

    (base * mult + offset) as isize
}
