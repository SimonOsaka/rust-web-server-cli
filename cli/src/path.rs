#[derive(Debug)]
pub(crate) struct Path {
    pub(crate) mustache_path: String,
    pub(crate) mustache_file_extension: String,
    pub(crate) example_path: String,
    pub(crate) file_api_name: FileName,
    pub(crate) file_domain_name: FileName,
    pub(crate) file_repository_name: FileName,
    pub(crate) file_types_name: FileName,
    pub(crate) file_redis_name: FileName,
}
#[derive(Debug)]
pub(crate) struct FileName {
    old_name: String,
    pub(crate) new_name: String,
}

impl Path {
    pub(crate) fn new() -> Self {
        Path {
            mustache_path: "./mustache".to_string(),
            mustache_file_extension: "mustache".to_string(),
            example_path: "./example".to_string(),
            file_api_name: FileName {
                old_name: "api".to_string(),
                new_name: "api".to_string(),
            },
            file_domain_name: FileName {
                old_name: "domain".to_string(),
                new_name: "domain".to_string(),
            },
            file_repository_name: FileName {
                old_name: "repository".to_string(),
                new_name: "repository".to_string(),
            },
            file_types_name: FileName {
                old_name: "types".to_string(),
                new_name: "types".to_string(),
            },
            file_redis_name: FileName {
                old_name: "redis".to_string(),
                new_name: "redis".to_string(),
            },
        }
    }
    pub(crate) fn rename_dir(&self, path: &str) -> String {
        //api
        let example_api_path_old =
            format!("{}/{}", &self.example_path, &self.file_api_name.old_name);
        let example_api_path_new =
            format!("{}/{}", &self.example_path, &self.file_api_name.new_name);
        //domain
        let example_domain_path_old =
            format!("{}/{}", &self.example_path, &self.file_domain_name.old_name);
        let example_domain_path_new =
            format!("{}/{}", &self.example_path, &self.file_domain_name.new_name);
        //repository
        let example_repository_path_old = format!(
            "{}/{}",
            &self.example_path, &self.file_repository_name.old_name
        );
        let example_repository_path_new = format!(
            "{}/{}",
            &self.example_path, &self.file_repository_name.new_name
        );
        //types
        let example_types_path_old =
            format!("{}/{}", &self.example_path, &self.file_types_name.old_name);
        let example_types_path_new =
            format!("{}/{}", &self.example_path, &self.file_types_name.new_name);
        //redis
        let example_redis_path_old =
            format!("{}/{}", &self.example_path, &self.file_redis_name.old_name);
        let example_redis_path_new =
            format!("{}/{}", &self.example_path, &self.file_redis_name.new_name);
        let rename_dir;
        if path.starts_with(&example_api_path_old) {
            rename_dir = path
                .replace(&example_api_path_old, &example_api_path_new)
                .to_string();
        } else if path.starts_with(&example_domain_path_old) {
            rename_dir = path
                .replace(&example_domain_path_old, &example_domain_path_new)
                .to_string();
        } else if path.starts_with(&example_repository_path_old) {
            rename_dir = path
                .replace(&example_repository_path_old, &example_repository_path_new)
                .to_string();
        } else if path.starts_with(&example_types_path_old) {
            rename_dir = path
                .replace(&example_types_path_old, &example_types_path_new)
                .to_string();
        } else if path.starts_with(&example_redis_path_old) {
            rename_dir = path
                .replace(&example_redis_path_old, &example_redis_path_new)
                .to_string();
        } else {
            rename_dir = path.to_string();
        }
        rename_dir
    }

    pub(crate) fn mustache_file_suffix(&self) -> String {
        format!(".{}", &self.mustache_file_extension)
    }
}

impl From<crate::Config> for Path {
    fn from(config: crate::Config) -> Self {
        let mut p = Self::new();
        if config.mustache_path.trim().len() > 0 {
            p.mustache_path = config.mustache_path;
        }
        if config.example_path.trim().len() > 0 {
            p.example_path = config.example_path;
        }

        if config.api.member_name.trim().len() > 0 {
            p.file_api_name.new_name = config.api.member_name;
        }

        if config.domain.member_name.trim().len() > 0 {
            p.file_domain_name.new_name = config.domain.member_name;
        }

        if config.repository.member_name.trim().len() > 0 {
            p.file_repository_name.new_name = config.repository.member_name;
        }

        if config.types.member_name.trim().len() > 0 {
            p.file_types_name.new_name = config.types.member_name;
        }

        if config.redis.member_name.trim().len() > 0 {
            p.file_redis_name.new_name = config.redis.member_name;
        }
        p
    }
}
