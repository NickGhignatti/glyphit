use crate::types::repository::get_current_repository;
use git2::{Config, Cred, Error, PushOptions, RemoteCallbacks, Repository};

// function to create remote callbacks for a https repo url
fn create_https_callback(repo_config: Config) -> RemoteCallbacks<'static> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |url, username_from_url, _allowed_types| {
        Cred::credential_helper(&repo_config, url, username_from_url)
    });

    callbacks
}

// function to create remote callbacks for an ssh repo url
fn create_ssh_callbacks() -> RemoteCallbacks<'static> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        let username = username_from_url.unwrap_or("git");
        Cred::ssh_key_from_agent(username)
    });

    callbacks
}

/// Pushes the current branch to the remote "origin" repository.
///
/// This function attempts to push the HEAD branch of the provided `repo`
/// to its "origin" remote. If no repository is provided (`None`),
/// it will try to discover and use the current local repository.
///
/// # Arguments
///
/// * `repo` - An optional reference to a `Repository`. If `None`,
///            the function tries to find the current repository automatically.
///
/// # Errors
///
/// Returns an `Err(String)` if:
/// - The current repository cannot be determined.
/// - The HEAD reference cannot be retrieved.
/// - The "origin" remote cannot be found.
/// - The push operation itself fails.
///
/// # Returns
///
/// * `Ok(0)` on successful push.
pub fn push(repo: Option<&Repository>) -> Result<(), Error> {
    let owned_repo;
    let current_repo = match repo {
        Some(r) => r,
        _ => {
            owned_repo = get_current_repository().map_err(|e| e)?;
            &owned_repo
        }
    };

    let head = match current_repo.head() {
        Ok(h) => h,
        Err(e) => return Err(e),
    };
    let branch = head.shorthand().unwrap();
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);

    let mut push_options = PushOptions::new();

    let repo_configuration = match current_repo.config() {
        Ok(config) => config,
        Err(e) => return Err(e)
    };

    let url = repo_configuration.get_string("remote.origin.url")?.to_string();
    if !url.contains("https") {
        let callbacks = create_ssh_callbacks();
        push_options.remote_callbacks(callbacks);
    } else {
        let config = match current_repo.config() {
            Ok(config) => config,
            Err(e) => return Err(e)
        };
        let callbacks = create_https_callback(config);
        push_options.remote_callbacks(callbacks);
    }

    let mut origin = match current_repo.find_remote("origin") {
        Ok(org) => org,
        Err(e) => return Err(e),
    };

    origin.push(&[refspec], Some(&mut push_options))
}

#[cfg(test)]
mod tests {
    use crate::functions::add::add;
    use crate::functions::commit::commit;
    use crate::functions::push::push;
    use git2::{Repository, Signature};
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_push() {
        let temp_dir = tempdir().unwrap();
        let repo = Repository::init(temp_dir.path()).unwrap();

        let file_path = temp_dir.path().join("initial.txt");
        File::create(&file_path).unwrap();

        add(&vec!["initial.txt".to_string()], Some(&repo)).unwrap();

        let mut config = repo.config().unwrap();
        config.set_str("user.name", "Test User").unwrap();
        config.set_str("user.email", "test@example.com").unwrap();

        let tree;
        let signature = Signature::now("Test User", "test@example.com").unwrap();
        let mut index = repo.index().unwrap();
        let tree_id = index.write_tree().unwrap();
        tree = repo.find_tree(tree_id).unwrap();
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            "Initial commit",
            &tree,
            &[],
        )
        .unwrap();

        let _ = commit(Some(&repo), true);

        let result = push(Some(&repo));

        // we expect a fail due the absence of a real remote
        assert!(result.is_err());
    }
}
