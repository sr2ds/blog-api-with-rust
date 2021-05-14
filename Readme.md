# Blog API - Writing in Rust with Actix Web

This Repository is only one practice project with Rust Language. I'm use Actix Web Framework and MongoDB.

MongoDB ODM: https://crates.io/crates/wither

Actix Web: https://actix.rs/

## Todo

1. Refactoring in progress with crate winther

### Store one article

```sh
 curl -X POST --header "Content-Type: application/json" --data \
    '{"author":"David","title":"Rust Blog Article","created_at":"2021-04-15","content":"blank"}' \
    http://localhost:8080/articles
```

## Setup Dev

## MongoDB - Docker Container
`docker run -d -p 27017:27017 -v `pwd`/data/db:/data/db --name blogApiRust mongo`

## Run
`cargo watch -x run`