use std::{env, fs::File, io::BufReader, path::Path, process};
use xml::reader::{EventReader, XmlEvent};

use core::fmt;

#[derive(Debug)]
struct TextFile;

#[derive(Debug)]
struct Xml<'a> {
    path: &'a Path,
    file: File,
    content: Option<String>,
}

impl<'a> fmt::Display for Xml<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Xml {{ path: {}, content: {:?} }} ",
            self.path.display(),
            self.content
        )
    }
}

trait Parser: fmt::Display {
    fn parse(&mut self);
}

impl<'a> Parser for Xml<'a> {
    fn parse(&mut self) {
        let file = BufReader::new(&self.file);
        let er = EventReader::new(file);

        let content: String = er
            .into_iter()
            .filter_map(|event| match event {
                Ok(XmlEvent::Characters(e)) => Some(e),
                _ => None,
            })
            .collect();

        self.content = if content.is_empty() {
            None
        } else {
            Some(content)
        }
    }
}

#[allow(clippy::new_ret_no_self)]
impl TextFile {
    fn new(path: &Path) -> impl Parser + '_ {
        let file = File::open(path)
            .map_err(|err| {
                eprintln!(
                    "ERROR: could not open file {file_path}: {err}",
                    file_path = path.display()
                );
                process::exit(1);
            })
            .unwrap();

        let extension = path
            .extension()
            .unwrap_or_else(|| {
                eprintln!(
                    "ERROR: could not parse file at {file_path}.",
                    file_path = path.display()
                );
                process::exit(1);
            })
            .to_string_lossy();

        let mut file = match extension.as_ref() {
            "xml" | "xhtml" | "html" => Xml {
                path,
                file,
                content: None,
            },
            _ => {
                eprintln!("ERROR: Unknown filetype!");
                process::exit(1);
            }
        };
        file.parse();

        file
    }
}

fn main() -> Result<(), ()> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        eprintln!("ERROR: not enough arguments provided");
        return Err(());
    }
    let file_path: &str = &args[1];
    let file_path = Path::new(&file_path);

    let document = TextFile::new(file_path);

    println!("{}", document);

    Ok(())
}
