use rudg::rs2dot;
use std::fs;
use std::path::{Path, PathBuf};

const SRC_DIR : &str = "src/";

/// The class diagramm is generated from the source path into the destionation path
fn generate_class_diagramm(src: &Path, dest: &Path){
    println!("generating class diagrams");
    let unescaped_results = rs2dot(src);
    let results = escape_results(&unescaped_results);


    let mut target_name = PathBuf::new();
    target_name.push(Path::new("target/docs/diagrams/class-diagram")
        .join(dest)
        .join(src.file_stem().unwrap()));
    target_name.set_extension("dot");


    fs::create_dir_all(target_name.clone().parent().unwrap()).expect("failed to create directories");
    fs::File::create(target_name.clone()).expect("failed to create file");
    println!("generating class diagrams done");

    match fs::write(&target_name, results) {
        Ok(_) => println!("File successfully written to {:?}", target_name),
        Err(e) => {
            println!("target_name is {:?}", target_name);
            println!("{:?}", e);
        }
    }
}

fn escape_results(results:&String)->String{
    let mut escaped_str = "".to_string();
    let lines = results.split_inclusive("\n");
    for result in lines {
        let mut splits  = result.split_inclusive("label");
        escaped_str = escaped_str + splits.next().unwrap();
        for split in splits{
            let escaped = split.replace("<", "\\<")
                .replace(">", "\\>");
            escaped_str = escaped_str + &escaped;
        }
    }

    escaped_str
}

fn main() {
    let mut dirs = vec![
        PathBuf::from(SRC_DIR)
    ];

    while !dirs.is_empty() {
        let dir = dirs.pop().unwrap();
        let dir_entries = fs::read_dir(dir)
            .expect("failed to read content of dir");

        for dir_entrie in dir_entries{
            let entry = dir_entrie.expect("Failed to get entry");
            let file_type = entry.file_type().expect("Failed to get file type");
            println!("{},{}",entry.path().as_path().to_str().unwrap().to_string(),file_type.is_file());
            if file_type.is_file() {
                let mut dest_dir_buf = entry.path();
                dest_dir_buf.pop();
                let dest_dir = dest_dir_buf.strip_prefix(SRC_DIR).unwrap();
                generate_class_diagramm(entry.path().as_path(), dest_dir)
            }else{
                dirs.push(entry.path());
            }
        }
    }
}
