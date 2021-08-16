#[macro_use]
extern crate lazy_static;

use chrono::{DateTime, FixedOffset};
use ramhorns::{Content, Template};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use walkdir::WalkDir;

mod error;

use crate::error::Error;

lazy_static! {
    static ref WHITESPACE_RE: Regex = Regex::new(r"\s+").unwrap();
    static ref UNDERSCORE_RE: Regex = Regex::new(r"_+").unwrap();
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PostHeader {
    title: String,
    author: String,
    date: String,
    tags: Option<Vec<String>>,
    summary: Option<String>,
}

#[derive(Content, Clone, Debug)]
struct Post {
    title: String,
    author: String,
    date: String,
    date_rfc822: String,
    date_iso8601: String,
    date_rfc3339: String,
    date_month_day_year: String,
    #[md]
    content: String,
    url: String,
    snake_url: String,
    domain: String,
    tags: Vec<Tag>,
    image: String,
    summary: Option<String>,
}

#[derive(Content, Clone, Debug)]
struct Tag {
    url: String,
    snake_url: String,
    tag: String,
    posts: Vec<Post>,
}

#[derive(Content, Clone, Debug)]
struct Rss {
    url: String,
    domain: String,
    posts: Vec<Post>,
}

#[derive(Content, Clone, Debug)]
struct Sitemap {
    posts: Vec<Post>,
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
struct Blog<'a> {
    title: &'a str,
    posts: Vec<Post>,
    tags: Vec<Tag>,
}

struct PostTemplate<'a>(Template<'a>);

impl<'a> PostTemplate<'a> {
    fn render(&self, context: &Post) -> String {
        self.0.render(context)
    }
}

struct IndexTemplate<'a>(Template<'a>);

impl<'a> IndexTemplate<'a> {
    fn render(&self, context: &Blog) -> String {
        self.0.render(context)
    }
}

struct RssTemplate<'a>(Template<'a>);

impl<'a> RssTemplate<'a> {
    fn render(&self, context: &Rss) -> String {
        self.0.render(context)
    }
}

struct SitemapTemplate<'a>(Template<'a>);

impl<'a> SitemapTemplate<'a> {
    fn render(&self, context: &Sitemap) -> String {
        self.0.render(context)
    }
}

struct TagsTemplate<'a>(Template<'a>);

impl<'a> TagsTemplate<'a> {
    fn render(&self, context: &Tag) -> String {
        self.0.render(context)
    }
}

fn parse_post<A>(path: A, domain: &str) -> Post
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

    let date_iso8601 = DateTime::parse_from_rfc3339(&header.date).expect("failed to parse date");
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

fn template(path: &str) -> Template {
    let source = std::fs::read_to_string(path).expect("could not read template");
    Template::new(source).unwrap()
}

fn create_deploy_dirs(deploy_prefix: &str, deploy_dirs: &[&str]) -> Result<(), Error> {
    deploy_dirs
        .iter()
        .map(|p| {
            let path = format!("{}/{}", deploy_prefix, p);
            let path = Path::new(&path);
            std::fs::create_dir_all(path).map_err(Error::IoError)
        })
        .collect()
}

fn render_posts(template_root: &str, deploy_prefix: &str, domain: &str) -> Vec<Post> {
    let post_template_path = &format!("{}/templates/post.html", template_root);
    let post_template = PostTemplate(template(post_template_path));
    let mut posts = WalkDir::new(&format!("{}/posts", template_root))
        .into_iter()
        .filter_map(|e| e.ok())
        .fold(vec![], |mut acc, entry| {
            let path = entry.path();
            if path.is_file() && path.extension() == Some(OsStr::new("md")) {
                let post = parse_post(path, domain);
                acc.push(post);
            }
            acc
        });
    posts.sort_by_key(|p| std::cmp::Reverse(p.date_iso8601.clone()));
    posts.iter().for_each(|post| {
        let rendered = post_template.render(&post);
        std::fs::write(format!("{}/{}", deploy_prefix, &post.url), &rendered)
            .expect("failed to write post to deploy");
        std::fs::write(format!("{}/{}", deploy_prefix, &post.snake_url), &rendered)
            .expect("failed to write post to deploy");
    });
    posts
}

