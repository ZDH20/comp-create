use std::path::PathBuf;

const CPP: &str = "#include <iostream>\n\nint main(void) {\n  // ...\n  return 0;\n}\n";
const C: &str = "#include <stdio.h>\n#include <stdlib.h>\n\nint main(void) {\n  // ...\n  return 0;\n}\n";
const PY: &str = "def main():\n    # ...\n\nif __name__ == '__main__':\n    main()\n";
const JAVA: &str = "public class Main {\n\n    public static void main(String[] args) {\n        // ...\n    }\n}\n";
const C_MAKEFILE: &str = "CC = gcc\nCFLAGS = -std=c17\n\nSRC = main.c\nOBJ = $(SRC:.c=.o)\n\n.PHONY: all clean\n\nall: main\n\nclean:\n\t$(RM) $(OBJ) main\n\nmain: $(OBJ)\n\t$(CC) $(CFLAGS) -o $@ $^\n\n%.o: %.c\n\t$(CC) $(CFLAGS) -c $< -o $@\n";
const CPP_MAKEFILE: &str = "CXX = g++\nCXXFLAGS = -std=c++20\n\nSRC = main.cpp\nOBJ = $(SRC:.cpp=.o)\n\n.PHONY: all clean\n\nall: main\n\nclean:\n\t$(RM) $(OBJ) main\n\nmain: $(OBJ)\n\t$(CXX) $(CXXFLAGS) -o $@ $^\n\n%.o: %.cpp\n\t$(CXX) $(CXXFLAGS) -c $< -o $@\n";

const DIR_FLAG: &str = "-d";
const LANG_FLAG: &str = "-l";
const FOLDER_NAME_FLAG: &str = "-f";
const HELP_FLAG: &str = "-h";

#[derive(Debug)]
enum Errors {
    EmptyCreatedFileError,
    IllegalLanguageError,
    NoLanguageError,
    NoDirectoryError,
    IllegalInputError,
    NoFolderNameError,
    UnbalancedLanguagesError,
}

#[derive(Debug)]
enum Langs {
    CPP,
    C,
    PYTHON,
    JAVA,
}

fn parse_input(tokens: &Vec<String>) -> Result<(Vec<String>, Vec<Langs>, String), Errors> {
    println!("parsing: {:?}", tokens);
    let mut langs = Vec::<Langs>::new();
    let mut folders = Vec::<String>::new();
    let (mut lang_flag, mut dir_flag, mut folder_flag) = (false, false, false);
    let mut directory = String::new();

    for token in tokens {
        print!("Token: {token}");
        let token: &str = token.as_str();
        if token == HELP_FLAG {
            print!("\tHELP FLAG\n");
            help();
            std::process::exit(0);
        }

        if token == DIR_FLAG {
            print!("\tDIR FLAG\n");
            if dir_flag || directory.len() > 0 {
                return Err(Errors::IllegalInputError);
            }
            dir_flag = true;
            lang_flag = false;
            folder_flag = false;
        } else if token == LANG_FLAG {
            print!("\tLANG FLAG\n");
            if lang_flag || langs.len() > 0 {
                return Err(Errors::IllegalInputError);
            }
            lang_flag = true;
            dir_flag = false;
            folder_flag = false;
        } else if token == FOLDER_NAME_FLAG {
            print!("\tFOLDER NAME FLAG\n");
            if folder_flag || folders.len() > 0 {
                return Err(Errors::IllegalInputError);
            }
            folder_flag = true;
            dir_flag = false;
            lang_flag = false;
        } else if dir_flag {
            print!("\tDIR\n");
            directory = token.to_string();
        } else if lang_flag {
            print!("\tLANG\n");
            match token {
                "cpp" => langs.push(Langs::CPP),
                "c" => langs.push(Langs::C),
                "py" => langs.push(Langs::PYTHON),
                "java" => langs.push(Langs::JAVA),
                _ => return Err(Errors::IllegalLanguageError),
            }
        } else if folder_flag {
            print!("\tFOLDER NAME\n");
            folders.push(token.to_string());
        } else {
            return Err(Errors::IllegalInputError);
        }
    }

    if langs.len() == 0 {
        return Err(Errors::NoLanguageError);
    }

    if directory.len() == 0 {
        return Err(Errors::NoDirectoryError);
    }

    if folders.len() == 0 {
        return Err(Errors::NoFolderNameError);
    }

    if langs.len() != folders.len() && langs.len() != 1 {
        return Err(Errors::UnbalancedLanguagesError);
    }

    Ok((folders, langs, directory))
}

