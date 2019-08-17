extern crate chrono;
extern crate ramhorns;
extern crate serde;
extern crate walkdir;

use chrono::{DateTime, FixedOffset};
use ramhorns::{Content, Template};
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

static BLOG_TITLE: &str = "justanotherdot";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct PostHeader {
    title: String,
    author: String,
    date: String,
    tags: Option<Vec<String>>,
}

// TODO: Drop clone.
#[derive(Content, Clone, Debug)]
struct Post<'a> {
    title: &'a str,
    author: &'a str,
    date: &'a str,
    #[md]
    content: &'a str,
    url: &'a str,
    tags: Vec<Tag>,
}

// TODO: Drop clone.
#[derive(Content, Clone, Debug)]
struct Tag {
    url: String,
    tag: String,
}

// TODO: Drop clone.
#[derive(Content, Clone, Debug)]
struct Blog<'a> {
    title: &'a str,
    posts: Vec<Post<'a>>,
    tags: Vec<Tag>,
}

#[allow(dead_code)]
fn parse_post_header(_path: &str) -> PostHeader {
    unimplemented!()
}
// fn format_date_to_sydney_timezone

// fn parse_post() -> Post

// TODO: should take Post and PostHeader
fn render_post<A>(path: A, tpl: &Template) -> String
where
    A: AsRef<Path>,
{
    let path = path.as_ref();
    let path_str = path.to_str().unwrap();
    let markdown = std::fs::read_to_string(path_str).unwrap_or_else(|_| {
        eprintln!("could not read post");
        std::process::exit(1);
    });

    let markdown_raw = markdown.split("---").collect::<Vec<&str>>();
    let markdown = markdown_raw.get(2).unwrap();

    let header_raw = markdown_raw.get(1).unwrap();
    let header: PostHeader = serde_yaml::from_str(&header_raw).expect("could not parse header");

    let tags = match header.tags {
        None => vec![],
        Some(tags) => tags
            .into_iter()
            .map(|tag| Tag {
                url: format!("/tags/{}", tag),
                tag: tag,
            })
            .collect(),
    };

    let date = DateTime::parse_from_rfc3339(&header.date).expect("failed to parse date");
    let date = date.with_timezone(&FixedOffset::east(10 * 3600));
    let date = date.format("%B %e %Y, %_I:%M%p").to_string();

    let url_str = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".md", ".html")
        .to_lowercase();

    let post = Post {
        title: &header.title,
        author: &header.author,
        date: &date,
        url: &url_str,
        content: &markdown,
        tags: tags.clone(),
    };
    let posts = vec![post.clone()];
    let _blog = Blog {
        title: BLOG_TITLE,
        posts: posts.clone(),
        tags: tags.clone(),
    };

    let rendered = tpl.render(&post);
    rendered
}

fn main() {
    // TODO: Sort posts.
    // TODO: Pre-render templates upfront?
    // TODO: Port all dates on posts as ISO8601, format differently post-parsing.
    // TODO: Pin version of bulma and embed.
    let source = std::fs::read_to_string("../site/templates/post.html").unwrap_or_else(|_| {
        eprintln!("could not read template");
        std::process::exit(1);
    });
    let tpl = Template::new(source).unwrap();

    for entry in WalkDir::new("../site/posts")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        println!("{}", entry.path().display());
        let path = entry.path();
        // TODO: Ensure we have markdown and only markdown (`.md`)
        if path.is_file() {
            render_post(path, &tpl);
        }
    }
}
