use anyhow::{anyhow, Ok, Result};
use clap::{Parser, Subcommand};
use reqwest::Url;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    subcmd: Subcmd,
}

#[derive(Subcommand, Debug)]
enum Subcmd {
    Get(Get),
    Post(Post),
}

#[derive(Debug, Parser)]
struct Get {
    #[arg(value_parser = parse_url)]
    url: String,
}

#[derive(Debug, Parser)]
struct Post {
    #[arg(value_parser = parse_url)]
    url: String,
    #[arg(value_parser=parse_kv_pair)]
    body: Vec<String>,
}

// 校验url参数是否符合url规则
fn parse_url(s: &str) -> Result<String> {
    // 通过能否使用str.parse()强制转换成功转换成reqwest的Url类型，判断url字符串是否符合url规范
    let _url: Url = s.parse()?;
    Ok(s.into())
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, Clone)]
struct KvPair {
    k: String,
    v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
