use std::io::Write;
use git2::{Repository, Signature};
use inquire::Select;
use crate::types::repository::get_current_repository;

fn user_input(message: String) -> String {
    use std::io;

    print!("{}", message);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(_no_updates_is_fine) => {},
    }
    input.trim().to_string()
}

fn select_emoji() -> Result<String, String> {
    let options = vec![
        "🎨 :art: Improve structure/format",
        "⚡️ :zap: Improve performance",
        "🔥 :fire: Remove code",
        "🐛 :bug: Fix a bug",
        "✨ :sparkles: Introduce new features",
    ];

    let prompt = Select::new("Select an emoji for your commit:", options);

    match prompt.prompt() {
        Ok(choice) => Ok(choice.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

/// Creates a new commit on the current branch in the specified repository.
///
/// This function creates and writes a commit object that captures the current state
/// of the repository index (staging area) along with a commit message composed of
/// an emoji, a user-provided description, and breaking changes notes. If no repository
/// is provided (`None`), it attempts to discover the current repository automatically.
///
/// # Arguments
///
/// * `repo` - An optional reference to a `Repository`. If `None`, the function
///            attempts to find the current repository automatically.
///
/// # Errors
///
/// Returns an `Err(String)` if:
/// - The current repository cannot be determined.
/// - Configuration values for user name or email cannot be retrieved.
/// - The emoji selection fails.
/// - There are problems accessing the repository index or writing the tree.
/// - The HEAD commit cannot be retrieved (in case of an existing commit).
/// - Committing to the repository fails.
///
/// # Returns
///
/// * `Ok(0)` on successful commit.
///
/// # Workflow
///
/// - Gets the current or specified repository.
/// - Reads user configuration for name and email.
/// - Prompts the user for a commit message and breaking changes description.
/// - Builds the commit message by prefixing it with an emoji.
/// - Constructs the commit tree from the current index.
/// - Retrieves the current `HEAD` commit as the parent (if any).
/// - Creates a commit with the assembled information.
pub fn commit(repo: Option<&Repository>) -> Result<i8, String> {
    let owned_repo;
    let current_repo = match repo{
        Some(r) => r,
        _ => {
            owned_repo = get_current_repository().map_err(|e| e.to_string())?;
            &owned_repo
        }
    };
    let repo_configuration = match current_repo.config() {
        Ok(config) => config,
        Err(e) => return Err(e.to_string())
    };

    let mut commit_message = match select_emoji() {
        Ok(emo) => match emo.chars().next() {
            Some(char) => char.to_string(),
            _ => "".to_string(),
        },
        Err(e) => return Err(e)
    };

    let description = user_input("Provide a commit message > ".to_string());
    commit_message.push_str(description.as_str());
    let breaking_changes = user_input("Breaking changes > ".to_string());
    commit_message.push_str(breaking_changes.as_str());

    let name = repo_configuration.get_string("user.name").unwrap().to_string();
    let email = repo_configuration.get_string("user.email").unwrap().to_string();

    // get index and write tree
    let mut index = match current_repo.index() {
        Ok(idx) => idx,
        Err(e) => return Err(e.message().to_string())
    };
    let tree_oid = match index.write_tree() {
        Ok(oid) => oid,
        Err(e) => return Err(e.message().to_string())
    };
    let tree = match current_repo.find_tree(tree_oid) {
        Ok(idx_tree) => idx_tree,
        Err(e) => return Err(e.message().to_string())
    };

    // get HEAD commit to set as parent
    let parents = match current_repo.head() {
        Ok(head) => {
            match head.peel_to_commit() {
                Ok(commit) => vec![commit],
                Err(e) => return Err(e.to_string()),
            }
        }
        Err(_) => vec![], // Unborn branch, so NO parent
    };

    let signature = Signature::now(&name, &email).unwrap();

    let parent_refs: Vec<&git2::Commit> = parents.iter().collect();
    current_repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &(commit_message.as_str()),
        &tree,
        &parent_refs,
    ).unwrap();

    Ok(0)
}