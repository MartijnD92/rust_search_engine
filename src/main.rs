use std::{
    env,
    fs::{self, File},
    io::{self, BufReader},
    path::Path,
    process::{exit, ExitCode},
};
use xml::reader::{EventReader, XmlEvent};

fn parse_xml_file(file_path: &Path) -> Result<String, ()> {
    let file = File::open(file_path).map_err(|err| {
        eprintln!(
            "ERROR: coult not open file {file_path}: {err}",
            file_path = file_path.display()
        );
    })?;
    let file = BufReader::new(file);
    let er = EventReader::new(file);

    let content: String = er
        .into_iter()
        .filter_map(|event| match event {
            Ok(XmlEvent::Characters(e)) => Some(e),
            _ => None,
        })
        .collect();

    Ok(content)
}

fn parse_txt_file(file_path: &Path) -> Result<String, ()> {
    fs::read_to_string(file_path).map_err(|err| {
        eprintln!(
            "ERROR: coult not open file {file_path}: {err}",
            file_path = file_path.display()
        );
    })
}

fn parse_file_by_extension(file_path: &Path) -> Result<String, ()> {
    let extension = file_path
        .extension()
        .ok_or_else(|| {
            eprintln!(
                "ERROR: could not parse file extension for {file_path}.",
                file_path = file_path.display()
            );
        })?
        .to_string_lossy();

    match extension.as_ref() {
        "xml" | "xhtml" | "html" => parse_xml_file(file_path),
        "txt" => parse_txt_file(file_path),
        _ => {
            eprintln!(
                "ERROR: can't detect file type of {file_path}: unsupported extension {extension}",
                file_path = file_path.display(),
                extension = extension
            );
            Err(())
        }
    }
}

fn entry() -> Result<(), ()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: not enough arguments provided");
        return Err(());
    }
    let file_path: &str = &args[1];
    let file_path = Path::new(&file_path);

    let doc = match parse_file_by_extension(file_path) {
        Ok(content) => content,
        Err(()) => return Err(()),
    };
    println!("{doc}");

    Ok(())
}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}
