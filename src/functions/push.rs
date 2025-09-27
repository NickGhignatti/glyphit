use git2::{Cred, PushOptions, RemoteCallbacks, Repository};
use crate::types::repository::get_current_repository;

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
pub fn push(repo: Option<&Repository>) -> Result<i8, String> {
    let owned_repo;
    let current_repo = match repo{
        Some(r) => r,
        _ => {
            owned_repo = get_current_repository().map_err(|e| e.to_string())?;
            &owned_repo
        }
    };

    let head = match current_repo.head() {
        Ok(h) => h,
        Err(e) => return Err(e.to_string()),
    };
    let branch = head.shorthand().unwrap();
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);

    let callbacks = create_ssh_callbacks();

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    let mut origin = match current_repo.find_remote("origin") {
        Ok(org) => org,
        Err(e) => return Err(e.to_string())
    };
    let _ = origin.push(&[refspec], Some(&mut push_options));

    Ok(0)
}