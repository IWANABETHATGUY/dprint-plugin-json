use std::time::Instant;

use dprint_core::formatting::format;
use dprint_plugin_json::{configuration::Configuration, format_text};
use jsonc_parser::{common::Ranged, parse_to_ast, ParseOptions};

fn main() {
    let file = include_str!("../test.json");
    //   let result = parse_to_ast(
    //     file,
    //     &ParseOptions {
    //       tokens: true,
    //       ..Default::default()
    //     },
    //   )
    //   .unwrap();
    //   println!("{:?}", result.value);
    //   // let finder = TokenFinder::new(&result.tokens.unwrap());
    //   // println!("{:?}", &result.tokens.unwrap());
    //   let tokens = result.tokens.unwrap();
    //   tokens.iter().for_each(|t| {
    //     println!("{:?}, {:?}", t.range(), t.token);
    //   });
    //   let result = tokens.binary_search_by(|t| t.end().cmp(&(1)));
    //   println!("{:?}", result);
    for _ in 0..1 {
        let start = Instant::now();

        let _res = format_text(
            file,
            &Configuration {
                line_width: 80,
                use_tabs: true,
                indent_width: 2,
                new_line_kind: dprint_core::configuration::NewLineKind::Auto,
                comment_line_force_space_after_slashes: true,
                ignore_node_comment_text: "".into(),
                array_prefer_single_line: false,
                object_prefer_single_line: false,
            },
        );
        println!("{:?}", start.elapsed());
    }
    // println!("{}", res.unwrap());
}
