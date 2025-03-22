use crate::structs::file::FileEntry;
use std::fs;
use std::path::PathBuf;
// TODO I dont like this error handling. Find a way to fix this
pub async fn dir_create<'a>(path: String) -> Result<&'a str, u8> {
    // Prevent someone from creating a Dir using ./../../../../Somewhere
    if path.find("..").is_some() {
        return Err(1);
    }
    let path = PathBuf::from("output/".to_owned() + &*path);

    match fs::create_dir_all(path) {
        Ok(_) => Ok("Dir Created"),
        Err(_) => Err(2),
    }
}

pub async fn dir_delete<'a>(path: String) -> Result<&'a str, u8> {
    // Prevent someone from deleting somewhere above the output directory
    if path.find("..").is_some() {
        return Err(1);
    }

    let path = PathBuf::from("output/".to_owned() + &*path);

    match fs::remove_dir_all(path) {
        Ok(_) => Ok("Dir deleted"),
        Err(_) => Err(2),
    }
}

pub async fn get_dir_size(path: String) -> Option<u64> {
    // Prevents someone from getting the Dir size of a different dir above Output
    if path.find("..").is_some() {
        return None;
    }

    let p = PathBuf::from(path);
    match fs_extra::dir::get_size(p) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

/// List files in a directory
/// Arg: path - PathBuf
/// Return: Result<Vec<FileEntry>, u8>
/// 1: Contains a ..
pub async fn list_files(path: PathBuf) -> Result<Vec<FileEntry>, std::io::Error> {
    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                // Here, `entry` is a `DirEntry`.
                if let Ok(metadata) = entry.metadata() {
                    // Now let's show our entry's permissions!
                    println!("{:?}: {:?}", entry.path(), metadata.permissions());
                } else {
                    println!("Couldn't get metadata for {:?}", entry.path());
                }

                let file_type = entry.file_type()?;

                files.push(FileEntry {
                    name: entry.file_name().into_string().unwrap_or_default(),
                    path: entry.path().to_string_lossy().into_owned(),
                    is_directory: file_type.is_dir(),
                });
            }
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_dir_creation() {
        if PathBuf::from("TESTS/RUST").exists() {
            fs::remove_dir_all("TESTS/RUST".to_string()).unwrap();
        }
        assert!(dir_create("TESTS/RUST".to_string()).await.is_ok());
    }

    #[tokio::test]
    async fn test_dir_creation_failing() {
        assert!(dir_create("../../should_fail".to_string()).await.is_err());
    }

    #[tokio::test]
    async fn test_dir_deletion() {
        if PathBuf::from("TESTS/RUST").exists() == false {
            dir_create("TESTS/RUST".to_string()).await.unwrap();
        }
        assert!(dir_delete("TESTS/RUST".to_string()).await.is_ok());
    }

    #[tokio::test]
    async fn test_dir_deletion_failing() {
        assert!(dir_delete("../../should_fail".to_string()).await.is_err());
    }

    #[tokio::test]
    async fn test_get_dir_size() {
        if PathBuf::from("target").exists() {
            assert!(get_dir_size("target".to_string()).await.is_some());
        }
    }

    #[tokio::test]
    async fn test_get_dir_size_failing_non_existant() {
        assert!(get_dir_size("../../should_fail".to_string())
            .await
            .is_none());
    }

    #[tokio::test]
    async fn test_get_dir_size_failing() {
        assert!(get_dir_size("..".to_string()).await.is_none());
    }
}
