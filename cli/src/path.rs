#[derive(Debug)]
pub(crate) struct Path {
    pub(crate) mustache_path: String,
    pub(crate) mustache_file_extension: String,
    pub(crate) example_path: String,
    pub(crate) crates_name: String,
    pub(crate) file_axum_api_name: FileName,
    pub(crate) file_domain_name: FileName,
    pub(crate) file_repository_name: FileName,
    pub(crate) file_types_name: FileName,
    pub(crate) file_redis_name: FileName,
    pub(crate) file_search_name: FileName,
    pub(crate) file_server_name: FileName,
    pub(crate) file_auth_name: FileName,
    pub(crate) file_logger_name: FileName,
    pub(crate) file_extra_name: FileName,
    pub(crate) file_i18n_name: FileName,
    pub(crate) file_util_name: FileName,
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
            crates_name: "crates".to_string(),
            file_axum_api_name: FileName {
                old_name: "axum-api".to_string(),
                new_name: "axum-api".to_string(),
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
            file_search_name: FileName {
                old_name: "search".to_string(),
                new_name: "search".to_string(),
            },
            file_server_name: FileName {
                old_name: "server".to_string(),
                new_name: "server".to_string(),
            },
            file_auth_name: FileName {
                old_name: "auth".to_string(),
                new_name: "auth".to_string(),
            },
            file_logger_name: FileName {
                old_name: "logger".to_string(),
                new_name: "logger".to_string(),
            },
            file_extra_name: FileName {
                old_name: "extra".to_string(),
                new_name: "extra".to_string(),
            },
            file_i18n_name: FileName {
                old_name: "i18n".to_string(),
                new_name: "i18n".to_string(),
            },
            file_util_name: FileName {
                old_name: "util".to_string(),
                new_name: "util".to_string(),
            },
        }
    }
    pub(crate) fn rename_dir(&self, path: &str) -> String {
        //axum-api
        let example_axum_api_path_old = format!(
            "{}/{}",
            &self.example_path, &self.file_axum_api_name.old_name
        );
        let example_axum_api_path_new = format!(
            "{}/{}",
            &self.example_path, &self.file_axum_api_name.new_name
        );
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
        //search
        let example_search_path_old =
            format!("{}/{}", &self.example_path, &self.file_search_name.old_name);
        let example_search_path_new =
            format!("{}/{}", &self.example_path, &self.file_search_name.new_name);
        //server
        let example_server_path_old =
            format!("{}/{}", &self.example_path, &self.file_server_name.old_name);
        let example_server_path_new =
            format!("{}/{}", &self.example_path, &self.file_server_name.new_name);
        //auth
        let example_auth_path_old =
            format!("{}/{}", &self.example_path, &self.file_auth_name.old_name);
        let example_auth_path_new =
            format!("{}/{}", &self.example_path, &self.file_auth_name.new_name);
        //logger
        let example_logger_path_old =
            format!("{}/{}", &self.example_path, &self.file_logger_name.old_name);
        let example_logger_path_new =
            format!("{}/{}", &self.example_path, &self.file_logger_name.new_name);
        //extra
        let example_extra_path_old =
            format!("{}/{}", &self.example_path, &self.file_extra_name.old_name);
        let example_extra_path_new =
            format!("{}/{}", &self.example_path, &self.file_extra_name.new_name);
        //i18n
        let example_i18n_path_old =
            format!("{}/{}", &self.example_path, &self.file_i18n_name.old_name);
        let example_i18n_path_new =
            format!("{}/{}", &self.example_path, &self.file_i18n_name.new_name);
        //util
        let example_util_path_old =
            format!("{}/{}", &self.example_path, &self.file_util_name.old_name);
        let example_util_path_new =
            format!("{}/{}", &self.example_path, &self.file_util_name.new_name);

        let rename_dir;
        if path.starts_with(&example_axum_api_path_old) {
            rename_dir = path
                .replace(&example_axum_api_path_old, &example_axum_api_path_new)
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
        } else if path.starts_with(&example_search_path_old) {
            rename_dir = path
                .replace(&example_search_path_old, &example_search_path_new)
                .to_string();
        } else if path.starts_with(&example_server_path_old) {
            rename_dir = path
                .replace(&example_server_path_old, &example_server_path_new)
                .to_string();
        } else if path.starts_with(&example_auth_path_old) {
            rename_dir = path
                .replace(&example_auth_path_old, &example_auth_path_new)
                .to_string();
        } else if path.starts_with(&example_logger_path_old) {
            rename_dir = path
                .replace(&example_logger_path_old, &example_logger_path_new)
                .to_string();
        } else if path.starts_with(&example_extra_path_old) {
            rename_dir = path
                .replace(&example_extra_path_old, &example_extra_path_new)
                .to_string();
        } else if path.starts_with(&example_i18n_path_old) {
            rename_dir = path
                .replace(&example_i18n_path_old, &example_i18n_path_new)
                .to_string();
        } else if path.starts_with(&example_util_path_old) {
            rename_dir = path
                .replace(&example_util_path_old, &example_util_path_new)
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

        if config.axum_api.member_name.trim().len() > 0 {
            p.file_axum_api_name.new_name = config.axum_api.member_name;
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

        if config.search.member_name.trim().len() > 0 {
            p.file_search_name.new_name = config.search.member_name;
        }

        if config.server.member_name.trim().len() > 0 {
            p.file_server_name.new_name = config.server.member_name;
        }

        if config.auth.member_name.trim().len() > 0 {
            p.file_auth_name.new_name = config.auth.member_name;
        }

        if config.logger.member_name.trim().len() > 0 {
            p.file_logger_name.new_name = config.logger.member_name;
        }

        if config.extra.member_name.trim().len() > 0 {
            p.file_extra_name.new_name = config.extra.member_name;
        }

        if config.i18n.member_name.trim().len() > 0 {
            p.file_i18n_name.new_name = config.i18n.member_name;
        }

        if config.util.member_name.trim().len() > 0 {
            p.file_util_name.new_name = config.util.member_name;
        }

        p
    }
}
