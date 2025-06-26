# RovKit

---

Rust kit for myself.

# 使用姿势
[![Crates.io](https://img.shields.io/crates/v/rovkit.svg)](https://crates.io/crates/rovkit)
[![Downloads](https://img.shields.io/crates/d/rovkit.svg)](https://crates.io/crates/rovkit)
[![Docs.rs](https://img.shields.io/badge/docs.rs-rovkit-blue)](https://docs.rs/rovkit)


```shell
cargo add rovkit
```

```toml
[dependencies]
rovkit = "0.0"
```

# 简介

* **项目定位**：一个 Rust 语言的全家桶工具库，提供字符串、时间、JSON、类型转换、HTTP、进程、文件、配置、日志、加密、工具函数等常用功能模块。
* **模块命名**：统一用 `xxxkit` 命名规范，同步模块如 `httpkit`，异步模块简写为 `ahttpkit` 等。
* **异步设计**：采用同步优先、异步可选的架构，通过 feature 开关切换，避免异步复杂度传递。
* **依赖管理**：使用 feature 控制依赖，按需编译，减小体积。
* **性能优化**：建议开启 LTO 和 release 模式优化，利用 Rust 自带的 dead code elimination，控制二进制大小。
* **代码风格**：模块职责清晰，功能单一，易于维护和扩展。
* **生态对比**：参考了 Java 全家桶及混淆优化经验，结合 Rust 特点做定制化设计。

---

# ToDo List

### 基础架构搭建

* [ ] 初始化 GitHub 仓库，添加 LICENSE、README.md、.gitignore
* [ ] 搭建项目基础目录结构（src、examples、tests、benches）
* [ ] 编写 `Cargo.toml`，配置 feature 开关和依赖

### 核心模块开发（同步）

#selectedCode:README.md
* [x] `stringkit`：字符串处理函数
    - 字符串拼接、分割、替换
    - Base64 编解码
    - 正则匹配与提取

* [x] `filekit`：文件读写、目录操作
    - 文件是否存在、创建、删除、重命名
    - 读取和写入文本/二进制文件
    - 遍历目录、创建临时文件/目录

* [x] `timekit`：时间日期操作
    - 获取当前时间戳
    - 时间格式化（ISO8601、RFC2822 等）
    - 时间加减、比较、时区转换

* [x] `convkit`：类型转换工具
    - 基本类型转换（i32, u64, f32, String 等）
    - Hex、Base64、UTF-8 转换
    - 自定义结构体序列化/反序列化封装

* [x] `jsonkit`：JSON 读写与路径访问
    - JSON 序列化/反序列化（基于 serde）
    - 支持通过 JSON Path 提取字段
    - 支持嵌套结构修改与合并

* [x] `httpkit`：同步 HTTP 客户端封装
    - GET / POST 请求封装
    - 设置 headers、timeout、代理等
    - 返回状态码、响应内容解析

* [x] `processkit`：同步进程命令执行
    - 执行 shell 命令并获取输出
    - 支持管道、标准输入输出重定向
    - 超时控制、错误码捕获

* [x] `configkit`：配置文件加载（JSON优先，支持YAML/TOML）
    - 支持多格式配置文件解析（json/yml/toml）
    - 环境变量覆盖机制
    - 默认值设置与校验

* [x] `logkit`：日志初始化及打印封装
    - 支持 info、debug、warn、error 等级别
    - 控制台和文件双输出
    - 日志轮转、最大保留数量配置

* [x] `cryptokit`：哈希和简单加密
    - MD5、SHA1、SHA256 等摘要算法
    - AES 对称加密（ECB/CBC/GCM 模式）
    - HMAC、签名生成与验证

### 异步模块开发（可选）

* [ ] `ahttpkit`：异步 HTTP 客户端（基于 reqwest + tokio）
* [ ] `aprocesskit`：异步进程调用（如有需求）

### 测试和示例

* [ ] 编写单元测试，覆盖主要功能点
* [ ] 编写示例代码，展示模块用法
* [ ] 配置 CI/CD 自动化测试和构建

### 未来扩展（长远计划）

* [ ] 配置热加载、动态配置更新
* [ ] 日志异步写入及文件轮转
* [ ] 支持更多加密算法
* [ ] 集成数据库工具包（dbkit）
* [ ] 支持缓存和消息队列模块