fn create_folder_name(input: &str) -> Result<String, Errors> {
    let folder: String = input
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
    if folder.len() == 0 {
        return Err(Errors::EmptyCreatedFileError);
    }
    Ok(folder)
}

fn create_folder_and_files(
    folder_name: &str,
    lang: &Langs,
    path: &PathBuf,
) -> Result<(), Errors> {
    let mut path = path.clone();
    path.push(folder_name);
    std::fs::create_dir(&path).unwrap();
    println!("Created folder: {}", path.display());
    path.push("main");
    match lang {
        Langs::CPP => {
            path.set_extension("cpp");
            std::fs::write(&path, CPP).unwrap();
            println!("Wrote to file: {}", path.display());
            path.set_file_name("Makefile");
            std::fs::write(&path, CPP_MAKEFILE).unwrap();
            println!("Wrote to Makefile file: {}", path.display());
        }
        Langs::C => {
            path.set_extension("c");
            std::fs::write(&path, C).unwrap();
            println!("Wrote to file: {}", path.display());
            path.set_file_name("Makefile");
            std::fs::write(&path, C_MAKEFILE).unwrap();
            println!("Wrote to Makefile file: {}", path.display());
        }
        Langs::PYTHON => {
            path.set_extension("py");
            std::fs::write(&path, PY).unwrap();
            println!("Wrote to file: {}", path.display());
        }
        Langs::JAVA => {
            path.set_extension("java");
            std::fs::write(&path, JAVA).unwrap();
            println!("Wrote to file: {}", path.display());
        }
    }
    Ok(())
}

fn usage() {
    println!("Usage: ./comp_create -f <folder name> -l <language> -d <directory>");
}

fn help() {
    usage();
    println!("Flags:");
    println!("  -f: Folder name. Can be a single string or multiple strings separated by spaces.");
    println!("  -l: Language. Can be a single language or multiple languages separated by spaces.");
    println!("  -d: Directory. The directory to create the folder in.");
    println!("  -h: Help. Prints this message.");
    println!("Languages:");
    println!("  cpp: C++: Creates a main.cpp file and a Makefile.");
    println!("  c: C: Creates a main.c file and a Makefile.");
    println!("  py: Python: Creates a main.py file.");
    println!("  java: Java: Creates a main.java file.");
    println!("Examples:");
    println!("  ./comp_create -f \"My Programming Problem.\" \"Another Problem.\" -l cpp -d ../");
    println!("  ./comp_create -f \"My Programming Problem.\" \"Another Problem.\" -l cpp py -d ~/dev/");
    println!("  ./comp_create -f \"My Programming Problem.\" -l cpp -d .");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args = args[1..].to_vec();

    if args.len() == 0 {
        usage();
        println!("Rerun with -h for examples.");
        return;
    } 

    match parse_input(&args) {
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        },
        Ok((folders, langs, dir)) => {
            let path = match std::fs::canonicalize(&dir) {
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return;
                },
                Ok(path) => path,
            };
            for i in 0..folders.len() {
                let folder_name = match create_folder_name(&folders[i]) {
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        return;
                    },
                    Ok(name) => name,
                };
                let lang = if langs.len() == 1 { &langs[0] } else { &langs[i] };
                match create_folder_and_files(&folder_name, lang, &path) {
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        return;
                    },
                    Ok(_) => (),
                }
                println!("Completed folder: {}", folder_name);
            }
        }
    }
}
