use std::fs;
use std::path::{Path, PathBuf};

fn is_problem(path: &Path) -> bool {
    let requirements = vec!["editorial", "teste", "surse", "enunt"];

    requirements.into_iter().all(|name| {
        path.join(name).is_dir()
    })
}

fn collect_problems(path: &Path, result: &mut Vec<PathBuf>) {
    if is_problem(path) {
        result.push(path.to_path_buf());
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

fn is_dir_empty(path: &Path) -> bool {
    path.read_dir()
        .unwrap()
        .count() == 0
}

fn generate_markdown(files: &Vec<PathBuf>) -> String {
    let mut result = String::new();

    result.push_str("| Nume | Enunt | Teste | Editorial | Surse |\n");
    result.push_str("| ---- | ----- | ----- | --------- | ----- |\n");

    let requirements = vec!["enunt", "teste", "editorial", "surse"];

    for e in files {
        let line = e.to_str().unwrap();

        result = result + format!("| {} |", line).as_str();

        for req in &requirements {
            let verdict = match is_dir_empty(&e.join(req)) {
                true => "Gol",
                false => "OK"
            };
            result = result + format!(" {} |", verdict).as_str();
        }

        result = result + "\n";
    }

    result
}

fn main() {
    let filename = ".";
    
    let mut problems: Vec<PathBuf> = Vec::new();

    collect_problems(Path::new("."), &mut problems);

    println!("{}", generate_markdown(&problems))
}
