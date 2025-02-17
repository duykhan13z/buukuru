use std::error::Error;

use scraper::{Html, Selector};

use crate::utils;

// Save selector
pub struct NovelSource {
    base_url: String,
    novel_name: String,
    novel_author: String,
    pub novel_language: String,
    first_chapter: String,
    chap_title: String,
    chap_content: String,
    next_chapter: String
}

pub struct NovelMetadata {
    pub novel_name: String,
    pub novel_author: String,
    pub first_chapter_url: String,
    pub novel_language: String
}

pub struct Chapter {
    pub title: String, 
	pub content: Vec<String>,
	pub next_chapter_url: String,
    pub any_chapter_left: bool
}

impl NovelSource {
    pub fn new(
        base_url: String,
        novel_name: String, 
        novel_author: String, 
        novel_language: String, 
        first_chapter: String, 
        chap_title: String, 
        chap_content: String, 
        next_chapter: String 
    ) -> Self {
        Self { 
            base_url,
            novel_name, 
            novel_author, 
            novel_language, 
            first_chapter, 
            chap_title, 
            chap_content, 
            next_chapter 
        }
    }

    pub fn fetch_metadata(&self, novel_url: &str) -> Result<NovelMetadata, Box<dyn Error>> {
        let body = utils::fetch_from_internet(&novel_url)?.body_mut().read_to_string()?;
        let document = Html::parse_document(&body);

        let title_selector = Selector::parse(&self.novel_name).unwrap();
        let author_selector = Selector::parse(&self.novel_author).unwrap();
        let first_chap_selector = Selector::parse(&self.first_chapter).unwrap();
        
        let novel_name: String = document.select(&title_selector).next().unwrap().text().collect();
        let novel_author: String = document.select(&author_selector).next().unwrap().text().collect();
        let first_chapter_url: String = document.select(&first_chap_selector).next().unwrap().value().attr("href").unwrap().to_string();
        
        Ok(NovelMetadata {
            novel_name,
            novel_author,
            first_chapter_url: format!("{}{}", &self.base_url, first_chapter_url),
            novel_language: String::from(&self.novel_language)
        })
    }

    pub fn download_current_chapter(&self, current_url: &str) -> Result<Chapter, Box<dyn Error>> {
        let body = utils::fetch_from_internet(&current_url)?.body_mut().read_to_string()?;
        let document = Html::parse_document(&body);
        let mut chap_lines: Vec<String> = Vec::new();  
        let mut next_chapter_url = String::from("");
        let mut any_chapter_left = true;

        let title_selector = Selector::parse(&self.chap_title).unwrap();
        let content_selector = Selector::parse(&self.chap_content).unwrap();
        let next_chap_selector = Selector::parse(&self.next_chapter).unwrap();
        let each_line = Selector::parse("p").unwrap();  

        let chap_name: String = document.select(&title_selector).next().unwrap().text().collect();
        println!("Downloading {}", &chap_name);

        match document.select(&next_chap_selector).next() {
            Some(chapter_url) => {
                next_chapter_url = chapter_url.value().attr("href").unwrap().to_string();
            },
            None => {
                any_chapter_left = false;
            },
        }
        
        ;  

        let chap_content = document.select(&content_selector).next().unwrap();
 
        for chap_line in chap_content.select(&each_line) {
            chap_lines.push(chap_line.text().collect::<String>());
        }

        Ok(Chapter { 
            title: chap_name, 
            content: chap_lines, 
            next_chapter_url: format!("{}{}", &self.base_url, next_chapter_url),
            any_chapter_left
        })
    }
}