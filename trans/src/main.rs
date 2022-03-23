use std::{
    fs::{read_to_string, remove_dir_all, remove_file, write},
    vec,
};

use dircpy::copy_dir;
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt, Debug)]
#[structopt(name = "tras")]
struct Opt {
    /// source path
    #[structopt(short, long)]
    source_path: String,

    /// dest path
    #[structopt(short, long)]
    dest_path: String,
}

fn main() {
    let opt = Opt::from_args();
    let source_path = opt.source_path;
    let dest_path = opt.dest_path;

    // let source_path = "/Volumes/code/github/rust-web-server-example";
    // let dest_path = "/Volumes/code/temp/rust-web-server-mustache";

    copy_dir(source_path, &dest_path).expect("copy dir error");
    remove_dir_all(format!("{}/target", &dest_path)).expect("remove target/ error");
    remove_file(format!("{}/Cargo.lock", &dest_path)).expect("remove Cargo.lock error");
    remove_dir_all(format!("{}/.git", &dest_path)).expect("remove .git/ error");
    remove_file(format!("{}/.DS_Store", &dest_path)).expect("remove .DS_Store error");

    let root_cargo_toml_path = format!("{}/Cargo.toml", dest_path.clone());
    let root_readme_md_path = format!("{}/README.md", dest_path.clone());

    let walk_dir = WalkDir::new(&dest_path);
    for entry in walk_dir {
        let entry = entry.unwrap();
        let path = entry.path().to_str();
        match path {
            Some(path) => {
                if entry.path().is_dir() || path.contains("/crates/")
                //keep
                {
                    continue;
                }
            }
            None => break,
        }
        // println!("{:?} {:?}", path, entry.path().file_name());
        let path = path.unwrap();
        // println!("{path}");
        let replace_contents = if path.ends_with(".rs") {
            rs_replace_content()
        } else if path.ends_with("Cargo.toml") {
            if path == root_cargo_toml_path {
                root_cargo_toml_replace_content()
            } else {
                lib_cargo_toml_replace_content()
            }
        } else if path.ends_with("README.md") {
            if path == root_readme_md_path {
                root_readme_md_replace_content()
            } else {
                vec![]
            }
        } else {
            vec![("rust-web-server-example", "{{project_name}}")]
        };
        let file_string = read_to_string(path);
        let mut file_string = file_string.unwrap();

        for (from, to) in replace_contents {
            file_string = file_string.replace(from, to);
        }
        write(format!("{}.mustache", path), file_string).expect("write file error");
        remove_file(path).expect("remove file error");
    }

    println!("trans completed!");
}

fn root_readme_md_replace_content() -> Vec<(&'static str, &'static str)> {
    vec![
        ("- api:", " - {{#api}}{{package_name}}{{/api}}:"),
        ("- domain:", "- {{#domain}}{{package_name}}{{/domain}}:"),
        ("- extra:", "- {{#extra}}{{package_name}}{{/extra}}:"),
        (
            "- repository:",
            "- {{#repository}}{{package_name}}{{/repository}}:",
        ),
        ("- search:", "- {{#search}}{{package_name}}{{/search}}:"),
        (
            "- server_app:",
            "- {{#server_app}}{{package_name}}{{/server_app}}:",
        ),
        (
            "- server_lib:",
            "- {{#server_lib}}{{package_name}}{{/server_lib}}:",
        ),
        ("- util:", "- {{#util}}{{package_name}}{{/util}}:"),
        ("- vars:", "- {{#vars}}{{package_name}}{{/vars}}:"),
    ]
}

fn root_cargo_toml_replace_content() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "\"repository\"",
            "\"{{#repository}}{{member_name}}{{/repository}}\"",
        ),
        ("\"domain\"", "\"{{#domain}}{{member_name}}{{/domain}}\""),
        ("\"vars\"", "\"{{#vars}}{{member_name}}{{/vars}}\""),
        ("\"search\"", "\"{{#search}}{{member_name}}{{/search}}\""),
        ("\"api\"", "\"{{#api}}{{member_name}}{{/api}}\""),
        (
            "\"server_lib\"",
            "\"{{#server_lib}}{{member_name}}{{/server_lib}}\"",
        ),
        (
            "\"server_app\"",
            "\"{{#server_app}}{{member_name}}{{/server_app}}\"",
        ),
        ("\"extra\"", "\"{{#extra}}{{member_name}}{{/extra}}\""),
        ("\"util\"", "\"{{#util}}{{member_name}}{{/util}}\""),
    ]
}

