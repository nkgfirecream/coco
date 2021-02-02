use git2::Repository;

use crate::infrastructure::url_format;
use std::fs;

pub struct GitRepository {}

impl GitRepository {
    pub fn open(url: &str) -> Repository {
        let local_path = url_format::uri_to_path(url);

        println!("target dir: {:?}", local_path.display());
        if local_path.exists() {
            // todo: make update for repo
            let repo = match Repository::open(local_path) {
                Ok(repo) => repo,
                Err(e) => panic!("failed to open: {}", e),
            };

            return repo;
        };

        let _ = fs::create_dir_all(&local_path.parent().unwrap());
        let repo = match Repository::clone(url, local_path) {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
        };

        return repo;
    }
}
