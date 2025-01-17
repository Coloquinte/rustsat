use std::{env, fs, path::Path, str};

fn main() {
    // Build C++ library
    // Full commit hash needs to be provided
    #[cfg(not(doc))]
    build(
        "https://github.com/chrjabs/glucose4.git",
        "main",
        "e06dbf6d9b17b68794767617cf453b31e72a3226",
    );

    let out_dir = env::var("OUT_DIR").unwrap();

    // Built solver is in out_dir
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rustc-link-search={}/lib", out_dir);
}

fn build(repo: &str, branch: &str, commit: &str) {
    let out_dir = env::var("OUT_DIR").unwrap();
    let mut glucose_dir_str = out_dir.clone();
    glucose_dir_str.push_str("/glucose");
    let glucose_dir = Path::new(&glucose_dir_str);
    if update_repo(glucose_dir, repo, branch, commit)
        || !Path::new(&out_dir)
            .join("lib")
            .join("libglucose4.a")
            .exists()
    {
        cmake::build(glucose_dir);
    };

    println!("cargo:rustc-link-lib=static=glucose4");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-flags=-l dylib=c++");

    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-flags=-l dylib=stdc++");
}

/// Returns true if there were changes, false if not
fn update_repo(path: &Path, url: &str, branch: &str, commit: &str) -> bool {
    let mut changed = false;
    let target_oid = git2::Oid::from_str(commit)
        .unwrap_or_else(|e| panic!("Invalid commit hash {}: {}", commit, e));
    let repo = match git2::Repository::open(path) {
        Ok(repo) => {
            // Check if already at correct commit
            if let Some(oid) = repo.head().unwrap().target_peel() {
                if oid == target_oid {
                    return changed;
                }
            };
            // Check if commit needs to be fetched
            if repo.find_commit(target_oid).is_err() {
                // Fetch repo
                let mut remote = repo.find_remote("origin").unwrap_or_else(|e| {
                    panic!("Expected remote \"origin\" in git repo {:?}: {}", path, e)
                });
                remote.fetch(&[branch], None, None).unwrap_or_else(|e| {
                    panic!(
                        "Could not fetch \"origin/{}\" for git repo {:?}: {}",
                        branch, path, e
                    )
                });
                drop(remote);
            }
            repo
        }
        Err(_) => {
            if path.exists() {
                fs::remove_dir_all(path).unwrap_or_else(|e| {
                    panic!(
                        "Could not delete directory {}: {}",
                        path.to_str().unwrap(),
                        e
                    )
                });
            };
            changed = true;
            git2::Repository::clone(url, path)
                .unwrap_or_else(|e| panic!("Could not clone repository {}: {}", url, e))
        }
    };
    let target_commit = repo
        .find_commit(target_oid)
        .unwrap_or_else(|e| panic!("Could not find commit {}: {}", commit, e));
    repo.checkout_tree(target_commit.as_object(), None)
        .unwrap_or_else(|e| panic!("Could not checkout commit {}: {}", commit, e));
    repo.set_head_detached(target_oid)
        .unwrap_or_else(|e| panic!("Could not detach head at {}: {}", commit, e));
    changed
}
