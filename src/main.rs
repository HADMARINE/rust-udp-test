use std::env;
use std::net;

struct TcpUdp<T, U> {
    tcp: T,
    udp: U,
}

// struct TcpUdp<T, U> {
//     udp:tokio::net::UdpSocket,
//     tcp:tokio::net::TcpSocket
// }

struct PortRange {
    start: u16,
    end: u16,
}

struct Properties {
    port_range: PortRange,
}

struct QuickSocketInstance {
    socket: TcpUdp<Vec<tokio::net::TcpListener>, Vec<tokio::net::UdpSocket>>,
    properties: Properties,
}

impl QuickSocketInstance {
    pub async fn new() -> Result<(), Box<dyn std::error::Error>> {
        use tokio::net::*;

        let port: u16 = 8080;
        let addr = format!("127.0.0.1:{}", &port);
        let default_tcp_channel = TcpListener::bind(&addr).await?;
        // let udp_instance = UdpSocket::bind(&addr).await?;

        let tcp_channels: Vec<TcpListener> = vec![default_tcp_channel];
        let udp_channels: Vec<UdpSocket> = vec![];

        let socket = TcpUdp {
            tcp: tcp_channels,
            udp: udp_channels,
        };

        let properties = Properties {
            port_range: PortRange {
                start: 20000,
                end: 65535,
            },
        };

        let mut instance = QuickSocketInstance { socket, properties };

        Ok(())
    }

    // fn
}

fn listen(socket: &net::UdpSocket, mut buffer: &mut [u8]) -> usize {
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("No data recieved");

    println!("{:?}", number_of_bytes);
    println!("{:?}", src_addr);

    number_of_bytes
}

// fn send(socket:&net::)

fn main() {
    println!("Hello, world!");
}
