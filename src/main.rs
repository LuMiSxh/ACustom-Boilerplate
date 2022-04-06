mod manager;
mod pathing;

use colored::*;
use dialoguer::*;
use std::collections::HashMap;
use std::fs;
use std::process;

use pathing::utilities;

#[tokio::main]
async fn main() {
    // variable declaration
    let bundle: pathing::Bundle;
    let ts: bool;
    let src: bool;

    // reqwest client
    let async_client: reqwest::Client = reqwest::Client::new();
    // reqwest client

    // URL's for the raw data
    let mut raw_content: HashMap<String, String> = HashMap::new();
    raw_content.insert("package.json".to_string(), "".to_string());
    // URL's for the raw data

    // 0 = components; 1 = constants; 2 = hooks; 3 = lib; 4 = modules; 5 = types; 6 = WASM
    let folders = vec![
        "components",
        "constants",
        "hooks",
        "lib",
        "modules",
        "types",
        "WASM",
    ];

    let d_manager = manager::DependencyManager::new(
        vec![],
        vec!["react-dom: ^17.0.2".into(), "next: ^12.1.0".into()],
    );
    d_manager.build_package("", "".into());

    // Asking if user wants to continue

    if !Confirm::new()
        .with_prompt(format!("{}", "Do you want to continue? ".blue().bold()))
        .default(true)
        .show_default(false)
        .wait_for_newline(true)
        .interact()
        .unwrap()
    {
        process::exit(0);
    }

    // getting the project / directory name

    let project_name: String = Input::<String>::new()
        .with_prompt( format!("{}", "Enter a project name ".blue().bold()))
        .default("drageast-boilerplate".into())
        .validate_with({
            move |input: &String| -> Result<(), &str> {
                if input.contains(char::is_whitespace) {
                    Err("Project name cannot contain whitespaces. Replace them with'-'.")
                }
                else if input.contains(char::is_uppercase) {
                    Err("Project name cannot contain uppercase characters. Replace them with lowercase characters.")
                }
                else {
                    Ok(())
                }
            }
        })
        .interact_text()
        .unwrap();

    // Asking whether or not to initialize as a typescript project
    ts = Confirm::new()
        .with_prompt(format!(
            "{}",
            "Do you want to use Typescript instead of JavaScript "
                .blue()
                .bold()
        ))
        .default(true)
        .interact()
        .unwrap();

    // Asking whether or not to initialize with a 'src'-folder
    src = Confirm::new()
        .with_prompt(format!(
            "{}",
            "Do you want to bundle your project with a 'src'-directory "
                .blue()
                .bold()
        ))
        .default(true)
        .interact()
        .unwrap();

    // Choosing folders that should be included

    let chosen_folders: Vec<usize> = MultiSelect::new()
        .items(&folders)
        .with_prompt(format!(
            "{}",
            "Choose folders you want to be included".blue().bold()
        ))
        .interact()
        .unwrap();

    // Setting the bundled path
    bundle = pathing::Bundle::new(&project_name).set_src(src).set_ts(ts);
    drop(ts);
    drop(src);

    // creation of the project //

    match fs::create_dir(format!("./{}/", &project_name)) {
        Err(reason) => {
            println!("{:?}", reason);
            process::exit(1)
        }
        Ok(_) => {}
    }

    // Creating the 'src' directory
    if bundle.with_src {
        match fs::create_dir(format!("./{}/src/", &project_name)) {
            Err(reason) => {
                println!("{:?}", reason);
                process::exit(1)
            }
            Ok(_) => {}
        }
    }

    // creating the different folders with index files
    for folder in chosen_folders {
        match fs::create_dir(format!("{}{}/", &bundle.bundle_path, &folders[folder])) {
            Err(reason) => {
                println!("{:?}", reason);
                process::exit(1)
            }
            Ok(_) => {
                utilities::create_index(
                    format!("{}{}", &bundle.bundle_path, &folders[folder]),
                    &bundle.is_ts,
                );
            }
        }
    }
}
