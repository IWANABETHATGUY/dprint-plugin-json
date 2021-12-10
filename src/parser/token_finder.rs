use dprint_core::formatting::tokens::{TokenCollection, TokenFinder as CoreTokenFinder};
use jsonc_parser::common::Ranged;
use jsonc_parser::tokens::{Token, TokenAndRange};

pub struct TokenFinder<'a> {
  inner: LocalTokenCollection<'a>,
}

impl<'a> TokenFinder<'a> {
  pub fn new(tokens: &'a Vec<TokenAndRange<'a>>) -> TokenFinder<'a> {
    TokenFinder {
      inner: LocalTokenCollection(tokens),
    }
  }
  #[inline]
  pub fn len(&self) -> usize {
    self.inner.0.len()
  }

  pub fn get_next_token_if_comma(&mut self, end_offset: usize) -> Option<&'a TokenAndRange<'a>> {
    // self.inner.get_next_token_if(node.end(), |token| token.token == Token::Comma)
    // unimplemented!() // TODO
    println!("next", );
    let next_token = match self.inner.0.binary_search_by(|tok| tok.start().cmp(&end_offset)) {
      Ok(index) => Some(&self.inner.0[index]),
      // previous token is a trivial token like whitespace
      Err(index) => {
        if index == self.len() - 1 {
          None
        } else {
          Some(&self.inner.0[index + 1])
        }
      }
    };
    if matches!(next_token, Some(&TokenAndRange { token: Token::Comma, .. })) {
      next_token
    } else {
      None
    }
  }

  #[inline]
  pub fn get_previous_token(&mut self, start_offset: usize) -> Option<&'a TokenAndRange<'a>> {
    println!("previous", );
    // self.inner.get_previous_token(start_offset)
    if start_offset == 0 {
      None
    } else {
      match self.inner.0.binary_search_by(|tok| tok.end().cmp(&start_offset)) {
        Ok(index) => Some(&self.inner.0[index]),
        // previous token is a trivial token like whitespace
        Err(index) => {
          // this would happen when whitespace is the first token
          // and you just found the previous token of the first none trivial token
          // for example:
          // ```json
          //     {"test": false}
          // ```
          // if you want to get previous token of first `{`, should return Err(0),
          // in this case should return none
          if index == 0 {
            None
          } else {
            Some(&self.inner.0[index - 1])
          }
        }
      }
    }
    // unimplemented!() // TODO
    // None
  }
}

// Wrap and implement a trait for the CoreTokenFinder

struct LocalTokenCollection<'a>(&'a Vec<TokenAndRange<'a>>);

// impl<'a> TokenCollection<'a> for LocalTokenCollection<'a> {
//   type TPos = usize;
//   type TToken = TokenAndRange<'a>;
//   fn get_start_at_index(&self, index: usize) -> usize {
//     self.0[index].range.start
//   }
//   fn get_end_at_index(&self, index: usize) -> usize {
//     self.0[index].range.end
//   }
//   fn get_token_at_index(&self, index: usize) -> &'a TokenAndRange {
//     &self.0[index]
//   }
//   fn len(&self) -> usize {
//     self.0.len()
//   }
//   fn is_empty(&self) -> bool {
//     self.0.is_empty()
//   }
// }
