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

use std::{collections::HashMap, path::PathBuf};

use colored::Colorize;
use itertools::Itertools;
use jwalk::WalkDir;
use log::info;
use regex::Regex;
use tree_sitter::Parser;

use crate::utilities::read_file;

use super::{
  piranha_arguments::PiranhaArguments, rule_store::RuleStore, source_code_unit::SourceCodeUnit,
};

// Maintains the state of Piranha and the updated content of files in the source code.
pub struct FlagCleaner {
  // Maintains Piranha's state
  rule_store: RuleStore,
  // Path to source code folder
  path_to_codebase: String,
  // Files updated by Piranha.
  relevant_files: HashMap<PathBuf, SourceCodeUnit>,
}

impl FlagCleaner {
  pub fn get_updated_files(&self) -> Vec<SourceCodeUnit> {
    self.relevant_files.values().cloned().collect_vec()
  }

  /// Performs cleanup related to stale flags
  pub fn perform_cleanup(&mut self) {
    // Setup the parser for the specific language
    let mut parser = Parser::new();
    parser
      .set_language(self.rule_store.language())
      .expect("Could not set the language for the parser.");

    // Keep looping until new `global` rules are added.
    loop {
      let current_rules = self.rule_store.global_rules();

      info!(
        "{}",
        format!("Number of global rules {}", current_rules.len())
      );
      // Iterate over each file containing the usage of the feature flag API
      for (path, content) in self.get_files_containing_feature_flag_api_usage() {
        self
          .relevant_files
          // Get the content of the file for `path` from the cache `relevant_files`
          .entry(path.to_path_buf())
          // Populate the cache (`relevant_files`) with the content, in case of cache miss (lazily)
          .or_insert_with(|| {
            // Create new source code unit
            SourceCodeUnit::new(
              &mut parser,
              content,
              &self.rule_store.input_substitutions(),
              &path,
            )
          })
          // Apply the rules to this file
          .apply_rules(&mut self.rule_store, &current_rules, &mut parser, None);

        // Break when a new `global` rule is added
        if self.rule_store.global_rules().len() > current_rules.len() {
          info!("Found a new global rule. Will start scanning all the files again.");
          break;
        }
      }
      // If no new `global_rules` were added, break.
      if self.rule_store.global_rules().len() == current_rules.len() {
        break;
      }
    }
  }

  /// Gets all the files from the code base that (i) have the language appropriate file extension, and (ii) contains the grep pattern.
  /// Note that `WalkDir` traverses the directory with parallelism.
  pub fn get_files_containing_feature_flag_api_usage(&self) -> HashMap<PathBuf, String> {
    let pattern = self.get_grep_heuristics();
    info!(
      "{}",
      format!("Searching for pattern {}", pattern.as_str()).green()
    );
    let files: HashMap<PathBuf, String> = WalkDir::new(&self.path_to_codebase)
      // Walk over the entire code base
      .into_iter()
      // Ignore errors
      .filter_map(|e| e.ok())
      // Filter files with the desired extension
      .filter(|de| {
        de.path()
          .extension()
          .and_then(|e| {
            e.to_str()
              .filter(|x| x.eq(&self.rule_store.language_name()))
          })
          .is_some()
      })
      // Read the file
      .map(|f| {
        (
          f.path(),
          read_file(&f.path()).unwrap(),
        )
      })
      // Filter the files containing the desired regex pattern
      .filter(|x| pattern.is_match(x.1.as_str()))
      .collect();
    #[rustfmt::skip]
    info!("{}", format!("Will parse and analyze {} files.", files.len()).green());
    files
  }

  /// Instantiate Flag-cleaner
  pub fn new(args: PiranhaArguments) -> Self {
    let graph_rule_store = RuleStore::new(&args);
    Self {
      rule_store: graph_rule_store,
      path_to_codebase: String::from(args.path_to_code_base()),
      relevant_files: HashMap::new(),
    }
  }

  /// To create the current set of global rules, certain substitutions were applied.
  /// This method creates a regex pattern matching these substituted values.
  ///
  /// At the directory level, we would always look to perform global rules. However this is expensive because
  /// it requires parsing each file. To overcome this, we apply this simple
  /// heuristic to find the (upper bound) files that would match one of our current global rules.
  /// This heuristic reduces the number of files to parse.
  ///
  pub fn get_grep_heuristics(&self) -> Regex {
    let reg_x = self
      .rule_store
      .global_rules()
      .iter()
      .flat_map(|r| r.grep_heuristics())
      .sorted()
      //Remove duplicates
      .dedup()
      //FIXME: Dirty trick to remove true and false. Ideally, grep heuristic could be a field in itself for a rule.
      // Since not all "holes" could be used as grep heuristic.
      .filter(|x| !x.as_str().eq("true") && !x.as_str().eq("false"))
      .join("|");
    Regex::new(reg_x.as_str()).unwrap()
  }
}
