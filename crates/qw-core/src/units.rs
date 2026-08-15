const LINEAR_FLOOR: f32 = 1.0e-10;

pub fn db_to_linear(db: f32) -> f32 {
    10.0_f32.powf(db / 20.0)
}

pub fn linear_to_db(linear: f32) -> f32 {
    20.0 * linear.abs().max(LINEAR_FLOOR).log10()
}

#[cfg(test)]
mod tests {
    use super::{db_to_linear, linear_to_db};

    #[test]
    fn unity_gain_roundtrip() {
        let linear = db_to_linear(0.0);
        assert!((linear - 1.0).abs() < 1.0e-6);
        assert!(linear_to_db(linear).abs() < 1.0e-5);
    }

    #[test]
    fn silence_does_not_explode() {
        assert!(linear_to_db(0.0) < -90.0);
    }
}
