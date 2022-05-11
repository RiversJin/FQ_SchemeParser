use base64::decode;
use serde::de;
use serde::{Deserialize, Deserializer, Serialize};
use std::str;

/// vmess config from v2rayN
/// see https://github.com/2dust/v2rayN/wiki/%E5%88%86%E4%BA%AB%E9%93%BE%E6%8E%A5%E6%A0%BC%E5%BC%8F%E8%AF%B4%E6%98%8E(ver-2)
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct VmessConfigV2raN {
    /// version 版本号
    pub v: String,
    /// alias 别名
    pub ps: String,
    /// address 主机地址
    pub add: String,
    #[serde(deserialize_with = "deserialize_u64_to_string")]
    pub port: String,
    /// UUID
    pub id: String,
    #[serde(deserialize_with = "deserialize_u64_to_string")]
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
    pub alpn: String,
}
/// 解析v2rayN的vmess订阅格式 vmess://base64(config)
pub fn parse_vmess_v2rayn_scheme(uri: &str) -> Option<VmessConfigV2raN> {
    if !uri.starts_with("vmess://") {
        return None;
    }
    let content = String::from_utf8(decode(&uri[8..]).ok()?).ok()?;
    print!("{}", content);
    serde_json::from_str::<VmessConfigV2raN>(&content).ok()
}

struct DeU64OrStringVisitor;
impl<'de> de::Visitor<'de> for DeU64OrStringVisitor {
    type Value = String;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("u64 or a string of an u64 value")
    }
    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(v.to_string())
    }
}

fn deserialize_u64_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(DeU64OrStringVisitor)
}
