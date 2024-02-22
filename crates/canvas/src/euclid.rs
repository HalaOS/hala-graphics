pub use glam::f32::*;

#[cfg(test)]
mod tests {
    use serde_json::to_string_pretty;

    use super::*;

    #[test]
    fn test_serialize() {
        _ = pretty_env_logger::try_init();
        let vec3 = Vec3::from_array([1f32, 2f32, 3.0]);
        log::info!("{}", to_string_pretty(&vec3).unwrap());
    }
}
