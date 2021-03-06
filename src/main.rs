extern crate git2;

use git2::*;

use std::env;
use std::cell::RefCell;

mod tree;

fn main() {
    let args: Vec<String> = env::args().collect();

    let repo = Repository::open(&args[1]).expect("Couldn't open repo");

    // Find the master branch
    let master_ref = repo.find_reference("refs/heads/master").unwrap();

    // Go from reference -> commit
    let master_commit = match master_ref.peel(ObjectType::Commit)
        .unwrap()
        .into_commit() {
        Ok(commit) => commit,
        Err(_) => panic!("Couldn't resolve to a commit")
    };

    let master_tree = master_commit.tree().unwrap();

    let mut paths = Vec::new();

    // Iterate through its tree
    tree::walk(master_tree, &repo, |root: &str, element: &TreeEntry|
               paths.push(tree::prefix(root, element.name().unwrap())));

    for p in paths.iter() {
        println!("{}", p);
    }
}
