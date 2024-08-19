use syntect::parsing::{SyntaxDefinition, SyntaxSetBuilder};

fn main() {
  println!("cargo::rerun-if-changed=syntax/include");

  let mut builder = SyntaxSetBuilder::new();

  for file in std::fs::read_dir("syntax/include").unwrap() {
    let file = file.unwrap().path();

    if let Some("sublime-syntax") = file.extension().and_then(|ext| ext.to_str()) {
      let syntax = std::fs::read_to_string(&file).unwrap();
      builder.add(SyntaxDefinition::load_from_str(&syntax, false, None).unwrap());
    }
  }

  syntect::dumps::dump_to_file(&builder.build(), "syntax/bin/syntax-set.bin").unwrap();
}
