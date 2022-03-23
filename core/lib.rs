use std::collections::HashMap;

// # Command
/// The Command trait represents a type that can handle commands.
pub trait Command {
  // 执行入口
  // TODO 是否需要一个统一的错误处理机制
  /// Entry for command execution;
  /// It will call the function that you registered when you input the right command and arguments.
  fn execute(&self, args: Vec<String>);
  // 帮助说明
  /// If you don't know how to input a command or arguments, you can type `help` and get the help information.
  fn help(&self);
}

/// The struct App is a collection that can store the commands instances
///  
#[derive(Default)]
pub struct App {
  // 命令列表
  /// commands set 
  /// knowledge: trait object
  commands: HashMap<String, Box<dyn Command>>,
}

impl App {
  // 注册命令
  /// commands register 
  pub fn register(&mut self, name: String, command: Box<dyn Command>) {
    let c = self.commands.get(&name);
    if c.is_some() { panic!("{} 命令已注册", name) }
    self.commands.insert(name, command);
  }
  // 获取命令
  /// get command this method return a Struct instance that can handle data 
  pub fn get_command(&mut self, name: &str) -> &(dyn Command) {
    self.commands.get(name).unwrap().as_ref()
  }
}
