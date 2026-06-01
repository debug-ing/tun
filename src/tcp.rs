use log::{debug, error, info, warn};
use tokio::{
    io,
    net::{TcpListener, TcpStream},
};

pub async fn run_tcp(listen: &str, target: &str) -> io::Result<()> {
    let listener = TcpListener::bind(listen).await?;
    loop {
        // addr is client address
        let (mut inbound, addr) = listener.accept().await?;
        // get target
        let target = target.to_string();
        tokio::spawn(async move {
            if let Err(e) = handle_connection(inbound, &target).await {
                error!("Connection error: {}", e);
            }
        });
    }
}

async fn handle_connection(mut inbound: TcpStream, target_addr: &str) -> io::Result<()> {
    let mut outbound = TcpStream::connect(target_addr).await?;
    let (from_client, from_server) = io::copy_bidirectional(&mut inbound, &mut outbound).await?;
    debug!(
        "Closed. Client->Server: {} bytes, Server->Client: {} bytes",
        from_client, from_server
    );
    Ok(())
}
