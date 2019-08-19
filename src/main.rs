extern crate chrono;
extern crate ramhorns;
extern crate serde;
extern crate walkdir;

use chrono::{DateTime, FixedOffset};
use ramhorns::{Content, Template};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use walkdir::WalkDir;

// TODO: Take by env var, provide default maybe. (can do this with clap).
static JUSTANOTHERDOT_TITLE: &str = "justanotherdot";
static JUSTANOTHERDOT_DOMAIN: &str = "https://justanotherdot.com";
// TODO replace hardcoded places.
static JUSTANOTHERDOT_DEPLOY_PREFIX: &str = "deploy";
// TODO replace hardcoded places.
static JUSTANOTHERDOT_TEMPLATE_ROOT: &str = "site";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PostHeader {
    title: String,
    author: String,
    date: String,
    tags: Option<Vec<String>>,
}

#[derive(Content, Clone, Debug)]
struct Post {
    title: String,
    author: String,
    date: String,
    date_rfc822: String,
    date_iso8601: String,
    date_month_day_year: String,
    #[md]
    content: String,
    url: String,
    domain: String,
    tags: Vec<Tag>,
}

#[derive(Content, Clone, Debug)]
struct Tag {
    url: String,
    tag: String,
    posts: Vec<Post>,
}

// TODO: Drop clone.
#[derive(Content, Clone, Debug)]
struct Rss {
    url: String,
    domain: String,
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

// TODO: Drop clone.
#[derive(Content, Clone, Debug)]
struct Blog<'a> {
    title: &'a str,
    posts: Vec<Post>,
    tags: Vec<Tag>,
}

struct PostTemplate<'a>(Template<'a>);
struct IndexTemplate<'a>(Template<'a>);
struct RssTemplate<'a>(Template<'a>);
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
                url: format!("/tags/{}.html", tag),
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
    let date_iso8601 = date_iso8601.to_string();

    let url = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".md", ".html")
        .to_lowercase();
    let url = format!("/posts/{}", url);

    Post {
        title: header.title,
        author: header.author,
        date,
        date_rfc822,
        date_iso8601,
        date_month_day_year,
        url,
        domain: JUSTANOTHERDOT_DOMAIN.to_string(),
        content: markdown.to_string(),
        tags: tags.clone(),
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

fn render_index(blog: &Blog, tpl: &IndexTemplate) -> String {
    tpl.0.render(blog)
}

fn template(path: &str) -> Template {
    let source = std::fs::read_to_string(path).expect("could not read template");
    Template::new(source).unwrap()
}

fn create_deploy_dirs() -> Result<Vec<()>, std::io::Error> {
    ["posts", "tags", "assets"]
        .into_iter()
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
    let tpl = PostTemplate(template("site/templates/post.html"));

    let mut posts = vec![];
    for entry in WalkDir::new("site/posts")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() && path.extension() == Some(OsStr::new("md")) {
            let post = parse_post(path);
            posts.push(post);
        }
    }
    posts.sort_by_key(|p| p.date.clone());

    for post in posts.iter() {
        let rendered = render_post(&post, &tpl);
        std::fs::write(
            format!("{}/{}", JUSTANOTHERDOT_DEPLOY_PREFIX, &post.url),
            rendered,
        )
        .expect("failed to write post to deploy");
    }

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
        .collect();;

    let tpl = IndexTemplate(template("site/templates/index.html"));
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

    let tpl = TagsTemplate(template("site/templates/tags.html"));
    for tag in tags.iter() {
        let rendered = render_tag(&tag, &tpl);
        std::fs::write(
            format!("{}/{}", JUSTANOTHERDOT_DEPLOY_PREFIX, tag.url),
            rendered,
        )
        .expect("failed to write post to deploy");
    }

    let tpl = RssTemplate(template("site/templates/rss.xml"));
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

    // TODO: Brand
}
