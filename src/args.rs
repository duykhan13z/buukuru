use url::Url;

use crate::epub::gen_epub;
use crate::model::NovelSource;
use crate::sources::kakuyomu::Kakuyomu;
use crate::sources::syosetu::Syosetu;
use std::env;
use std::error::Error;

static HELP_MSG: &str = r##"Buukuru
Downloading Japanese web novels as .epub files.

Usage:
    buukuru get <url>
    buukuru get https://ncode.syosetu.com/n6755gk/
Options:
    -h --help       Show this screen.
    -v --version    Show version.
    get             Download the webnovel as an .epub file"##;

pub struct Config {
    pub command: String,
    pub novel_url: String
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        // Including get-file and get-link
        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("Bạn không nhập lệnh gì cả, tự động thoát đây..."),
        };

        // Depended on "command"
        let novel_url = match args.next() {
            Some(arg) => arg,
            None => String::from("..."),
        };

        Ok(Config {
            command,
            novel_url
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let novel_source = match_url(&config.novel_url)?;
    let novel_metadata = novel_source.fetch_metadata(&config.novel_url)?;
    match config.command.as_str() {
        "get" => {
            gen_epub(novel_source, novel_metadata)?;
        },
        "help" => {
            println!("{}", HELP_MSG);
        },
        _ => {
            println!("{}", HELP_MSG)
        }
    }

    Ok(())
}

pub fn match_url(url: &str) -> Result<NovelSource, Box<dyn Error>> {
    match Url::parse(url)?.host_str().unwrap() {
        "kakuyomu.jp" => {
            Ok(Kakuyomu::gen_info())
        },
        "ncode.syosetu.com" | "syosetu.com" => {
            Ok(Syosetu::gen_info())
        },
        _ => {
            Err("No way to be found".into())
        }
    }
}