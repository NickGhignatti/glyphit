use git2::Repository;
use crate::types::repository::get_current_repository;

/// Adds a list of files to the staging area (index) of a Git repository.
///
/// This function takes a list of file paths and adds them to the index (staging area)
/// of the specified `repo`. If no repository is provided (`None`), it attempts
/// to discover the current repository automatically.
///
/// # Arguments
///
/// * `files` - A reference to a vector of strings representing file paths to add.
/// * `repo` - An optional reference to a `Repository`. If `None`, the function tries
///            to find the current repository automatically.
///
/// # Errors
///
/// Returns an `Err(String)` if:
/// - The repository cannot be determined.
/// - The repository index cannot be accessed.
/// - Adding any of the specified files to the index fails.
/// - Writing the index to disk fails.
///
/// # Returns
///
/// * `Ok(0)` on success.
pub fn add(files: &Vec<String>, repo: Option<&Repository>) -> Result<i8, String> {
    let owned_repo;
    let current_repo = match repo{
        Some(r) => r,
        _ => {
            owned_repo = get_current_repository().map_err(|e| e.to_string())?;
            &owned_repo
        }
    };

    // get the index (staging area)
    let mut index = match current_repo.index() {
        Ok(index) => index,
        Err(e) => return Err(e.to_string()),
    };

    let corrected_files: Vec<String> = files
        .iter()
        .map(|f| {
            f.strip_prefix(".\\").unwrap_or(f)
                .to_string()
        })
        .collect();

    index.add_all(corrected_files, git2::IndexAddOption::DEFAULT, None).expect("Error while adding all files");

    // write index to disk
    index.write().expect("Error while writing the index in the disk");

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use tempfile::tempdir;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test_add() {
        // create temp dir and init empty repo (avoiding collision)
        let temp_dir = tempdir().unwrap();
        let repo = &Repository::init(temp_dir.path()).unwrap();

        // create a dummy file
        let file_path = temp_dir.path().join("dummy.txt");
        File::create(&file_path).unwrap();

        let files = vec!["dummy.txt".to_string()];

        add(&files, Some(repo)).unwrap();

        // check index contains dummy.txt
        let index = repo.index().unwrap();
        assert!(index.get_path(Path::new("dummy.txt"), 0).is_some());
    }
}