use std::ascii::AsciiExt;

fn usage() {
    println!("./comp_create <name 1> <name 2> ... <name n>");
}

fn process(input: &str) -> Option<String> {
    let proc: String = input
        .chars()
        .map(|c| {
            if c == ' ' {
                '_'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect();
    if proc.len() == 0 {
        return None;
    }
    Some(proc)
}

fn main() {
    // let args: Vec<_> = std::env::args().collect();
    // if args.len() < 1 {
    //     usage();
    //     std::process::exit(1);
    // }

    let args = vec!["Single source shortest path, non-negative weights"];

    for words in args {
        let filename = process(&words).unwrap_or_else(|| {
            eprintln!("ERROR: unable to process {words} because the length of the filename that was created is 0.");
            std::process::exit(1);
        });
        println!("{filename}");
    }
}
