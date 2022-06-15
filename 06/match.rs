#[warn(dead_code)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let addr = IpAddr::V4(127, 0, 0, 1);

    match addr {
        IpAddr::V4(i1, i2, i3, i4) => {
            println!("{},{},{},{}", i1, i2, i3, i4);
        },
        IpAddr::V6(ip) => {
            println!("{}", ip);
        }
    };
}
