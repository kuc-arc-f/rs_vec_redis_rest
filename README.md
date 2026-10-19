# rs_vec_redis_rest

 Version: 0.9.1

 date    : 2026/10/19
 
 update :

***

Rust Axum API , Redis Vector DB example

* Redis database
* rustc 1.97.1

***
### related client

* C++ CLI , vector search

https://github.com/kuc-arc-f/cpp_18ex/tree/main/redis_cl_11


***
* build
```
cargo build --release
```

***
* start
```
./target/release/rs_vec_redis_rest
```
***
* Test-code
* INSERT
* prefix: prefix data type
* content: text
* vector: vector data

```
curl -X POST http://localhost:8888/api/insert \
  -H "Content-Type: application/json" \
  -d '{"prefix": "doc:" , "content": "hello", "vector": [0.02 , 0.03, 0.05]}'

```

* select
* prefix: prefix data type
* limit: get max data
* vector: vector data

```
curl -X POST http://localhost:8888/api/select \
  -H "Content-Type: application/json" \
  -d '{"prefix": "doc:" , "limit": 3 ,"vector": [0.02 , 0.03, 0.14]}'

```


***
### blog

https://zenn.dev/knaka0209/scraps/e28f6fe1dd9697

