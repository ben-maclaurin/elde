use colored::Colorize;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

pub fn update(buf: PathBuf) {
    println!(
        "{}",
        [
            "[info] Org directory updated, fetching changes at",
            buf.as_path().to_str().unwrap()
        ]
        .join(" ")
        .bright_cyan()
    );

    let e = OsStr::new("org");

    if let Some(e) = buf.extension() {
        read(buf.as_path());
    }
}

fn read(path: &Path) -> std::io::Result<()> {
    let mut file = File::open(path).expect("Could not open file ...");

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    write(path, contents);

    Ok(())
}

fn write(path: &Path, contents: String) -> std::io::Result<()> {
    let transformed_contents = [
        "#+OPTIONS: html-postamble:nil\n
        #+OPTIONS: toc:nil\n
        #+HTML_HEAD: <link rel=\"stylesheet\" type=\"text/css\" href=\"a11yana.css\" />\n
        #+OPTIONS: html-style:nil\n
        #+OPTIONS: num:nil"
            .to_owned(),
        contents,
    ]
    .join("\n");

    let mut file =
        File::create("./static/".to_owned() + path.file_name().unwrap().to_str().unwrap())
            .expect("Error while trying to write static org file ...");

    file.write_all(transformed_contents.as_bytes())
        .expect("Error while trying to save org file");

    export("./static/".to_owned() + path.file_name().unwrap().to_str().unwrap());

    Ok(())
}

fn export(path: String) {
    Command::new("emacs")
        .arg("--batch")
        .arg(&path)
        .arg("-f")
        .arg("org-html-export-to-html")
        .output()
        .expect("Error while running Emacs export ...");

    clean(path);
}

fn clean(path: String) {
    fs::remove_file(path).expect("Error on final cleanup ...");
}
