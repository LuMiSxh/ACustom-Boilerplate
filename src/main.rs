use colored::*;
use dialoguer::*;
use std::fs;
use std::process;

mod pathing;
use pathing::{bundling, utilities};

#[tokio::main]
async fn main() {
    // variable declaration
    let bundle: bundling::Bundle;
    let ts: bool;
    let src: bool;

    // 0 = WASM
    let base_modules = vec!["WASM"];

    // 0 = components; 1 = constants; 2 = hooks; 3 = lib; 4 = modules; 5 = types
    let folders = vec![
        "components",
        "constants",
        "hooks",
        "lib",
        "modules",
        "types",
    ];

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
        .default("drageast-biolerplate".into())
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

    // Choosing base-modules

    let chosen_base_modules: Vec<usize> = MultiSelect::new()
        .items(&base_modules)
        .with_prompt(format!(
            "{}",
            "Choose modules you want to be included".blue().bold()
        ))
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
    bundle = bundling::Bundle::new(&project_name, ts, src);
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
    if src {
        match fs::create_dir(format!("./{}/src/", &project_name)) {
            Err(reason) => {
                println!("{:?}", reason);
                process::exit(1)
            }
            Ok(_) => {}
        }
    }

    for option in chosen_base_modules {
        match option {
            0 => match fs::create_dir(format!("{}WASM/", &bundle.bundle_path)) {
                Err(reason) => {
                    println!("{:?}", reason);
                    process::exit(1)
                }
                Ok(_) => {
                    utilities::create_index(format!("{}WASM", &bundle.bundle_path), &bundle.is_ts);
                }
            },
            _ => {}
        }
    }
}
