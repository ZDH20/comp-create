# comp-create

This script creates a folder and files for the given programming language. It takes in 3 arguments: the name of the folder to create, the programming language to use, and the directory in which to create the folder.

# Usage
  ```bash
$ create_folder_and_files -d <directory> -l <language> -f <folder name>
  ```

| Argument | Description |
| --- | --- |
| -d \<directory> |  Specifies the parent directory for the project folder.|
| -l \<language> | The programming language(s) to use. Can be "cpp", "c", "py", or "java". |
| -f \<folder name> | Specifies the name of the project folder(s).|
| -h \<help> | Display the help message. |

### Note
When using the -l argument, you must either use 1 language for all, or specify a language for each folder name.

# Examples
## With Cargo
```bash
$ cargo run -- -d ~/Documents/Projects -f "My Project" -l cpp
```
or
```bash
$ cargo run -- -d ~/Documents/Projects -f "My Project" "My Second Project" -l cpp
```
or
```bash
$ cargo run -- -d ~/Documents/Projects -f "My Project" "My Second Project" -l cpp py
```
  
## With the executable
```bash
$ ./comp_create -d ~/Documents/Projects -f "My Project" -l cpp
```
or
```bash
$ ./comp_create -d ~/Documents/Projects -f "My Project" "My Second Project" -l cpp
```
or
```bash
$ ./comp_create -d ~/Documents/Projects -f "My Project" "My Second Project" -l cpp py
```



