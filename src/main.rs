extern crate chrono;
extern crate ramhorns;
extern crate regex;
extern crate serde;
extern crate walkdir;
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

const JUSTANOTHERDOT_TITLE: &'static str = "justanotherdot";
const JUSTANOTHERDOT_DOMAIN: &'static str = "https://justanotherdot.com";
const JUSTANOTHERDOT_DEPLOY_PREFIX: &'static str = "deploy";
const JUSTANOTHERDOT_TEMPLATE_ROOT: &'static str = "site";

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
    hero_font_color: Option<String>,
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
    hero_font_color: String,
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
struct IndexTemplate<'a>(Template<'a>);
struct RssTemplate<'a>(Template<'a>);
struct SitemapTemplate<'a>(Template<'a>);
struct TagsTemplate<'a>(Template<'a>);

fn parse_post<A>(path: A) -> Post
where
    A: AsRef<Path>,
{
    let path = path.as_ref();
    let path_str = path.to_str().unwrap();
    let markdown = std::fs::read_to_string(path_str).expect("could not read post");

    let markdown_raw = markdown.split("---").collect::<Vec<&str>>();
    let markdown = markdown_raw.get(2).unwrap();

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
        format!("could not find image: {}", image),
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
        domain: JUSTANOTHERDOT_DOMAIN.to_string(),
        content: markdown.to_string(),
        tags: tags.clone(),
        image: snake_image,
        summary: header.summary,
        hero_font_color: header.hero_font_color.unwrap_or("white-bis".to_string()),
    }
}

fn render_post(post: &Post, tpl: &PostTemplate) -> String {
    tpl.0.render(post)
}

fn render_tag(tags: &Tag, tpl: &TagsTemplate) -> String {
    tpl.0.render(tags)
}

fn render_rss(rss: &Rss, tpl: &RssTemplate) -> String {
    tpl.0.render(rss)
}

fn render_sitemap(rss: &Sitemap, tpl: &SitemapTemplate) -> String {
    tpl.0.render(rss)
}

fn render_index(blog: &Blog, tpl: &IndexTemplate) -> String {
    tpl.0.render(blog)
}

fn template(path: &str) -> Template {
    let source = std::fs::read_to_string(path).expect("could not read template");
    Template::new(source).unwrap()
}

fn create_deploy_dirs() -> Result<Vec<()>, std::io::Error> {
    ["posts", "tags", "assets"]
        .iter()
        .map(|p| {
            let path = format!("{}/{}", JUSTANOTHERDOT_DEPLOY_PREFIX, p);
            let path = Path::new(&path);
            std::fs::create_dir_all(path)
        })
        .collect()
}

fn main() {
    create_deploy_dirs().unwrap_or_else(|_| {
        eprintln!("could not create initial directories");
        std::process::exit(1);
    });

    // TODO: Pre-render templates upfront?
    // TODO: Pin version of bulma and embed.
    let tpl = &format!("{}/templates/post.html", JUSTANOTHERDOT_TEMPLATE_ROOT);
    let tpl = PostTemplate(template(tpl));

    let mut posts = WalkDir::new(&format!("{}/posts", JUSTANOTHERDOT_TEMPLATE_ROOT))
        .into_iter()
        .filter_map(|e| e.ok())
        .fold(vec![], |mut acc, entry| {
            let path = entry.path();
            if path.is_file() && path.extension() == Some(OsStr::new("md")) {
                let post = parse_post(path);
                acc.push(post);
            }
            acc
        });
    posts.sort_by_key(|p| std::cmp::Reverse(p.date_iso8601.clone()));

    posts.iter().for_each(|post| {
        let rendered = render_post(&post, &tpl);
        std::fs::write(
            format!("{}/{}", JUSTANOTHERDOT_DEPLOY_PREFIX, &post.url),
            &rendered,
        )
        .expect("failed to write post to deploy");
        std::fs::write(
            format!("{}/{}", JUSTANOTHERDOT_DEPLOY_PREFIX, &post.snake_url),
            &rendered,
        )
        .expect("failed to write post to deploy");
    });

    let tags = posts
        .clone()
        .into_iter()
        .fold(BTreeSet::new(), |mut acc, p| {
            for tag in p.tags {
                acc.insert(tag);
            }
            acc
        });
    let tags: BTreeSet<_> = tags
        .into_iter()
        .map(|tag| Tag {
            posts: posts
                .clone()
                .into_iter()
                .filter(|p| p.tags.iter().find(|x| x.tag == tag.tag).is_some())
                .collect(),
            ..tag
        })
        .collect();

    let tpl = format!("{}/templates/index.html", JUSTANOTHERDOT_TEMPLATE_ROOT);
    let tpl = IndexTemplate(template(&tpl));
    let blog = Blog {
        title: JUSTANOTHERDOT_TITLE,
        posts: posts.clone(),
        tags: tags.clone().into_iter().collect(),
    };
    let rendered = render_index(&blog, &tpl);
    std::fs::write(
        &format!("{}/index.html", JUSTANOTHERDOT_DEPLOY_PREFIX),
        rendered,
    )
    .expect("failed to write post to deploy");

    let tpl = format!("{}/templates/tags.html", JUSTANOTHERDOT_TEMPLATE_ROOT);
    let tpl = TagsTemplate(template(&tpl));
    for tag in tags.iter() {
        let rendered = render_tag(&tag, &tpl);
        std::fs::write(
            format!("{}/{}", JUSTANOTHERDOT_DEPLOY_PREFIX, tag.url),
            &rendered,
        )
        .expect("failed to write post to deploy");
        std::fs::write(
            format!("{}/{}", JUSTANOTHERDOT_DEPLOY_PREFIX, tag.snake_url),
            &rendered,
        )
        .expect("failed to write post to deploy");
    }

    let tpl = format!("{}/templates/rss.xml", JUSTANOTHERDOT_TEMPLATE_ROOT);
    let tpl = RssTemplate(template(&tpl));
    let rss = Rss {
        domain: JUSTANOTHERDOT_DOMAIN.to_string(),
        url: "/rss.xml".to_string(),
        posts: posts.clone(),
    };
    let rendered = render_rss(&rss, &tpl);
    std::fs::write(
        &format!("{}/rss.xml", JUSTANOTHERDOT_DEPLOY_PREFIX),
        rendered,
    )
    .expect("failed to write post to deploy");

    let tpl = format!("{}/templates/sitemap.xml", JUSTANOTHERDOT_TEMPLATE_ROOT);
    let tpl = SitemapTemplate(template(&tpl));
    let sitemap = Sitemap {
        posts: posts.clone(),
    };
    let rendered = render_sitemap(&sitemap, &tpl);
    std::fs::write(
        &format!("{}/sitemap.xml", JUSTANOTHERDOT_DEPLOY_PREFIX),
        rendered,
    )
    .expect("failed to write post to deploy");
}
