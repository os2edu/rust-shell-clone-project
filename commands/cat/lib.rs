use core::*;
use std::{error::Error, fmt::Display};

struct Cat {}

impl Cat {
  fn new() -> Self {
    Cat {}
  }
  fn cat(&self, path: &str) -> Result<String> {
    Ok(std::fs::read_to_string(path)?)
  }
}

#[derive(Debug)]
struct ArgError;
impl Display for ArgError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "需要输入路径, 且只能输入一个路径, 如果你雀食只输入了一个路径, 那应该是框架的转换参数的地方有问题")
  }
}
impl Error for ArgError {}

impl Command for Cat {
  fn execute(&self, args: Vec<String>) -> Result<()> {
    if args.len() != 2 {
      return Err(ArgError.into());
    }
    let path = &args[1];
    let r = self.cat(path)?;
    println!("{}", r);
    Ok(())
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
