# Axum API Server

一个使用 Rust 构建的现代化、高性能的 Web API 服务器骨架项目。

## 技术栈

- **Web 框架**: [Axum](https://github.com/tokio-rs/axum) - 基于 Tokio 的人体工程学 Web 框架
- **异步运行时**: [Tokio](https://tokio.rs/) - 异步运行时
- **数据库**: PostgreSQL + [SQLx](https://github.com/launchbadge/sqlx) - 编译时检查的 SQL
- **错误处理**: [anyhow](https://github.com/dtolnay/anyhow) + [thiserror](https://github.com/dtolnay/thiserror)
- **日志**: [tracing](https://github.com/tokio-rs/tracing) - 结构化日志
- **API 文档**: [utoipa](https://github.com/juhaku/utoipa) - OpenAPI 3.0 文档生成
- **配置**: [dotenvy](https://github.com/allan2/dotenvy) - 环境变量管理
- **验证**: [validator](https://github.com/Keats/validator) - 数据验证
- **参考**: [Axum Backend](https://blog.0xshadow.dev/posts/backend-engineering-with-axum/axum-model-setup/) - Axum Backend

## 项目特性

### ✨ 核心功能
- 📦 模块化架构（Handler → Service → Repository）
- 🔄 异步处理，高性能
- 🗃️ PostgreSQL 数据库连接池
- 🔍 自动生成 OpenAPI 文档（Swagger UI）
- 🏥 健康检查端点
- 🔐 请求 ID 追踪
- 📊 结构化 JSON 日志
- ✅ 数据验证
- 🐳 Docker 支持
- 🔧 完整的 Makefile 工具链

### 🏗️ 架构设计
```
请求 → Handler（处理HTTP） → Service（业务逻辑） → Repository（数据访问） → Database
```

## 快速开始

### 前置要求
- Rust 1.75+
- PostgreSQL 14+
- Docker & Docker Compose（可选）

### 安装开发工具
```bash
make install-dev
```

### 1. 克隆并配置
```bash
# 克隆项目
git clone <your-repo>
cd zero2prod

# 复制环境变量
cp .env.example .env

# 编辑 .env 文件，配置数据库连接等
```

### 2. 启动数据库
```bash
# 使用 Docker 启动 PostgreSQL
make db-up
```

### 3. 运行数据库迁移
```bash
make migrate
```

### 4. 启动服务
```bash
# 开发模式（热重载）
make dev

# 或者直接运行
cargo run
cargo watch -x 'run'

# 测试
cargo test -- --nocapture # --nocapture 表示不要捕获测试输出
```

### 5. 访问服务
- API 服务: http://localhost:3000
- Swagger UI: http://localhost:3000/swagger-ui
- 健康检查: http://localhost:3000/health

## 开发指南

### 如何添加dev-dependencies

```shell
cargo add sqlx-cli --dev --features postgres,rustls
```

### 创建新的 API 端点

1. **定义模型** (`src/models/`)
```rust
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct YourModel {
    pub id: Uuid,
    pub name: String,
}
```

2. **创建 Repository** (`src/repository/`)
```rust
pub struct YourRepository {
    pool: PgPool,
}

impl YourRepository {
    pub async fn find_all(&self) -> Result<Vec<YourModel>> {
        // 数据库查询
    }
}
```

3. **创建 Service** (`src/services/`)
```rust
pub struct YourService {
    repository: YourRepository,
}

impl YourService {
    pub async fn list(&self) -> Result<Vec<YourModel>> {
        self.repository.find_all().await
    }
}
```

4. **创建 Handler** (`src/handlers/`)
```rust
#[utoipa::path(
    get,
    path = "/api/your-endpoint",
    tag = "your-tag",
    responses(...)
)]
pub async fn your_handler(
    State(state): State<AppState>
) -> Result<Json<Vec<YourModel>>> {
    // 处理逻辑
}
```

5. **注册路由** (`src/routes/mod.rs`)

### 数据库迁移

```bash
# 创建新迁移
make migrate-create
# 输入迁移名称，例如: add_products_table

# 运行迁移
make migrate

# 回滚迁移
make migrate-revert
```

### 常用命令

```bash
# 开发
make dev              # 开发模式运行（热重载）
make build            # 构建 release 版本
make test             # 运行测试
make check            # 代码检查（check + clippy）
make fmt              # 格式化代码

# 数据库
make db-up            # 启动数据库
make db-down          # 停止数据库
make migrate          # 运行迁移
make migrate-revert   # 回滚迁移

# Docker
make docker-build     # 构建镜像
make docker-up        # 启动所有服务
make docker-down      # 停止所有服务
make docker-logs      # 查看日志

# 清理
make clean            # 清理构建文件
```

## API 端点

### 健康检查
```http
GET /health
```

### 用户管理
```http
GET    /api/users           # 获取用户列表
POST   /api/users           # 创建用户
GET    /api/users/:id       # 获取用户详情
DELETE /api/users/:id       # 删除用户
```

### 示例请求

创建用户:
```bash
curl -X POST http://localhost:3000/api/users \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "name": "Test User"
  }'
```

## 部署

### Docker 部署
```bash
# 构建并启动所有服务
docker-compose up -d

# 查看日志
docker-compose logs -f

# 停止服务
docker-compose down
```

### 生产环境配置
1. 更新 `.env` 文件中的环境变量
2. 设置 `ENVIRONMENT=production`
3. 配置适当的 `LOG_LEVEL`
4. 使用强密码和安全的数据库连接

## 项目结构

```
├── src/
│   ├── main.rs              # 应用入口
│   ├── config/              # 配置管理
│   ├── error/               # 错误处理
│   ├── middleware/          # 中间件
│   ├── models/              # 数据模型
│   ├── handlers/            # HTTP 处理器
│   ├── services/            # 业务逻辑
│   ├── repository/          # 数据访问
│   ├── routes/              # 路由定义
│   └── state.rs             # 应用状态
├── migrations/              # 数据库迁移
├── Dockerfile              # Docker 镜像
├── docker-compose.yml      # Docker Compose 配置
├── Makefile                # 开发工具命令
└── .env.example            # 环境变量模板
```

## 扩展建议

### 认证与授权
- 添加 JWT 认证中间件
- 实现 RBAC 权限控制
- 集成 OAuth2

### 缓存
- 添加 Redis 缓存层
- 实现查询缓存

### 监控
- 集成 Prometheus metrics
- 添加分布式追踪（Jaeger/Zipkin）

### 消息队列
- 集成 RabbitMQ 或 Kafka
- 实现异步任务处理

### 其他
- 添加单元测试和集成测试
- 实现 GraphQL 支持
- 添加限流中间件
- WebSocket 支持

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT License