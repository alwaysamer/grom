use git2::{self, FetchOptions, IndexAddOption, PushOptions, RemoteCallbacks, Repository};

pub fn init_sync(path: String, remote: String) -> Result<(), git2::Error> {
    let repo: Repository = Repository::init(path.clone())?;
    repo.remote_set_url("origin", remote.as_str())?;
    push_initial_changes(path)?;
    Ok(())
}

pub fn push_initial_changes(path: String) -> Result<(), git2::Error> {
    let repo: Repository = Repository::open(path)?;

    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let signature = repo.signature()?;
    repo.commit(
        Some("reafs/heads/main"),
        &signature,
        &signature,
        "Initial",
        &tree,
        &[],
    )?;

    let mut remote = repo.find_remote("origin")?;
    let mut callbacks = git2::RemoteCallbacks::new();

    let mut auth_attempts = 0;
    let max_auth_attempts = 1;

    callbacks.credentials(move |url, username_from_url, _| {
        if auth_attempts >= max_auth_attempts {
            return Err(git2::Error::from_str(
                "Maximum authentication attempts exceeded.",
            ));
        }
        auth_attempts += 1;
        if url.starts_with("ssh://") || url.starts_with("git@") {
            match git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                Ok(cred) => Ok(cred),
                Err(_) => {
                    let username: String = cliclack::input("Enter your Git Username.")
                        .interact()
                        .unwrap();
                    let password: String = cliclack::input("Enter your Git Password.")
                        .interact()
                        .unwrap();
                    match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                        Ok(cred) => Ok(cred),
                        Err(e) => return Err(e),
                    }
                }
            }
        } else {
            let username: String = cliclack::input("Enter your Git Username.")
                .interact()
                .unwrap();
            let password: String = cliclack::input("Enter your Git Password.")
                .interact()
                .unwrap();
            match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                Ok(cred) => Ok(cred),
                Err(e) => return Err(e),
            }
        }
    });
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    match remote.push(
        &["refs/heads/main:refs/heads/main"],
        Some(&mut push_options),
    ) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn push_changes(path: String, message: String) -> Result<(), git2::Error> {
    let repo: Repository = Repository::open(path)?;

    let mut index = repo.index()?;
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let signature = repo.signature()?;
    let head = repo.head()?.peel_to_commit()?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &message,
        &tree,
        &[&head],
    )?;

    let mut remote = repo.find_remote("origin")?;
    let mut callbacks = git2::RemoteCallbacks::new();

    let mut auth_attempts = 0;
    let max_auth_attempts = 1;

    callbacks.credentials(move |url, username_from_url, _| {
        if auth_attempts >= max_auth_attempts {
            return Err(git2::Error::from_str(
                "Maximum authentication attempts exceeded.",
            ));
        }
        auth_attempts += 1;
        if url.starts_with("ssh://") || url.starts_with("git@") {
            match git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                Ok(cred) => Ok(cred),
                Err(_) => {
                    let username: String = cliclack::input("Enter your Git Username.")
                        .interact()
                        .unwrap();
                    let password: String = cliclack::input("Enter your Git Password.")
                        .interact()
                        .unwrap();
                    match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                        Ok(cred) => Ok(cred),
                        Err(e) => return Err(e),
                    }
                }
            }
        } else {
            let username: String = cliclack::input("Enter your Git Username.")
                .interact()
                .unwrap();
            let password: String = cliclack::input("Enter your Git Password.")
                .interact()
                .unwrap();
            match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                Ok(cred) => Ok(cred),
                Err(e) => return Err(e),
            }
        }
    });
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    match remote.push(
        &["refs/heads/main:refs/heads/main"],
        Some(&mut push_options),
    ) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e),
    }
}

pub fn pull_changes(path: String) -> Result<(), git2::Error> {
    let repo = Repository::open(path)?;

    let mut remote = repo.find_remote("origin")?;
    let mut remote_callbacks = RemoteCallbacks::new();

    let mut auth_attempts = 0;
    let max_auth_attempts = 1;

    remote_callbacks.credentials(move |url, username_from_url, _| {
        if auth_attempts >= max_auth_attempts {
            return Err(git2::Error::from_str(
                "Maximum authentication attempts exceeded.",
            ));
        }
        auth_attempts += 1;
        if url.starts_with("ssh://") || url.starts_with("git@") {
            match git2::Cred::ssh_key_from_agent(username_from_url.unwrap_or("git")) {
                Ok(cred) => Ok(cred),
                Err(_) => {
                    let username: String = cliclack::input("Enter your Git Username.")
                        .interact()
                        .unwrap();
                    let password: String = cliclack::input("Enter your Git Password.")
                        .interact()
                        .unwrap();
                    match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                        Ok(cred) => Ok(cred),
                        Err(e) => return Err(e),
                    }
                }
            }
        } else {
            let username: String = cliclack::input("Enter your Git Username.")
                .interact()
                .unwrap();
            let password: String = cliclack::input("Enter your Git Password.")
                .interact()
                .unwrap();
            match git2::Cred::userpass_plaintext(username.as_str(), password.as_str()) {
                Ok(cred) => Ok(cred),
                Err(e) => return Err(e),
            }
        }
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);

    remote.fetch(&["main"], Some(&mut fetch_options), None)?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", "main");
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                r.set_target(fetch_commit.id(), "Fast-Forward")?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;
            }
            Err(_) => {
                repo.reference(&refname, fetch_commit.id(), true, "Setting reference")?;
                repo.set_head(&refname)?;
                repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
            }
        }
    } else {
        return Err(git2::Error::from_str(
            "Non-fast-forward merge not supported.",
        ));
    }
    Ok(())
}
