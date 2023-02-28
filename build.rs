use rudg::rs2dot;
use serde::{Deserialize, Serialize};
use serde_yaml;
use sha256::digest;
use std::collections::HashMap;
use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::vec;

const STR_PARENT_DIR: &str = "../";
const RTD_DEFAULT_IMAGE_WIDTH: f32 = 750.0;
const RTD_FIGURE_HINT: &str =
    ".. hint:: If the diagrams are not shown big enough to read, please click on them.\n";
const RTD_BASE_FOLDER: &str = "docs/source/033_class_diagrams_generated";
const RTD_CLASS_DG_INDEX_TEMPLATE: &str = "docs/templates/class_dg_index.rst";
const RTD_CLASS_DG_FIGURE_TEMPLATE: &str = "docs/templates/class_dg_figure.rst";
const RTD_CLASS_DG_BLANK_TEMPLATE: &str = "docs/templates/class_dg_blank.rst";
const DIAG_TYPE: &str = "png";
const DIAG_PATH: &str = "planning/diagrams/classdg_generated";
const DOT_PATH: &str = "target/diagrams/classdg_generated";
const CLASS_DIAG_PERSISTENCE_FILE: &str = "persistence.yml";

#[derive(Debug, Deserialize, Serialize, Clone, PartialOrd, PartialEq, Eq)]
struct RtdFigure {
    figure_path: String,
    figure: String,
}

impl Ord for RtdFigure {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.figure_path.cmp(&other.figure_path)
    }
}

pub fn generate_read_the_docs_class_diagrams() {
    let index_template = fs::read_to_string(Path::new(RTD_CLASS_DG_INDEX_TEMPLATE))
        .expect("failed to read index template");
    let figure_template =
        fs::read_to_string(RTD_CLASS_DG_FIGURE_TEMPLATE).expect("failed to read figure template");
    let blank_template =
        fs::read_to_string(RTD_CLASS_DG_BLANK_TEMPLATE).expect("failed to read figure template");

    let mut dirs = vec![PathBuf::from(DIAG_PATH)];

    while !dirs.is_empty() {
        let dir = dirs.pop().unwrap();
        let dir_entries = fs::read_dir(dir.clone()).expect("failed to read content of dir");
        let outfile_relative_path = dir
            .as_path()
            .strip_prefix(DIAG_PATH)
            .unwrap()
            .as_os_str()
            .to_str()
            .unwrap();
        let outfile_path = format!("{}/{}/index.rst", RTD_BASE_FOLDER, outfile_relative_path);
        let mut out_dir = PathBuf::from(outfile_path.clone());
        out_dir.pop();

        let mut subdirs: Vec<String> = vec![];
        let mut figures: Vec<RtdFigure> = vec![];

        for dir_entrie in dir_entries {
            let entry = dir_entrie.expect("Failed to get entry");
            let file_type = entry.file_type().expect("Failed to get file type");

            if file_type.is_file() {
                let is_diagram = entry
                    .file_name()
                    .as_os_str()
                    .to_str()
                    .unwrap()
                    .to_string()
                    .ends_with(DIAG_TYPE);
                if !is_diagram {
                    continue;
                }

                let path = entry.path().as_os_str().to_str().unwrap().to_string();
                let mut template = figure_template.clone();
                template = template
                    .replace("$ImgName", entry.file_name().to_str().unwrap())
                    .replace("$Label", &get_rtd_figure_label(entry));

                let levels_up = outfile_path.split("/").count() - 1;
                let figure_path = format!("{}{}", STR_PARENT_DIR.repeat(levels_up), path);
                template = template.replace("$Path", figure_path.as_str());

                template = template.replace(
                    "$Width_Percentage",
                    RTD_DEFAULT_IMAGE_WIDTH.to_string().as_str(),
                );
                figures.push(RtdFigure {
                    figure_path: figure_path.to_string(),
                    figure: template.clone(),
                });
            } else {
                let entry_path = entry.path().as_os_str().to_str().unwrap().to_string();
                let dir_name = entry_path.split("/").last().unwrap();

                let mut subdir = "        ".to_string();
                subdir.push_str(format!("{}/index", dir_name).as_str());
                subdir.push('\n');
                subdirs.push(subdir);

                dirs.push(entry.path());
            }
        }

        let mut index = if !subdirs.is_empty() {
            subdirs.sort();
            let subdir_str = subdirs.join("");
            index_template.replace("$Diagram_Tree", &subdir_str)
        } else {
            String::new()
        };
        if !figures.is_empty() {
            figures.sort();
            let figure_str: String = figures.iter().map(|figure| figure.figure.clone()).collect();

            index.push_str(RTD_FIGURE_HINT);
            index.push_str(&figure_str);
        }

        let label = out_dir.as_os_str().to_str().unwrap().replace("/", "_");
        let content = blank_template
            .replace("$Content", index.as_str())
            .replace("$Label", label.as_str())
            .replace("$Heading", &get_label_heading(out_dir.clone()));

        fs::create_dir_all(out_dir.as_os_str().to_str().unwrap().to_string())
            .expect("failed to create dir");
        fs::write(outfile_path, content).expect("failed to write file");
    }
}

