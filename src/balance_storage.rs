use crate::model::Client;
use chrono::Utc;
use std::fs;
use std::io::Write;
use std::path::Path;

pub const FILE_PATH: &str = "./db/";

/// A trait defining the behavior for data storage implementations.
pub trait DataStorage {
    /// Stores a slice of `Client` items.
    ///
    /// # Arguments
    /// * `items` - A slice of `Client` items to be stored.
    ///
    /// # Returns
    /// * An `io::Result<()>` indicating the outcome of the storage operation.
    fn store_data(&self, items: &[Client]) -> std::io::Result<()>;
}

/// A data storage implementation that stores data in files on the filesystem.
pub struct FileStorage {
    /// The directory where files will be stored.
    directory: String,
}

impl FileStorage {
    /// Constructs a new `FileStorage`.
    ///
    /// # Arguments
    /// * `directory` - The path to the directory where files will be stored.
    ///
    /// # Returns
    /// * An instance of `FileStorage`.
    pub fn new(directory: &str) -> Self {
        Self {
            directory: directory.to_owned(),
        }
    }

    /// Ensures that the storage directory exists, creating it if it does not.
    ///
    /// # Returns
    /// * An `io::Result<()>` indicating the success or failure of directory creation.
    fn ensure_directory_exists(&self) -> std::io::Result<()> {
        let path = Path::new(&self.directory);
        if !path.exists() {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    /// Counts the existing files in the directory that match the current date format.
    ///
    /// # Returns
    /// * An `io::Result<i32>` indicating the number of matching files found or an error.
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
    /// Implements the `store_data` method of the `DataStorage` trait for `FileStorage`.
    ///
    /// This method stores the given `Client` items into a file, creating a new file each day
    /// with a unique sequence number.
    ///
    /// # Arguments
    /// * `items` - A slice of `Client` items to be stored.
    ///
    /// # Returns
    /// * An `io::Result<()>` indicating the success or failure of the data storage operation.
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
