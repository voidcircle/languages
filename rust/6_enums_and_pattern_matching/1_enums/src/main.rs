enum IPAddressKinds {
    V4,
    V6,
}

enum NewIPAddressKinds {
    V4(u8, u8, u8, u8),
    V6(String),
}

// this kind of enums are what have been defined on the standard library
struct IpAddrV4 {
    somethign: u8,
}
struct IpAddrV6 {
    something: u8,
}
enum IpAddr {
    V4(IpAddrV4),
    V6(IpAddrV6),
}

// struct IPAddress {
//     version: IPAddressKinds,
//     address: String,
// }

fn main() {
    let four = IPAddressKinds::V4;
    let _six = IPAddressKinds::V6;

    // either can work
    route(four);
    route(IPAddressKinds::V6);

    // This way sucks!
    // let _ip_four = IPAddress {
    //     version: IPAddressKinds::V4,
    //     address: String::from("111.1.1.1"),
    // };

    let _home = NewIPAddressKinds::V4(127, 0, 0, 1);
    let _loadpack = NewIPAddressKinds::V6(String::from("::1"));

    // wow standard libary
    let fourfour = IpAddr::V4(IpAddrV4 { somethign: 234 });
}

fn route(ip_address: IPAddressKinds) -> bool {
    return false;
}
