use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub fn create_file(dir_path: &Path, file_name: &str, content: &[u8]) -> Result<(), std::io::Error> {
    let full_path = dir_path.join(file_name);

    let mut file = File::create(full_path)?;
    file.write_all(content)?;

    Ok(())
}

pub fn create_dir(full_path: &Path, recursive: bool) -> Result<(), std::io::Error> {
    if recursive {
        fs::create_dir_all(full_path)?;
    } else {
        fs::create_dir(full_path)?;
    }

    Ok(())
}

pub fn remove_dir(full_path: &Path, recursive: bool) -> Result<(), std::io::Error> {
    if recursive {
        fs::remove_dir_all(full_path)?;
    } else {
        fs::remove_dir(full_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_create_file_with_content() {
        let tmp_dir = tempdir().unwrap();
        let tmp_dir_path = tmp_dir.path();
        let file_name = "test.txt";
        let content = b"fake content";

        let result = create_file(tmp_dir_path, file_name, content);

        assert!(result.is_ok());

        let created_path = tmp_dir_path.join(file_name);
        assert!(created_path.exists());

        let read = fs::read(created_path).unwrap();
        assert_eq!(read, content);
    }

    #[test]
    fn create_dir_non_recursive() {
        let tmp_dir = tempdir().unwrap();
        let new_dir = tmp_dir.path().join("new_dir");

        let result = create_dir(&new_dir, false);
        assert!(result.is_ok());

        assert!(new_dir.exists());
        assert!(new_dir.is_dir());
    }

    #[test]
    fn create_dir_recursive() {
        let tmp_dir = tempdir().unwrap();
        let nested = tmp_dir.path().join("a/b/c");

        let result = create_dir(&nested, true);

        assert!(result.is_ok());

        assert!(nested.exists());
        assert!(nested.is_dir());

        let semi_nested = tmp_dir.path().join("a/b");
        assert!(semi_nested.exists());
        assert!(semi_nested.is_dir());
    }

    #[test]
    fn remove_dir_non_recursive() {
        let tmp_dir = tempdir().unwrap();

        assert!(tmp_dir.path().exists());

        let result = remove_dir(tmp_dir.path(), false);

        assert!(result.is_ok());
    }
}
