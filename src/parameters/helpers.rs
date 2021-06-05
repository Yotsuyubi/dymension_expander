#[allow(non_snake_case)]
pub fn dB_to_linear(dB: f32) -> f32 {
    (10.0_f32).powf(dB / 20.)
}

#[allow(non_snake_case)]
pub fn linear_to_dB(linear: f32) -> f32 {
    20. * (linear).log10()
}

pub fn normalize(value: f32, max: f32, min: f32) -> f32 {
    let clipped_value = if value > max {
        max
    } else if value < min {
        min
    } else {
        value
    };
    (clipped_value - min) / (max - min)
}

pub fn denormalize(value: f32, max: f32, min: f32) -> f32 {
    let clipped_value = if value > 1.0 {
        1.0
    } else if value < 0.0 {
        0.0
    } else {
        value
    };
    (max - min) * clipped_value + min
}
