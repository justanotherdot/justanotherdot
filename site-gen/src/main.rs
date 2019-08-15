extern crate ramhorns;
extern crate serde;

use ramhorns::{Content, Template};
use serde::{Deserialize, Serialize};

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

fn main() {
    // TODO: Sort posts.
    // TODO: Pre-render templates upfront?
    // TODO: Function to strip headers and parse into YAML.
    let source = std::fs::read_to_string("../site/templates/post.html").unwrap_or_else(|_| {
        eprintln!("could not read template");
        std::process::exit(1);
    });
    let tpl = Template::new(source).unwrap();
    let markdown = std::fs::read_to_string("../site/posts/hi.md").unwrap_or_else(|_| {
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

    let posts = vec![Post {
        title: &header.title,
        author: &header.author,
        date: &header.date,
        url: "name-of-markdown.html",
        content: &markdown,
        tags: tags.clone(),
    }];

    let _blog = Blog {
        title: "justanotherdot",
        posts: posts.clone(),
        tags: tags.clone(),
    };

    let rendered = tpl.render(&posts[0]);
    println!("{}", rendered);
}
