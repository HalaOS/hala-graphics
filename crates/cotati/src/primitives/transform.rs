use std::str::FromStr;

/// A memory represents of svg element's `transform` attribute.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
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

/// transform string parser.
mod parser {

    use nom::{
        branch::alt,
        bytes::complete::{tag, take_while},
        combinator::{map_res, opt},
        IResult,
    };

    use std::str::FromStr;

    use super::Transform;

    pub(super) fn parse_transform(input: &str) -> IResult<&str, Transform> {
        alt((
            parse_translate,
            parse_matrix,
            parse_scale,
            parse_rotation,
            parse_skewx,
            parse_skewy,
        ))(input)
    }

    fn wsp(input: &str) -> IResult<&str, &str> {
        take_while(|c: char| c.is_ascii_whitespace())(input)
    }

    fn number(input: &str) -> IResult<&str, f32> {
        let (input, sign) = opt(alt((tag("+"), tag("-"))))(input)?;
        let (input, value) = map_res(
            take_while(|c: char| c.is_ascii_alphanumeric() || c == '.'),
            f32::from_str,
        )(input)?;

        match sign {
            Some("-") => Ok((input, -value)),
            _ => Ok((input, value)),
        }
    }

    fn parse_translate(input: &str) -> IResult<&str, Transform> {
        let (input, _) = tag("translate")(input)?;
        let (input, _) = wsp(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, _) = wsp(input)?;
        let (input, dx) = number(input)?;
        let (input, _) = wsp(input)?;
        let (input, comma) = opt(tag(","))(input)?;
        let (input, _) = wsp(input)?;

        if !comma.is_some() {
            let (input, _) = tag(")")(input)?;
            return Ok((input, Transform::Translate { tx: dx, ty: 0.0 }));
        }

        let (input, dy) = number(input)?;

        let (input, _) = wsp(input)?;

        let (input, _) = tag(")")(input)?;

        return Ok((input, Transform::Translate { tx: dx, ty: dy }));
    }

    fn parse_matrix(input: &str) -> IResult<&str, Transform> {
        let (input, _) = tag("matrix")(input)?;
        let (input, _) = wsp(input)?;

        let (input, _) = tag("(")(input)?;

        let (input, _) = wsp(input)?;

        let (input, t11) = number(input)?;

        let mut mat3x3 = vec![t11];

        let mut i = input;

        for _ in 0..5 {
            let (input, _) = wsp(i)?;
            let (input, _) = tag(",")(input)?;
            let (input, _) = wsp(input)?;

            let (input, tx) = number(input)?;

            mat3x3.push(tx);

            i = input;
        }

        let (input, _) = wsp(i)?;
        let (input, _) = tag(")")(input)?;

        Ok((
            input,
            Transform::Matrix {
                a: mat3x3[0],
                b: mat3x3[1],
                c: mat3x3[2],
                d: mat3x3[3],
                e: mat3x3[4],
                f: mat3x3[5],
            },
        ))
    }

    fn parse_scale(input: &str) -> IResult<&str, Transform> {
        let (input, _) = tag("scale")(input)?;
        let (input, _) = wsp(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, _) = wsp(input)?;
        let (input, dx) = number(input)?;
        let (input, _) = wsp(input)?;
        let (input, comma) = opt(tag(","))(input)?;
        let (input, _) = wsp(input)?;

        if !comma.is_some() {
            let (input, _) = tag(")")(input)?;
            return Ok((input, Transform::Scale { sx: dx, sy: dx }));
        }

        let (input, dy) = number(input)?;

        let (input, _) = wsp(input)?;

        let (input, _) = tag(")")(input)?;

        return Ok((input, Transform::Scale { sx: dx, sy: dy }));
    }

