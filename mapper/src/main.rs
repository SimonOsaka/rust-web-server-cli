use std::fs::{read_to_string, DirBuilder};

use case::CaseExt;
use ramhorns::{Content, Ramhorns};
use serde::Deserialize;
use sqlx::{
    pool::PoolOptions, postgres::PgArguments, Arguments, Error, Executor, PgConnection, PgPool,
};
use structopt::StructOpt;
use walkdir::WalkDir;

#[derive(StructOpt, Debug)]
#[structopt(name = "rust-web-server-cli")]
struct Opt {
    /// rust-web-server-mustache's path
    #[structopt(short, long)]
    mustache_config_path: String,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    let mustache_config_path = opt.mustache_config_path;
    // load config
    let config_str = read_to_string(mustache_config_path).unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();
    println!("{:?}", &config);
    // database pool
    let database_url = config.database_url;
    let pool = connect(&database_url).await;
    // template
    for mapper_table in config.mapper_tables {
        let table_name = mapper_table.table_name;
        let table_alias = mapper_table.table_name_alias;
        let table_pks = mapper_table.pk;
        let relations = mapper_table.relations;
        let tpls =
            Ramhorns::from_folder_with_extension(&config.templates_path, "mustache").unwrap();

        let infos = load_info(&table_name, &pool)
            .await
            .expect("load info failed");
        let (mut table_model, mut repository_model) = get(infos, table_pks).await;
        table_model.struct_name = Some(to_struct_name(table_name.clone()));
        table_model.table_name = Some(table_name.clone());
        table_model.table_alias = Some(table_alias);
        table_model.get_relations(relations);
        repository_model.table_name_uppercase = Some(table_name.to_uppercase());
        println!("{:?}", table_model);

        let real_path = format!("{}/{}", &config.examples_path, table_name);
        DirBuilder::new()
            .recursive(true)
            .create(&real_path)
            .unwrap();

        let walk_dir = WalkDir::new(&config.templates_path);
        for entry in walk_dir {
            let entry = entry.unwrap();
            if entry.file_type().is_dir() {
                continue;
            }
            let mustache_file_path = entry.file_name().to_str().unwrap();

            println!("{:?}", mustache_file_path);

            let mustache_file_name = mustache_file_path.replace(".rs.mustache", "");

            let tpl = tpls.get(mustache_file_path).unwrap();
            tpl.render_to_file(
                format!("{}/{}.rs", &real_path, mustache_file_name),
                &Root {
                    model: table_model.clone(),
                    repository: repository_model.clone(),
                },
            )
            .expect("render to file failed");
        }
    }
}

async fn connect(url: &str) -> PgPool {
    let connection_pool = PoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .connect_timeout(std::time::Duration::from_secs(30))
        .after_connect(|conn: &mut PgConnection| {
            Box::pin(async move {
                conn.execute("SET TIME ZONE 'Asia/Shanghai';").await?;

                Ok(())
            })
        })
        .connect(url)
        .await
        .expect("init database error");

    connection_pool
}

