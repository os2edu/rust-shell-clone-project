use std::{collections::HashSet, io};

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

#[derive(PartialEq, Eq, Hash)]
enum EPointerState {
  White(White),
  Normal(Normal),
  Str(Str),
  EscapeSequence(EscapeSequence),
}
impl IPointerState for EPointerState {
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> HashSet<DecoderAction> {
    use EPointerState::*;
    match self {
      White(state) => state.next_char(stack, c),
      Normal(state) => state.next_char(stack, c),
      Str(state) => state.next_char(stack, c),
      EscapeSequence(state) => state.next_char(stack, c),
    }
  }
}

#[derive(PartialEq, Eq, Hash)]
enum DecoderAction {
  PushChar,
  Wait,
  RequestNewLine,
  PushNewState(EPointerState),
  PopCurrentState,
  RefreshBuf,
}
trait IPointerState {
  // TODO 用 bit 来做, 而不是 HashSet
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> HashSet<DecoderAction>;
}

#[derive(Default, PartialEq, Eq, Hash)]
struct White;
#[derive(Default, PartialEq, Eq, Hash)]
struct Normal;
#[derive(Default, PartialEq, Eq, Hash)]
struct Str;
#[derive(PartialEq, Eq, Hash)]
struct EscapeSequence(bool);
impl Default for EscapeSequence {
  fn default() -> Self {
    Self(false)
  }
}
impl IPointerState for White {
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> HashSet<DecoderAction> {
    let mut acts = HashSet::new();
    use DecoderAction::*;
    use EPointerState as P;
    if !c.is_whitespace() {
      acts.insert(PushNewState(P::Normal(Default::default())));
      acts.insert(Wait);
    };
    acts
  }
}
impl IPointerState for Normal {
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> HashSet<DecoderAction> {
    let mut acts = HashSet::new();
    use DecoderAction::*;
    use EPointerState as P;
    if c.is_whitespace() {
      acts.insert(PopCurrentState);
      acts.insert(RefreshBuf);
    } else if c == '\"' {
      acts.insert(PushNewState(P::Str(Default::default())));
    } else if c == '\\' {
      acts.insert(PushNewState(P::EscapeSequence(Default::default())));
    } else {
      acts.insert(PushChar);
    }
    acts
  }
}

impl IPointerState for Str {
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> HashSet<DecoderAction> {
    let mut acts = HashSet::new();
    use DecoderAction::*;
    use EPointerState as P;
    if c == '\"' {
      acts.insert(PopCurrentState);
    } else if c == '\\' {
      acts.insert(PushNewState(P::EscapeSequence(Default::default())));
    } else {
      acts.insert(PushChar);
    }
    acts
  }
}

impl IPointerState for EscapeSequence {
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> HashSet<DecoderAction> {
    let mut acts = HashSet::new();
    use DecoderAction::*;
    use EPointerState as P;
    macro_rules! new_line {
      () => {
        acts.insert(PopCurrentState);
        // 这个判断不加也行
        // 这个判断主要是排除掉, 在 "abc\ 这种情况
        // 因为这种情况属于引号没匹配, 在外面处理了
        if let &P::Normal(_) = stack.last().unwrap() {
          acts.insert(RequestNewLine);
        }
      };
    }
    if !self.0 {
      if c == '\n' {
        new_line!();
      } else if c == '\r' {
        self.0 = true;
      } else if c == '\"' || c == '\\' {
        acts.insert(PushChar);
        acts.insert(PopCurrentState);
      } else {
        acts.insert(PopCurrentState);
        acts.insert(Wait);
      }
    } else {
      if c == '\n' {
        new_line!();
      } else {
        acts.insert(PopCurrentState);
        acts.insert(Wait);
      }
    }
    acts
  }
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
  stack: Vec<EPointerState>,
  buf: String,
}
impl Default for Decoder {
  fn default() -> Self {
    Self {
      res: Default::default(),
      stack: vec![EPointerState::White(White)],
      buf: String::new(),
    }
  }
}
impl Decoder {
  fn decode(&mut self, str: &str) -> Result<LineState, DecodeError> {
    for c in str.chars() {
      let mut acts;
      loop {
        let (state, stack) = self.stack.split_last_mut().unwrap();
        acts = state.next_char(stack, c);
        let mut br = true;
        let mut new_line = false;
        for act in acts.drain() {
          use DecoderAction::*;
          match act {
            RequestNewLine => new_line = true,
            Wait => br = false,
            PushChar => self.buf.push(c),
            RefreshBuf => {
              let mut s = String::new();
              std::mem::swap(&mut s, &mut self.buf);
              self.res.push(s);
            }
            PushNewState(s) => self.stack.push(s),
            PopCurrentState => {
              self.stack.pop();
            }
          }
        }
        // 同时为 true 则 Wait 失效. 要修复, 可能需要存储 br 和 c?
        if new_line {
          return Ok(LineState::NeedNewLine);
        }
        if br {
          break;
        }
      }
    }
    return if let EPointerState::Str(_) = self.stack.last().unwrap() {
      Ok(LineState::NeedNewLine)
    } else {
      Ok(LineState::CanFinish)
    };
  }
  fn result(self) -> Vec<String> {
    self.res
  }
}
