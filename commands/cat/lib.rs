use core::{App, Command};

struct Cat {}

impl Cat {
  fn new() -> Self {
    Cat {}
  }
  fn read(&self, path: &str) -> Result<String, std::io::Error> {
    let r = std::fs::read_to_string(path);
    match r {
      Ok(text) => Ok(text),
      Err(e) => Err(e),
    }
  }
}

impl Command for Cat {
  fn execute(&self, args: Option<&str>) {
    match args {
      Some(path) => {
        let r = self.read(path);
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
    let test = cat.read("test.txt").unwrap();
    assert_eq!(test, "test data");
  }
  #[test]
  fn test_panic() {
    use crate::Cat;
    let cat = Cat::new();
    let result = cat.read("no_file.txt");
    assert!(result.is_err())
  }
}
