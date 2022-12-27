use portal::types::dns::Name;

#[test]
fn test_name_iter() {
    let n = Name::try_from("www.example.com").unwrap();
    let s = n
        .iter()
        .map(|l| {
            let mut label = l.to_string();
            label.push('.');
            label
        })
        .collect::<String>();
    assert_eq!(s, String::from("www.example.com."))
}

#[test]
fn test_name_iter_rev() {
    let n = Name::try_from("www.example.com").unwrap();
    let s = n
        .iter()
        .rev()
        .map(|l| {
            let mut label = String::from(".");
            label.push_str(l.to_string().as_str());
            label
        })
        .collect::<String>();
    assert_eq!(s, String::from(".com.example.www"))
}

#[test]
fn test_name_fragments() {
    let n = Name::try_from("www.example.com").unwrap();
    let f = n.fragments();

    assert_eq!(f.len(), 3);
    assert_eq!(
        f,
        vec![
            Name::try_from("com").unwrap(),
            Name::try_from("example.com").unwrap(),
            Name::try_from("www.example.com").unwrap()
        ]
    )
}
