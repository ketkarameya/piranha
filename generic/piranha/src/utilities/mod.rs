/*
Copyright (c) 2022 Uber Technologies, Inc.

 <p>Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 except in compliance with the License. You may obtain a copy of the License at
 <p>http://www.apache.org/licenses/LICENSE-2.0

 <p>Unless required by applicable law or agreed to in writing, software distributed under the
 License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 express or implied. See the License for the specific language governing permissions and
 limitations under the License.
*/

pub mod tree_sitter_utilities;

use std::collections::HashMap;
#[cfg(test)]
use std::fs::{self, DirEntry};
use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::{BufReader, Read};
use std::path::PathBuf;

// Reads a file.
pub(crate) fn read_file(file_path: &PathBuf) -> Result<String, String> {
  File::open(&file_path)
    .map(|file| {
      let mut content = String::new();
      let _ = BufReader::new(file).read_to_string(&mut content);
      content
    })
    .map_err(|error| error.to_string())
}

// Reads a toml file. In case of error, it returns a default value (if return_default is true) else panics.
pub(crate) fn read_toml<T>(file_path: &PathBuf, return_default: bool) -> T
where
  T: serde::de::DeserializeOwned + Default,
{
  match read_file(file_path)
    .and_then(|content| toml::from_str::<T>(content.as_str()).map_err(|e| e.to_string()))
  {
    Ok(obj) => obj,
    Err(err) => {
      if return_default {
        T::default()
      } else {
        #[rustfmt::skip]
      panic!("Could not read file: {:?} \n Error : \n {:?}", file_path, err);
      }
    }
  }
}

pub(crate) trait MapOfVec<T, V> {
  fn collect(&mut self, key: T, value: V);
}

// Implements trait `MapOfVec` for `HashMap<T, Vec<U>>`.
impl<T: Hash + Eq, U> MapOfVec<T, U> for HashMap<T, Vec<U>> {
  // Adds the given `value` to the vector corresponding to the `key`.
  // Like an adjacency list.
  fn collect(self: &mut HashMap<T, Vec<U>>, key: T, value: U) {
    self.entry(key).or_insert_with(Vec::new).push(value);
  }
}

/// Initialize logger.
pub(crate) fn initialize_logger(is_test: bool) {
  let log_file = OpenOptions::new()
    .write(true)
    .create(true) // Create a log file if it doesn't exists
    .append(true) // Append to the log file if it exists
    .open("piranha.log")
    .unwrap();
  let _ = env_logger::builder()
    .format_timestamp(None)
    .target(env_logger::Target::Pipe(Box::new(log_file)))
    .is_test(is_test)
    .try_init();
}

/// Compares two strings, ignoring new lines, and space.
#[cfg(test)] // Rust analyzer FP
pub(crate) fn eq_without_whitespace(s1: &str, s2: &str) -> bool {
  s1.replace('\n', "")
    .replace(' ', "")
    .eq(&s2.replace('\n', "").replace(' ', ""))
}

/// Checks if the given `dir_entry` is a file named `file_name`
#[cfg(test)] // Rust analyzer FP
pub(crate) fn has_name(dir_entry: &DirEntry, file_name: &str) -> bool {
  println!("{:?}", dir_entry);
  dir_entry
    .path()
    .file_name()
    .map(|e| e.eq(file_name))
    .unwrap_or(false)
}

/// Returns the file with the given name within the given directory.
#[cfg(test)] // Rust analyzer FP
pub(crate) fn find_file(input_dir: &PathBuf, name: &str) -> PathBuf {
  fs::read_dir(input_dir)
    .unwrap()
    .filter_map(|d| d.ok())
    .find(|de| has_name(de, name))
    .unwrap()
    .path()
}

#[cfg(test)]
mod test {
  
  use std::path::PathBuf;
  use serde_derive::Deserialize;
  use crate::utilities::find_file;

#[cfg(test)]
  use super::{read_file, read_toml};
  

  #[derive(Deserialize, Default)]
  struct TestStruct {
    ip: String,
  }

  #[test]
  pub fn test_read_file() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path_to_test_file = project_root.join("test-resources/utility_tests/sample.toml");
    let result = read_file(&path_to_test_file);
    assert!(result.is_ok());
    let content = result.ok().unwrap();
    assert!(!content.is_empty());
    assert!(content.eq(r#"ip = '127.0.0.1'"#));
  }

  #[test]
  pub fn test_read_toml() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path_to_test_file = project_root.join("test-resources/utility_tests/sample.toml");
    let result: TestStruct = read_toml(&path_to_test_file, false);
    assert!(result.ip.eq("127.0.0.1"));
  }

  #[test]
  pub fn test_read_toml_default() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path_to_test_file = project_root.join("test-resources/utility_tests/sample1.toml");
    let result: TestStruct = read_toml(&path_to_test_file, true);
    assert!(result.ip.eq(""));
  }


  #[test]
  pub fn test_find_file_positive() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-resources/utility_tests/");
    let f = find_file(&project_root, "sample.toml");
    assert!(f.is_file());
  }

  #[test]
  #[should_panic]
  pub fn test_find_file_negative() {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("test-resources/utility_tests/");
    let f = find_file(&project_root, "sample1.toml");
    assert!(f.is_file());
  }
}
