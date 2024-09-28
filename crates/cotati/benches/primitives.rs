fn main() {
    divan::main();
}

#[divan::bench_group(sample_count = 100, sample_size = 500)]
mod length {
    use std::hint::black_box;

    use cotati::primitives::{Length, LengthUnit};

    #[divan::bench]
    fn display() {
        black_box(Length::cm(100.0).to_string());
    }

    #[divan::bench]
    fn parse() {
        black_box("100cm".parse::<Length>().unwrap());
    }

    #[divan::bench]
    fn display_unit() {
        black_box(LengthUnit::Cm.to_string());
    }

    #[divan::bench]
    fn parse_unit() {
        black_box("cm".parse::<LengthUnit>().unwrap());
    }
}

#[divan::bench_group(sample_count = 100, sample_size = 500)]
mod transform {
    use std::hint::black_box;

    use cotati::primitives::Transform;

    #[divan::bench]
    fn translate() {
        black_box("translate(3.1)".parse::<Transform>().unwrap());
    }

    #[divan::bench]
    fn skew() {
        black_box("skewX(3.1)".parse::<Transform>().unwrap());
    }

    #[divan::bench]
    fn scale() {
        black_box("scale(3.1,2.0)".parse::<Transform>().unwrap());
    }

    #[divan::bench]
    fn rotate() {
        black_box("rotate(30.0,0,1)".parse::<Transform>().unwrap());
    }

    #[divan::bench]
    fn matrix() {
        black_box("matrix(1,2,3,4,5,6)".parse::<Transform>().unwrap());
    }
}
