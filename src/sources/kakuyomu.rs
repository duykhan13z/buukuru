use crate::model::NovelSource;



pub struct Kakuyomu {}

impl Kakuyomu {
    pub fn gen_info() -> NovelSource {
        NovelSource::new(
            // BaseURL
            String::from("https://kakuyomu.jp"),
            // novel_name
            String::from("h1.Heading_heading__lQ85n"), 
            // novel_author
            String::from(".Gap_size-3s__fjxCP > div:nth-child(2) > div:nth-child(1) > div:nth-child(1) > a:nth-child(1)"), 
            String::from("ja"), 
            String::from("div.Gap_size-2l__HWqrr:nth-child(2) > div:nth-child(2) > div:nth-child(1) > a:nth-child(1)"), 
            String::from("#contentMain-header-workTitle"), 
            String::from(".widget-episode-inner"), 
            String::from("#contentMain-readNextEpisode") 
        )
    }
}