use super::Variable;

/// A memory represents of svg element's `transform` attribute.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Transform {
    Translate {
        tx: f32,
        ty: f32,
    },
    /// compressed 3x3 matrix.
    Matrix {
        a: f32,
        b: f32,
        c: f32,
        d: f32,
        e: f32,
        f: f32,
    },
    Scale {
        sx: f32,
        sy: f32,
    },
    Rotate {
        angle: f32,
        cx: f32,
        cy: f32,
    },
    SkewX(f32),
    SkewY(f32),
}

/// Transform can be used as context variant type.
impl Variable for Transform {}