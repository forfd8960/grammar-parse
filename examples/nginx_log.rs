#[derive(Debug)]
struct Nginxlog {
    pub remote_addr: String,
    pub remote_user: String,
    pub time_local: String,
    pub request: String,
    pub status: String,
    pub body_bytes_sent: String,
    pub http_referer: String,
    pub http_user_agent: String,
}

fn main() -> anyhow::Result<()> {
    Ok(())
}
