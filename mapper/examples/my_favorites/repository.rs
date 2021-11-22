use super::models::MyFavorites;
use crate::db::{SqlParams, SqlReader, SqlWriter};
use sql_builder::SqlBuilder;
use sqlx::{Error, Postgres, Transaction};
use types::ID;

const MY_FAVORITES_FIELDS: &[&str; 5] = &[

    "id",

    "user_id",

    "adventure_id",

    "is_deleted",

    "created_at",

];

const MY_FAVORITES_INSERT_FIELDS: &[&str; 2] = &[

    "user_id",

    "adventure_id",

];

const MY_FAVORITES_STRUCT_FIELDS: &[&str; 5] = &[

    "(fav.id",
    "fav.user_id",
    "fav.adventure_id",
    "fav.is_deleted",
    "fav.created_at",
    ") AS \"my_favorites\""

];

pub async fn insert_my_favorites<'a>(
    fav: MyFavorites,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<ID, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::insert_into("my_favorites");
    sql_builder
        .fields(MY_FAVORITES_INSERT_FIELDS)
        .values(&[
            
    param.add_value(fav.user_id),
    
    param.add_value(fav.adventure_id),
    
        ])
        .returning_id();

    let id = sql_builder.insert_one(param, transaction).await?;
    debug!("insert id: {:?}", id);

    Ok(id)
}

pub async fn update_my_favorites<'a>(
    id: ID,
    fav: MyFavorites,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<bool, Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::update_table("my_favorites");
    sql_builder
    
        .set("user_id", param.add_value(fav.user_id))
    
        .set("adventure_id", param.add_value(fav.adventure_id))
    
        .and_where_eq("is_deleted", 0)
        .and_where_eq("id", param.add_value(id));

    let affect = sql_builder.update_one(param, transaction).await?;
    Ok(affect > 0)
}

pub async fn delete_my_favorites<'a>(
    id: ID,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<bool, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::update_table("my_favorites");
    sql_builder
        .set("is_deleted", 1)
        .and_where_eq("is_deleted", 0)
        .and_where_eq("id", id);

    let affect_rows = sql_builder.delete_one(param, transaction).await?;
    debug!("delete affect_rows: {:?}", affect_rows);

    Ok(affect_rows > 0)
}


pub async fn get_my_favorites_by_id<'a>(
    id: ID,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyFavorites>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));
    sql_builder
        .fields(MY_FAVORITES_FIELDS)
        .and_where_eq("fav.is_deleted", 0)
        .and_where_eq("fav.id", param.add_value(id));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_favorites_by_user_id<'a>(
    user_id: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyFavorites>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));
    sql_builder
        .fields(MY_FAVORITES_FIELDS)
        .and_where_eq("fav.is_deleted", 0)
        .and_where_eq("fav.user_id", param.add_value(user_id));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_favorites_by_adventure_id<'a>(
    adventure_id: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyFavorites>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));
    sql_builder
        .fields(MY_FAVORITES_FIELDS)
        .and_where_eq("fav.is_deleted", 0)
        .and_where_eq("fav.adventure_id", param.add_value(adventure_id));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_favorites_by_is_deleted<'a>(
    is_deleted: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyFavorites>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));
    sql_builder
        .fields(MY_FAVORITES_FIELDS)
        .and_where_eq("fav.is_deleted", 0)
        .and_where_eq("fav.is_deleted", param.add_value(is_deleted));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_favorites_by_created_at<'a>(
    created_at: DateTime,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyFavorites>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));
    sql_builder
        .fields(MY_FAVORITES_FIELDS)
        .and_where_eq("fav.is_deleted", 0)
        .and_where_eq("fav.created_at", param.add_value(created_at));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}



pub async fn find_my_favorites_by_id<'a>(
    id: ID,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyFavorites)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));

    sql_builder
        .fields(MY_FAVORITES_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("fav", "is_deleted"), 0)
        .and_where_eq(name!("fav", "id"), params.add_value(id));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_favorites.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_favorites_by_user_id<'a>(
    user_id: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyFavorites)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));

    sql_builder
        .fields(MY_FAVORITES_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("fav", "is_deleted"), 0)
        .and_where_eq(name!("fav", "user_id"), params.add_value(user_id));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_favorites.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_favorites_by_adventure_id<'a>(
    adventure_id: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyFavorites)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));

    sql_builder
        .fields(MY_FAVORITES_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("fav", "is_deleted"), 0)
        .and_where_eq(name!("fav", "adventure_id"), params.add_value(adventure_id));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_favorites.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_favorites_by_is_deleted<'a>(
    is_deleted: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyFavorites)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));

    sql_builder
        .fields(MY_FAVORITES_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("fav", "is_deleted"), 0)
        .and_where_eq(name!("fav", "is_deleted"), params.add_value(is_deleted));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_favorites.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_favorites_by_created_at<'a>(
    created_at: DateTime,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyFavorites)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_favorites";"fav"));

    sql_builder
        .fields(MY_FAVORITES_STRUCT_FIELDS)
        
        .left()
        
        .and_where_eq(name!("fav", "is_deleted"), 0)
        .and_where_eq(name!("fav", "created_at"), params.add_value(created_at));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_favorites.into(),
                
            )
        })
        .collect();
    Ok(c)
}
