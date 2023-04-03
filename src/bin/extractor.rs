use extractor::extract;

fn main() {
    let args = std::env::args();
    if args.len() < 2 {
        eprintln!("a directory path is needed");
        std::process::exit(1);
    }
    let filepath = match args.last() {
        Some(p) => p,
        None => {
            eprintln!("a directory path is needed");
            std::process::exit(1);
        }
    };
    println!("filepath: {filepath}");

    extract(filepath, &|e| {
        println!("file data: {e:?}")
    }).expect("should not err...");
}