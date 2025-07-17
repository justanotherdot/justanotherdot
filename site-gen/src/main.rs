#[macro_use]
extern crate lazy_static;

use std::path::Path;

mod context;
mod error;
mod template;

use crate::context::*;
use crate::error::Error;

fn create_deploy_dirs(deploy_prefix: &str, deploy_dirs: &[&str]) -> Result<(), Error> {
    deploy_dirs
        .iter()
        .map(|p| {
            let path = format!("{}/{}", deploy_prefix, p);
            let path = Path::new(&path);
            std::fs::create_dir_all(path).map_err(|_| Error::DeployDirectoryCreateError)
        })
        .collect()
}

fn go() -> Result<(), Error> {
    let template_root = std::env::var("TEMPLATE_ROOT").unwrap_or("site".to_string());
    let deploy_prefix = std::env::var("DEPLOY_PREFIX").unwrap_or("deploy".to_string());
    let title = std::env::var("TITLE").unwrap_or("justanotherdot".to_string());
    let domain = std::env::var("DOMAIN").unwrap_or("https://justanotherdot.com".to_string());
    create_deploy_dirs(&deploy_prefix, &["posts", "tags", "assets"])?;
    let posts = list_posts(&template_root, &domain);
    let tags = list_tags(&posts);
    let index = Index {
        title: &*title,
        posts: posts.to_vec(),
        tags: tags.clone().into_iter().collect(),
    };
    let rss = Rss {
        domain: domain.to_string(),
        url: "/rss.xml".to_string(),
        posts: posts.to_vec(),
    };
    let sitemap = Sitemap {
        posts: posts.to_vec(),
    };
    [
        Context::Posts(posts),
        Context::Tags(tags),
        Context::Index(index),
        Context::Rss(rss),
        Context::Sitemap(sitemap),
    ]
    .iter()
    .for_each(|c| c.render(&template_root, &deploy_prefix));
    Ok(())
}

fn main() {
    go().unwrap_or_else(|e| {
        eprintln!("[justanotherdot] {}", e);
        std::process::exit(1);
    });
}