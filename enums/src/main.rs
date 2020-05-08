enum IpAddr {
    V4(String),
    V6(String)
}


fn main() {
    let ipv4 = IpAddr::V4("10.0.0.1");
    let ipv6 = IpAddr::V6("::1");

    println!("{}", ipv4);
    println!("{}", ipv6);
}
