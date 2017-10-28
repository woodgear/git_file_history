use std::path::{Path, PathBuf};
use chrono::prelude::*;
use error::*;
use history::*;

#[derive(Debug)]
pub struct FileTimeRecord {
    pub file:PathBuf,
    pub create_time:DateTime<Local>,
    pub last_modify_time:DateTime<Local>
}


trait GitFileTimeRecord {
    fn get<P: AsRef<Path>>(git_path: P, file_path: P) -> Result<FileTimeRecord>;
}

impl GitFileTimeRecord for FileTimeRecord {
    fn get<P: AsRef<Path>>(git_path: P, file_path: P) -> Result<FileTimeRecord>{
        let git_path: PathBuf = PathBuf::from(git_path.as_ref());
        let file_path: PathBuf = PathBuf::from(file_path.as_ref());
        let history=Git::history(&git_path,&file_path)?;
        let create_time=history.first().ok_or("could not get first")?.date;
        let last_modify_time=history.last().ok_or("could not get last")?.date;
        Ok(FileTimeRecord{file:file_path,create_time:create_time,last_modify_time:last_modify_time})
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get(){
        use std::env;
        let git_path = env::current_dir().unwrap().join("test").join("data").join(
            "test_git",
        );
        let res = FileTimeRecord::get(git_path.clone(), PathBuf::from("./test1.md"));
        assert_eq!(format!("{:?}",res.unwrap()),"FileTimeRecord { file: \"./test1.md\", create_time: 2017-10-21T15:00:33+08:00, last_modify_time: 2017-10-21T14:58:35+08:00 }".to_string());
    }
}