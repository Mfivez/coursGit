use chrono::prelude::*;
use std::ffi::OsStr;
use std::fs::exists;
use std::io::Result;
use std::thread::sleep;
use std::time::Duration;
use std::{env, fs, io::stdin};

const VERSION: &str = "0.2.0";

fn main() -> Result<()> {
    println!("Rust Photo Manager V.{}", VERSION);
    let mut program_status: bool = true;

    while program_status {
        let user_input: String = input_manager()?;

        match user_input.to_lowercase().as_str() {
            "h" => {
                println!("h -> help");
                println!("q -> quit");
                println!("cf -> class files");
            }

            "cf" => {
                println!("Running...");
                for f in env::current_dir()?.read_dir()? {
                    let file = f?;
                    let file_path = file.path();
                    let file_name = file.file_name();
                    let file_ext = file_path.extension().unwrap_or(OsStr::new("Error"));
                    let file_date: DateTime<Local> =
                        chrono::DateTime::from(file.metadata()?.modified()?);

                    let extension: String = String::from(file_ext.to_str().unwrap_or("Error"));

                    let total_date: String = format!(
                        "{}-{}-{}",
                        file_date.day(),
                        file_date.month(),
                        file_date.year()
                    );

                    if !exists(&total_date)? {
                        fs::create_dir(&total_date)?;
                        sleep(Duration::from_millis(1));
                    }

                    if !exists(format!("{}\\{}", &total_date, &extension.trim()))? {
                        fs::create_dir(format!("{}\\{}", &total_date, &extension.trim()))?;
                        sleep(Duration::from_millis(1));
                    }
                    fs::rename(
                        &file_path,
                        format!(
                            "{}\\{}\\{}\\{}",
                            env::current_dir()?.to_str().unwrap(),
                            &total_date,
                            &extension.trim(),
                            &file_name.to_str().unwrap()
                        ),
                    )?;
                }
                println!("Complete !")
            }

            "q" => program_status = false,

            _ => continue,
        }
    }
    Ok(())
}

fn input_manager() -> Result<String> {
    let mut input_buffer: String = String::new();
    stdin().read_line(&mut input_buffer)?;
    Ok(String::from(input_buffer.trim()))
}
