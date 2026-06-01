use tun_rust::config::{Protocol, load_config};
use tun_rust::tcp::run_tcp;
use tun_rust::udp::run_udp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let config = load_config("config.yaml")?;
    for tunnel in config.tunnels {
        match tunnel.protocol {
            Protocol::Tcp => {
                // tcp tunnel
                tokio::spawn(async move {
                    let _ = run_tcp(&tunnel.listen, &tunnel.target).await;
                });
            }
            Protocol::Udp => {
                // udp tunnel
                tokio::spawn(async move {
                    let _ = run_udp(&tunnel.listen, &tunnel.target).await;
                });
            }
        }
    }
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
    }
}
