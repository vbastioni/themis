use std::process;

use clap;
use secrecy::ExposeSecret;

use toolbox::domain::acco::Acco;
use toolbox::extractor::{self, get_files, FileData};

#[tokio::main]
async fn main() {
    let acco_flag = clap::Arg::new("index")
        .short('i')
        .long("index")
        .help("read an existing index")
        .default_value("acco");
    let matches = clap::Command::new("extractor")
        .about("DILA ACCO xml and docx extraction and ingestion")
        .version("0.1.0")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .author("vbastion")
        .subcommand(
            clap::Command::new("read")
                .short_flag('r')
                .long_flag("read")
                .about("Read an index.")
                .arg(&acco_flag),
        )
        .subcommand(
            clap::Command::new("push")
                .short_flag('p')
                .long_flag("push")
                .about("Push to an index")
                .arg(&acco_flag)
                .arg(
                    clap::Arg::new("filepath")
                        .short('f')
                        .long("file")
                        .help("path to the tar.gz file to read from")
                        .required(true),
                )
                .arg(
                    clap::Arg::new("conf")
                    .short('c')
                    .long("conf")
                    .help("configuration file path")
                    .required(false)
                )
        )
        .get_matches();
    match matches.subcommand() {
        Some(("read", read_matches)) => {
            let index_name: &String = read_matches.get_one("index").unwrap();
            let conf = extractor::Setting::get(None::<String>).unwrap();
            let meili = extractor::meili::new(
                conf.meilisearch.url(),
                conf.meilisearch.key.expose_secret(),
                index_name,
            );
            match index_name.as_str() {
                "acco" => {
                    let accos = meili.get::<Acco>().expect("no.");
                    for acco in accos.results {
                        println!("{:?}", acco);
                    }
                }
                _ => {
                    eprintln!("unknown indice `{index_name}`");
                    process::exit(1);
                }
            }
        }
        Some(("push", push_matches)) => {
            let index_name: &String = push_matches.get_one("index").unwrap();
            let filepath: &String = push_matches.get_one("filepath").unwrap();
            let confpath:Option<&String> = push_matches.get_one("conf");
            let conf = extractor::Setting::get(confpath).unwrap();
            // let meili = extractor::meili::new(
            //     conf.meilisearch.url(),
            //     conf.meilisearch.key.expose_secret(),
            //     index_name,
            // );
            let es = extractor::elastic::Elastic::builder()
                .pass(conf.elasticsearch.pass.expose_secret())
                .cert(conf.elasticsearch.cert.expose_secret())
                .build()
                .unwrap();
            match index_name.as_str() {
                "acco" => {
                    let fs = get_files(filepath).expect("could not read file list");
                    let accos = fs
                        .iter()
                        .filter_map(|fd| match fd {
                            &FileData::Xml(ref xml) => Some(xml.clone()),
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                    for chunk in accos.chunks(20) {
                        es.bulk(chunk).expect("chunk error");
                    }
                    // extract(filepath, &|e| {
                    //     if let &FileData::Xml(ref xml) = e {
                    //         let acco: Acco = xml.into();
                    //         meili.send(&acco, Some("numero")).expect("meili nope");
                    //     }
                    //     Ok(())
                    // })
                    // .expect("should not err...");
                }
                _ => {
                    eprintln!("unknown indice `{index_name}`");
                    process::exit(1);
                }
            }
        }
        _ => unreachable!(),
    }
}
