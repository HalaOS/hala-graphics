pub mod proto;

#[cfg(test)]
mod tests {
    use protobuf::Message;

    use crate::proto;

    #[test]
    fn test_serialize() {
        let mut sub_path = proto::opcode::SubPath::new();

        sub_path.set_arc(proto::opcode::Arc {
            start_angle: 25.1,
            sweep_angle: 100.13,
            ..Default::default()
        });

        println!("{:?}", sub_path.write_to_bytes().unwrap().len());

        sub_path.set_path(proto::opcode::ExtendWithPath::new());

        println!("{:?}", sub_path.write_to_bytes().unwrap().len());

        sub_path.set_closed(true);

        println!("{:?}", sub_path.write_to_bytes().unwrap().len());
    }
}
