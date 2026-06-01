use log::{debug, error, info, warn};
use tokio::{io, net::UdpSocket};

pub async fn run_udp(listen: &str, target: &str) -> io::Result<()> {
    let inbound = UdpSocket::bind(listen).await?;
    let outbound = UdpSocket::bind("0.0.0.0:0").await?;
    outbound.connect(target).await?;
    let mut buf = vec![0u8; 65535];
    loop {
        let (len, client_addr) = inbound.recv_from(&mut buf).await?;
        // client -> taget
        outbound.send(&buf[..len]).await?;

        // taget -> client
        let resp_len = outbound.recv(&mut buf).await?;
        inbound.send_to(&buf[..resp_len], client_addr).await?;
    }
}
