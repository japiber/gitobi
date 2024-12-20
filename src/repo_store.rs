use gitwrap::git;
use crate::repo_document::{JsonDocument, RepoDocument};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use gitwrap::{add, clone, commit, config, pull, push, rev_parse};
use serde_json::Value;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

pub enum RepoStoreError {
    Initialize(Box<dyn Error>),
    Clone(Box<dyn Error>),
    Pull(Box<dyn Error>),
    Push(Box<dyn Error>),
    Commit(Box<dyn Error>),
    Clean(Box<dyn Error>),
}

impl Error for RepoStoreError {}

impl RepoStoreError {
    fn format(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoStoreError::Initialize(e) => write!(f, "failed to initialize repo: {}", e),
            RepoStoreError::Clone(e) => write!(f, "failed to clone repo: {}", e),
            RepoStoreError::Pull(e) => write!(f, "failed to pull repo: {}", e),
            RepoStoreError::Push(e) => write!(f, "failed to push repo: {}", e),
            RepoStoreError::Commit(e) => write!(f, "failed to commit repo: {}", e),
            RepoStoreError::Clean(e) => write!(f, "failed to clean repo: {}", e),
        }
    }
}

impl Display for RepoStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

impl Debug for RepoStoreError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.format(f)
    }
}

//pub type FnModify<T> = dyn Fn(&dyn RepoStore<T>) -> Result<(), Box<dyn Error>>;

pub trait RepoStore<T: PartialEq> {
    fn initialize(&self) -> Result<(), RepoStoreError>;
    fn document(&self, name: &str) -> impl RepoDocument<T>;
    fn pull(&self, rebase: bool) -> Result<(), RepoStoreError>;
    fn push(&self) -> Result<(), RepoStoreError>;
    fn commit(&self, msg: &str) -> Result<(), RepoStoreError>;
    fn clean(&self) -> Result<(), RepoStoreError>;
}


#[derive(Debug, Clone, Default)]
pub struct GitAuth {
    user: Option<String>,
    password: Option<String>,
    token: Option<String>,
    insecure: bool
}

#[derive(Debug, Clone, Default)]
pub struct GitCommit {
    commit_user: String,
    commit_email: String,
}

impl GitCommit {
    pub fn new(commit_user: &str, commit_email: &str) -> Self {
        Self {
            commit_user: String::from(commit_user),
            commit_email: String::from(commit_email),
        }
    }
    
    pub fn pair(&self) -> (String, String) {
        (self.commit_user.clone(), self.commit_email.clone())
    }
}

#[derive(Debug, Clone, Default)]
pub struct GitStore {
    name: String,
    repo_url: String,
    auth: GitAuth,
    repo_path: PathBuf,
    branch: Option<String>,
    commit: GitCommit,
}

impl GitStore {
    pub fn new(name: &str, url: &str, path: &str, branch: Option<&str>, auth: GitAuth, commit: GitCommit) -> Self {
        Self {
            name: String::from(name),
            repo_url: String::from(url),
            repo_path: Path::new("").join(path),
            branch: branch.map(String::from),
            auth,
            commit,
        }
    }

    fn clone(&self) -> Result<(), RepoStoreError> {
        let mut cmd = clone!(None,
            clone::repository(self.repo_url.as_str()),
            clone::directory(self.repo_path.to_str().unwrap())
        );
        if let Some(branch) = self.branch.clone() {
            cmd.option(clone::branch(branch.clone().as_str()))
        }
        if let Some(auth_header) = self.build_auth_header() {
            cmd.option(clone::config("http.extraHeader", &auth_header))
        }
        if self.auth.insecure {
            cmd.option(clone::config("http.sslVerify", "false"))
        }
        match cmd.execute() {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoStoreError::Clone(Box::new(e))),
        }
    }

    fn build_auth_header(&self) -> Option<String> {
        match self.auth.token.clone() {
            None => {
                if let Some(user) = self.auth.user.clone() {
                    if let Some(password) = self.auth.password.clone() {
                        let basic_token = BASE64_STANDARD.encode(format!("{}:{}", user, password));
                        let basic_auth = format!("Authorization: Basic {}", basic_token);
                        return Some(basic_auth)
                    }
                }
                None
            }
            Some(token) => {
                let bearer_auth = format!("Authorization: Bearer  {}", token);
                Some(bearer_auth)
            }
        }
    }

    fn set_repo_config(&self) -> Result<(), RepoStoreError> {
        let mut cmd = config::config(Some(self.repo_path.to_str().unwrap()));
        let (user, email) = self.commit.pair();
        cmd.option(config::entry("user.email", email.as_str()));
        cmd.option(config::entry("user.name", user.as_str()));
        match cmd.execute() {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoStoreError::Initialize(Box::new(e))),
        }
    }
    
    fn is_valid_repo(&self) -> bool {
        let mut cmd = rev_parse::rev_parse(Some(self.repo_path.to_str().unwrap()));
        cmd.option(rev_parse::is_inside_work_tree());
        match cmd.execute() {
            Ok(o) => o.contains("true"),
            Err(_) => false,
        }
    }
    
    fn create_dir_and_clone(&self) -> Result<(), RepoStoreError> {
        match fs::create_dir_all(&self.repo_path) {
            Ok(_) => {
                if let Err(e) = self.clone() {
                    return Err(RepoStoreError::Initialize(Box::new(e)));
                }
                self.set_repo_config()
            },
            Err(e) => Err(RepoStoreError::Initialize(Box::new(e))),
        }       
    }
}

impl RepoStore<Value> for GitStore {
    fn initialize(&self) -> Result<(), RepoStoreError> {
        match fs::exists(&self.repo_path) {
            Ok(exists) => {
                if exists {
                    if self.is_valid_repo() {
                        Ok(())
                    } else {
                        match fs::remove_dir_all(&self.repo_path) {
                            Ok(_) => self.create_dir_and_clone(),
                            Err(e) => Err(RepoStoreError::Initialize(Box::new(e))),
                        }
                    }
                } else {
                    self.create_dir_and_clone()
                }
            },
            Err(e) => Err(RepoStoreError::Initialize(Box::new(e))),
        }
    }

    fn document(&self, path: &str) -> impl RepoDocument<Value> {
        JsonDocument::new(self.repo_path.to_str().unwrap(), path)
    }

    fn pull(&self, rebase: bool) -> Result<(), RepoStoreError> {
        let mut cmd = pull::pull(Some(self.repo_path.to_str().unwrap()));
        if rebase {
            cmd.option(pull::rebase(""));
        }
        match cmd.execute() {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoStoreError::Pull(Box::new(e))),
        }
    }

    fn push(&self) -> Result<(), RepoStoreError> {
        let cmd = push::push(Some(self.repo_path.to_str().unwrap()));
        match cmd.execute() {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoStoreError::Push(Box::new(e))),
        }
    }

    fn commit(&self, msg: &str) -> Result<(), RepoStoreError> {
        let mut cmd_commit = commit::commit(Some(self.repo_path.to_str().unwrap()));
        cmd_commit.option(commit::all());
        cmd_commit.option(commit::message(msg));
        match cmd_commit.execute() {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoStoreError::Commit(Box::new(e))),
        }
    }

    fn clean(&self) -> Result<(), RepoStoreError> {
        todo!()
    }
}