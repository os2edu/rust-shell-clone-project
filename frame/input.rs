use std::{io};

pub fn get_input<'a>() -> Vec<String> {
  let mut decoder = Decoder::default();
  let mut s = String::new();
  io::stdin().read_line(&mut s).unwrap();
  while Ok(LineState::NeedNewLine) == decoder.decode(&s) {
    s = String::new();
    io::stdin().read_line(&mut s).unwrap();
  }
  decoder.result()
}

#[derive(PartialEq, Eq)]
enum PointerState {
  NeedNewLine,
  Normal(String),
  Split,
  Str(String),
  EscapeSequence(String),
  EscapeSequenceInStr(String),
}

impl PointerState {
  fn next_char(self, c: char) -> (Self, Option<String>) {
    use PointerState::*;
    macro_rules! escape_sequence {
      ($it:ident,$ret:ident, $s:ident, newline) => {
        escape_sequence!(@implement $it, $ret,$s, {(NeedNewLine, None)})
      };
      ($it:ident,$ret:ident, $s:ident, go_on) => {
        escape_sequence!(@implement $it, $ret,$s, {($ret(String::new()), Some(String::new()))})
      };
      (@implement $it:ident,$ret:ident, $s:ident, $newline_ret:tt) => {{
        if $s.is_empty() {
          if c == '\n' {
            $newline_ret
          } else if c == '\r' {
            ($it('\r'.to_string()), None)
          } else if c == '\"' {
            ($ret(String::new()), Some('\"'.to_string()))
          } else if c == '\\' {
            ($ret(String::new()), Some('\\'.to_string()))
          } else {
            $ret(String::new()).next_char(c)
          }
        } else if $s == "\r" {
          if c == '\n' {
            $newline_ret
          } else {
            $ret(String::new()).next_char(c)
          }
        } else {
          unreachable!("不应该有这种情况啊???");
        }
      }};
    }

    match self {
      Normal(mut s) => {
        if c.is_whitespace() {
          (Split, Some(s))
        } else if c == '\"' {
          (Str(String::new()), Some(s))
        } else if c == '\\' {
          (EscapeSequence(String::new()), Some(s))
        } else {
          s.push(c);
          (Normal(s), None)
        }
      }
      Split => {
        if c.is_whitespace() {
          (Split, None)
        } else {
          Normal(String::new()).next_char(c)
        }
      }
      Str(mut s) => {
        if c == '\"' {
          (Normal(String::new()), Some(s))
        } else if c == '\\' {
          (EscapeSequenceInStr(String::new()), Some(s))
        } else {
          s.push(c);
          (Str(s), None)
        }
      }
      EscapeSequence(mut s) => {
        escape_sequence!(EscapeSequence, Normal, s, newline)
      }
      EscapeSequenceInStr(mut s) => {
        escape_sequence!(EscapeSequenceInStr, Str, s, go_on)
      }
      NeedNewLine => Normal(String::new()).next_char(c),
    }
  }
}

enum OldPointerState {
  Str(String),
  Split,
}

#[derive(PartialEq, Eq)]
enum LineState {
  CanFinish,
  NeedNewLine,
}

// TODO 在中间状态可能会有 EOF, 所以会有 error
#[derive(PartialEq, Eq)]
struct DecodeError {}
struct Decoder {
  res: Vec<String>,
  // 这里需要用 Option 是因为,
  // 我需要在 decode 中(接收的是 &mut self), 取得 pointer state 里数据的所有权,
  // 用了 std::mem::take.
  // 但是 None 只是中间状态, decode 的实现, 保证其在 decode 的前后都不空.
  // 所以有没有办法不用 Option ?
  pointer_state: Option<PointerState>,
  old_pointer_state: Option<OldPointerState>,
}
impl Default for Decoder {
  fn default() -> Self {
    Self {
      res: Default::default(),
      pointer_state: Some(PointerState::Split),
      old_pointer_state: Some(OldPointerState::Split),
    }
  }
}
impl Decoder {
  fn decode<'a>(&mut self, str: &'a str) -> Result<LineState, DecodeError> {
    for c in str.chars() {
      let state = std::mem::take(&mut self.pointer_state).unwrap();
      let (new_state, out_str) = state.next_char(c);
      {
        use OldPointerState as O;
        use PointerState as P;
        let old_state = std::mem::take(&mut self.old_pointer_state).unwrap();
        match old_state {
          O::Split => {
            if new_state != P::Split {
              self.old_pointer_state = Some(O::Str(String::new()))
            } else {
              self.old_pointer_state = Some(O::Split)
            }
          }
          O::Str(mut s) => {
            s.push_str(&out_str.unwrap_or_default());
            if new_state == P::Split {
              self.res.push(s);
              self.old_pointer_state = Some(O::Split)
            } else {
              self.old_pointer_state = Some(O::Str(s))
            }
          }
        };
      }
      self.pointer_state = Some(new_state);

      match self.pointer_state {
        Some(PointerState::NeedNewLine) => return Ok(LineState::NeedNewLine),
        _ => {}
      };
    }

    if let Some(PointerState::Str(_)) = self.pointer_state {
      return Ok(LineState::NeedNewLine);
    } else {
      Ok(LineState::CanFinish)
    }
  }

  fn result(self) -> Vec<String> {
    self.res
  }
}
