use std::{
    fs,
    path::{Path, PathBuf},
};

/// get and create the cache file dir.
pub fn cache_file_dir(name: &str) -> crate::Result<PathBuf> {
    let home = home_dir(name)?;

    let cache_dir = home.join("cache");

    verify_or_create(&cache_dir)?;

    Ok(cache_dir)
}

/// get the home directory or the director in the CACHE_VAR environment variable.
fn home_dir(name: &str) -> crate::Result<PathBuf> {
    let home = match std::env::var("CACHE_VAR") {
        Ok(h) => h.into(),
        Err(e) => dirs::home_dir()
            .ok_or_else(|| crate::Error::DirError(format!("Error getting the Home Directory: {}", e)))?,
    };

    let home_dir = home.join(format!(".{}", name));

    verify_or_create(&home_dir)?;

    Ok(home_dir)
}

/// verify that the folder exists or create it.
fn verify_or_create(dir: &Path) -> crate::Result<()> {
    if dir.is_dir() {
        return Ok(());
    }
    Ok(fs::create_dir_all(dir)?)
}
