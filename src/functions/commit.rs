use std::io::Write;
use git2::{Error, ErrorClass, ErrorCode, Oid, Repository, Signature};
use inquire::{InquireError, Select};
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

fn select_emoji() -> Result<String, InquireError> {
    let options = vec![
        "🎨 :art: Improve structure/format",
        "⚡  :zap: Improve performance",
        "🔥 :fire: Remove code or files",
        "🐛 :bug: Fix a bug",
        "🚑️ :ambulance: Critical hotfix",
        "✨ :sparkles: Introduce new features",
        "📝 :memo: Add or update documentation",
        "🚀 :rocket: Deploy or release something",
        "💄 :lipstick: Add or update UI/style files",
        "🎉 :tada: Initial commit",
        "✅ :white_check_mark: Add, update, or pass tests",
        "🔒 :lock: Fix or improve security issues",
        "🔐 :closed_lock_with_key: Add or update secrets",
        "🔖 :bookmark: Release or version tags",
        "🚨 :rotating_light: Fix compiler/linter warnings",
        "🚧 :construction: Work in progress",
        "💚 :green_heart: Fix CI build",
        "⬇️ :arrow_down: Downgrade dependencies",
        "⬆️ :arrow_up: Upgrade dependencies",
        "📌 :pushpin: Pin dependencies to specific versions",
        "👷 :construction_worker: Add or update CI/CD build system",
        "📈 :chart_with_upwards_trend: Add or update analytics/tracking code",
        "♻️ :recycle: Refactor code",
        "➕ :heavy_plus_sign: Add a dependency",
        "➖ :heavy_minus_sign: Remove a dependency",
        "🔧 :wrench: Add or update configuration files",
        "🔨 :hammer: Add or update build scripts",
        "🌐 :globe_with_meridians: Internationalization or localization",
        "✏️ :pencil2: Fix typos",
        "💩 :poop: Write bad code that needs improvement",
        "⏪ :rewind: Revert changes",
        "🔀 :twisted_rightwards_arrows: Merge branches",
        "📦 :package: Add or update compiled files or dependencies",
        "👽 :alien: Update code due to external API changes",
        "🚚 :truck: Move or rename files",
        "📄 :page_facing_up: Add or update license",
        "💥 :boom: Introduce breaking changes",
        "🍱 :bento: Add or update assets",
        "♿ :wheelchair: Improve accessibility",
        "💡 :bulb: Add or update comments in source code",
        "🍻 :beers: Celebrate or add fun Easter eggs",
        "💬 :speech_balloon: Add or update text and messages",
        "🗃️ :card_file_box: Perform database-related changes",
        "🔊 :loud_sound: Add or update logs",
        "🔇 :mute: Remove logs",
        "👥 :busts_in_silhouette: Add or update contributor(s)",
        "🚸 :children_crossing: Improve UX or UI accessibility",
        "🏗️ :building_construction: Make architectural changes",
        "📱 :iphone: Work on responsive design or mobile support",
        "🤡 :clown_face: Mock related changes",
        "🥚 :egg: Add or update Easter eggs",
        "🙈 :see_no_evil: Add or update .gitignore file",
        "🧠 :brain: Add or update logic or algorithms",
        "🧰 :toolbox: Add or update tooling/utilities",
        "🧪 :test_tube: Add or update tests",
        "🧱 :bricks: Infrastructure changes",
        "🩹 :adhesive_bandage: Simple fix not critical",
        "🩺 :stethoscope: Add or update health checks",
        "🧩 :jigsaw: Add or update modular code/components",
        "🧹 :broom: Remove useless files or code",
        "🧵 :thread: Add or update multithreading/concurrency",
        "🕹️ :joystick: Add or update scripts/tooling",
        "🧑‍💻 :technologist: Improve developer experience",
        "🗑️ :wastebasket: Deprecate or remove obsolete code",
        "🏁 :checkered_flag: Finish a feature or milestone",
        "🪄 :magic_wand: Minor visual or UX enhancements",
    ];

    let prompt = Select::new("Select an emoji for your commit:", options);

    match prompt.prompt() {
        Ok(choice) => Ok(choice.to_string()),
        Err(e) => Err(e),
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
/// - Configuration values for username or email cannot be retrieved.
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
pub fn commit(repo: Option<&Repository>, debug: bool) -> Result<Oid, Error> {
    let owned_repo;
    let current_repo = match repo{
        Some(r) => r,
        _ => {
            owned_repo = get_current_repository().map_err(|e| e)?;
            &owned_repo
        }
    };
    let repo_configuration = match current_repo.config() {
        Ok(config) => config,
        Err(e) => return Err(e)
    };

    let mut commit_message = String::new();

    if !debug {
        let binding;
        let emoji = match select_emoji() {
            Ok(emo) => match emo.chars().next() {
                Some(char) => {
                    binding = char.to_string();
                    binding.as_str()
                }
                _ => "",
            },
            Err(e) => return Err(Error::new(ErrorCode::NotFound, ErrorClass::Invalid, e.to_string()))
        };

        commit_message.push_str(emoji);

        let title = user_input("Provide a commit title > ".to_string());
        commit_message.push_str(title.as_str());
        commit_message.push_str("\n");
        let message = user_input("Provide a commit message > ".to_string());
        commit_message.push_str(message.as_str());
        commit_message.push_str("\n");
        let breaking_changes = user_input("Provide a breaking changes description > ".to_string());
        commit_message.push_str(format!("BREAKING CHANGES: {}\n", breaking_changes).as_str());
    } else {
        commit_message.push_str("unit testing");
    }

    let name = repo_configuration.get_string("user.name")?.to_string();
    let email = repo_configuration.get_string("user.email")?.to_string();

    // get index and write tree
    let mut index = match current_repo.index() {
        Ok(idx) => idx,
        Err(e) => return Err(e)
    };
    let tree_oid = match index.write_tree() {
        Ok(oid) => oid,
        Err(e) => return Err(e)
    };
    let tree = match current_repo.find_tree(tree_oid) {
        Ok(idx_tree) => idx_tree,
        Err(e) => return Err(e)
    };

    // get HEAD commit to set as parent
    let parents = match current_repo.head() {
        Ok(head) => {
            match head.peel_to_commit() {
                Ok(commit) => vec![commit],
                Err(e) => return Err(e),
            }
        }
        Err(_) => vec![], // Unborn branch, so NO parent
    };

    let signature = Signature::now(&name, &email)?;

    let parent_refs: Vec<&git2::Commit> = parents.iter().collect();
    current_repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &(commit_message.as_str()),
        &tree,
        &parent_refs,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use git2::Repository;
    use std::fs::File;
    use tempfile::tempdir;
    use crate::functions::add::add;


    #[test]
    fn test_commit() {
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
        repo.commit(Some("HEAD"), &signature, &signature, "Initial commit", &tree, &[]).unwrap();

        let result = commit(Some(&repo), true);

        assert!(result.is_ok());

        let head = repo.head().unwrap();
        let commit = head.peel_to_commit().unwrap();
        assert_eq!(commit.message().unwrap(), "unit testing");
    }
}