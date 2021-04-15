# Blog API - With Rust

Esta API é a implementação de um simples blog desenvolvido com Rust no framework web Actix.

Esse projeto é apenas para fins de testes do Actix e estudos da linguagem Rust.

Mongo Container

docker run -d -p 27017:27017 -v `pwd`/data/db:/data/db --name blogApiRust mongo

1. Crud Article;
2. Refactor Crud -> Improve/Organize;
3. Dockerizar

Cadastro de Artigo

```sh
 curl -X POST --header "Content-Type: application/json" --data \
    '{"author":"David","title":"Rust Blog Article","created_at":"2021-04-15","content":"blank"}' \
    http://localhost:8080/articles
```