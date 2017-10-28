// #[macro_use]
// extern crate error_chain;

// extern crate git_file_history;
// use git_file_history::history::Git;

// mod error;
// use error::*;

// use std::collections::HashMap;

// fn git_file_history(
//     git_path: &str,
//     files: Vec<String>,
// ) -> Result<HashMap<String, (String, String)>> {

//     let git = Git::new(git_path);
//     let file_commits = git.bluk_history(files).chain_err(|| "bluk_history fail")?;
//     let mut file_record_map = HashMap::new();
//     for (file, commits) in file_commits {
//         if (commits.is_empty()) {
//             println!("error");
//         }
//         let last_modify_time = commits.first().unwrap().date.clone();
//         let create_time = commits.last().unwrap().date.clone();
//         file_record_map.insert(String::from(file), (create_time.format("%Y-%m-%d %H:%M:%S").to_string(), last_modify_time.format("%Y-%m-%d %H:%M:%S").to_string()));
//     }
//     Ok(file_record_map)
// }

// fn main() {
//     // use std::env;
//     // let mut git_path = env::current_dir().unwrap().join("test").join("data").join(
//     //     "test_git",
//     // );
//     // println!("{:?}", git_path);
//     // let res = git_file_history(git_path.to_str().unwrap(), vec!["test1.md".to_owned()]);
//     // println!("res {:?}", res);
// }
