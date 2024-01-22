use std::fs;
use std::io::Write;

pub enum MarkdownElement {
    Header1(String),
    Header2(String),
    Header3(String),
    ListItem(String),
    Paragraph(String),
    NewLine,
}

pub fn parse(token: &str) -> MarkdownElement {
    if token.is_empty() {
        MarkdownElement::NewLine
    } else if let Some(stripped) = token.strip_prefix("# ") {
        MarkdownElement::Header1(stripped.to_string())
    } else if let Some(stripped) = token.strip_prefix("## ") {
        MarkdownElement::Header2(stripped.to_string())
    } else if let Some(stripped) = token.strip_prefix("### ") {
        MarkdownElement::Header3(stripped.to_string())
    } else if let Some(stripped) = token.strip_prefix("- ") {
        MarkdownElement::ListItem(stripped.to_string())
    } else {
        MarkdownElement::Paragraph(token.to_string())
    }
}

pub fn lexical(path: std::path::PathBuf) -> std::io::Result<()> {
    if path.is_file() && path.extension() == Some(std::ffi::OsStr::new("md")) {
        let mut output_file = fs::File::create(path.with_extension("html"))?;
        match std::fs::read_to_string(&path) {
            Ok(contents) => {
                println!("File name is {}", path.display());
                let tokens: Vec<&str> = contents.lines().collect();
                let mut in_list = false;
                for token in tokens {
                    let element: MarkdownElement = parse(token);
                    match &element {
                        MarkdownElement::ListItem(_) if !in_list => {
                            writeln!(output_file, "<ul>")?;
                            in_list = true;
                        }
                        _ if in_list && !matches!(element, MarkdownElement::ListItem(_)) => {
                            writeln!(output_file, "</ul>")?;
                            in_list = false;
                        }
                        _ => {}
                    }
                    match element {
                        MarkdownElement::Header1(text) => {
                            writeln!(output_file, "<h1>{}</h1>", text)?
                        }
                        MarkdownElement::Header2(text) => {
                            writeln!(output_file, "<h2>{}</h2>", text)?
                        }
                        MarkdownElement::Header3(text) => {
                            writeln!(output_file, "<h3>{}</h3>", text)?
                        }
                        MarkdownElement::ListItem(text) => {
                            writeln!(output_file, "<li>{}</li>", text)?
                        }
                        MarkdownElement::Paragraph(text) => {
                            writeln!(output_file, "<p>{}</p>", text)?
                        }
                        MarkdownElement::NewLine => writeln!(output_file)?,
                    }
                }
            }
            Err(e) => println!("Failed to read {}: {}", path.display(), e),
        }
    }
    Ok(())
}
