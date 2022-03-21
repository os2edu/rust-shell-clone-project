pub trait Command {
  // 默认方法 调试用的
  fn default(&self);
  // 执行入口
  fn execute(&self, args:Option<&str>);
  // 帮助说明
  fn help(&self);
}
