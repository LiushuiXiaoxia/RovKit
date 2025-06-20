# RovKit

---

Rust kit for myself.

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

* [ ] `stringkit`：字符串处理函数
* [ ] `filekit`：文件读写、目录操作
* [ ] `timekit`：时间日期操作
* [ ] `convkit`：类型转换工具
* [ ] `jsonkit`：JSON 读写与路径访问
* [ ] `httpkit`：同步 HTTP 客户端封装
* [ ] `processkit`：同步进程命令执行
* [ ] `configkit`：配置文件加载（JSON优先，支持YAML/TOML）
* [ ] `logkit`：日志初始化及打印封装
* [ ] `cryptokit`：哈希和简单加密

### 异步模块开发（可选）

* [ ] `ahttpkit`：异步 HTTP 客户端（基于 reqwest + tokio）
* [ ] `aprocesskit`：异步进程调用（如有需求）

### 测试和示例

* [ ] 编写单元测试，覆盖主要功能点
* [ ] 编写示例代码，展示模块用法
* [ ] 配置 CI/CD 自动化测试和构建

### 性能优化与发布

* [ ] 开启 Release 配置的 LTO 和 Codegen 优化
* [ ] 使用 `cargo bloat` 分析体积热点，调整依赖
* [ ] 发布第一个稳定版本（0.1.0）

### 未来扩展（长远计划）

* [ ] 配置热加载、动态配置更新
* [ ] 日志异步写入及文件轮转
* [ ] 支持更多加密算法
* [ ] 集成数据库工具包（dbkit）
* [ ] 支持缓存和消息队列模块