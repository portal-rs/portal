use portal_proto::{Label, Name};

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

#[test]
fn test_name_casing_basic() {
    let n1 = Name::try_from("EXAMPLE.COM").unwrap();
    let n2 = Name::try_from("example.com").unwrap();

    assert_eq!(n1, n2)
}

#[test]
fn test_name_casing_from_labels_str() {
    let mut n1 = Name::default();
    n1.add_label(Label::try_from("EXAMPLE").unwrap()).unwrap();
    n1.add_label(Label::try_from("COM").unwrap()).unwrap();

    let mut n2 = Name::default();
    n2.add_label(Label::try_from("example").unwrap()).unwrap();
    n2.add_label(Label::try_from("com").unwrap()).unwrap();

    assert_eq!(n1, n2)
}

#[test]
fn test_name_casing_from_labels_slice() {
    let mut n1 = Name::default();
    n1.add_label(Label::try_from(vec![69, 88, 65, 77, 80, 76, 69].as_slice()).unwrap())
        .unwrap();
    n1.add_label(Label::try_from(vec![67, 79, 77].as_slice()).unwrap())
        .unwrap();

    let mut n2 = Name::default();
    n2.add_label(Label::try_from(vec![101, 120, 97, 109, 112, 108, 101].as_slice()).unwrap())
        .unwrap();
    n2.add_label(Label::try_from(vec![99, 111, 109].as_slice()).unwrap())
        .unwrap();

    assert_eq!(n1, n2)
}

#[test]
#[should_panic(expected = "Invalid byte in domain name label")]
fn test_invalid_domain_name_label_byte() {
    match Name::try_from("invälid.com") {
        Ok(_) => {}
        Err(err) => panic!("{}", err),
    };
}
