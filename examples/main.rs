use dprint_plugin_json::TokenFinder;
use jsonc_parser::{common::Ranged, parse_to_ast, ParseOptions};

fn main() {
  let file = r#" {"test": 2}"#;
  let result = parse_to_ast(
    file,
    &ParseOptions {
      tokens: true,
      ..Default::default()
    },
  )
  .unwrap();
  println!("{:?}", result.value);
  // let finder = TokenFinder::new(&result.tokens.unwrap());
  // println!("{:?}", &result.tokens.unwrap());
  let tokens = result.tokens.unwrap();
  tokens.iter().for_each(|t| {
    println!("{:?}, {:?}", t.range(), t.token);
  });
  let result = tokens.binary_search_by(|t| t.end().cmp(&(1)));
  println!("{:?}", result);
}
