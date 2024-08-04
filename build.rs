use syntect::parsing::{SyntaxDefinition, SyntaxSetBuilder};

// Example custom build script.
fn main() {
  // Tell Cargo that if the given file changes, to rerun this build script.
  // println!("cargo::rerun-if-changed=syntaxes/");

  // let mut builder = SyntaxSetBuilder::new();

  // for file in std::fs::read_dir("syntaxes").unwrap() {
  //   let file = file.unwrap().path();

  //   if let Some("sublime-syntax") = file.extension().and_then(|ext| ext.to_str()) {
  //     let syntax = std::fs::read_to_string(&file).unwrap();
  //     builder.add(SyntaxDefinition::load_from_str(&syntax, false, None).unwrap());
  //   }
  // }

  // syntect::dumps::dump_to_file(&builder.build(), "syntaxes/syntax-set.bin").unwrap();
}
