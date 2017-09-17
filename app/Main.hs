{-# language OverloadedStrings #-}

module Main where

import SitePipe

main :: IO ()
main = site $ do
  -- Load all the posts from site/posts/
  posts <- resourceLoader markdownReader ["posts/*.md"]

  -- Render out tags
  let tags = getTags makeTagUrl posts

  let indexContext :: Value
      indexContext = object [ "posts" .= posts
                            , "tags" .= tags
                            , "url" .= ("/index.html" :: String)
                            ]

  -- Render index page and posts
  writeTemplate "templates/index.html" [indexContext]
  writeTemplate "templates/post.html" posts
  writeTemplate "templates/tags.html" tags


makeTagUrl :: String -> String
makeTagUrl tagName = "/tags/" ++ tagName ++ ".html"
