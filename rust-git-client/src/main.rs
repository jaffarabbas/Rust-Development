use git2::{Commit, ObjectType, Repository, Signature, StatusOptions};
use chrono::{Utc, TimeZone};
use std::fs;
use std::path::Path;
use std::io::{self, Write};

// Function to get a list of directories in a folder
pub fn get_directories_in_folder(folder_path: &str) -> Vec<String> {
    let entries = fs::read_dir(folder_path)
        .expect("Failed to read directory")
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                if e.path().is_dir() {
                    Some(e.file_name().to_string_lossy().into_owned())
                } else {
                    None
                }
            })
        })
        .collect();

    entries
}

// Function to access a Git repository given a path
pub fn open_repo(path: &str) -> Result<Repository, git2::Error> {
    Repository::open(path)
}

// Function to commit changes and push to the remote repository
pub fn commit_and_push_auto(repo: &Repository) -> Result<(), git2::Error> {
    // Create a signature with default author and committer details
    let signature = Signature::now("", "")?;

    // Get the repository's index
    let mut index = repo.index()?;

    // Get the status of the repository
    let statuses = repo.statuses(None)?;

    // Iterate over the statuses and commit each change separately
    for entry in statuses.iter() {
        let path = entry.path().unwrap();
        let status = entry.status();

        // Skip untracked files
        if status.is_wt_new() {
            continue;
        }

        // Display the change details
        println!("Change: {:?}", path);
        println!("Status: {:?}", status);

        // Prompt the user to commit the change
        print!("Do you want to commit this change? (Y/N): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let user_input = input.trim().to_uppercase();

        if user_input == "Y" {
            // Stage the change
            index.add_path(Path::new(path))?;

            // Write the tree and create a new commit
            let oid = index.write_tree()?;
            let tree = repo.find_tree(oid)?;
            let parent_commit = repo.find_commit(repo.head()?.target().unwrap())?;

            let commit_message = format!("Auto committed at {}", Utc::now());

            repo.commit(
                Some("HEAD"),       // point HEAD to the new commit
                &signature,         // author
                &signature,         // committer
                &commit_message,    // commit message
                &tree,              // tree
                &[&parent_commit],  // parents
            )?;

            // Reset the index for the next iteration
            index.clear()?;
        }
    }

    // Push to the remote repository (assuming there is a remote named "origin" and a branch named "master")
    let mut remote = repo.find_remote("origin")?;
    let refspec = "refs/heads/master:refs/heads/master";
    remote.push(&[refspec], None)?;

    Ok(())
}

// Function to get the status of the repository
pub fn get_repo_status(repo: &Repository) -> Result<(), git2::Error> {
    let mut status_opts = StatusOptions::new();
    status_opts.include_untracked(true).recurse_untracked_dirs(true);

    let statuses = repo.statuses(Some(&mut status_opts))?;

    for entry in statuses.iter() {
        let path = entry.path().unwrap();
        let status = entry.status();

        println!("{:?}: {:?}", path, status);
    }

    Ok(())
}

fn main() {
    // Example usage
    let folder_path = "J://GitHub"; // Use your folder path
    let directories = get_directories_in_folder(folder_path);
    
    // Print the list of directories
    for (i, dir) in directories.iter().enumerate() {
        println!("{}: {}", i + 1, dir);
    }

    // Prompt the user to enter a directory number
    print!("Enter the directory number to select as a repository: ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let selected_index: usize = input.trim().parse().expect("Invalid input");

    if selected_index > 0 && selected_index <= directories.len() {
        let selected_directory = &directories[selected_index - 1];
        let repo_path = format!("{}/{}", folder_path, selected_directory);
        
        let repo = open_repo(&repo_path).expect("Failed to open repository");
        
        commit_and_push_auto(&repo).expect("Failed to commit and push");
        
        get_repo_status(&repo).expect("Failed to get repository status");
    } else {
        println!("Invalid directory number.");
    }
}
