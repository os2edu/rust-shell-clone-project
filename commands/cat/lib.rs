use core::Command;

struct Cat {}

impl Cat {
  fn new() -> Self {
    Cat {}
  }
}

impl Command for Cat {
  fn default(&self) {
    println!("default fn");
  }

  fn execute(&self, args:Option<&str>) {
    println!("execute fn");
  }

  fn help(&self) {
    println!("help fn");
  }
}

pub fn new() -> impl Command {
  println!("cat lib");
  let c = Cat::new();
  c
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
