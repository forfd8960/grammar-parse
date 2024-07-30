use anyhow::anyhow;
use regex::Regex;

#[allow(unused)]
#[derive(Debug)]
struct Nginxlog {
    pub addr: String,
    pub date_time: String,
    pub method: String,
    pub url: String,
    pub protocol: String,
    pub status: u16,
    pub body_bytes: u64,
    pub referer: String,
    pub user_agent: String,
}

fn main() -> anyhow::Result<()> {
    let log = r#"93.180.71.3 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"#;
    let nginx_log = parse_nginx_log(log)?;
    println!("{:?}", nginx_log);
    Ok(())
}

fn parse_nginx_log(log: &str) -> anyhow::Result<Nginxlog> {
    // log example:
    // 93.180.71.3 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)
    let re = Regex::new(
        r#"^(?P<ip>\S+) - - $$(?P<date>\d{2}/\w{3}/\d{4}:\d{2}:\d{2}:\d{2} [+-]\d{4})$$ "(?P<method>\S+) (?P<path>\S+) (?P<protocol>\S+)" (?P<status>\d+) (?P<size>\d+) "(?P<referer>[^"]*)" "(?P<user_agent>[^"]*)""#,
    )?;

    let cap = re.captures(log).ok_or(anyhow!("parse error"))?;

    let addr = cap.name("addr").unwrap().as_str().to_string();
    let date_time = cap.name("date").unwrap().as_str().to_string();
    let method = cap.name("method").unwrap().as_str().to_string();
    let url = cap.name("path").unwrap().as_str().to_string();
    let protocol = cap.name("protocol").unwrap().as_str().to_string();
    let status = cap.name("status").unwrap().as_str().parse()?;
    let body_bytes = cap.name("size").unwrap().as_str().parse()?;
    let referer = cap.name("referer").unwrap().as_str().to_string();
    let user_agent = cap.name("user_agent").unwrap().as_str().to_string();

    Ok(Nginxlog {
        addr,
        date_time,
        method,
        url,
        protocol,
        status,
        body_bytes,
        referer,
        user_agent,
    })
}
