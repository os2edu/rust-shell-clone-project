use core::Command;

struct Cat {}

impl Cat {
  fn new() -> Self {
    Cat {}
  }
}

impl Command for Cat {
  fn execute(&self, args: Option<&str>) {
    match args {
      Some(path) => {
        let r = std::fs::read_to_string(path);
        match r {
          Ok(text) => println!("{}", text),
          Err(e) => println!("{}", e),
        }
      }
      None => {
        println!("请输入文件路径");
      }
    };
  }

  fn help(&self) {
    println!("help fn");
  }
}

pub fn new() -> impl Command {
  Cat::new()
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
