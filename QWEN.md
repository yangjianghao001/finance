# Finance 项目概述

## 项目简介

这是一个基于 Rust 开发的金融相关后端服务项目，采用 Cargo Workspace 多 crate 架构组织代码。项目使用 `axum` 作为 Web 框架，`sea-orm` 作为 ORM 框架，并集成了 Redis 缓存、结构化日志等功能。

**核心特性：**
- 基于 axum 的 HTTP 服务
- SeaORM 数据库操作（MySQL）
- Redis 缓存支持
- 结构化日志（tracing + tracing-subscriber）
- 请求 ID 追踪（x-request-id）
- 配置文件管理（YAML 格式）
- 自定义过程宏（finance-macro）

## 技术栈

- **Web 框架**: axum 0.8.8
- **ORM**: SeaORM 1.1.19 (MySQL)
- **缓存**: Redis 1.0.2
- **异步运行时**: tokio 1.49.0
- **序列化**: serde 1.0.228
- **错误处理**: thiserror 2.0.17, anyhow 1.0.100
- **日志**: tracing 0.1.44, tracing-subscriber 0.3.22, tracing-appender 0.2.4
- **配置管理**: config 0.15.19
- **HTTP 中间件**: tower-http 0.6.8
- **过程宏**: proc-macro2, quote, syn

## 项目结构

```
finance/
├── Cargo.toml              # Workspace 根配置
├── src/main.rs             # 应用入口，调用 finance_api::main()
├── resource/
│   └── application.yaml    # 应用配置文件
├── logs/                   # 运行时日志输出目录（production 模式）
└── crates/
    ├── finance-api/        # API 层：HTTP 服务器启动、路由定义、中间件
    ├── finance-common/     # 公共模块：错误类型定义、常量
    ├── finance-config/     # 配置模块：应用配置加载、日志初始化、应用状态
    ├── finance-entity/     # 数据实体层：SeaORM 实体定义（待实现）
    ├── finance-macro/      # 过程宏库：自定义 derive/attribute 宏（待实现）
    └── finance-service/    # 业务逻辑层：服务实现、工具函数
```

### Crate 依赖关系

```
finance (main)
  └── finance-api
        ├── finance-config
        ├── finance-common
        ├── finance-service
        │     ├── finance-common
        │     ├── finance-entity
        │     └── redis, rayon, uuid
        └── axum, tower-http, tokio
```

## 构建与运行

### 前置条件

- Rust 工具链（edition 2024）
- MySQL 数据库（待配置）
- Redis 服务器（待配置）

### 常用命令

```bash
# 构建项目
cargo build

# 运行项目
cargo run

# 运行测试
cargo test

# 检查代码
cargo check

# 格式化代码
cargo fmt

# Clippy lint 检查
cargo clippy
```

### 配置说明

应用配置文件位于 `resource/application.yaml`，当前仅包含基础服务器配置：

```yaml
server:
  port: 3000
```

服务器默认监听本地 IP（通过 `local_ip_addr` 获取）的 3000 端口。

### 日志配置

- **开发模式** (`debug_assertions` = true): 日志输出到 stdout
- **生产模式** (release): 日志输出到 `logs/` 目录，按小时轮转，文件名为 `finance.log`

## API 路由

当前已定义的路由：

| 方法 | 路径 | 处理函数 | 说明 |
|------|------|----------|------|
| GET | `/finance/detail` | `finance_detail` | 返回 "hello finance detail" |

中间件：
- 请求 ID 传播 (`PropagateRequestIdLayer`)
- 请求日志记录 (`log_requests`)
- 请求 ID 设置 (`SetRequestIdLayer`)

## 开发约定

- **错误处理**: 使用 `thiserror` 定义 `AppError` 枚举，支持数据库错误和内部服务器错误，自动转换为 HTTP 500 响应
- **日志**: 使用 `tracing` 进行结构化日志记录，包含时间戳、线程信息、行号等
- **请求追踪**: 通过 `x-request-id` header 实现请求链路追踪
- **配置加载**: 使用 `LazyLock` 实现配置的懒加载和全局共享
- **代码组织**: 采用分层架构，API 层、Service 层、Entity 层职责清晰

## 待完善功能

从代码结构来看，以下功能尚未实现：

1. **数据库连接**: `AppState` 已定义数据库和 Redis 连接字段，但尚未初始化
2. **Entity 定义**: `finance-entity` crate 目前为空，需要添加 SeaORM 实体
3. **业务逻辑**: `finance-service` 只有一个示例函数，需要实现具体业务
4. **过程宏**: `finance-macro` crate 目前为空，预留自定义宏扩展
5. **Redis 配置**: 配置文件中已引入 redis 依赖，但尚未配置连接参数
6. **更多 API 路由**: 目前只有一个测试路由，需要补充业务接口

## 注意事项

- 项目使用 edition 2024，确保 Rust 工具链版本足够新
- `finance-config` 使用 `include_str!` 宏嵌入 `resource/application.yaml`，路径是相对固定的
- 生产环境日志会写入 `logs/` 目录，确保运行时有写入权限
