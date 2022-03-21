use std::collections::HashMap;

pub trait Command {
  // 执行入口
  fn execute(&self, args: Option<&str>);
  // 帮助说明
  fn help(&self);
}

pub struct App {
  // 命令列表
  commands: HashMap<String, Box<dyn Command>>,
}

impl App {
  // 新建
  pub fn new() -> Self {
    App {
      commands: HashMap::new(),
    }
  }
  // 注册命令
  pub fn register(&mut self, name: String, command: Box<dyn Command>) {
    self.commands.insert(name.into(), command);
  }
  // 获取命令
  pub fn get_command(&mut self, name: &str) -> &Box<dyn Command> {
    &self.commands.get(name).unwrap()
  }
}
