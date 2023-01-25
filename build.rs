use rudg::rs2dot;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

const SRC_DIR : &str = "src/";
const DIAG_TYPE : &str = "png";

/// The class diagramm is generated from the source path into the destionation path
fn generate_class_diagramm(src: &Path, dest: &Path){
    println!("generating class diagrams");
    let results = rs2dot(src);

    let mut target_name = PathBuf::new();
    target_name.push(Path::new("target/docs/diagrams/class-diagram")
        .join(dest)
        .join(src.file_stem().unwrap()));
    target_name.set_extension("dot");

    fs::create_dir_all(target_name.clone().parent().unwrap()).expect("failed to create directories");
    write_to_file(target_name.clone(), results);

    let dot_file_name = target_name.clone().into_os_string().into_string().unwrap();
    let svg_file_name = dot_file_name.replace("dot", DIAG_TYPE);

    match Command::new("dot").spawn() {
        Ok(_) => {
            println!("converting {dot_file_name} to {svg_file_name}");
            converte_to_svg(dot_file_name, svg_file_name);
        },
        Err(_) => {
            println!("Not converting graphviz files")
        }
    };
}

fn write_to_file(target_name: PathBuf, results: String){
    let mut file = fs::File::create(target_name.clone()).expect("failed to create file");

    match file.write_all(results.as_bytes()) {
        Ok(_) => {
            println!("File successfully written to {:?}", target_name);
        },
        Err(e) => {
            println!("target_name is {:?}", target_name);
            println!("{:?}", e);
            return;
        }
    }
}

fn converte_to_svg(input_file_path:String, output_file_path:String){
    let file = fs::File::create(output_file_path.clone()).expect("failed to create file");
    drop(file);

    // Do not insert space between -o{value} und -T{value}
    let result = Command::new("dot")
        .arg(input_file_path.clone())
        .arg(format!("-T{DIAG_TYPE}"))
        .arg(format!("-o{output_file_path}"))
        .spawn();

    if let Err(error)=result{
        panic!("failed to converte {input_file_path} to {output_file_path} {error}");
    }
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
