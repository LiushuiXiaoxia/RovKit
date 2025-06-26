use std::net::{IpAddr, Ipv4Addr, TcpListener, ToSocketAddrs, UdpSocket};

/// 获取本机默认 IPv4 地址（非回环）
pub fn get_local_ipv4() -> Option<Ipv4Addr> {
    let addrs = local_ip_address::list_afinet_netifas().ok()?;
    for (_name, ip) in addrs {
        if let IpAddr::V4(v4) = ip {
            if !v4.is_loopback() {
                return Some(v4);
            }
        }
    }
    None
}

/// 获取主机名
pub fn get_hostname() -> Option<String> {
    hostname::get().ok().and_then(|s| s.into_string().ok())
}

/// 解析域名为 IP 地址列表
pub fn resolve_hostname(host: &str, port: u16) -> Vec<IpAddr> {
    let addr_str = format!("{}:{}", host, port);
    addr_str
        .to_socket_addrs()
        .map(|iter| iter.map(|sa| sa.ip()).collect())
        .unwrap_or_default()
}

/// 检查端口是否被占用（绑定失败表示已占用）
pub fn is_port_in_use(port: u16) -> bool {
    TcpListener::bind(("0.0.0.0", port)).is_err()
}

/// 检查 UDP 端口是否被占用（绑定失败表示已占用）
pub fn is_udp_port_in_use(port: u16) -> bool {
    UdpSocket::bind(("0.0.0.0", port)).is_err()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_local_ipv4() {
        let ip = get_local_ipv4();
        println!("Local IPv4: {:?}", ip);
        // Can't assert exact value since it varies
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
