use std::{borrow::Cow, io::{self, BufReader}, path::Path, fs};

use quick_xml::{de::from_reader, DeError};

use domain::acco::TextAcco;

#[derive(Debug)]
pub enum FileData {
    Xml(TextAcco),
    Docx,
    Odt,
}

#[derive(Debug)]
enum ContentType {
    Xml,
    Docx,
    Odt,
    Dir(String),
    None(String),
}

impl<'a> From<Cow<'a, Path>> for ContentType {
    fn from(value: Cow<Path>) -> Self {
        if value.is_dir() {
            return ContentType::Dir(value.display().to_string());
        }
        value
            .extension()
            .map(|e| match e {
                _ if e == "xml" => ContentType::Xml,
                _ if e == "docx" => ContentType::Docx,
                _ if e == "odt" => ContentType::Odt,
                _ => ContentType::None(value.display().to_string()),
            })
            .unwrap_or(ContentType::None(value.display().to_string()))
    }
}

pub fn extract<F, P>(p: P, f: &F) -> Result<(), io::Error>
where
    F: Fn(&FileData),
    P: AsRef<Path>,
{
    run_reading_tar(p).map(|data| {
        data.iter().for_each(|d| {
            f(d);
        });
    })
}

pub fn run_reading_tar<P>(p: P) -> Result<Vec<FileData>, io::Error>
where
    P: AsRef<Path>,
{
    let tar_gz = fs::File::open(p)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    let entries = archive.entries()?;
    Ok(entries
        .filter_map(|e| {
            if let Ok(f) = e {
                let (_raw, content_type) = {
                    let p = f.path().expect("could not get path");
                    let raw = p.display().to_string();
                    let content_type: ContentType = p.into();
                    (raw, content_type)
                };
                match content_type {
                    ContentType::Xml => {
                        let br = BufReader::new(f);
                        match from_reader(br) as Result<TextAcco, DeError> {
                            Ok(v) => Some(FileData::Xml(v)),
                            Err(_e) => None,
                        }
                    }
                    ContentType::Docx => {
                        // println!("unhandled docx reading");
                        Some(FileData::Docx)
                    }
                    ContentType::Odt => {
                        // println!("unhandled odt  reading");
                        // handle odt extraction
                        Some(FileData::Odt)
                    }
                    ContentType::None(_p) | ContentType::Dir(_p) => {
                        // println!("{}", p);
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}
