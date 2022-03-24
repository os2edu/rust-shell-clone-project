use core::App;

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

        if args.is_empty() {
            continue;
        }

        // TODO: 暂时先这样 后续再优化交互方式
        // 输出帮助文档
        if args[0] == "help" {
            app.help();
            continue;
        }

        // 获取命令
        let command = app.get_command(&args[0]);

        // 执行命令
        command.execute(args);
    }
}
