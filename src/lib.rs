//! # JSON File Walker
//! 
//! `walk_json_files` function produces an iterator to get all json file paths in a directory and its sub-directories.

use std::ffi::{OsStr};
use std::path::{Path, PathBuf};
use walkdir::{IntoIter, WalkDir};

const JSON_EXTENSION : &str = "json";

/// Iterator object to iterate through the JSON file paths
pub struct WalkJsonFiles(IntoIter);

/// Create an iterator returning `Option<std::path::PathBuf>` objects of json files in the root directory and sub-directories
/// 
/// # Examples
/// 
/// ```
/// use json_file_walker::walk_json_files;
/// 
/// for path in walk_json_files("./directory") {
///     println!("{}", path.to_string_lossy());
/// }
/// ```
pub fn walk_json_files<P: AsRef<Path>>(root: P) -> WalkJsonFiles {
    WalkJsonFiles(WalkDir::new(root).into_iter())
}

fn is_json_file(path: &Path) -> bool {
    path.is_file() && path.extension() == Some(OsStr::new(JSON_EXTENSION))
}

impl Iterator for WalkJsonFiles {
    type Item = PathBuf;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(r) => match r {
                Ok(e) => if is_json_file(e.path())  {
                    Some(PathBuf::from(e.path()))
                } else {
                    self.next()
                },
                Err(..) => self.next()
            },
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::{self, File};
    use rand::RngCore;

    struct TempDir(PathBuf);

    impl TempDir {
        fn join<P: AsRef<Path>>(&self, path: P) -> PathBuf {
            let TempDir(ref p) = *self;
            p.join(path)
        }

        fn path(&self) -> &Path {
            let TempDir(ref p) = *self;
            p
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let TempDir(ref p) = *self;
            fs::remove_dir_all(p).unwrap();
        }
    }

    fn create_temp_dir() -> TempDir {
        let mut temp_dir = env::temp_dir();
        let mut random = rand::thread_rng();

        temp_dir.push(&format!("test{}", random.next_u64()));

        fs::create_dir(&temp_dir).unwrap();

        TempDir(temp_dir)
    }

    #[test]
    fn test_walk_json_files_empty() {
        let temp_dir = create_temp_dir();

        let mut iter = walk_json_files(temp_dir.path());

        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_walk_json_files_sub_directories() {
        let temp_dir = create_temp_dir();

        let fp1 = temp_dir.join("a.json");

        File::create(&fp1).unwrap();

        let dir1 = temp_dir.join("b");

        fs::create_dir(&dir1).unwrap();

        let fp2 = dir1.join("c.json");

        File::create(&fp2).unwrap();

        let mut iter = walk_json_files(temp_dir.path());

        assert_eq!(Some(fp1), iter.next());
        assert_eq!(Some(fp2), iter.next());
        assert_eq!(None, iter.next());
    }

    #[test]
    fn test_walk_json_files_skipping_other_file_types() {
        let temp_dir = create_temp_dir();

        let fp1 = temp_dir.join("a.txt");

        File::create(&fp1).unwrap();

        let fp2 = temp_dir.join("b.json");

        File::create(&fp2).unwrap();

        let mut iter = walk_json_files(temp_dir.path());

        assert_eq!(Some(fp2), iter.next());
        assert_eq!(None, iter.next());
    }
}
