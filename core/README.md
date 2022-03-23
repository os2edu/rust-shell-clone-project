<h1 align="center">Core Crate</h1>

## Features
- 为所有要实现的命令定义了一个trait规范，
- 存储所有实现的命令函数
- 定义了执行器函数的规范

## Example

* [步骤](#example)
    * 1 将core crate添加到lib项目依赖中去
        ```toml
        [lib]
        path = "./lib.rs"

        [dependencies]
        core = { path = "../../core" }
        ```
    * 2 实现trait中声明的函数
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

