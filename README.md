# rust-shell-clone-project
Shell clone project re-writing in Rust

## 项目需求整理
实现一个类Shell的命令解释器，学习小组按组为单位，每个小组都要做至少一个命令，加入到框架里

## 如何加入项目开发
* 发送 github 的 id 给群管理员，可以添加到项目库的 Collaborator 里
* 群管理员加入后，可以点击此处 [接受邀请](https://github.com/os2edu/rust-shell-clone-project/invitations)
* git clone 本项目代码，**切换到 dev 开发分支下**
* 可以同步最新代码，更新代码，提交代码，解决冲突（如果有的话）
* 如果有和项目开发有关的技术问题，尽可能通过发 issue 的方式来沟通，方便后面加入的开发组员可以看到，微信群里的沟通尽量不涉及技术讨论
* commit 的时候，如果有可以关联的 issue，尽量通过 #id 的方式和 issue 建立关联，这样其他组员都能学到，是什么样提交解决了什么样问题
* 这个大作业项目最重要的是，尽可能覆盖到学过的知识点，所以大家可以共同来定义需求，以能练习到学过的内容为依据，鼓励创造出某种新命令来满足复习需要

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

在 commands/example 文件夹下的 Cargo.toml 指定 lib 文件入口配置和 core 模块的开发依赖，配置如下

```toml
[lib]
path = "./lib.rs"

[dependencies]
core = { path = "../../core" }
```

模块需要实现 Command trait 代码如下

```rust
use core::{App, Command};

impl Command for Example {
  fn execute(&self, args: Option<&str>) {
    todo!()
  }
  fn help(&self) {
    todo!()
  }
}
```

模块需要暴露一个 init 方法, 代码如下

```rust
pub fn init(app: &mut App) {}
```

模块需要到 frame 模块手动 init 一下

```rust
example::init(&mut app);
```

### 运行模块的测试用例

```bash
cargo test --package <you_command_name>
```