#[derive(Deserialize, Content, Clone, Debug)]
struct Config {
    mapper_tables: Vec<MapperTable>,
    database_url: String,
    templates_path: String,
    examples_path: String,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct MapperTable {
    table_name: String,
    table_name_alias: String,
    pk: Vec<String>,
    relations: Option<Vec<RelationTable>>,
}

#[derive(Deserialize, Content, Clone, Debug)]
struct RelationTable {
    rel_table_name: String,
    rel_table_name_alias: String,
    rel_table_pk: String,
    table_fk: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct InfoSchema {
    table_catalog: String,  // my_example
    table_schema: String,   // public
    table_name: String,     // my_users
    column_name: String,    // id
    ordinal_position: i32,  // 1
    column_default: String, // 12
    data_type: String,      // bigint
    udt_name: String,       // int8
}

async fn load_info(table_name: &str, pool: &PgPool) -> Result<Vec<InfoSchema>, Error> {
    let mut args = PgArguments::default();
    args.add(table_name);
    let infos = sqlx::query_as_with(
        r#"
    SELECT
        table_catalog,
        table_schema,
        "table_name",
        "column_name",
        ordinal_position,
        column_default,
        data_type,
        udt_name
    FROM
        information_schema.COLUMNS
    WHERE
        table_name = $1
    ORDER BY 
        ordinal_position
    "#,
        args,
    )
    .fetch_all(pool)
    .await?;

    Ok(infos)
}

#[derive(Content, Debug)]
struct Root {
    model: StructAndTableModel,
    repository: RepositoryModel,
}

#[derive(Content, Debug, Clone)]
struct StructAndTableModel {
    struct_name: Option<String>,
    table_name: Option<String>,
    table_alias: Option<String>,
    columns: Vec<ColumnAndPropertyModel>,
    columns_for_insert: Vec<ColumnAndPropertyModel>,
    relations: Option<Vec<RelationModel>>,
}

impl StructAndTableModel {
    fn get_relations(&mut self, relations: Option<Vec<RelationTable>>) {
        match relations {
            Some(rel) => {
                let mut rels: Vec<RelationModel> = Vec::new();
                for rt in rel {
                    rels.push(RelationModel {
                        table_name: rt.rel_table_name.clone(),
                        table_name_uppercase: rt.rel_table_name.clone().to_uppercase(),
                        struct_name: to_struct_name(rt.rel_table_name),
                        table_name_alias: rt.rel_table_name_alias,
                        pk: rt.rel_table_pk,
                        fk: rt.table_fk,
                    })
                }
                self.relations = Some(rels);
            }
            None => self.relations = None,
        }
    }
}

#[derive(Content, Debug, Clone)]
struct ColumnAndPropertyModel {
    column: ColumnModel,
    property: PropertyModel,
}

#[derive(Content, Debug, Clone)]
struct ColumnModel {
    column_name: String,
    column_type: String,
}

#[derive(Content, Debug, Clone)]
struct PropertyModel {
    type_name: String,
    import: Option<String>,
}

#[derive(Content, Debug, Clone)]
struct RepositoryModel {
    table_name_uppercase: Option<String>,
    table_fields_length: u8,
    table_insert_fields_length: u8,
}

#[derive(Content, Debug, Clone)]
struct RelationModel {
    table_name: String,
    table_name_uppercase: String,
    struct_name: String,
    table_name_alias: String,
    pk: String,
    fk: String,
}

async fn get(infos: Vec<InfoSchema>, pks: Vec<String>) -> (StructAndTableModel, RepositoryModel) {
    let mut columns: Vec<ColumnAndPropertyModel> = Vec::new();
    let mut columns_for_insert: Vec<ColumnAndPropertyModel> = Vec::new();
    for is in &infos {
        // match pk
        let column_type = match pks.contains(&is.column_name) {
            true => "pk".to_string(),
            false => is.data_type.clone(),
        };

        let property_type = to_property_type(column_type.clone()).expect("to_property_type failed");

        columns.push(ColumnAndPropertyModel {
            column: ColumnModel {
                column_name: is.column_name.clone(),
                column_type: is.data_type.clone(),
            },
            property: property_type.clone(),
        });

        if column_type != "pk"
            && is.column_name.clone() != "is_deleted"
            && is.column_name.clone() != "created_at"
        {
            columns_for_insert.push(ColumnAndPropertyModel {
                column: ColumnModel {
                    column_name: is.column_name.clone(),
                    column_type: is.data_type.clone(),
                },
                property: property_type,
            });
        }
    }

    (
        StructAndTableModel {
            struct_name: None,
            table_name: None,
            columns: columns.clone(),
            columns_for_insert: columns_for_insert.clone(),
            table_alias: None,
            relations: None,
        },
        RepositoryModel {
            table_name_uppercase: None,
            table_fields_length: columns.len() as u8,
            table_insert_fields_length: columns_for_insert.len() as u8,
        },
    )
}

fn to_property_type(column_type: String) -> Result<PropertyModel, String> {
    let result = match column_type.as_str() {
        "pk" => PropertyModel {
            type_name: "ID".to_string(),
            import: Some("use types::ID;".to_string()),
        },
        "bigint" => PropertyModel {
            type_name: "i64".to_string(),
            import: None,
        },
        "integer" => PropertyModel {
            type_name: "i32".to_string(),
            import: None,
        },
        "smallint" => PropertyModel {
            type_name: "i16".to_string(),
            import: None,
        },
        "character varying" => PropertyModel {
            type_name: "String".to_string(),
            import: None,
        },
        "ARRAY" => PropertyModel {
            type_name: "Vec<String>".to_string(),
            import: None,
        },
        "timestamp without time zone" => PropertyModel {
            type_name: "DateTime".to_string(),
            import: Some("use types::DateTime;".to_string()),
        },
        _ => return Err("column type not found".to_string()),
    };

    Ok(result)
}

fn to_struct_name(name: String) -> String {
    let col = name.split("_");
    let capital: Vec<String> = col.map(|name| name.to_capitalized()).collect();
    capital.join("")
}
