use std::path::{Path, PathBuf};
use std::env;
use std::fs;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Write the markdown into the file.
    #[arg(short, long)]
    write: bool,

    /// Target folder to summarize.
    #[arg()]
    folder: String,

    /// Display a big table with all the problems.
    #[arg(short, long)]
    table: bool,

    /// Overwrite already existing content.
    #[arg(short, long)]
    overwrite: bool,

    /// Make a README for every directory that contains a problem.
    #[arg(short, long)]
    recursive: bool,
}

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

fn markdown_from_problems(files: &Vec<PathBuf>, prefix: &Path) -> String {
    let mut result = String::new();

    result.push_str("| Nume | Enunt | Teste | Editorial | Surse |\n");
    result.push_str("| ---- | ----- | ----- | --------- | ----- |\n");

    for e in files {
        let line = e.to_str().unwrap();

        result = result + format!("| {} | {} | {} | {} | {} |\n", 
            line.strip_prefix(prefix.to_str().unwrap()).unwrap(),
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
                if started_table {
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

fn write_into_file(path: &Path, content: &str) {
    fs::write(path, content)
        .unwrap();
}

fn make_markdown_table(path: &Path) -> String {
    let mut problems: Vec<PathBuf> = Vec::new();

    collect_problems(&path, &mut problems);

    markdown_from_problems(&problems, &path)
}

fn make_markdown_readme(path: &Path) -> String {
    let mut tree: Vec<Tree> = Vec::new();

    build_tree(&path, &mut tree);

    markdown_from_tree(&tree, &path)
}

fn replace_summary(path: &Path, new_content: &str) -> String {
    let mut result = String::new();

    result = fs::read_to_string(path)
        .unwrap();

    result = match result.find("# Generated Summary") {
        Some(pos) => {
            result[..pos].to_string()
        },
        None => {
            result
        }
    };

    result = result + "# Generated Summary\n\n" + new_content;

    result
}

fn create_readme_recursive(path: &Path, args: &Args) {
    let mut tree: Vec<Tree> = Vec::new();

    build_tree(&path, &mut tree);

    let dirs: Vec<&PathBuf> = tree.iter()
        .filter(|node| {
            match node {
                Tree::Node(path) => true,
                Tree::Leaf(path) => false,
            }
        })
        .map(|node| {
            match node {
                Tree::Node(path) => path,
                Tree::Leaf(path) => path,
            }
        })
        .collect();

    for dir in dirs {
        create_readme(dir, args);    
    }
}

fn create_readme(path: &Path, args: &Args) {
    let mut markdown = if (args.table) {
        make_markdown_table(&path)
    } else {
        make_markdown_readme(&path)
    };

    if !args.overwrite {
        markdown = replace_summary(&path.join("README.md"), &markdown);
    }

    if args.write {
        write_into_file(&path.join("README.md"), &markdown);
    } else {
        println!("{}", markdown);
    }
}

fn main() {
    let args = Args::parse();
    
    let path = Path::new(&args.folder)
        .canonicalize()
        .unwrap();
    
    if args.recursive {
        create_readme_recursive(&path, &args);
    } else {
        create_readme(&path, &args);
    }
}