fn get_label_heading(out_dir: PathBuf) -> String {
    let entry_path = out_dir.as_os_str().to_str().unwrap().to_string();
    let dir_name = entry_path.split("/").last().unwrap();

    if dir_name != RTD_BASE_FOLDER.split("/").last().unwrap() {
        dir_name.to_string()
    } else {
        "Class diagrams".to_string()
    }
}

fn get_rtd_figure_label(entry: DirEntry) -> String {
    let file_name = entry
        .file_name()
        .to_str()
        .unwrap()
        .to_string()
        .replace(&format!(".{}", DIAG_TYPE), "");
    let label = if file_name == "mod" {
        let entry_path = entry.path().as_os_str().to_str().unwrap().to_string();
        let path_split: Vec<&str> = entry_path.split("/").collect();
        path_split[path_split.len() - 2].to_string()
    } else {
        file_name
    };
    label.replace("_", " ")
}

#[derive(Debug, Clone)]
struct DiagramRoot {
    root_folder: String,
    sub_folder_diagram: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialOrd, PartialEq, Eq)]
struct ClassDiagramPersistence {
    diag_file: String,
    dot_hash: String,
}

impl Ord for ClassDiagramPersistence {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.diag_file.cmp(&other.diag_file)
    }
}

/// The class diagramm is generated from the source path into the destionation path
fn converte_to_visual(dot_str: String, dest_dot: PathBuf, dest_diag: PathBuf) {
    println!("generating class diagrams");

    fs::create_dir_all(dest_dot.clone().parent().unwrap()).expect("failed to create directories");
    fs::create_dir_all(dest_diag.clone().parent().unwrap()).expect("failed to create directories");
    write_to_file(PathBuf::from(dest_dot.clone()), dot_str);

    let dot_file_name = dest_dot.clone().into_os_string().into_string().unwrap();
    let svg_file_name = dest_diag.into_os_string().into_string().unwrap();

    converte_to_svg(dot_file_name, svg_file_name);
}

fn write_to_file(target_name: PathBuf, results: String) {
    let mut file = fs::File::create(target_name.clone()).expect("failed to create file");

    match file.write_all(results.as_bytes()) {
        Ok(_) => {
            println!("File successfully written to {:?}", target_name);
        }
        Err(e) => {
            println!("target_name is {:?}", target_name);
            println!("{:?}", e);
            return;
        }
    }
}

fn converte_to_svg(input_file_path: String, output_file_path: String) {
    let file = fs::File::create(output_file_path.clone()).expect("failed to create file");
    drop(file);

    // Do not insert space between -o{value} und -T{value}
    let result = Command::new("dot")
        .arg(input_file_path.clone())
        .arg(format!("-T{DIAG_TYPE}"))
        .arg(format!("-o{output_file_path}"))
        .spawn();

    if let Err(error) = result {
        println!("failed to converte {input_file_path} to {output_file_path} {error}");
    }
}

fn get_class_diag_persistence(root: DiagramRoot) -> HashMap<String, String> {
    let path = &format!(
        "{}/{}{}",
        DIAG_PATH, root.sub_folder_diagram, CLASS_DIAG_PERSISTENCE_FILE
    );
    let persistence_path = Path::new(path);
    let persistence = if persistence_path.exists() {
        let persistence_file = File::open(persistence_path).unwrap();
        let persistence: Result<Vec<ClassDiagramPersistence>, serde_yaml::Error> =
            serde_yaml::from_reader(persistence_file);

        match persistence {
            Ok(p) => p,
            Err(_) => vec![],
        }
    } else {
        vec![]
    };

    let mut result: HashMap<String, String> = HashMap::new();
    for p in persistence {
        result.insert(p.diag_file, p.dot_hash);
    }
    result
}

fn write_diag_persistence(root: DiagramRoot, persistence: HashMap<String, String>) {
    let mut value: Vec<ClassDiagramPersistence> = vec![];
    for (src, hash) in persistence {
        value.push(ClassDiagramPersistence {
            diag_file: src,
            dot_hash: hash,
        });
    }
    value.sort();

    let path = &format!(
        "{}/{}{}",
        DIAG_PATH, root.sub_folder_diagram, CLASS_DIAG_PERSISTENCE_FILE
    );
    let persistence_path = Path::new(path);

    fs::create_dir_all(persistence_path.parent().unwrap()).expect("failed to create dir for diags");
    let persistence_file = File::create(persistence_path).unwrap();
    serde_yaml::to_writer(persistence_file, &value)
        .expect("failed to write diag persistence value to file");
}

