# rust-shell-clone-project
Shell clone project re-writing in Rust

## 项目需求整理
实现一个类Shell的命令解释器，学习小组按组为单位，每个小组都要做至少一个命令，加入到框架里

### 框架 Frame
* 负责解析命令行
* 调用相应模块来执行命令
* 框架能够显示所有的命令使用说明

### 命令模块
命令具备的行为
* 执行入口
* 帮助说明
* 错误提示

每个命令需要是独立的 crate lib
至少要有一个单元测试

#### 可供实现的命令
* cat
* head
* tail
* echo
* wc
* cal
* find
* grep

可参考 [command-line-rust](https://gitee.com/chyyuu/command-line-rust) 的实现

### 命令开发规范

在 commands 文件夹下新建命令的 crate lib 包，如 example

在 commands/example 文件夹下的 Cargo.toml 指定 lib 文件入口配置，配置如下

```toml
[lib]
path = "./lib.rs"
```

在根目录中 Cargo.toml 添加 members 的配置，配置如下

```toml
members = [
    "commands/example"
]
```