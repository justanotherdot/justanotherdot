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
struct Post {
    title: String,
    author: String,
    date: String,
    #[md]
    content: String,
    url: String,
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
    posts: Vec<Post>,
    tags: Vec<Tag>,
}

fn parse_post<A>(path: A) -> Post
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

    Post {
        title: header.title,
        author: header.author,
        date: date,
        url: url_str,
        content: markdown.to_string(),
        tags: tags.clone(),
    }
}

// TODO: should take Post and PostHeader
fn render_post(post: &Post, tpl: &Template) -> String {
    let rendered = tpl.render(post);
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

    let mut posts = vec![];
    for entry in WalkDir::new("../site/posts")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        //println!("{}", entry.path().display());
        let path = entry.path();
        // TODO: Ensure we have markdown and only markdown (`.md`)
        //if path.is_file() && path.extension().unwrap() == ".md" {
        if path.is_file() {
            let post = parse_post(path);
            posts.push(post);
        }
    }
    posts.sort_by_key(|p| p.date.clone());

    for post in posts {
        let rendered = render_post(&post, &tpl);
        std::fs::write(format!("../deploy/posts/{}", &post.url), rendered)
            .expect("failed to write post to deploy");
    }

    // TODO: Render tags.
    // TODO: Render index.
    // TODO: Render rss.
    //let posts = vec![post.clone()];
    //let _blog = Blog {
    //title: BLOG_TITLE,
    //posts: posts.clone(),
    //tags: tags.clone(),
    //};
}
