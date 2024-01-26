use crate::model::Client;
use chrono::Utc;
use std::fs;
use std::io::Write;
use std::path::Path;

const FILE_PATH: &str = "./db/";

fn count_existing_files() -> std::io::Result<i32> {
    ensure_directory_exists()?;
    let today = Utc::now().format("%d%m%Y").to_string();
    let mut file_count = 0;

    for entry in fs::read_dir(FILE_PATH)? {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name = file_name.to_string_lossy();

        if file_name.starts_with(&today) && file_name.ends_with(".DAT") {
            file_count += 1;
        }
    }

    Ok(file_count)
}

pub fn ensure_directory_exists() -> std::io::Result<()> {
    let path = Path::new(FILE_PATH);
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

pub fn store_balances(clients: &[Client]) -> std::io::Result<()> {
    let file_count = count_existing_files()?;
    let file_name = format!(
        "{}{}_{}.DAT",
        FILE_PATH,
        Utc::now().format("%d%m%Y"),
        file_count + 1
    );

    let mut file = fs::File::create(&file_name)?;

    for client in clients {
        writeln!(file, "{} {}", client.id, client.balance)?;
    }

    Ok(())
}
