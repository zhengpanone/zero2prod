## 安装依赖

```shell
cargo add axum
cargo add serde -F derive
cargo add serde_json
cargo add tokio
cargo add tracing
cargo add tracing_subscriber -F env-filter
cargo run -x 'run'

cargo add sqlx -F sqlite,runtime-tokio,tls-rustls,time

cargo install sqlx-cli --no-default-features --features native-tls,sqlite
sqlx database create
sqlx migrate add -r create_users

sqlx migrate run
sqlx migrate revert
```
