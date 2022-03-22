use std::collections::HashMap;

pub trait Command {
    // 执行入口
    fn execute(&self, args: Vec<String>);
    // 帮助说明
    fn help(&self);
}

#[derive(Default)]
pub struct App {
    // 命令列表
    commands: HashMap<String, Box<dyn Command>>,
}

impl App {
    // 注册命令
    pub fn register(&mut self, name: String, command: Box<dyn Command>) {
        let c = self.commands.get(&name);
        if c.is_some() {
            panic!("{} 命令已注册", name)
        }
        self.commands.insert(name, command);
    }
    // 获取命令
    pub fn get_command(&mut self, name: &str) -> &(dyn Command) {
        self.commands.get(name).unwrap().as_ref()
    }
    // 获取帮助
    pub fn help(&mut self) {
        for (_, command) in self.commands.iter() {
            command.help();
        }
    }
}
