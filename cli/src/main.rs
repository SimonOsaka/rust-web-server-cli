mod path;

use path::Path;
use ramhorns::Content;
use ramhorns::Ramhorns;
use serde::Deserialize;
use std::fs::read_to_string;
use std::fs::DirBuilder;
use walkdir::WalkDir;
fn main() {
    println!("gen start");

    let config_str = read_to_string("mustache.config.toml").unwrap();
    println!("{}", config_str);

    let config: Config = toml::from_str(&config_str).unwrap();
    println!("{:?}", config);

    let my_path = Path::from(config.clone());
    println!("{:?}", my_path);

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
            println!("mustache_file: {}", mustache_file);

            tpls.get(&mustache_file).unwrap().render_to_file(
                my_path
                    .rename_dir(&example_path)
                    .replace(my_path.mustache_file_suffix().as_str(), ""),
                &config,
            );
        } else if entry.path().is_dir() {
            //dir
            println!("example_path:  {:?}", example_path);

            let rename_dir = my_path.rename_dir(&example_path);
            println!("rename_dir: {}", rename_dir);

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
    example_path:String,
    cargo_toml: CargoToml,
    env: ENV,
    types: Types,
    server: Server,
    repository: Repository,
    domain: Domain,
    api: Api,
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
