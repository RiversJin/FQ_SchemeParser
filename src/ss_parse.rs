use lazy_static::lazy_static;
use regex::Regex;
use std::str;
use base64::decode;
use urlencoding::decode as url_decode;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct ShadowsocksConfig {
    pub userinfo: String,
    pub hostname: String,
    pub port: u16,
    pub tag: Option<String>,
    pub plugin: Option<String>
}


lazy_static! {
    static ref PATTERN: Regex = Regex::new(r"ss://(?P<userinfo>[\da-zA-Z]+={0,2})@(?P<hostname>[\.a-zA-Z\d]+):(?P<port>\d{1,5})/?(\?plugin=(?P<plugin>[\w\-%]+))?(#(?P<tag>.+))?").unwrap();
}
///
/// Parse Shadowsocks uri
///
pub fn parse_shadowsocks_scheme(uri: &str) -> Option<ShadowsocksConfig> {
    //let Pattern: Regex = Regex::new(r"ss:\/\/(?<userinfo>[\da-zA-Z]+={0,2})@(?<hostname>[\.a-zA-Z\d]+):(?<port>\d{1,5})\/?(\?plugin=(?<plugin>[\w\-%]+))?(#(?<tag>.+))?").unwrap();
    let cap = PATTERN.captures(uri)?;
    let userinfo = cap.name("userinfo")?.as_str().to_string();
    let userinfo = match decode(userinfo).map(|buffer| str::from_utf8(&buffer[..]).unwrap_or("").to_string()) {
        Ok(value) => value,
        Err(_) => return None
    };
    let hostname = cap.name("hostname")?.as_str().to_string();
    let port: u16 = match cap.name("port")?.as_str().parse::<u16>() {
        Ok(value) => value,
        Err(_) => {
            return None
        }
    };
    let tag: Option<String> = cap.name("tag").map(|t| t.as_str().to_string());
    let plugin: Option<String> = cap.name("plugin").map(|p| url_decode(p.as_str()).unwrap_or_default().into_owned().to_string());

    Some(ShadowsocksConfig{
        userinfo,
        hostname,
        port,
        tag,
        plugin
    })
}