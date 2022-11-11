use yaml_rust::{YamlLoader, YamlEmitter};

fn main(){

let docs = YamlLoader::load_from_str("[1, 2, 3]").unwrap();
let doc = &docs[0]; // select the first document
assert_eq!(doc[0].as_i64().unwrap(), 1); // access elements by index

let mut out_str = String::new();
let mut emitter = YamlEmitter::new(&mut out_str);
emitter.dump(doc).unwrap(); // dump the YAML object to a String


}