fn render_tags(template_root: &str, deploy_prefix: &str, posts: &[Post]) -> BTreeSet<Tag> {
    let tags = posts.into_iter().fold(BTreeSet::new(), |mut acc, p| {
        for tag in &p.tags {
            acc.insert(tag.clone());
        }
        acc
    });
    let tags: BTreeSet<_> = tags
        .into_iter()
        .map(|tag| Tag {
            posts: posts
                .into_iter()
                .filter(|p| p.tags.iter().find(|x| x.tag == tag.tag).is_some())
                .cloned()
                .collect(),
            ..tag
        })
        .collect();
    let tags_template_path = format!("{}/templates/tags.html", template_root);
    let tags_template = TagsTemplate(template(&tags_template_path));
    for tag in tags.iter() {
        let rendered = tags_template.render(&tag);
        std::fs::write(format!("{}/{}", deploy_prefix, tag.url), &rendered)
            .expect("failed to write post to deploy");
        std::fs::write(format!("{}/{}", deploy_prefix, tag.snake_url), &rendered)
            .expect("failed to write post to deploy");
    }
    tags
}

fn render_index(
    template_root: &str,
    deploy_prefix: &str,
    title: &str,
    posts: &[Post],
    tags: &BTreeSet<Tag>,
) {
    let index_template_path = format!("{}/templates/index.html", template_root);
    let index_template = IndexTemplate(template(&index_template_path));
    let blog = Blog {
        title: title,
        posts: posts.to_vec(),
        tags: tags.clone().into_iter().collect(),
    };
    let rendered = index_template.render(&blog);
    std::fs::write(&format!("{}/index.html", deploy_prefix), rendered)
        .expect("failed to write post to deploy");
}

fn render_rss(template_root: &str, deploy_prefix: &str, domain: &str, posts: &[Post]) {
    let rss_template_path = format!("{}/templates/rss.xml", template_root);
    let rss_template = RssTemplate(template(&rss_template_path));
    let rss = Rss {
        domain: domain.to_string(),
        url: "/rss.xml".to_string(),
        posts: posts.to_vec(),
    };
    let rendered = rss_template.render(&rss);
    std::fs::write(&format!("{}/rss.xml", deploy_prefix), rendered)
        .expect("failed to write post to deploy");
}

fn render_sitemap(template_root: &str, deploy_prefix: &str, posts: &[Post]) {
    let sitemap_template_path = format!("{}/templates/sitemap.xml", template_root);
    let sitemap_template = SitemapTemplate(template(&sitemap_template_path));
    let sitemap = Sitemap {
        posts: posts.to_vec(),
    };
    let rendered = &sitemap_template.render(&sitemap);
    std::fs::write(&format!("{}/sitemap.xml", deploy_prefix), rendered)
        .expect("failed to write post to deploy");
}

fn main() {
    // TODO: render -> write split out.
    // TODO: Pre-render templates upfront?
    // TODO: generic version would not have unwraps here.
    let template_root = std::env::var("TEMPLATE_ROOT").unwrap_or("site".to_string());
    let deploy_prefix = std::env::var("DEPLOY_PREFIX").unwrap_or("deploy".to_string());
    let title = std::env::var("TITLE").unwrap_or("justanotherdot".to_string());
    let domain = std::env::var("DOMAIN").unwrap_or("https://justanotherdot.com".to_string());

    create_deploy_dirs(&deploy_prefix, &["posts", "tags", "assets"]).unwrap_or_else(|_| {
        eprintln!("could not create initial directories");
        std::process::exit(1);
    });

    let posts = render_posts(&template_root, &deploy_prefix, &domain);
    let tags = render_tags(&template_root, &deploy_prefix, &posts);
    render_index(&template_root, &deploy_prefix, &title, &posts, &tags);
    render_rss(&template_root, &deploy_prefix, &domain, &posts);
    render_sitemap(&template_root, &deploy_prefix, &posts);
}
