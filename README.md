# JSON File Walker

Iterates through json file paths in directory hierarchy

Create an iterator returning `Option<std::path::PathBuf>` objects of json files in the root directory and sub-directories

## Examples

```
use json_file_walker::walk_json_files;

for path in walk_json_files("./directory") {
    println!("{}", path.to_string_lossy());
}
```