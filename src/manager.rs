mod dependency;

use std::fs;
use std::io::Write;
use std::process;

pub struct DependencyManager {
    // Dependency should look like this: "react: ^18.0.0"
    pub dev_dependencies: Vec<String>,
    pub standard_dependencies: Vec<String>,
}

impl DependencyManager {
    pub fn new(
        initial_dev_dependencies: Vec<String>,
        initial_standard_dependencies: Vec<String>,
    ) -> Self {
        Self {
            dev_dependencies: initial_dev_dependencies,
            standard_dependencies: initial_standard_dependencies,
        }
    }

    pub fn build_package(self, root_path: &str, template: String) {
        let mut f: fs::File = fs::File::create(format!("{}package.json", &root_path)).unwrap();
        let quote: char = '"';
        let mut dev_json: String = "".into();
        let mut normal_json: String = "".into();
        // iterate trough development dependencies
        for dependency in self.dev_dependencies {
            let snippets: Vec<String> = dependency.split(": ").map(str::to_string).collect();
            dev_json.push_str(&format!(
                "{}{}{}: {}{}{},\n",
                quote, snippets[0], quote, quote, snippets[1], quote
            ))
        }
        for dependency in self.standard_dependencies {
            let snippets: Vec<String> = dependency.split(": ").map(str::to_string).collect();
            normal_json.push_str(&format!(
                "{}{}{}: {}{}{},\n",
                quote, snippets[0], quote, quote, snippets[1], quote
            ))
        }

        match f.write_all(b"") {
            Err(reason) => {
                println!("{:?}", reason);
                process::exit(1)
            }
            Ok(_) => {}
        }
    }
}
