use portal::{
    client::Client,
    types::{
        dns::Name,
        rr::{Class, Type},
    },
};

#[tokio::test]
async fn test_client() {
    let c = Client::new().await.unwrap();
    let n = Name::try_from("example.com").unwrap();

    match c
        .query((n, Type::A, Class::IN), "9.9.9.9:53".parse().unwrap())
        .await
    {
        Ok(_) => println!("Cool"),
        Err(err) => panic!("{}", err),
    };
}
