#[cfg(test)]
mod tests {
    use rovkit::netkit::*;

    #[test]
    fn test_get_local_ipv4() {
        let ip = get_local_ipv4();
        println!("Local IPv4: {:?}", ip);
    }

    #[test]
    fn test_get_hostname() {
        let name = get_hostname();

        println!("Hostname: {:?}", name);
        assert!(name.is_some());
    }

    #[test]
    fn test_resolve_hostname() {
        let ips = resolve_hostname("localhost", 80);
        println!("IPs: {:?}", ips);
        assert!(!ips.is_empty());
    }

    #[test]
    fn test_is_port_in_use() {
        let port = 65534; // unlikely to be in use
        assert_eq!(is_port_in_use(port), false);
        println!("Port {} is in use: {}", port, is_port_in_use(port))
    }

    #[test]
    fn test_is_udp_port_in_use() {
        let port = 65533; // unlikely to be in use
        assert_eq!(is_udp_port_in_use(port), false);
    }
}
