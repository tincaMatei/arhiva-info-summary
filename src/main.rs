use std::path::{Path, PathBuf};
use std::fs;
use clap::Parser;
use serde_json::{Map};

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

    requirements
        .into_iter()
        .all(|name| {
            path.join(name).is_dir()
        })
}

fn get_dir_entries(path: &Path) -> Result<Vec<PathBuf>, ()> {
    let res = path.read_dir()
        .map_err(|_| ())?
        .filter_map(|res| { res.ok() } )
        .map(|res| { res.path() })
        .filter(|res| { res.is_dir() } )
        .collect();
    
    Ok(res)
}

fn collect_problems(path: &Path, result: &mut Vec<PathBuf>) {
    if is_problem(path) {
        result.push(path.to_path_buf());
        return;
    }

    let mut entries: Vec<PathBuf> = get_dir_entries(path)
        .unwrap_or_default();

    entries.sort();

    for e in entries {
        collect_problems(&e, result);
    }
}

fn build_tree(path: &Path, result: &mut Vec<Tree>) -> bool {
    if is_problem(path) {
        result.push(Tree::Leaf(path.to_path_buf()));
        return true;
    }

    let mut important = false;

    result.push(Tree::Node(path.to_path_buf()));

    let mut entries = get_dir_entries(path)
        .unwrap_or_default();

    entries.sort();

    for e in entries {
        if build_tree(&e, result) {
            important = true;
        }
    }

    if !important {
        result.pop();
    }

    important
}

fn is_dir_empty(path: &Path) -> bool {
    let dir_count = path.read_dir()
        .ok()
        .map_or(0, |dir| { dir.count() });

    let contains_missing = path.join("missing.md")
        .is_file();

    dir_count == 0 || contains_missing
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

fn get_mirror_links(path: &Path) -> Result<Vec<(String, String)>, String> {
    let mirror_path = path.join("mirrors.json");

    let contents = fs::read_to_string(mirror_path)
        .map_err(|x| { x.to_string() })?;

    let mirrors: Map<String, serde_json::Value> = serde_json::from_str(&contents)
        .map_err(|x| { x.to_string() })?;

    let mut res: Vec<(String, String)> = Vec::new();

    for (k, v) in mirrors {
        println!("{} {:?}", k, v);
        
        if let serde_json::Value::String(val) = v {
            res.push((k, val));
        }
    }

    Ok(res)
}

fn markdown_link(text: String, url: String) -> String {
    format!("[{}]({})", text, url).to_string()
}

fn mirrors_to_markdown(mirrors: Vec<(String, String)>) -> String {
    mirrors
        .into_iter()
        .map(|(mirror, link)| { markdown_link(mirror, link) })
        .collect::<Vec<String>>()
        .join(", ")
}

fn markdown_from_problems(files: &Vec<PathBuf>, prefix: &Path) -> String {
    let mut result = String::new();

    result.push_str("| Nume | Enunt | Teste | Editorial | Surse | Mirrors |\n");
    result.push_str("| ---- | ----- | ----- | --------- | ----- | ------- |\n");

    let prefix_str = prefix.to_str()
        .unwrap_or("");

    for e in files {
        let line = e.to_str()
            .unwrap_or("");

        let problem_name = line.strip_prefix(prefix_str)
            .unwrap_or("");

        let mirrors = get_mirror_links(e)
            .map(|val| { mirrors_to_markdown(val) })
            .unwrap_or("".to_string());

        result = result + format!("| {} | {} | {} | {} | {} | {} |\n", 
            problem_name,
            get_verdict(&e.join("enunt")),
            get_verdict(&e.join("teste")),
            get_verdict(&e.join("editorial")),
            get_verdict(&e.join("surse")),
            mirrors).as_str();
    }

    result
}

fn get_dirname_from_path(path: &Path) -> &str {
    path.file_name()
        .map(|os_str| { os_str.to_str().unwrap_or("") } )
        .unwrap_or("")
}

fn markdown_from_tree(tree: &Vec<Tree>, prefix: &Path) -> String {
    let mut result = String::new();
    let mut started_table = false;

    for e in tree {
        match e {
            Tree::Node(path) => {
                if started_table {
                    result.push('\n');
                    started_table = false;
                }

                let parent_prefix = prefix.parent()
                    .map_or(PathBuf::new(), |res| { res.to_path_buf() } );

                let relative_dirname = path
                    .strip_prefix(parent_prefix)
                    .map_or(PathBuf::new(), |res| { res.to_path_buf() } );
                
                let count = relative_dirname
                    .components()
                    .count();

                let dirname = get_dirname_from_path(path);

                result = result + &"#".to_string().repeat(count) + &format!(" {}\n\n", dirname);
            },
            Tree::Leaf(path) => {
                if !started_table {
                    started_table = true;
                    result.push_str("| Nume | Enunt | Teste | Editorial | Surse | Mirrors |\n");
                    result.push_str("| ---- | ----- | ----- | --------- | ----- | ------- |\n");
                }
            
                let dirname = get_dirname_from_path(path);

                let mirrors = get_mirror_links(path)
                    .map(|val| { mirrors_to_markdown(val) })
                    .unwrap_or("".to_string());

                result = result + format!("| {} | {} | {} | {} | {} | {} |\n",
                    dirname,
                    get_verdict(&path.join("enunt")),
                    get_verdict(&path.join("teste")),
                    get_verdict(&path.join("editorial")),
                    get_verdict(&path.join("surse")),
                    mirrors).as_str();
            },
        }
    }

    result
}

fn write_into_file(path: &Path, content: &str) {
    fs::write(path, content)
        .unwrap_or(())
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
    let mut result = match fs::read_to_string(path) {
        Ok(res) => res,
        Err(_) => String::new(),
    };

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
                Tree::Node(_) => true,
                Tree::Leaf(_) => false,
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
    let mut markdown = if args.table {
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
        .canonicalize();

    let path = match path {
        Ok(path) => path,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    
    if args.recursive {
        create_readme_recursive(&path, &args);
    } else {
        create_readme(&path, &args);
    }
}

