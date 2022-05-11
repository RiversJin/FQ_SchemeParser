use std::{str::from_utf8, error::Error};
use base64::decode;
use reqwest::{self};
pub mod ss_parse;
pub mod v2rayn_vmess_parse;

#[tokio::main]
async fn main() -> Result<(),Box<dyn Error>>{
    let url = "https://rss.6yearsok.xyz/link/DuDQmlLdgAKcadeo?mu=2";
    let res =reqwest::get(url).await?;
    let bytes = res.bytes().await?;
    let body = decode(&bytes)?;
    let body = from_utf8(&body)?;
    for (i, s) in body.split('\n').enumerate(){
        print!("\n{}: \n",i);
        if let Some(config) = ss_parse::parse_shadowsocks_scheme(s){
            println!("{:?}",config);
        }else if let Some(config) = v2rayn_vmess_parse::parse_vmess_v2rayn_scheme(s){
            print!("{:?}",config)
        }else {
            print!("{}",s);
        }
    }
    Ok(())
}