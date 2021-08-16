#[macro_use]
extern crate lazy_static;

use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use walkdir::WalkDir;

mod context;
mod error;
mod template;

use crate::context::*;
use crate::error::Error;
use crate::template::*;

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

fn render_posts(template_root: &str, deploy_prefix: &str, domain: &str) -> Vec<Post> {
    let post_template_path = &format!("{}/templates/post.html", template_root);
    let post_template = PostTemplate(template(post_template_path));
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

fn go() -> Result<(), Error> {
    let template_root = std::env::var("TEMPLATE_ROOT").unwrap_or("site".to_string());
    let deploy_prefix = std::env::var("DEPLOY_PREFIX").unwrap_or("deploy".to_string());
    let title = std::env::var("TITLE").unwrap_or("justanotherdot".to_string());
    let domain = std::env::var("DOMAIN").unwrap_or("https://justanotherdot.com".to_string());
    create_deploy_dirs(&deploy_prefix, &["posts", "tags", "assets"])?;
    // TODO: conexts -> render -> write split out.
    let posts = render_posts(&template_root, &deploy_prefix, &domain);
    let tags = render_tags(&template_root, &deploy_prefix, &posts);
    render_index(&template_root, &deploy_prefix, &title, &posts, &tags);
    render_rss(&template_root, &deploy_prefix, &domain, &posts);
    render_sitemap(&template_root, &deploy_prefix, &posts);
    Ok(())
}

fn main() {
    go().unwrap_or_else(|e| {
        eprintln!("[justanotherdot] {}", e);
        std::process::exit(1);
    });
}
