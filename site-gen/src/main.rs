extern crate chrono;
extern crate ramhorns;
extern crate serde;
extern crate walkdir;

use chrono::{DateTime, FixedOffset};
use ramhorns::{Content, Template};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
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
    date_iso8601: String,
    date_month_day_year: String,
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

//struct PostTemplate(Template);
//struct IndexTemplate(Template);
//struct RssTemplate(Template);
//struct TagsTemplate(Template);

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

    let date_iso8601 = DateTime::parse_from_rfc3339(&header.date).expect("failed to parse date");
    let date_shifted = date_iso8601.with_timezone(&FixedOffset::east(10 * 3600));
    let date = date_shifted.format("%B %e %Y, %_I:%M%p").to_string();
    let date_month_day_year = date_shifted.format("%D").to_string();
    let date_iso8601 = date_iso8601.to_string();

    let url = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".md", ".html")
        .to_lowercase();
    let url = format!("posts/{}", url);

    Post {
        title: header.title,
        author: header.author,
        date,
        date_iso8601,
        date_month_day_year,
        url,
        content: markdown.to_string(),
        tags: tags.clone(),
    }
}

// TODO: should take Post and PostHeader
fn render_post(post: &Post, tpl: &Template) -> String {
    let rendered = tpl.render(post);
    rendered
}

//fn render_tags(tags: &Vec<Tag>, tpl: &Template) -> String {
//let rendered = tpl.render(tags);
//rendered
//}

fn render_index(blog: &Blog, tpl: &Template) -> String {
    let rendered = tpl.render(blog);
    rendered
}

fn template(path: &str) -> Template {
    let source = std::fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("could not read template");
        std::process::exit(1);
    });
    Template::new(source).unwrap()
}

fn main() {
    // TODO: Pre-render templates upfront?
    // TODO: Pin version of bulma and embed.
    let tpl = template("../site/templates/post.html");

    let mut posts = vec![];
    for entry in WalkDir::new("../site/posts")
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        // TODO: Ensure we have markdown and only markdown (`.md`)
        //if path.is_file() && path.extension().unwrap() == ".md" {
        if path.is_file() {
            let post = parse_post(path);
            posts.push(post);
        }
    }
    posts.sort_by_key(|p| p.date.clone());

    for post in posts.iter() {
        let rendered = render_post(&post, &tpl);
        std::fs::write(format!("../deploy/{}", &post.url), rendered)
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

    //let tpl = template("../site/templates/tags.html");
    //let rendered = render_tags(&tags, &tpl);
    //println!("{}", rendered);
    //std::fs::write("../deploy/tags.html", rendered).expect("failed to write tags to deploy");

    let tpl = template("../site/templates/index.html");
    let blog = Blog {
        title: BLOG_TITLE,
        posts: posts.clone(),
        tags: tags.into_iter().collect(),
    };
    let rendered = render_index(&blog, &tpl);
    std::fs::write("../deploy/index.html", rendered).expect("failed to write post to deploy");

    // TODO: Render tags.
    // TODO: Render rss.
    // TODO: Brand
}
