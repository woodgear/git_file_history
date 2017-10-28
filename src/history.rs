use error::*;
use serde_json;
use std::collections::HashMap;
use chrono::prelude::*;
use std::path::{Path, PathBuf};
use std::hash::Hash;
#[derive(Debug, Deserialize)]
pub struct Commit {
    pub author: String,
    pub date: DateTime<Local>,
    pub msg: String,
    pub hash: String,
}

pub trait GitFileHistory {
    fn history<P: AsRef<Path>>(git_path: P, file_path: P) -> Result<Vec<Commit>>;
    fn history_bulk<P>(git_path: P, file_paths: Vec<P>) -> Result<HashMap<P, Vec<Commit>>>
    where
        P: AsRef<Path> + Hash + Eq + Clone,
    {
        let mut file_map = HashMap::new();
        for file in file_paths.into_iter() {
            let commits: Vec<Commit> = Self::history(git_path.clone(), file.clone())?;
            file_map.insert(file.clone(), commits);
        }
        Ok(file_map)
    }
}

#[derive(Debug)]
pub struct Git {}

fn call_git<P: AsRef<Path>>(dir: P, cmd: &str) -> Result<String> {
    use std::process::Command;
    Command::new("cmd")
        .current_dir(dir)
        .args(&["/C", &format!("git {}", cmd)])
        .output()
        .chain_err(|| format!("eval git fail"))
        .and_then(|output| {
            String::from_utf8(output.stdout).chain_err(|| "tran output to string fail")
        })
}

impl Git {
    pub fn trans(msg: String) -> Result<Vec<Commit>> {
        let mut list: Vec<Commit> = vec![];
        for item in msg.lines().into_iter() {
            let commit: Commit = serde_json::from_str(item).map_err(|e| {
                Error::with_chain(e, "parse from json fail")
            })?;
            list.push(commit);
        }
        Ok(list)
    }
}

impl GitFileHistory for Git {
    fn history<P: AsRef<Path>>(git_path: P, file_path: P) -> Result<Vec<Commit>> {
        let git_path: PathBuf = PathBuf::from(git_path.as_ref());
        let file_path: PathBuf = PathBuf::from(file_path.as_ref());
        let file_path = file_path.into_os_string().into_string().map_err(|e| {
            format!("{:?}", e)
        })?;
        let format = r#"{"date":"%cI","author":"%an","msg":"%s","hash":"%H"}"#;
        let cmd = format!(
            "log --follow --date-order --pretty={} {}",
            format,
            file_path
        );
        let out = call_git(git_path, &cmd)?;
        Git::trans(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_call_git() {
        let git_path = env::current_dir().unwrap().join("test").join("data").join(
            "test_git",
        );
        let res = call_git(git_path, "status");
        assert_eq!(
            res.unwrap(),
            "On branch master\nnothing to commit, working tree clean\n".to_owned()
        );
    }

    #[test]
    fn test_trans() {
        let data=r#"{"date":"2017-10-21T15:00:33+08:00","author":"cong wu","msg":"rename test.md","hash":"0d0c8aa7ecbf087574f0232127e486eafa5f91aa"}
        {"date":"2017-10-21T14:58:35+08:00","author":"cong wu","msg":"add test.md","hash":"9400e002ce0da2b0d36e9a88b0c4b358caec7e55"}"#;
        let res = Git::trans(data.to_owned());
        assert_eq!(format!("{:?}",res),r#"Ok([Commit { author: "cong wu", date: 2017-10-21T15:00:33+08:00, msg: "rename test.md", hash: "0d0c8aa7ecbf087574f0232127e486eafa5f91aa" }, Commit { author: "cong wu", date: 2017-10-21T14:58:35+08:00, msg: "add test.md", hash: "9400e002ce0da2b0d36e9a88b0c4b358caec7e55" }])"#)
    }

    #[test]
    fn test_direct_history() {
        let git_path = env::current_dir().unwrap().join("test").join("data").join(
            "test_git",
        );
        let res = Git::history(git_path.clone(), PathBuf::from("./test1.md"));
        let res = format!("{:?}", res);
        assert_eq!(res,r#"Ok([Commit { author: "cong wu", date: 2017-10-21T15:00:33+08:00, msg: "rename test.md", hash: "0d0c8aa7ecbf087574f0232127e486eafa5f91aa" }, Commit { author: "cong wu", date: 2017-10-21T14:58:35+08:00, msg: "add test.md", hash: "9400e002ce0da2b0d36e9a88b0c4b358caec7e55" }])"#)
    }

    #[test]
    fn test_bulk_history() {
        let git_path = env::current_dir().unwrap().join("test").join("data").join(
            "test_git",
        );
        let res = Git::history_bulk(git_path.clone(), vec![PathBuf::from("./test1.md")]);
        let res = format!("{:?}", res);
        assert_eq!(res,r#"Ok({"./test1.md": [Commit { author: "cong wu", date: 2017-10-21T15:00:33+08:00, msg: "rename test.md", hash: "0d0c8aa7ecbf087574f0232127e486eafa5f91aa" }, Commit { author: "cong wu", date: 2017-10-21T14:58:35+08:00, msg: "add test.md", hash: "9400e002ce0da2b0d36e9a88b0c4b358caec7e55" }]})"#)
    }
}
