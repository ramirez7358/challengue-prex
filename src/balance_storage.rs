use crate::model::Client;
use chrono::Utc;
use std::fs;
use std::io::Write;
use std::path::Path;

pub const FILE_PATH: &str = "./db/";

pub trait DataStorage {
    fn store_data(&self, items: &[Client]) -> std::io::Result<()>;
}

pub struct FileStorage {
    directory: String,
}

impl FileStorage {
    pub fn new(directory: &str) -> Self {
        Self {
            directory: directory.to_owned(),
        }
    }

    fn ensure_directory_exists(&self) -> std::io::Result<()> {
        let path = Path::new(&self.directory);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    fn count_existing_files(&self) -> std::io::Result<i32> {
        self.ensure_directory_exists()?;
        let today = Utc::now().format("%d%m%Y").to_string();
        let mut file_count = 0;

        for entry in fs::read_dir(&self.directory)? {
            let entry = entry?;
            let file_name = entry.file_name();
            let file_name = file_name.to_string_lossy();

            if file_name.starts_with(&today) && file_name.ends_with(".DAT") {
                file_count += 1;
            }
        }

        Ok(file_count)
    }
}

impl DataStorage for FileStorage {
    fn store_data(&self, items: &[Client]) -> std::io::Result<()> {
        let file_count = self.count_existing_files()?;
        let file_name = format!(
            "{}{}_{}.DAT",
            self.directory,
            Utc::now().format("%d%m%Y"),
            file_count + 1
        );

        let mut file = fs::File::create(&file_name)?;

        for item in items {
            writeln!(file, "{} {}", item.id, item.balance)?;
        }

        Ok(())
    }
}
