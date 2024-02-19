mod proto;
pub use proto::*;

#[cfg(test)]
mod tests {
    use crate::canvas::{OpCode, Scale};

    #[test]
    fn test_union() {
        let mut op = OpCode::new();
        op.set_scale(Scale::default());
    }
}
