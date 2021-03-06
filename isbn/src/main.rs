extern crate xml;
use std::env;

use xml::reader::{EventReader, XmlEvent};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {

    let txt = match env::args().nth(1) {
        None => "".to_string(),
        Some(isbn) => {
            let url = "https://iss.ndl.go.jp/api/sru?operation=searchRetrieve&query=isbn=";
            let t: String = format!("{}{}&recordSchema=dcndl_simple",url, isbn);
            let mut res = surf::get(t).await?;
            let body = res.body_string().await?;
            body
        }
    };
    // println!("{}", txt);
    let parser = EventReader::new(txt.as_bytes());
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name_matcher(&name.local_name) {
                    print!("{}", name.local_name);
                    depth += 1;
                }
                if &name.local_name == "record" {
                    println!("");
                } 
            }
            Ok(XmlEvent::Characters(s)) => {
                if depth != 0 {
                    print!(" : {} ", s);
                }
            }
            Ok(XmlEvent::EndElement { .. }) => {
                if depth != 0 {
                    depth -= 1;
                    println!("");
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    Ok(())
}

fn name_matcher (name: &str) -> bool {
    match name {
        "identifier"| "title"| "creator" | "publisher" | "price" => true,
        _ => false,
    }
}