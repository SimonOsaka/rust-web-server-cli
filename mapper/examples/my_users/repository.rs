use super::models::MyUsers;
use crate::db::{SqlParams, SqlReader, SqlWriter};
use sql_builder::SqlBuilder;
use sqlx::{Error, Postgres, Transaction};
use types::ID;

const MY_USERS_FIELDS: &[&str; 6] = &[

    "id",

    "username",

    "password",

    "roles",

    "is_deleted",

    "created_at",

];

const MY_USERS_INSERT_FIELDS: &[&str; 3] = &[

    "username",

    "password",

    "roles",

];

const MY_USERS_STRUCT_FIELDS: &[&str; 6] = &[

    "(u.id",
    "u.username",
    "u.password",
    "u.roles",
    "u.is_deleted",
    "u.created_at",
    ") AS \"my_users\""

];

pub async fn insert_my_users<'a>(
    u: MyUsers,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<ID, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::insert_into("my_users");
    sql_builder
        .fields(MY_USERS_INSERT_FIELDS)
        .values(&[
            
    param.add_value(u.username),
    
    param.add_value(u.password),
    
    param.add_value(u.roles),
    
        ])
        .returning_id();

    let id = sql_builder.insert_one(param, transaction).await?;
    debug!("insert id: {:?}", id);

    Ok(id)
}

pub async fn update_my_users<'a>(
    id: ID,
    u: MyUsers,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<bool, Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::update_table("my_users");
    sql_builder
    
        .set("username", param.add_value(u.username))
    
        .set("password", param.add_value(u.password))
    
        .set("roles", param.add_value(u.roles))
    
        .and_where_eq("is_deleted", 0)
        .and_where_eq("id", param.add_value(id));

    let affect = sql_builder.update_one(param, transaction).await?;
    Ok(affect > 0)
}

pub async fn delete_my_users<'a>(
    id: ID,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<bool, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::update_table("my_users");
    sql_builder
        .set("is_deleted", 1)
        .and_where_eq("is_deleted", 0)
        .and_where_eq("id", id);

    let affect_rows = sql_builder.delete_one(param, transaction).await?;
    debug!("delete affect_rows: {:?}", affect_rows);

    Ok(affect_rows > 0)
}


pub async fn get_my_users_by_id<'a>(
    id: ID,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));
    sql_builder
        .fields(MY_USERS_FIELDS)
        .and_where_eq("u.is_deleted", 0)
        .and_where_eq("u.id", param.add_value(id));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_users_by_username<'a>(
    username: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));
    sql_builder
        .fields(MY_USERS_FIELDS)
        .and_where_eq("u.is_deleted", 0)
        .and_where_eq("u.username", param.add_value(username));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_users_by_password<'a>(
    password: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));
    sql_builder
        .fields(MY_USERS_FIELDS)
        .and_where_eq("u.is_deleted", 0)
        .and_where_eq("u.password", param.add_value(password));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_users_by_roles<'a>(
    roles: Vec<String>,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));
    sql_builder
        .fields(MY_USERS_FIELDS)
        .and_where_eq("u.is_deleted", 0)
        .and_where_eq("u.roles", param.add_value(roles));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_users_by_is_deleted<'a>(
    is_deleted: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));
    sql_builder
        .fields(MY_USERS_FIELDS)
        .and_where_eq("u.is_deleted", 0)
        .and_where_eq("u.is_deleted", param.add_value(is_deleted));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_users_by_created_at<'a>(
    created_at: DateTime,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyUsers>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));
    sql_builder
        .fields(MY_USERS_FIELDS)
        .and_where_eq("u.is_deleted", 0)
        .and_where_eq("u.created_at", param.add_value(created_at));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}



pub async fn find_my_users_by_id<'a>(
    id: ID,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));

    sql_builder
        .fields(MY_USERS_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("u", "is_deleted"), 0)
        .and_where_eq(name!("u", "id"), params.add_value(id));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_users_by_username<'a>(
    username: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));

    sql_builder
        .fields(MY_USERS_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("u", "is_deleted"), 0)
        .and_where_eq(name!("u", "username"), params.add_value(username));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_users_by_password<'a>(
    password: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));

    sql_builder
        .fields(MY_USERS_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("u", "is_deleted"), 0)
        .and_where_eq(name!("u", "password"), params.add_value(password));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_users_by_roles<'a>(
    roles: Vec<String>,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));

    sql_builder
        .fields(MY_USERS_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("u", "is_deleted"), 0)
        .and_where_eq(name!("u", "roles"), params.add_value(roles));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_users_by_is_deleted<'a>(
    is_deleted: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));

    sql_builder
        .fields(MY_USERS_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("u", "is_deleted"), 0)
        .and_where_eq(name!("u", "is_deleted"), params.add_value(is_deleted));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_users_by_created_at<'a>(
    created_at: DateTime,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_users";"u"));

    sql_builder
        .fields(MY_USERS_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("u", "is_deleted"), 0)
        .and_where_eq(name!("u", "created_at"), params.add_value(created_at));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}
