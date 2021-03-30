// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub fn generate_unique_name<'a, I>(iter: I, ident: &str) -> String
where
  I: Clone + Iterator<Item = &'a str>,
{
  let mut this: State<'_> = State::new(ident);

  while iter.clone().any(|current| current == this.get()) {
    this.index += 1;
  }

  this.into_string()
}

struct State<'a> {
  ident: &'a str,
  index: u32,
  buffer: Option<String>,
}

impl<'a> State<'a> {
  const fn new(ident: &'a str) -> Self {
    Self {
      ident,
      index: 0,
      buffer: None,
    }
  }

  fn fill(&mut self) {
    let mut buffer: &mut String = self.buffer.get_or_insert_with(String::new);

    buffer.push_str(self.ident);
    buffer.push_str(" (");
    itoa::fmt(&mut buffer, self.index).unwrap();
    buffer.push(')');
  }

  fn get(&mut self) -> &str {
    if self.index == 0 {
      self.ident
    } else {
      self.fill();
      self.buffer.as_deref().unwrap_or_default()
    }
  }

  fn into_string(self) -> String {
    if self.index == 0 {
      self.ident.to_string()
    } else {
      self.buffer.unwrap_or_default()
    }
  }
}
