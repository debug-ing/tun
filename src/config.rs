use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub tunnels: Vec<TunnelConfig>,
}

#[derive(Debug, Deserialize)]
pub struct TunnelConfig {
    pub name: String,
    pub protocol: Protocol,
    pub listen: String,
    pub target: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Tcp,
    Udp,
}

impl Config {
    pub fn tunnel_count(&self) -> usize {
        self.tunnels.len()
    }

    pub fn find(&self, name: &str) -> Option<&TunnelConfig> {
        self.tunnels.iter().find(|t| t.name == name)
    }
}

impl Protocol {
    pub fn is_tcp(&self) -> bool {
        matches!(self, Protocol::Tcp)
    }

    pub fn is_udp(&self) -> bool {
        matches!(self, Protocol::Udp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> Config {
        Config {
            tunnels: vec![
                TunnelConfig {
                    name: "web".to_string(),
                    protocol: Protocol::Tcp,
                    listen: "0.0.0.0:8080".to_string(),
                    target: "127.0.0.1:80".to_string(),
                },
                TunnelConfig {
                    name: "dns".to_string(),
                    protocol: Protocol::Udp,
                    listen: "0.0.0.0:5353".to_string(),
                    target: "8.8.8.8:53".to_string(),
                },
            ],
        }
    }

    #[test]
    fn test_tunnel_count() {
        let config = sample_config();
        assert_eq!(config.tunnel_count(), 2);
    }

    #[test]
    fn test_find_tunnel_found() {
        let config = sample_config();

        let tunnel = config.find("web");

        assert!(tunnel.is_some());
        assert_eq!(tunnel.unwrap().target, "127.0.0.1:80");
    }

    #[test]
    fn test_find_tunnel_not_found() {
        let config = sample_config();

        let tunnel = config.find("not-exists");

        assert!(tunnel.is_none());
    }
}

pub fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&content)?;
    Ok(config)
}
