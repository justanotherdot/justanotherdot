use chrono::DateTime;
use chrono_tz::Tz;
use ramhorns::Content;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use walkdir::WalkDir;

use crate::template::*;

lazy_static! {
    static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
    static ref UNDERSCORE_RE: Regex = Regex::new(r"_+").unwrap();
}

pub enum Context<'a> {
    Posts(Vec<Post>),
    Tags(BTreeSet<Tag>),
    Index(Index<'a>),
    Rss(Rss),
    Sitemap(Sitemap),
}

impl<'a> Context<'a> {
    // TODO: render + write, rather than just render + write in same spot.
    pub fn render(&self, template_root: &str, deploy_prefix: &str) {
        use Context::*;
        match self {
            Posts(posts) => {
                let post_template_path = &format!("{}/templates/post.html", template_root);
                let post_template = PostTemplate(template(post_template_path));
                posts.iter().for_each(|post| {
                    let rendered = post_template.render(&post);
                    std::fs::write(format!("{}/{}", deploy_prefix, &post.url), &rendered)
                        .expect("failed to write post to deploy");
                    std::fs::write(format!("{}/{}", deploy_prefix, &post.snake_url), &rendered)
                        .expect("failed to write post to deploy");
                });
            }
            Tags(tags) => {
                let tags_template_path = format!("{}/templates/tags.html", template_root);
                let tags_template = TagsTemplate(template(&tags_template_path));
                for tag in tags.iter() {
                    let rendered = tags_template.render(&tag);
                    std::fs::write(format!("{}/{}", deploy_prefix, tag.url), &rendered)
                        .expect("failed to write post to deploy");
                    std::fs::write(format!("{}/{}", deploy_prefix, tag.snake_url), &rendered)
                        .expect("failed to write post to deploy");
                }
            }
            Index(index) => {
                let index_template_path = format!("{}/templates/index.html", template_root);
                let index_template = IndexTemplate(template(&index_template_path));
                let rendered = index_template.render(&index);
                std::fs::write(&format!("{}/index.html", deploy_prefix), rendered)
                    .expect("failed to write post to deploy");
            }
            Rss(rss) => {
                let rss_template_path = format!("{}/templates/rss.xml", template_root);
                let rss_template = RssTemplate(template(&rss_template_path));
                let rendered = rss_template.render(&rss);
                std::fs::write(&format!("{}/rss.xml", deploy_prefix), rendered)
                    .expect("failed to write post to deploy");
            }
            Sitemap(sitemap) => {
                let sitemap_template_path = format!("{}/templates/sitemap.xml", template_root);
                let sitemap_template = SitemapTemplate(template(&sitemap_template_path));
                let rendered = &sitemap_template.render(&sitemap);
                std::fs::write(&format!("{}/sitemap.xml", deploy_prefix), rendered)
                    .expect("failed to write post to deploy");
            }
        }
    }
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
    //pub image: String,
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
        let date_shifted = date_iso8601.with_timezone(&Tz::Australia__Sydney);
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
        //let image = path
        //    .file_name()
        //    .unwrap()
        //    .to_str()
        //    .unwrap()
        //    .replace(".md", ".jpg")
        //    .to_lowercase();
        //assert!(
        //    std::path::Path::new(&format!("deploy/assets/images/{}", image)).exists(),
        //    "could not find image: {}",
        //    image
        //);
        let snake_url = format!("/posts/{}", UNDERSCORE_RE.replace_all(&url, r"-"));
        //let snake_image = UNDERSCORE_RE.replace_all(&image, r"-").to_string();
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
            //image: snake_image,
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
pub struct Index<'a> {
    pub title: &'a str,
    pub posts: Vec<Post>,
    pub tags: Vec<Tag>,
}

pub fn list_posts(template_root: &str, domain: &str) -> Vec<Post> {
    let mut posts = WalkDir::new(&format!("{}/posts", template_root))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() && path.extension() == Some(OsStr::new("md")) {
                Some(Post::parse(path, domain))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    posts.sort_by_key(|p| std::cmp::Reverse(p.date_iso8601.clone()));
    posts
}

pub fn list_tags(posts: &[Post]) -> BTreeSet<Tag> {
    let tags = posts.into_iter().fold(BTreeSet::new(), |mut acc, p| {
        for tag in &p.tags {
            acc.insert(tag.clone());
        }
        acc
    });
    tags.into_iter()
        .map(|tag| Tag {
            posts: posts
                .into_iter()
                .filter(|p| p.tags.iter().find(|x| x.tag == tag.tag).is_some())
                .cloned()
                .collect(),
            ..tag
        })
        .collect()
}