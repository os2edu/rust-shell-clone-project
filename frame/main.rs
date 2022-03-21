use core::App;
use std::io;

fn main() {
  // 新建 app
  let mut app = App::default();

  /* 注册模块 start */
  cat::init(&mut app);
  /* 注册模块 end */

  loop {
    let mut s = String::new();

    // 等待用户输入
    io::stdin().read_line(&mut s).unwrap();
    s = s.trim_end().to_string();

    if s.is_empty() {
      panic!("空字符串");
    }

    let (cmd, args) = {
      match s.split_once(' ') {
        Some(res) => (res.0, Some(res.1)),
        None => (&*s, None),
      }
    };

    // 获取命令
    let command = app.get_command(cmd);

    // 执行命令
    command.execute(args);
  }
}