fn get_dot_file_path(root: DiagramRoot, src: &Path) -> PathBuf {
    let mut dest_dir_buf = PathBuf::from(src);
    dest_dir_buf.pop();
    let dest_dir = dest_dir_buf.strip_prefix(root.root_folder).unwrap();
    let mut target_name = PathBuf::new();
    target_name.push(
        Path::new(DOT_PATH)
            .join(dest_dir)
            .join(src.file_stem().unwrap()),
    );
    target_name.set_extension("dot");
    target_name
}

fn get_diag_file_path(root: DiagramRoot, src: &Path) -> PathBuf {
    let mut dest_dir_buf = PathBuf::from(src);
    dest_dir_buf.pop();
    let dest_dir = dest_dir_buf.strip_prefix(root.root_folder).unwrap();
    let mut target_name = PathBuf::new();
    target_name.push(
        Path::new(DIAG_PATH)
            .join(root.sub_folder_diagram)
            .join(dest_dir)
            .join(src.file_stem().unwrap()),
    );
    target_name.set_extension(DIAG_TYPE);
    target_name
}

fn is_dot_string_empty(dot_str: String) -> bool {
    let mut content_started = false;
    for c in dot_str.chars() {
        if c == '{' {
            content_started = true;
            continue;
        }
        if content_started && c != ' ' && c != '\r' && c != '\n' && c != '}' {
            return false;
        }
    }
    true
}

fn format_code() {
    let result = Command::new("cargo").arg("fmt").spawn();
    if let Err(error) = result {
        panic!("failed ro run cargo fmt {error}");
    }
}

fn generate_diagrams(root: DiagramRoot) {
    let mut dirs = vec![PathBuf::from(root.root_folder.clone())];

    let diag_persistence = get_class_diag_persistence(root.clone());
    let mut new_persistence: HashMap<String, String> = HashMap::new();

    while !dirs.is_empty() {
        let dir = dirs.pop().unwrap();
        println!(
            "reading dir {}",
            dir.as_os_str().to_str().unwrap().to_string()
        );
        let dir_entries = fs::read_dir(dir.clone()).expect("failed to read content of dir");

        for dir_entrie in dir_entries {
            let entry = dir_entrie.expect("Failed to get entry");
            let file_type = entry.file_type().expect("Failed to get file type");
            println!(
                "{},{}",
                entry.path().as_path().to_str().unwrap().to_string(),
                file_type.is_file()
            );
            if file_type.is_file() {
                let results = rs2dot(entry.path().as_path());
                if is_dot_string_empty(results.clone()) {
                    continue;
                }

                let dest_dot = get_dot_file_path(root.clone(), entry.path().as_path());
                let dest_diag = get_diag_file_path(root.clone(), entry.path().as_path());
                let dest_diag_str = dest_diag.as_os_str().to_str().unwrap().to_string();

                let new_hash = digest(results.clone());
                if let Some(persistence_hash) = diag_persistence.get(&dest_diag_str) {
                    if *persistence_hash == new_hash {
                        new_persistence.insert(dest_diag_str, new_hash);
                        continue;
                    }
                }

                converte_to_visual(results, dest_dot, dest_diag);
                new_persistence.insert(dest_diag_str, new_hash);
            } else {
                dirs.push(entry.path());
            }
        }
    }

    for file in diag_persistence.keys() {
        if !new_persistence.contains_key(file) {
            fs::remove_file(file).expect("failed to delete old dot file");
        }
    }

    write_diag_persistence(root, new_persistence);
}

fn main() {
//    format_code();

    let diagram_roots: Vec<DiagramRoot> = vec![
        DiagramRoot {
            root_folder: "src/".to_string(),
            sub_folder_diagram: "windowmanager/".to_string(),
        },
        DiagramRoot {
            root_folder: "extensions/oxide-msg/src/".to_string(),
            sub_folder_diagram: "extensions/oxide-msg/".to_string(),
        },
        DiagramRoot {
            root_folder: "extensions/oxide-bar/src/".to_string(),
            sub_folder_diagram: "extensions/oxide-bar/".to_string(),
        },
        DiagramRoot {
            root_folder: "extensions/oxide-ipc/src/".to_string(),
            sub_folder_diagram: "extensions/oxide-ipc/".to_string(),
        },
    ];

    for root in diagram_roots {
        generate_diagrams(root);
    }

    fs::remove_dir_all(RTD_BASE_FOLDER).expect("failed to clean read the docs dir");
    fs::create_dir(RTD_BASE_FOLDER).expect("failed to create read the docs dir");
    generate_read_the_docs_class_diagrams();
}
