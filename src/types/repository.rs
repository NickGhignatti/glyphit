use git2::{Error, Repository};

/// Attempts to discover and open the current Git repository starting from the current directory.
///
/// This function uses the `git2` crate's `Repository::discover` method to find the
/// nearest enclosing Git repository by searching parent directories.
///
/// # Returns
///
/// * `Ok(Repository)` - The discovered Git repository object.
/// * `Err(Error)` - If no repository could be found or an error occurred while accessing it.
pub fn get_current_repository() -> Result<Repository, Error> {
    Repository::discover(".")
}

#[cfg(test)]
mod test {
    use crate::types::repository::get_current_repository;

    #[test]
    fn test_current_repository() {
        match get_current_repository() {
            Err(e) => panic!("{}", e.message()),
            Ok(repo) => {
                println!("{:?}", repo.workdir().unwrap().file_name().unwrap().to_str());
                assert!(true)
            }
        }
    }
}