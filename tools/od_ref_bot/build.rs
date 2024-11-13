use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

const SOURCE_DIR: &str = "../../content";

fn main() -> Result<(), Box<dyn Error>> {
    let dest_path = Path::new("./src/content.rs");
    let from_path = std::env::var("SOURCE_DIR").unwrap_or(SOURCE_DIR.to_string());

    let mut out_files = File::create(dest_path)?;

    writeln!(
        &mut out_files,
        r##"use std::collections::HashMap;pub fn get_all() -> HashMap<&'static str, &'static str> {{ let mut out = HashMap::new();"##,
    )?;

    let _ = visit_dir(&mut out_files, from_path.as_str(), &from_path);

    writeln!(&mut out_files, r##"out}}"##,)?;

    Ok(())
}

fn visit_dir(file: &mut File, dir: &str, from_dir: &String) -> Result<(), Box<dyn Error>> {
    for inner_file in fs::read_dir(dir)? {
        let inner_file = inner_file?;

        let file_type = inner_file.file_type()?;

        if !file_type.is_file() {
            if file_type.is_dir() {
                let _ = visit_dir(file, inner_file.path().to_str().unwrap(), from_dir);
            }

            continue;
        }

        writeln!(
            file,
            r##"out.insert("{name}", include_str!(r#"{path}"#));"##,
            name = inner_file
                .path()
                .strip_prefix(from_dir)
                .unwrap()
                .to_string_lossy()
                .replace('\\', "/"),
            path = inner_file.path().canonicalize().unwrap().display(),
        )?;
    }

    Ok(())
}
