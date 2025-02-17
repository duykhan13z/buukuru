use epub_builder::EpubBuilder;
use epub_builder::EpubContent;
use epub_builder::EpubVersion;
use epub_builder::ReferenceType;
use epub_builder::ZipLibrary;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::path::Path;
use crate::model::NovelMetadata;
use crate::model::NovelSource;

pub fn gen_epub(novel: NovelSource, novel_metadata: NovelMetadata) -> Result<(), Box<dyn Error>> {
    let file_name = &gen_filename();

    let mut epub_builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();
    epub_builder.metadata("title", &novel_metadata.novel_name)?;
    epub_builder.metadata("author", &novel_metadata.novel_author)?;
    epub_builder.metadata("lang", &novel_metadata.novel_language)?;
    epub_builder.epub_version(EpubVersion::V30);

    let mut any_chap_left = Some(true);
    let mut current_url = novel_metadata.first_chapter_url;
    let mut current_chap_number = 0;

    while let Some(_i) = any_chap_left {
        let current_chapter = novel.download_current_chapter(&current_url)?;
        if current_chapter.any_chapter_left {
            epub_builder
            .add_content(
                EpubContent::new(
                    format!("{}.html", current_chap_number),
                    compose_html(&current_chapter.content, 
                        &current_chapter.title, 
                        &novel_metadata.novel_language).as_bytes(),
                )
                .title(&current_chapter.title)
                .reftype(ReferenceType::Text),
            )?;
            current_url = current_chapter.next_chapter_url;
            current_chap_number += 1;
        } else {
            any_chap_left = None;
        }
    }

    epub_builder.inline_toc();
    // Write EPUB to file
    let mut epub_file = generate_empty_epub_file(file_name)?;
    epub_builder.generate(&mut epub_file)?;
    Ok(())
}

pub fn compose_html(html_input: &Vec<String>, title: &str, lang: &str) -> String {
    format!(
        r##"<?xml version='1.0' encoding='utf-8'?>
<html xmlns:epub="http://www.idpf.org/2007/ops" xmlns="http://www.w3.org/1999/xhtml" xml:lang="{}" lang="{}">
    <head>
        <title>{}</title>
        <meta http-equiv="Content-Type" content="text/html; charset=utf-8"/>
    </head>
    <body>
        <h2>{}</h2>
        {}
    {}"##,
        lang, lang, title, title, gen_content_html(html_input), "\n\t</body>\n</html>"
    )
}

pub fn gen_content_html(html_input: &Vec<String>) -> String {
    html_input.iter().map(|line| format!("<p>{}</p>", line)).collect::<String>()
}

fn generate_empty_epub_file(file_name: &str) -> Result<File, std::io::Error> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{}.epub", file_name))
}

fn gen_filename() -> String {
    let mut state = Some(0);
    let mut file_name = String::new();

    while let Some(i) = state {
        if Path::new(&format!("Novel{}.epub", i)).exists() {
            println!("Novel{i}.epub exists, generating a new filename");

            state = Some(i + 1);
        } else {
            file_name = format!("Novel{}", i);
            state = None;
        }
    }
    file_name
}