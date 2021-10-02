mod path;

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

    debug_opt.debug(format!("opt: {:#?}", opt));

    // exit(1);
    println!("Gen...");

    let mustache_config_path = opt.mustache_config_path;
    debug_opt.debug(format!("config path: {}", mustache_config_path));

    let config_str = read_to_string(mustache_config_path).unwrap();
    debug_opt.debug(format!("{}", config_str));

    let config: Config = toml::from_str(&config_str).unwrap();
    debug_opt.debug(format!("{:?}", config));

    let my_path = Path::from(config.clone());
    debug_opt.debug(format!("{:?}", my_path));

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

            tpls.get(&mustache_file).unwrap().render_to_file(
                my_path
                    .rename_dir(&example_path)
                    .replace(my_path.mustache_file_suffix().as_str(), ""),
                &config,
            );
        } else if entry.path().is_dir() {
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

    println!("Gen success!");
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Config {
    project_name: String,
    features_name: String,
    mustache_path: String,
    example_path: String,
    cargo_toml: CargoToml,
    env: ENV,
    types: Types,
    server: Server,
    repository: Repository,
    domain: Domain,
    api: Api,
    redis: Redis,
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
    log_path: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Repository {
    package_name: String,
    member_name: String,
    time_zone: String,
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
struct Api {
    package_name: String,
    member_name: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Redis {
    package_name: String,
    member_name: String,
    redis_url: String,
}
struct DebugOpt(bool);
impl DebugOpt {
    fn debug(&self, s: String) {
        if self.0 {
            println!("{}", &s);
        }
    }
}
