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
    let args = input::get_input();
    println!("args:\n{args:?}");

    // 获取命令
    let command = app.get_command(&args[0]);

    // 执行命令
    command.execute(args);
  }
}
