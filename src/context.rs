use chrono::{DateTime, FixedOffset};
use ramhorns::Content;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::Path;

lazy_static! {
    static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
    static ref UNDERSCORE_RE: Regex = Regex::new(r"_+").unwrap();
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PostHeader {
    pub title: String,
    pub author: String,
    pub date: String,
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
}

#[derive(Content, Clone, Debug)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub date: String,
    pub date_rfc822: String,
    pub date_iso8601: String,
    pub date_rfc3339: String,
    pub date_month_day_year: String,
    #[md]
    pub content: String,
    pub url: String,
    pub snake_url: String,
    pub domain: String,
    pub tags: Vec<Tag>,
    pub image: String,
    pub summary: Option<String>,
}

impl Post {
    pub fn parse<A>(path: A, domain: &str) -> Post
    where
        A: AsRef<Path>,
    {
        let path = path.as_ref();
        let path_str = path.to_str().unwrap();
        let markdown = std::fs::read_to_string(path_str).expect("could not read post");

        let markdown_raw = markdown.split("---").collect::<Vec<&str>>();
        let markdown = &markdown_raw[2..].join("");

        let header_raw = markdown_raw.get(1).unwrap();
        let header: PostHeader = serde_yaml::from_str(&header_raw).expect("could not parse header");

        let tags = match header.tags {
            None => vec![],
            Some(tags) => tags
                .into_iter()
                .map(|tag| Tag {
                    url: format!("/tags/{}.html", WHITESPACE_RE.replace_all(&tag, r"-")),
                    snake_url: format!("/tags/{}.html", WHITESPACE_RE.replace_all(&tag, r"_")),
                    tag: tag,
                    posts: vec![],
                })
                .collect(),
        };

        let date_iso8601 =
            DateTime::parse_from_rfc3339(&header.date).expect("failed to parse date");
        let date_shifted = date_iso8601.with_timezone(&FixedOffset::east(10 * 3600));
        let date = date_shifted.format("%B %e %Y, %_I:%M%p").to_string();
        let date_month_day_year = date_shifted.format("%D").to_string();
        let date_rfc822 = date_shifted.format("%a, %d %b %Y %T %z").to_string();
        let date_rfc3339 = &date_iso8601.to_rfc3339();
        let date_rfc3339 = date_rfc3339.to_string();
        let date_iso8601 = date_iso8601.to_string();

        let url = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".md", ".html")
            .to_lowercase();
        let image = path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".md", ".jpg")
            .to_lowercase();
        assert!(
            std::path::Path::new(&format!("deploy/assets/images/{}", image)).exists(),
            "could not find image: {}",
            image
        );
        let snake_url = format!("/posts/{}", UNDERSCORE_RE.replace_all(&url, r"-"));
        let snake_image = UNDERSCORE_RE.replace_all(&image, r"-").to_string();
        let url = format!("/posts/{}", url);

        Post {
            title: header.title,
            author: header.author,
            date,
            date_rfc822,
            date_iso8601,
            date_rfc3339,
            date_month_day_year,
            url,
            snake_url,
            domain: domain.to_string(),
            content: markdown.to_string(),
            tags: tags.clone(),
            image: snake_image,
            summary: header.summary,
        }
    }
}

#[derive(Content, Clone, Debug)]
pub struct Tag {
    pub url: String,
    pub snake_url: String,
    pub tag: String,
    pub posts: Vec<Post>,
}

#[derive(Content, Clone, Debug)]
pub struct Rss {
    pub url: String,
    pub domain: String,
    pub posts: Vec<Post>,
}

#[derive(Content, Clone, Debug)]
pub struct Sitemap {
    pub posts: Vec<Post>,
}

impl PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl PartialOrd for Tag {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.tag.partial_cmp(&other.tag)
    }
}

impl Eq for Tag {}

impl Ord for Tag {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.tag.cmp(&other.tag)
    }
}

#[derive(Content, Clone, Debug)]
pub struct Blog<'a> {
    pub title: &'a str,
    pub posts: Vec<Post>,
    pub tags: Vec<Tag>,
}
