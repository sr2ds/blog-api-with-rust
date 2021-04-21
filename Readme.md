# Blog API - With Rust - Actix

Esta projeto é apenas para fins de testes e estudos e o contexto é a implementação de uma simples api desenvolvido com Rust no framework web Actix.

O objetivo primário é compreender como o framework funciona e por em prática a sintaxe do Rust.


## Todo

1. Model;
2. Dockerize

### Cadastrar Artigo

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