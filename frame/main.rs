use core::App;
use std::io;

mod input;

fn main() {
  // 新建 app
  let mut app = App::default();

  /* 注册模块 start */
  cat::init(&mut app);
  /* 注册模块 end */

  loop {
    // 等待用户输入
    let args = {
      match input::get_input() {
        Ok(args) => args,
        Err(e) => {
          println!("");
          println!("{e}");
          continue;
        }
      }
    };
    // TODO
    // 用户如果用 EOF (windows ctrl+z; linux 据说是 ctrl+d) 结束输入, 后面的输出就会和输入打印到同一行
    // 所以这里多加个空行, 后面在考虑其他解决方案
    println!("");

    if args.is_empty() {
      continue;
    }

    // TODO: 输出帮助文档
    // if args[0] == "help" {
    //   app.help();
    //   continue;
    // }

    // 获取命令
    let command = app.get_command(&args[0]);

    // 执行命令
    match command.execute(args) {
      Ok(_) => {}
      Err(e) => println!("{e}"),
    };
  }
}
