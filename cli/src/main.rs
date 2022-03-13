mod path;

use dircpy::copy_dir;
use path::Path;
use ramhorns::Content;
use ramhorns::Ramhorns;
use serde::Deserialize;
use std::fs::read_to_string;
use std::fs::DirBuilder;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-web-server-cli")]
struct Opt {
    /// more details will be shown
    #[structopt(short, long)]
    debug: bool,

    /// rust-web-server-mustache's path
    #[structopt(short, long)]
    mustache_config_path: String,
}
fn main() {
    let opt = Opt::from_args();
    let debug_opt = DebugOpt(opt.debug);
    // let debug_opt = DebugOpt(true);

    debug_opt.debug(format!("opt: {:#?}", opt));

    // exit(1);
    println!("Gen...");

    let mustache_config_path = opt.mustache_config_path;
    // let mustache_config_path = String::from("/Volumes/code/github/rust-web-server-cli/mustache.config.toml"); //opt.mustache_config_path;
    debug_opt.debug(format!("mustache_config_path: {}", mustache_config_path));

    let config_str = read_to_string(mustache_config_path).unwrap();
    debug_opt.debug(format!("config_str: {}", config_str));

    let config: Config = toml::from_str(&config_str).unwrap();
    debug_opt.debug(format!("config: {:?}", config));

    let my_path = Path::from(config.clone());
    debug_opt.debug(format!("my_path: {:?}", my_path));

    let tpls = Ramhorns::from_folder_with_extension(
        &my_path.mustache_path,
        my_path.mustache_file_extension.as_str(),
    )
    .unwrap();
    // tpls.get("api/Cargo.toml.mustache")
    //     .unwrap()
    //     .render_to_file("./example/api/Cargo.toml", &root);

    let walk_dir = WalkDir::new(&my_path.mustache_path);
    for entry in walk_dir {
        let entry = entry.unwrap();
        let mustache_path = entry.path().to_str();
        // exclude .git
        if mustache_path.unwrap().contains(".git") {
            continue;
        }

        let example_path = mustache_path.unwrap().replace(
            my_path.mustache_path.as_str(),
            my_path.example_path.as_str(),
        );

        // mustache file
        if entry.path().is_file()
            && entry.path().extension().is_some()
            && entry.path().extension().unwrap() == my_path.mustache_file_extension.as_str()
        {
            // remove mustache file root dir
            let mustache_file = mustache_path
                .unwrap()
                .replace(format!("{}/", &my_path.mustache_path).as_str(), "");
            debug_opt.debug(format!("mustache_file: {}", mustache_file));

            tpls.get(&mustache_file)
                .unwrap()
                .render_to_file(
                    my_path
                        .rename_dir(&example_path)
                        .replace(my_path.mustache_file_suffix().as_str(), ""),
                    &config,
                )
                .unwrap();
        } else if entry.path().is_dir() {
            if let Some(c) = entry.path().file_name() {
                if c.to_str().is_some() && c.to_str().unwrap().contains(&my_path.crates_name) {
                    // don't create crates dir
                    continue;
                }
            }
            //dir
            debug_opt.debug(format!("example_path:  {:?}", example_path));

            let rename_dir = my_path.rename_dir(&example_path);
            debug_opt.debug(format!("rename_dir: {}", rename_dir));

            DirBuilder::new()
                .recursive(true)
                .create(rename_dir)
                .unwrap();
        } else {
            // non mustache file
            continue;
        }
    }

    println!("Copy dir...");

    copy_dir(
        format!("{}/{}", my_path.mustache_path, my_path.crates_name),
        format!("{}/{}", &my_path.example_path, my_path.crates_name),
    )
    .expect("copy dir failed");

    println!("Gen success!");
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Config {
    project_name: String,
    mustache_path: String,
    example_path: String,
    cargo_toml: CargoToml,
    env: ENV,
    types: Types,
    server: Server,
    repository: Repository,
    domain: Domain,
    warp_api: WarpApi,
    axum_api: AxumApi,
    redis: Redis,
    search: Search,
    auth: Auth,
    logger: Logger,
    extra: Extra,
    i18n: I18n,
    util: Util,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct CargoToml {
    bin_name: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct ENV {
    database_url: String,
    bind_address: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Types {
    member_name: String,
    package_name: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Server {
    package_name: String,
    member_name: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Repository {
    package_name: String,
    member_name: String,
    postgres_time_zone: String,
    max_conn: i8,
    min_conn: i8,
    time_out: i8,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Domain {
    package_name: String,
    member_name: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct WarpApi {
    package_name: String,
    member_name: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct AxumApi {
    package_name: String,
    member_name: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Redis {
    package_name: String,
    member_name: String,
    redis_url: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Search {
    package_name: String,
    member_name: String,
    search_url: String,
    search_key: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Auth {
    package_name: String,
    member_name: String,
    jwt_secret: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Logger {
    package_name: String,
    member_name: String,
    log_path: String,
    log_file: String,
    log_tz_hour: i8,
    log_tz_minute: i8,
    log_tz_second: i8,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Extra {
    package_name: String,
    member_name: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct I18n {
    package_name: String,
    member_name: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Util {
    package_name: String,
    member_name: String,
}

struct DebugOpt(bool);
impl DebugOpt {
    fn debug(&self, s: String) {
        if self.0 {
            println!("{}", &s);
            println!();
        }
    }
}
