use std::net;

struct TcpUdp<T, U> {
    tcp: T,
    udp: U,
}

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

struct ChannelClient {
    uuid: String,
    ip: String,
    port: u16,
}

trait Channel {
    fn emit_all(self, message: String) -> Result<String, Box<dyn std::error::Error>>;
    fn emit_to<T>(
        self,
        clients: [ChannelClient],
        message: String,
    ) -> Result<T, Box<dyn std::error::Error>>;
    fn register_event_handler<T>(
        event: String,
        func: dyn Fn(String) -> Result<T, Box<dyn std::error::Error>>,
    );
}

struct TcpChannel {
    registered_client: Vec<ChannelClient>,
}

struct UdpChannel {}

impl QuickSocketInstance {
    pub async fn new() -> Result<QuickSocketInstance, Box<dyn std::error::Error>> {
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

        Ok(instance)
    }

    fn create_udp_channel() -> Result<u32, Box<dyn std::error::Error>> {}

    fn delete_udp_channel(ch_num: u32) -> Result<_, Box<dyn std::error::Error>> {}

    fn create_tcp_channel() -> Result<u32, Box<dyn std::error::Error>> {}
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
