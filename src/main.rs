#[derive(Debug)]
struct Repository {
    _name: String,
    last_commit_id: i32,
    commits: Vec<Commit>,
    head: Option<Commit>,
}

#[derive(Debug, Clone)]
struct Commit {
    _id: i32,
    _message: String,
    _parent: Option<Box<Commit>>,
}

impl Repository {
    fn add_commit(&mut self, id: i32, message: String) {
        let new_commit = Commit {
            _id: id,
            _message: message,
            _parent: self.head.clone().map(Box::new),
        };

        self.last_commit_id = id;
        self.commits.push(new_commit.clone());
        self.head = Some(new_commit.clone());

        println!(
            "Commit created with id: {} and message: {}",
            new_commit._id, new_commit._message
        );
    }

    fn get_commit(&mut self, id: usize) -> &Commit {
        let commit = &self.commits[id];
        println!(
            "commit id: {} commit message: {}",
            commit._id, commit._message
        );

        return commit;
    }
}

fn init_git(name: String) -> Repository {
    return Repository {
        _name: name,
        last_commit_id: 0,
        commits: vec![],
        head: None,
    };
}

fn main() {
    let mut repo = init_git(String::from("jou"));

    repo.add_commit(1, String::from("First Commit"));

    repo.add_commit(2, String::from("2nd Commit"));
    repo.add_commit(3, String::from("Third"));

    println!(
        "{:?}",
        repo.head.as_ref().unwrap()._parent.as_ref().unwrap()._id
    );

    println!("{:?}", repo.get_commit(2)._id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_commits() {
        let mut git = init_git(String::from("test_repo"));

        git.add_commit(1, String::from("Initial Commit"));
        // Test last commit and amount of commits match
        assert_eq!(git.last_commit_id, 1);
        assert_eq!(git.commits.len(), 1);
        // Test current head
        assert_eq!(git.head.as_ref().unwrap()._id, 1);
        assert_eq!(git.head.as_ref().unwrap()._message, "Initial Commit");

        // Add new commit
        git.add_commit(2, String::from("Second commit"));
        // Test last commit and amount of commits match
        // when new commit was added
        assert_eq!(git.last_commit_id, 2);
        assert_eq!(git.commits.len(), 2);

        // Test current head
        assert_eq!(git.head.as_ref().unwrap()._id, 2);
        assert_eq!(git.head.as_ref().unwrap()._message, "Second commit");
    }
}
