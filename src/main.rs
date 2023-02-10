use std::path::{Path, PathBuf};
use std::env;
use std::fs;

#[derive(Debug)]
enum Tree {
    Node(PathBuf),
    Leaf(PathBuf),
}

fn is_problem(path: &Path) -> bool {
    let requirements = vec!["editorial", "teste", "surse", "enunt"];

    requirements.into_iter().all(|name| {
        path.join(name).is_dir()
    })
}

fn collect_problems(path: &Path, result: &mut Vec<PathBuf>) {
    if is_problem(path) {
        result.push(path.to_path_buf());
        return;
    }

    let mut entries: Vec<PathBuf> = path.read_dir()
        .unwrap()
        .map(|res| { res.unwrap().path() })
        .filter(|res| { res.is_dir() } )
        .collect();

    entries.sort();

    for e in entries {
        if e.is_dir() {
            collect_problems(&e, result);
        }
    }
}

fn build_tree(path: &Path, result: &mut Vec<Tree>) -> bool {
    if is_problem(path) {
        result.push(Tree::Leaf(path.to_path_buf()));
        return true;
    }

    let mut important = false;

    result.push(Tree::Node(path.to_path_buf()));

    let mut entries: Vec<PathBuf> = path.read_dir()
        .unwrap()
        .map(|res| { res.unwrap().path() })
        .filter(|res| { res.is_dir() })
        .collect();
    
    entries.sort();

    for e in entries {
        if e.is_dir() && build_tree(&e, result) {
            important = true;
        }
    }

    if !important {
        result.pop();
    }

    important
}

fn is_dir_empty(path: &Path) -> bool {
    path.read_dir()
        .unwrap()
        .count() == 0
}

fn is_dir_broken(path: &Path) -> bool {
    path.join("broken.md")
        .is_file()
}

fn get_verdict(path: &Path) -> &str {
    match (is_dir_empty(&path), is_dir_broken(&path)) { 
        (true, false) => "Gol",
        (true, true)  => panic!("Can't have an empty dir and a broken one"),
        (false, true) => "Incomplet",
        (false, false) => "Ok",
    }
}

fn markdown_from_problems(files: &Vec<PathBuf>, prefix: &str) -> String {
    let mut result = String::new();

    result.push_str("| Nume | Enunt | Teste | Editorial | Surse |\n");
    result.push_str("| ---- | ----- | ----- | --------- | ----- |\n");

    for e in files {
        let line = e.to_str().unwrap();

        result = result + format!("| {} | {} | {} | {} | {} |\n", 
            line.strip_prefix(prefix).unwrap(),
            get_verdict(&e.join("enunt")),
            get_verdict(&e.join("teste")),
            get_verdict(&e.join("editorial")),
            get_verdict(&e.join("surse"))).as_str();
    }

    result
}

fn markdown_from_tree(tree: &Vec<Tree>, prefix: &Path) -> String {
    let mut result = String::new();
    let mut started_table = false;

    let requirements = vec!["enunt", "teste", "editorial", "surse"];

    for e in tree {
        match e {
            Tree::Node(path) => {
                if (started_table) {
                    result.push('\n');
                    started_table = false;
                }

                let relative_dirname = path
                    .strip_prefix(prefix.parent().unwrap()).unwrap();
                let count = relative_dirname.components().count();

                let dirname = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();

                result = result + &"#".to_string().repeat(count) + &format!(" {}\n\n", dirname);
            },
            Tree::Leaf(path) => {
                if !started_table {
                    started_table = true;
                    result.push_str("| Nume | Enunt | Teste | Editorial | Surse |\n");
                    result.push_str("| ---- | ----- | ----- | --------- | ----- |\n");
                }
            
                let dirname = path.
                    file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();

                result = result + format!("| {} | {} | {} | {} | {} |\n",
                    dirname,
                    get_verdict(&path.join("enunt")),
                    get_verdict(&path.join("teste")),
                    get_verdict(&path.join("editorial")),
                    get_verdict(&path.join("surse"))).as_str();
            },
        }
    }

    result
}

fn main() {
    let mut args = env::args();
    args.next();

    for argument in args {
        let mut tree: Vec<Tree> = Vec::new();

        let path = Path::new(&argument)
            .canonicalize()
            .unwrap();

        build_tree(&path, &mut tree);

        println!("{}", markdown_from_tree(&tree, &path));
    }
}
