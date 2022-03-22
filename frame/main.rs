use cat;
use core::Command;
use std::{collections::HashMap, io};

struct App {
  // 命令列表
  commands: HashMap<String, Box<dyn Command>>,
}

impl App {
  fn new() -> Self {
    App {
      commands: HashMap::new(),
    }
  }
}

fn main() {
  println!("rust-shell-clone-project");
  let mut app = App::new();

  let command = cat::new();

  app.commands.insert(String::from("cat"), Box::new(command));

  loop {
    let mut s = String::new();
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

    let command = app.commands.get(cmd).unwrap();
    command.default();
    command.execute(args);
    // command.help();
  }


}
