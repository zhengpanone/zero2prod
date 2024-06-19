https://www.bilibili.com/video/BV13C4y1k7xJ/?p=2&spm_id_from=pageDriver
https://blog.csdn.net/qq_36268452/article/details/128045377
https://blog.csdn.net/qq_34168515/article/details/135162147
## 安装依赖

```shell
cargo add axum
cargo add serde -F derive
cargo add serde_json
cargo add tokio
cargo add tracing
cargo add tracing_subscriber -F env-filter
cargo watch -x 'run'

cargo add sqlx -F sqlite,runtime-tokio,tls-rustls,time

cargo install sqlx-cli --no-default-features --features native-tls,sqlite
sqlx database create
sqlx migrate add -r create_users

sqlx migrate run
sqlx migrate revert

sqlx migrate add -r create_counters
sqlx migrate run
```
