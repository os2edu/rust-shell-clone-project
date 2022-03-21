pub trait Command {
  // 执行入口
  fn execute(&self, args: Option<&str>);
  // 帮助说明
  fn help(&self);
}