    fn parse_rotation(input: &str) -> IResult<&str, Transform> {
        let (input, _) = tag("rotate")(input)?;
        let (input, _) = wsp(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, _) = wsp(input)?;
        let (input, angle) = number(input)?;
        let (input, _) = wsp(input)?;
        let (input, comma) = opt(tag(","))(input)?;
        let (input, _) = wsp(input)?;

        if !comma.is_some() {
            let (input, _) = tag(")")(input)?;
            return Ok((
                input,
                Transform::Rotate {
                    angle,
                    cx: 0.0,
                    cy: 0.0,
                },
            ));
        }

        let (input, cx) = number(input)?;

        let (input, _) = wsp(input)?;

        let (input, _) = tag(",")(input)?;
        let (input, _) = wsp(input)?;

        let (input, cy) = number(input)?;

        let (input, _) = wsp(input)?;

        let (input, _) = tag(")")(input)?;

        return Ok((input, Transform::Rotate { angle, cx, cy }));
    }

    fn parse_skewx(input: &str) -> IResult<&str, Transform> {
        let (input, _) = tag("skewX")(input)?;
        let (input, _) = wsp(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, _) = wsp(input)?;
        let (input, sx) = number(input)?;
        let (input, _) = wsp(input)?;
        let (input, _) = tag(")")(input)?;

        return Ok((input, Transform::SkewX(sx)));
    }

    fn parse_skewy(input: &str) -> IResult<&str, Transform> {
        let (input, _) = tag("skewY")(input)?;
        let (input, _) = wsp(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, _) = wsp(input)?;
        let (input, sx) = number(input)?;
        let (input, _) = wsp(input)?;
        let (input, _) = tag(")")(input)?;

        return Ok((input, Transform::SkewY(sx)));
    }
}

impl FromStr for Transform {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, transform) = parser::parse_transform(s)
            .map_err(|err| crate::Error::TransformStr(err.to_string()))?;

        Ok(transform)
    }
}

#[cfg(test)]
mod tests {
    use super::Transform;

    #[test]
    fn matrix() {
        assert_eq!(
            "matrix(1,2,3,4,5,6)".parse::<Transform>().unwrap(),
            Transform::Matrix {
                a: 1.0,
                b: 2.0,
                c: 3.0,
                d: 4.0,
                e: 5.0,
                f: 6.0
            }
        );

        assert_eq!(
            "matrix (1,2,3,4,5,6) ".parse::<Transform>().unwrap(),
            Transform::Matrix {
                a: 1.0,
                b: 2.0,
                c: 3.0,
                d: 4.0,
                e: 5.0,
                f: 6.0
            }
        );

        assert_eq!(
            "matrix (1  ,2  ,3
            ,4,5,6) "
                .parse::<Transform>()
                .unwrap(),
            Transform::Matrix {
                a: 1.0,
                b: 2.0,
                c: 3.0,
                d: 4.0,
                e: 5.0,
                f: 6.0
            }
        );
    }

    #[test]
    fn rotate() {
        assert_eq!(
            "rotate(30.0,0,1 )".parse::<Transform>().unwrap(),
            Transform::Rotate {
                angle: 30.0,
                cx: 0.0,
                cy: 1.0
            }
        );

        assert_eq!(
            "rotate(30)".parse::<Transform>().unwrap(),
            Transform::Rotate {
                angle: 30.0,
                cx: 0.0,
                cy: 0.0
            }
        );
    }

    #[test]
    fn scale() {
        assert_eq!(
            "scale(30)".parse::<Transform>().unwrap(),
            Transform::Scale { sx: 30.0, sy: 30.0 }
        );

        assert_eq!(
            "scale(1.5  ,  
            0.5  )"
                .parse::<Transform>()
                .unwrap(),
            Transform::Scale { sx: 1.5, sy: 0.5 }
        );
    }

    #[test]
    fn skew() {
        assert_eq!(
            "skewX(3.1)".parse::<Transform>().unwrap(),
            Transform::SkewX(3.1)
        );

        assert_eq!(
            "skewY(2.1)".parse::<Transform>().unwrap(),
            Transform::SkewY(2.1)
        );
    }

    #[test]
    fn translate() {
        assert_eq!(
            "translate(3.1)".parse::<Transform>().unwrap(),
            Transform::Translate { tx: 3.1, ty: 0.0 }
        );

        assert_eq!(
            "translate(3.1,2.1)".parse::<Transform>().unwrap(),
            Transform::Translate { tx: 3.1, ty: 2.1 }
        );
    }
}
