use portal::{resolver::Hint, types::dns::Name, zone::Zone};

#[test]
fn test_parse_zone_file() {
    match Zone::from_file("./tests/files/test.hints".into()) {
        Ok(zone) => {
            // println!("{:#?}", zone);

            let node = zone.tree.find_node(Name::try_from(".").unwrap()).unwrap();

            println!("{:#?}", node.records());
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_parse_zone_into_hints() {
    let zone = match Zone::from_file("./named.root".into()) {
        Ok(zone) => zone,
        Err(err) => panic!("{}", err),
    };

    let hints: Vec<Hint> = zone.into();
    assert_eq!(hints.len(), 13);

    println!("{:#?}", hints);
}
