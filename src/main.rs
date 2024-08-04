#[derive(Debug)]
struct Repository {
    _name: String,
    last_commit_id: i32,
    commits: Vec<Commit>,
    head: Branch,
    branch: Branch,
    branches: Vec<Branch>,
}

#[derive(Debug, Clone)]
struct Commit {
    id: i32,
    message: String,
    _parent: Option<Box<Commit>>,
}

#[derive(Debug, Clone)]
struct Branch {
    name: String,
    commit: Commit,
}

impl Repository {
    fn add_commit(&mut self, id: i32, message: String) {
        let new_commit = Commit {
            id,
            message,
            _parent: None,
        };

        self.last_commit_id = id;
        self.commits.push(new_commit.clone());

        self.head.commit = new_commit.clone();
        self.branch.commit = new_commit.clone();

        println!(
            "Commit created with id: {} and message: {}",
            new_commit.id, new_commit.message
        );
    }

    fn _get_commit(&mut self, id: usize) -> &Commit {
        let commit = &self.commits[id];
        println!(
            "commit id: {} commit message: {}",
            commit.id, commit.message
        );

        return commit;
    }

    fn log(&mut self) -> &Commit {
        let commit = &self.head.commit;
        println!("Running log {:?}", commit);
        return commit;
    }

    fn checkout(&mut self, name: String) {
        for i in 0..self.branches.len() {
            if self.branches[i].name == name {
                println!("Switching to existing branch: {}", name);
                self.head = self.branches[i].clone();
                return;
            }
        }

        let new_branch = Branch {
            name: name.clone(),
            commit: self.head.commit.clone(),
        };

        self.branches.push(new_branch.clone());
        self.head = new_branch.clone();
        println!("Switching to new branch: {}", name.clone());
        return;
    }
}

fn init_git(name: String) -> Repository {
    let empty_commit = Commit {
        id: 0,
        message: "".to_string(),
        _parent: None,
    };

    let master = Branch {
        name: "master".to_string(),
        commit: empty_commit,
    };

    return Repository {
        _name: name,
        last_commit_id: 0,
        commits: vec![],
        head: master.clone(),
        branch: master.clone(),
        branches: vec![master],
    };
}

fn main() {
    let mut repo = init_git(String::from("jou"));

    repo.add_commit(0, String::from("First Commit"));

    repo.add_commit(1, String::from("2nd Commit"));
    repo.add_commit(2, String::from("Third"));

    repo.log();

    repo.checkout("name".to_string());
    repo.checkout("jou".to_string());
    repo.checkout("name".to_string());
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

        // Add new commit
        git.add_commit(2, String::from("Second commit"));
        // Test last commit and amount of commits match
        // when new commit was added
        assert_eq!(git.last_commit_id, 2);
        assert_eq!(git.commits.len(), 2);
    }
}
