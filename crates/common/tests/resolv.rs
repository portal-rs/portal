use std::net::{IpAddr, Ipv4Addr};

use portal_common::{ResolvConfig, ResolvOption};

#[test]
fn test_parse_resolv_file() {
    let option = ResolvOption::Nameserver(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

    match ResolvConfig::from_file("./tests/files/resolv.conf".into()) {
        Ok(config) => {
            assert_eq!(config.options().len(), 1);
            assert_eq!(config.options().first().unwrap(), &option);
        }
        Err(err) => panic!("{err}"),
    }
}
