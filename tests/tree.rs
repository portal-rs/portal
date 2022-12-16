use std::net::Ipv4Addr;

use portal::{
    tree::Tree,
    types::{
        dns::Name,
        rr::{RData, Record},
    },
};

#[test]
fn test_tree_population() {
    let mut tree = Tree::new();

    let example_name = Name::try_from("example.com.").unwrap();

    let mut record = Record::new();
    record.set_rdata(RData::A(Ipv4Addr::new(127, 0, 0, 1)));

    tree.insert(example_name.clone(), &mut vec![record])
        .unwrap();

    let example_node = tree.find_node(example_name).unwrap();

    println!("{:#?}", tree);
    println!("{:#?}", example_node);
}
