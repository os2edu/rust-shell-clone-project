use std::{collections::HashSet, error::Error, fmt::Display, io};

pub fn get_input() -> anyhow::Result<Vec<String>> {
  let mut decoder = Decoder::default();
  loop {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    if let LineState::NeedNewLine = decoder.decode(&s)? {
    } else {
      break;
    }
  }
  Ok(decoder.result())
}

type DecodeResult<T> = Result<T, DecodeError>;

#[derive(PartialEq, Eq, Hash)]
enum EPointerState {
  White(White),
  Normal(Normal),
  Str(Str),
  EscapeSequence(EscapeSequence),
}

macro_rules! dispatch_pointer_state_enum {
    ($name:ident, $self:expr, $($param:expr),*) => {{
        use EPointerState::*;
        match $self {
            White(state) => state.$name($($param),*),
            Normal(state) => state.$name($($param),*),
            Str(state) => state.$name($($param),*),
            EscapeSequence(state) => state.$name($($param),*),
        }
    }};
}
impl IPointerState for EPointerState {
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> DecodeResult<HashSet<DecoderAction>> {
    dispatch_pointer_state_enum!(next_char, self, stack, c)
  }

  fn enter(
    &mut self,
    stack: &[EPointerState],
  ) -> DecodeResult<HashSet<DecoderAction>> {
    dispatch_pointer_state_enum!(enter, self, stack)
  }

  fn leave(
    &mut self,
    stack: &[EPointerState],
  ) -> DecodeResult<HashSet<DecoderAction>> {
    dispatch_pointer_state_enum!(leave, self, stack)
  }
}

#[derive(PartialEq, Eq, Hash)]
enum DecoderAction {
  PushChar(char),
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
  ) -> DecodeResult<HashSet<DecoderAction>>;
  fn enter(
    &mut self,
    stack: &[EPointerState],
  ) -> DecodeResult<HashSet<DecoderAction>> {
    Ok(Default::default())
  }
  fn leave(
    &mut self,
    stack: &[EPointerState],
  ) -> DecodeResult<HashSet<DecoderAction>> {
    Ok(Default::default())
  }
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
  ) -> DecodeResult<HashSet<DecoderAction>> {
    let mut acts = HashSet::new();
    use DecoderAction::*;
    use EPointerState as P;
    if !c.is_whitespace() {
      acts.insert(PushNewState(P::Normal(Default::default())));
      acts.insert(Wait);
    };
    Ok(acts)
  }
}
impl IPointerState for Normal {
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> DecodeResult<HashSet<DecoderAction>> {
    let mut acts = HashSet::new();
    use DecoderAction::*;
    use EPointerState as P;
    if c.is_whitespace() {
      acts.insert(PopCurrentState);
    } else if c == '\"' {
      acts.insert(PushNewState(P::Str(Default::default())));
    } else if c == '\\' {
      acts.insert(PushNewState(P::EscapeSequence(Default::default())));
    } else {
      acts.insert(PushChar(c));
    }
    Ok(acts)
  }
  fn leave(
    &mut self,
    stack: &[EPointerState],
  ) -> DecodeResult<HashSet<DecoderAction>> {
    let mut acts = HashSet::new();
    use DecoderAction::*;
    acts.insert(RefreshBuf);
    Ok(acts)
  }
}

impl IPointerState for Str {
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> DecodeResult<HashSet<DecoderAction>> {
    let mut acts = HashSet::new();
    use DecoderAction::*;
    use EPointerState as P;
    if c == '\"' {
      acts.insert(PopCurrentState);
    } else if c == '\\' {
      acts.insert(PushNewState(P::EscapeSequence(Default::default())));
    } else {
      acts.insert(PushChar(c));
    }
    Ok(acts)
  }
}

impl IPointerState for EscapeSequence {
  fn next_char(
    &mut self,
    stack: &[EPointerState],
    c: char,
  ) -> DecodeResult<HashSet<DecoderAction>> {
    let mut acts = HashSet::new();
    use DecoderAction::*;
    use EPointerState as P;
    macro_rules! new_line {
      () => {
        acts.insert(PopCurrentState);
        // 这个判断不加也行
        // 这个判断主要是排除掉, 输入 "abc\ 这种情况
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
        acts.insert(PushChar(c));
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
    Ok(acts)
  }
}
#[derive(PartialEq, Eq)]
enum LineState {
  CanFinish,
  NeedNewLine,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecodeError {
  OpenStr,
}
impl Display for DecodeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::OpenStr => write!(f, "字符串没有关闭,缺少第二个双引号"),
    }
  }
}
impl Error for DecodeError {}
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
  fn push_new_state(
    &mut self,
    mut s: EPointerState,
  ) -> DecodeResult<(bool, bool)> {
    let mut acts = s.enter(&self.stack)?;
    let mut wait = false;
    let mut new_line = false;
    for act in acts.drain() {
      let (wait2, new_line2) = self.do_actions(act)?;
      wait |= wait2;
      new_line |= new_line2;
    }
    self.stack.push(s);
    Ok((wait, new_line))
  }

  fn pop_cur_state(&mut self) -> DecodeResult<(bool, bool)> {
    let mut s = self.stack.pop().unwrap();
    let mut wait = false;
    let mut new_line = false;
    let mut acts = s.leave(&self.stack)?;
    for act in acts.drain() {
      let (wait2, new_line2) = self.do_actions(act)?;
      wait |= wait2;
      new_line |= new_line2;
    }
    Ok((wait, new_line))
  }

  fn do_actions(&mut self, act: DecoderAction) -> DecodeResult<(bool, bool)> {
    use DecoderAction::*;
    let mut wait = false;
    let mut new_line = false;
    match act {
      RequestNewLine => new_line = true,
      Wait => wait = true,
      PushChar(c) => self.buf.push(c),
      RefreshBuf => {
        let mut s = String::new();
        std::mem::swap(&mut s, &mut self.buf);
        self.res.push(s);
      }
      PushNewState(mut s) => {
        let (wait2, new_line2) = self.push_new_state(s)?;
        wait |= wait2;
        new_line |= new_line2;
      }
      PopCurrentState => {
        let (wait2, new_line2) = self.pop_cur_state()?;
        wait |= wait2;
        new_line |= new_line2;
      }
    };
    Ok((wait, new_line))
  }

  fn decode(&mut self, str: &str) -> DecodeResult<LineState> {
    for c in str.chars() {
      let mut acts;
      loop {
        let (state, stack) = self.stack.split_last_mut().unwrap();
        acts = state.next_char(stack, c)?;
        let mut wait = false;
        let mut new_line = false;
        for act in acts.drain() {
          let (wait2, new_line2) = self.do_actions(act)?;
          wait |= wait2;
          new_line |= new_line2;
        }
        // 同时为 true 则 Wait 失效. 要修复, 可能需要存储 br 和 c?
        if new_line {
          return Ok(LineState::NeedNewLine);
        }
        if !wait {
          break;
        }
      }
    }
    return if str.len() == 0 {
      if let EPointerState::Str(_) = self.stack.last().unwrap() {
        Err(DecodeError::OpenStr.into())
      } else {
        Ok(LineState::CanFinish)
      }
    } else if let EPointerState::Str(_) = self.stack.last().unwrap() {
      Ok(LineState::NeedNewLine)
    } else {
      Ok(LineState::CanFinish)
    };
  }

  fn result(mut self) -> Vec<String> {
    loop {
      let last = self.stack.last().unwrap();
      if let &EPointerState::White(_) = last {
        break;
      }
      self.pop_cur_state().unwrap();
    }
    self.res
  }
}
