use cat;
use core::Command;
use std::collections::HashMap;

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

  let command = app.commands.get("cat").unwrap();

  command.default();
  command.execute();
  command.help();
}
