<!-- # rust-shell-clone-project -->
<h1 align="center">Shell clone project re-writing in Rust</h1>

***

This repo contains the following main components:

| Crate                                                                                                             | Description                    | Documentation                        | ChangeLog                                  |
|-------------------------------------------------------------------------------------------------------------------|--------------------------------|--------------------------------------|--------------------------------------------|
| **core**                                | Command RunTime                       | [(README)](core/README.md)           | [(CHANGELOG)](core/CHANGELOG.md)           |
| **frame**           | Process Entry            | [(README)](frame/README.md)    | [(CHANGELOG)](frame/CHANGELOG.md)    |
| **commands**        | Command Set           | [(README)](commands/README.md)   | [(CHANGELOG)](commands/CHANGELOG.md)   |


***
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
- [x] cat
- [ ] head
- [ ] tail
- [ ] echo
- [ ] wc
- [ ] cal
- [ ] find
- [ ] grep

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


## Contributing

Thanks for your help improving the project! We are so happy to have you!


## License

Licensed under either of

* Apache License, Version 2.0,([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in Shell by Rust by you, shall be licensed as Apache, without any additional terms or conditions.
