use crate::model::NovelSource;

pub struct Syosetu {}

impl Syosetu {
    pub fn gen_info() -> NovelSource {
        NovelSource::new(
            // BaseURL
            String::from("https://ncode.syosetu.com"),
            // novel_name
            String::from(".p-novel__title"), 
            // novel_author
            String::from(".p-novel__author > a:nth-child(1)"), 
            String::from("ja"), 
            String::from("div.p-eplist__sublist:nth-child(1) > a:nth-child(1)"), 
            String::from(".p-novel__title"), 
            String::from(".js-novel-text"), 
            String::from("a.c-pager__item--next") 
        )
    }
}