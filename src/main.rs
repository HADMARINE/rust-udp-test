use std::env;
use std::net;


struct Sockets {
    udp:tokio::net::UdpSocket,
    tcp:tokio::net::TcpSocket
}

struct QuickSocketInstance {
    socket: Sockets
}

impl QuickSocketInstance {
    fn new() -> Result<(), Box<dyn std::error::Error>> {
        use tokio::net::*;


        let port: u8 = 8080;
        let addr = format!("127.0.0.1:{}", &port);
        let tcp_listener = TcpListener::bind(&addr).await?;
        let udp_listener = UdpSocket::bind(&addr).await?;
        
        let sockets = Sockets {
            udp, tcp
        };

        
        let mut instance = QuickSocketInstance { 
            socket: sockets
        };
    }

}

fn listen(socket: &net::UdpSocket, mut buffer: &mut [u8]) -> usize {
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buffer).expect("No data recieved");

    println!("{:?}", number_of_bytes);
    println!("{:?}", src_addr);

    number_of_bytes
}


fn send(socket:&net::)


fn main() {
    println!("Hello, world!");
}