fn lib_cargo_toml_replace_content() -> Vec<(&'static str, &'static str)> {
    vec![
        // domain
        (
            "name = \"domain\"",
            "name = \"{{#domain}}{{package_name}}{{/domain}}\"",
        ),
        ("domain =", "{{#domain}}{{package_name}}{{/domain}} ="),
        ("../domain", "../{{#domain}}{{member_name}}{{/domain}}"),
        // extra
        (
            "name = \"extra\"",
            "name = \"{{#extra}}{{package_name}}{{/extra}}\"",
        ),
        ("extra =", "{{#extra}}{{package_name}}{{/extra}} ="),
        ("../extra", "../{{#extra}}{{member_name}}{{/extra}}"),
        // util
        (
            "name = \"util\"",
            "name = \"{{#util}}{{package_name}}{{/util}}\"",
        ),
        ("util =", "{{#util}}{{package_name}}{{/util}} ="),
        ("../util", "../{{#util}}{{member_name}}{{/util}}"),
        // repository
        (
            "name = \"repository\"",
            "name = \"{{#repository}}{{package_name}}{{/repository}}\"",
        ),
        (
            "repository = {",
            "{{#repository}}{{package_name}}{{/repository}} = {",
        ),
        (
            "../repository",
            "../{{#repository}}{{member_name}}{{/repository}}",
        ),
        (
            "[\"repository\"]",
            "[\"{{#repository}}{{package_name}}{{/repository}}\"]",
        ),
        // search
        (
            "name = \"search\"",
            "name = \"{{#search}}{{package_name}}{{/search}}\"",
        ),
        ("search = {", "{{#search}}{{package_name}}{{/search}} = {"),
        ("../search", "../{{#search}}{{member_name}}{{/search}}"),
        // server_lib
        (
            "name = \"server_lib\"",
            "name = \"{{#server_lib}}{{package_name}}{{/server_lib}}\"",
        ),
        (
            "server_lib =",
            "{{#server_lib}}{{package_name}}{{/server_lib}} =",
        ),
        (
            "../server_lib",
            "../{{#server_lib}}{{member_name}}{{/server_lib}}",
        ),
        // vars
        (
            "name = \"vars\"",
            "name = \"{{#vars}}{{package_name}}{{/vars}}\"",
        ),
        ("vars =", "{{#vars}}{{package_name}}{{/vars}} ="),
        ("../vars", "../{{#vars}}{{member_name}}{{/vars}}"),
        // api
        (
            "name = \"api\"",
            "name = \"{{#api}}{{package_name}}{{/api}}\"",
        ),
        ("api =", "{{#api}}{{package_name}}{{/api}} ="),
        ("../api", "../{{#api}}{{member_name}}{{/api}}"),
        ("[\"api\"]", "[\"{{#api}}{{package_name}}{{/api}}\"]"),
        // server_app
        (
            "name = \"server_app\"",
            "name = \"{{#server_app}}{{package_name}}{{/server_app}}\"",
        ),
        ("name = \"app\"", "name = \"{{project_name}}\""),
    ]
}

fn rs_replace_content() -> Vec<(&'static str, &'static str)> {
    vec![
        // domain
        (
            "use domain::",
            "use {{#domain}}{{package_name}}{{/domain}}::",
        ),
        (", domain::", ", {{#domain}}{{package_name}}{{/domain}}::"),
        (": domain::", ": {{#domain}}{{package_name}}{{/domain}}::"),
        ("<domain::", "<{{#domain}}{{package_name}}{{/domain}}::"),
        ("(domain::", "({{#domain}}{{package_name}}{{/domain}}::"),
        // extra
        (" extra::", " {{#extra}}{{package_name}}{{/extra}}::"),
        // util
        ("use util::", "use {{#util}}{{package_name}}{{/util}}::"),
        // repository
        (
            "use repository::",
            "use {{#repository}}{{package_name}}{{/repository}}::",
        ),
        (
            "repository::db::Repo",
            "{{#repository}}{{package_name}}{{/repository}}::db::Repo",
        ),
        // search
        (
            "use search::",
            "use {{#search}}{{package_name}}{{/search}}::",
        ),
        // server_lib
        (
            " server_lib::",
            " {{#server_lib}}{{package_name}}{{/server_lib}}::",
        ),
        // vars
        ("use vars::", "use {{#vars}}{{package_name}}{{/vars}}::"),
        ("\"vars::", "\"{{#vars}}{{package_name}}{{/vars}}::"),
        // api
        ("api::start()", "{{#api}}{{package_name}}{{/api}}::start()"),
    ]
}
