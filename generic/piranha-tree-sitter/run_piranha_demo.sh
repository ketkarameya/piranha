# Copyright (c) 2022 Uber Technologies, Inc.

# <p>Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
# except in compliance with the License. You may obtain a copy of the License at
# <p>http://www.apache.org/licenses/LICENSE-2.0

# <p>Unless required by applicable law or agreed to in writing, software distributed under the
# License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
# express or implied. See the License for the specific language governing permissions and
# limitations under the License.

cargo build --release
cp target/release/piranha-tree-sitter  demo/java/piranha-tree-sitter
./demo/java/piranha-tree-sitter -c demo/java/ -f demo/java/configurations -p demo/java/demo_piranha_arguments.toml