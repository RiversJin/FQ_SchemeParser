use serde::{Deserialize, Serialize};
use base64::decode;
use std::str;

/// vmess config from v2rayN 
/// see https://github.com/2dust/v2rayN/wiki/%E5%88%86%E4%BA%AB%E9%93%BE%E6%8E%A5%E6%A0%BC%E5%BC%8F%E8%AF%B4%E6%98%8E(ver-2)
#[derive(Serialize, Deserialize)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct VmessConfigV2raN {
    /// version 版本号
    pub v: String,
    /// alias 别名
    pub ps: String,
    /// address 主机地址
    pub add: String,

    pub port: String,
    /// UUID
    pub id: String,
    /// alterId
    pub aid: String,
    /// security method
    pub scy: String,
    /// Transport Protocol (like tcp/kcp/wl/h2/quic) 传输协议
    pub net: String,
    /// Camouflage type (like none/http/srtp/utp/wechat-video) 伪装类型
    pub r#type: String,
    /// Camouflage host 伪装域名
    pub host: String,
    pub path: String,
    /// whether to use tls
    pub tls: String,
    /// Server name
    pub sni: String,
    #[serde(default)]
    pub alpn: String
}
/// 解析v2rayN的vmess订阅格式 vmess://base64(config)
pub fn parse_vmess_v2rayn_scheme(uri: &str) -> Option<VmessConfigV2raN> {
    if !uri.starts_with("vmess://") {
        return None;
    }
    let content = 
        match decode(&uri[8..])
            .map(|buffer| String::from_utf8(buffer)) 
            {
        Ok(value) => match value {
            Ok(value)=>value,
            Err(_) => {
                return None;
            }
        },
        Err(_) => {
            return None;
        }
    };
    Some(serde_json::from_str::<VmessConfigV2raN>(&content).unwrap())
}