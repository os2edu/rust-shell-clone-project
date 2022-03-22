use core::{App, Command};

struct Cat {}

impl Cat {
  fn new() -> Self {
    Cat {}
  }
  fn cat(&self, path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
  }
}

impl Command for Cat {
  fn execute(&self, args: Vec<String>) {
    if args.len() != 2 {
      println!("需要输入路径, 且只能输入一个路径, 如果你雀食只输入了一个路径, 那应该是框架的转换参数的地方有问题");
      return;
    }
    let path = &args[1];
    let r = self.cat(path);
    match r {
      Ok(text) => println!("{}", text),
      Err(e) => println!("{}", e),
    }
  }

  fn help(&self) {
    println!("cat README.md");
  }
}

pub fn init(app: &mut App) {
  app.register(String::from("cat"), Box::new(Cat::new()));
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_ok() {
    use crate::Cat;
    let cat = Cat::new();
    let test = cat.cat("test.txt").unwrap();
    assert_eq!(test, "test data");
  }
  #[test]
  fn test_panic() {
    use crate::Cat;
    let cat = Cat::new();
    let result = cat.cat("no_file.txt");
    assert!(result.is_err())
  }
}
