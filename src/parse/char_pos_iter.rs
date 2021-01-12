use std::str::Chars;

// going my_string.char_indices actually returns an iterator where the index
// is the byte offset rather than the actual index of the char. So we've got our
// custom CharPosIter struct here to get the behaviour we want.
pub struct CharPosIter<'a> {
  s: Chars<'a>,
  index: usize,
}

impl<'a> CharPosIter<'a> {
  pub fn new(s: &'a str) -> CharPosIter {
    CharPosIter {
      s: s.chars(),
      index: 0,
    }
  }
}

impl<'a> Iterator for CharPosIter<'a> {
  type Item = (usize, char);

  fn next(&mut self) -> Option<(usize, char)> {
    let val = self.s.next()?;

    let result = Some((self.index, val));

    self.index += 1;
    result
  }
}
