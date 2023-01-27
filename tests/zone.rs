use portal::{types::dns::Name, zone::Zone};

#[test]
fn test_parse_zone_file() {
    match Zone::from_file("./tests/test.hints".into()) {
        Ok(zone) => {
            let node = zone
                .tree
                .find_node(Name::try_from("A.ROOT-SERVERS.NET.").unwrap())
                .unwrap();

            println!("{:#?}", node.records());
        }
        Err(err) => panic!("{}", err),
    };
}
