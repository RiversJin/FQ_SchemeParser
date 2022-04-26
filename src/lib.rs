pub mod ss_parse;
pub mod v2rayn_vmess_parse;

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
    use crate::v2rayn_vmess_parse;
    #[test]
    fn test_parse_vmess_config(){
        let str1 = "vmess://ewogICJ2IjogIjIiLAogICJwcyI6ICJJJ20gcmVtYXJrIiwKICAiYWRkIjogIndvcmRwcmVzcy5vcmciLAogICJwb3J0IjogIjQ0MyIsCiAgImlkIjogInh4eHgteHh4eC14eHh4LXh4eHgiLAogICJhaWQiOiAiMCIsCiAgInNjeSI6ICJub25lIiwKICAibmV0IjogInRjcCIsCiAgInR5cGUiOiAibm9uZSIsCiAgImhvc3QiOiAiIiwKICAicGF0aCI6ICIiLAogICJ0bHMiOiAiIiwKICAic25pIjogIiIsCiAgImFscG4iOiAiIgp9";
        let config = v2rayn_vmess_parse::parse_vmess_v2rayn_scheme(str1).expect("Parse Failed.");
        let expected_config = v2rayn_vmess_parse::VmessConfigV2raN{
            v: "2".to_string(),
            ps: "I'm remark".to_string(),
            add: "wordpress.org".to_string(),
            port: "443".to_string(),
            id: "xxxx-xxxx-xxxx-xxxx".to_string(),
            aid: "0".to_string(),
            scy: "none".to_string(),
            net: "tcp".to_string(),
            r#type: "none".to_string(),
            host: "".to_string(),
            path: "".to_string(),
            tls: "".to_string(),
            sni: "".to_string(),
            alpn: "".to_string()
        };
        assert_eq!(config,expected_config);
    }
}


