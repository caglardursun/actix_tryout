# Actix Tryouts
This repository is for testing some of the features of actix web framework 

It's crazy ... According to https://www.techempower.com/benchmarks/#section=data-r18&hw=ph&test=fortune 
the framework handle 700.000 request per sec ... 

## Library That We Use
We already have 

```toml
[dependencies]
actix-rt = "1.1.1"
actix-web = "3.1.0"
actix-http = "2.0.0"
actix-service = "1.0.6"
serde = { version = "1.0.104", features = ["derive"] }
dotenv = "0.15.0"
config = "0.10.1"
tokio-pg-mapper = "0.1.4"
tokio-pg-mapper-derive = "0.1.4"
deadpool-postgres = "0.5.0"
tokio-postgres = "0.5.1"
```
- `actix-rt` is a runtime library which provide async runtime and methods and some basic macros
- `serde` is for json serialize deserialization 
- `dotenv` is for .env envoirement 




