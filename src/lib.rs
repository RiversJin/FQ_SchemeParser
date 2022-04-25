pub mod ss_parse;

#[cfg(test)]
mod tests {
    use crate::ss_parse;
    
    #[test]
    fn test_ss(){
        let str1 = "ss://YWVzLTEyOC1nY206dGVzdA==@192.168.100.1:8888#Example1";
        let config = ss_parse::parse_shadowsocks_scheme(str1).unwrap();
        let expected_config = ss_parse::ShadowsocksConfig{
            userinfo: "aes-128-gcm:test".to_string(),
            hostname: "192.168.100.1".to_string(),
            port: 8888,
            tag: Some("Example1".to_string()),
            plugin: None
        };
        assert_eq!(config,expected_config);
        let str2 = "ss://cmM0LW1kNTpwYXNzd2Q=@192.168.100.1:8888/?plugin=obfs-local%3Bobfs%3Dhttp#Example2";
        let config = ss_parse::parse_shadowsocks_scheme(str2).unwrap();
        let expected_config = ss_parse::ShadowsocksConfig{
            userinfo: "rc4-md5:passwd".to_string(),
            hostname: "192.168.100.1".to_string(),
            port: 8888,
            tag: Some("Example2".to_string()),
            plugin: Some("obfs-local;obfs=http".to_string())
        };
        assert_eq!(config,expected_config);
    }
}


