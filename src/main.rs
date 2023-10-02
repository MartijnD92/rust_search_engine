use std::{
    fs::File,
    io::{self, BufReader}
};
use xml::reader::{EventReader, XmlEvent};

fn parse_xml_doc(file_path: &str) -> io::Result<String> {
    let file = File::open(file_path)?;
    let file = BufReader::new(file);
    let er = EventReader::new(file);

    let content: String = er.into_iter().filter_map(|event| match event {
        Ok(XmlEvent::Characters(e)) => Some(e),
        _ => None
    }).collect();

    Ok(content)
}

fn main() {
    let file_path = "data/gl2/glAccum.xhtml";
    let doc = parse_xml_doc(file_path).expect("ERROR: could not read {file_path}");
    println!("{doc}");
}
