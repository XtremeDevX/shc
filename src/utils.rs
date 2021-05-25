use crate::commands::remove;
use crate::commands::{add, export, show};
use crate::helper::{delete_shortcut, generate_shortcut};
use crate::model::shortkut::ShortKut;
use colored::*;
use reqwest::blocking;
use std::{process, time::Instant};

pub fn get_shortcut(name: &str) -> ShortKut {
    let mut res: String = String::new();

    match blocking::get(format!(
        "https://raw.githubusercontent.com/XtremeDevX/shc/master/shortcuts/{}.json",
        name
    )) {
        Ok(response) => {
            if response.status() == reqwest::StatusCode::OK {
                // Response Code Is 200 OK
                match response.text() {
                    Ok(text) => {
                        res = text;
                    }
                    Err(e) => {
                        println!(
                            "{} {}",
                            "Failed To Parse Response:".to_string().yellow(),
                            e.to_string().bright_red()
                        );
                        process::exit(1);
                    }
                }
            } else {
                println!(
                    "{} is not a valid shortcut pack.",
                    name.to_string().bright_magenta()
                );
                process::exit(1);
            }
        }
        Err(err) => println!("Failed To Request {}.json => {}", name, err),
    }

    let data: ShortKut;
    match serde_json::from_str(&res) {
        Ok(json) => {
            data = json;
        }
        Err(err) => {
            println!(
                "Failed to parse {}.json, {}",
                name.to_string().bright_magenta(),
                err.to_string().bright_red(),
            );
            process::exit(1);
        }
    };

    data
}

pub fn parse() {
    let start = Instant::now();
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => {
            // Display Help Menu
            let help = format!(
                r#"{}
{} add - Add a shortcut
{} remove - Remove a shortcut
{} show - Show a shortcut pack
{} search - Search for a shortcut pack
{} export - Export a list of your current shortcuts"#,
                format!("shc {}", "1.0.0".bright_green()),
                "*".bright_magenta().bold(),
                "*".bright_magenta().bold(),
                "*".bright_magenta().bold(),
                "*".bright_magenta().bold(),
                "*".bright_magenta().bold(),
            );
            println!("{}", help);
        }
        2 => {
            if args[1] == "--version" {
                println!("shc v{}", "1.0.0".bright_green().bold())
            } else if args.contains(&"export".to_string()) {
                export::export();
            } else {
                match args[1].as_str() {
                    "add" => {
                        let error = format!(
                            r#"{}: expected a shortkut pack or alias-command pair to add.

usage:
  {} add {} or {} add {} {}

examples:
  {} add flutter  {}
  {} add npm,yarn  {}
  {} add cru {}  {}

links: 
  {}: {}"#,
                            "error".bright_red().bold(),
                            "shc".bright_green().bold(),
                            "<pack-name>".bright_cyan(),
                            "shc".bright_green().bold(),
                            "<alias>".bright_cyan(),
                            "<command>".bright_cyan(),
                            "shc".bright_green().bold(),
                            "// add flutter shortkut pack".bright_black(),
                            "shc".bright_green().bold(),
                            "// add npm and yarn shortkut packs".bright_black(),
                            "shc".bright_green().bold(),
                            "\"cargo run\"".bright_yellow(),
                            "// command \"cargo run\" aliased to cru".bright_black(),
                            "1".blue(),
                            "https://shortkut.sh/docs/add".bright_purple().underline(),
                        );

                        println!("{}", error);
                    }
                    "remove" => {
                        let error = format!(
                            r#"{}: expected a shortkut pack or alias-command pair to remove.

usage:
  {} remove {} or {} remove {} {}

examples:
  {} remove flutter  {}
  {} remove npm,yarn  {}
  {} remove cru {}  {}

links: 
  {}: {}"#,
                            "error".bright_red().bold(),
                            "shc".bright_green().bold(),
                            "<pack-name>".bright_cyan(),
                            "shc".bright_green().bold(),
                            "<alias>".bright_cyan(),
                            "<command>".bright_cyan(),
                            "shc".bright_green().bold(),
                            "// remove flutter shortkut pack".bright_black(),
                            "shc".bright_green().bold(),
                            "// remove npm and yarn shortkut packs".bright_black(),
                            "shc".bright_green().bold(),
                            "\"cargo run\"".bright_yellow(),
                            "// remove command \"cargo run\" aliased to cru".bright_black(),
                            "1".blue(),
                            "https://shortkut.sh/docs/remove"
                                .bright_purple()
                                .underline(),
                        );

                        println!("{}", error);
                    }
                    "show" => {
                        let error = format!(
                            r#"{}: expected name of a shortkut pack to display.
usage:
  {} show {}

examples:
  {} show {}"#,
                            "error".bright_red().bold(),
                            "shc".bright_green().bold(),
                            "<pack-name>".bright_cyan().bold(),
                            "shc".bright_green().bold(),
                            "flutter".bright_cyan().bold()
                        );

                        println!("{}", error);
                    }
                    "search" => {
                        let error = format!(
                            r#"{}: expected approximate name of a shortkut pack to search for.
usage:
  shc search <pack-name>

examples:
  shc search flutter"#,
                            "error".bright_red().bold()
                        );

                        println!("{}", error);
                    }
                    _ => {}
                }
            }
        }
        3 => match args[1].as_str() {
            "add" => {
                add::add();
            }
            "remove" => {
                remove::remove();
            }
            "show" => {
                show::show();
            }
            _ => {
                println!(
                    "{}: {} is not a valid command.",
                    "error".bright_red().bold(),
                    args[1].bright_cyan()
                );
            }
        },
        4 => match args[1].as_str() {
            "add" => {
                println!("shc {} {}", "1.0.0", "add".bright_green().bold());

                let alias = &args[2];
                let command = &args[3].to_string().replace("\"", "");
                generate_shortcut(alias, command);
                let end = Instant::now();
                println!(
                    "Added {} shortcut in {:.2}s",
                    1.to_string().bright_green(),
                    (end - start).as_secs_f32()
                );
            }
            "remove" => {
                println!("shc {} {}", "1.0.0", "remove".bright_green().bold());

                let alias = &args[2];
                let command = &args[3].to_string().replace("\"", "");
                delete_shortcut(alias, command);
                let end = Instant::now();
                println!(
                    "Removed {} shortcut in {:.2}s",
                    1.to_string().bright_green(),
                    (end - start).as_secs_f32()
                );
            }
            _ => {}
        },
        _ => {
            println!(
                "{}: shc recieved unexpected arguments",
                "error".bright_red().bold()
            );
        }
    }
}
