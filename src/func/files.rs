use std::path::PathBuf;
use tokio::fs;


// TODO I dont like this error handling. Find a way to fix this
pub async fn dir_create<'a>(path: String) -> Result<&'a str, u8> {
    // Prevent someone from creating a Dir using ./../../../../Somewhere
    if path.find("..").is_some() {
        return Err(1)
    }
    let path = PathBuf::from("output/".to_owned() + &*path);

    match fs::create_dir_all(path).await {
        Ok(_) => {
            Ok("Dir Created")
        }
        Err(_) => {
            Err(2)
        }
    }


}