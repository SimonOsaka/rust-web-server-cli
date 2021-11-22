use super::models::MyAdventures;
use crate::db::{SqlParams, SqlReader, SqlWriter};
use sql_builder::SqlBuilder;
use sqlx::{Error, Postgres, Transaction};
use types::ID;

const MY_ADVENTURES_FIELDS: &[&str; 21] = &[

    "id",

    "title",

    "created_at",

    "is_deleted",

    "image_url",

    "item_type",

    "link",

    "source",

    "journey_destiny",

    "script_content",

    "play_list",

    "douban_id",

    "douban_rank",

    "address",

    "shop_name",

    "province",

    "city",

    "district",

    "user_id",

    "title_crypto",

    "fav_count",

];

const MY_ADVENTURES_INSERT_FIELDS: &[&str; 18] = &[

    "title",

    "image_url",

    "item_type",

    "link",

    "source",

    "journey_destiny",

    "script_content",

    "play_list",

    "douban_id",

    "douban_rank",

    "address",

    "shop_name",

    "province",

    "city",

    "district",

    "user_id",

    "title_crypto",

    "fav_count",

];

const MY_ADVENTURES_STRUCT_FIELDS: &[&str; 21] = &[

    "(at.id",
    "at.title",
    "at.created_at",
    "at.is_deleted",
    "at.image_url",
    "at.item_type",
    "at.link",
    "at.source",
    "at.journey_destiny",
    "at.script_content",
    "at.play_list",
    "at.douban_id",
    "at.douban_rank",
    "at.address",
    "at.shop_name",
    "at.province",
    "at.city",
    "at.district",
    "at.user_id",
    "at.title_crypto",
    "at.fav_count",
    ") AS \"my_adventures\""

];

pub async fn insert_my_adventures<'a>(
    at: MyAdventures,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<ID, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::insert_into("my_adventures");
    sql_builder
        .fields(MY_ADVENTURES_INSERT_FIELDS)
        .values(&[
            
    param.add_value(at.title),
    
    param.add_value(at.image_url),
    
    param.add_value(at.item_type),
    
    param.add_value(at.link),
    
    param.add_value(at.source),
    
    param.add_value(at.journey_destiny),
    
    param.add_value(at.script_content),
    
    param.add_value(at.play_list),
    
    param.add_value(at.douban_id),
    
    param.add_value(at.douban_rank),
    
    param.add_value(at.address),
    
    param.add_value(at.shop_name),
    
    param.add_value(at.province),
    
    param.add_value(at.city),
    
    param.add_value(at.district),
    
    param.add_value(at.user_id),
    
    param.add_value(at.title_crypto),
    
    param.add_value(at.fav_count),
    
        ])
        .returning_id();

    let id = sql_builder.insert_one(param, transaction).await?;
    debug!("insert id: {:?}", id);

    Ok(id)
}

pub async fn update_my_adventures<'a>(
    id: ID,
    at: MyAdventures,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<bool, Error> {
    let mut param = SqlParams::new();
    let mut sql_builder = SqlBuilder::update_table("my_adventures");
    sql_builder
    
        .set("title", param.add_value(at.title))
    
        .set("image_url", param.add_value(at.image_url))
    
        .set("item_type", param.add_value(at.item_type))
    
        .set("link", param.add_value(at.link))
    
        .set("source", param.add_value(at.source))
    
        .set("journey_destiny", param.add_value(at.journey_destiny))
    
        .set("script_content", param.add_value(at.script_content))
    
        .set("play_list", param.add_value(at.play_list))
    
        .set("douban_id", param.add_value(at.douban_id))
    
        .set("douban_rank", param.add_value(at.douban_rank))
    
        .set("address", param.add_value(at.address))
    
        .set("shop_name", param.add_value(at.shop_name))
    
        .set("province", param.add_value(at.province))
    
        .set("city", param.add_value(at.city))
    
        .set("district", param.add_value(at.district))
    
        .set("user_id", param.add_value(at.user_id))
    
        .set("title_crypto", param.add_value(at.title_crypto))
    
        .set("fav_count", param.add_value(at.fav_count))
    
        .and_where_eq("is_deleted", 0)
        .and_where_eq("id", param.add_value(id));

    let affect = sql_builder.update_one(param, transaction).await?;
    Ok(affect > 0)
}

pub async fn delete_my_adventures<'a>(
    id: ID,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<bool, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::update_table("my_adventures");
    sql_builder
        .set("is_deleted", 1)
        .and_where_eq("is_deleted", 0)
        .and_where_eq("id", id);

    let affect_rows = sql_builder.delete_one(param, transaction).await?;
    debug!("delete affect_rows: {:?}", affect_rows);

    Ok(affect_rows > 0)
}


pub async fn get_my_adventures_by_id<'a>(
    id: ID,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.id", param.add_value(id));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_title<'a>(
    title: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.title", param.add_value(title));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_created_at<'a>(
    created_at: DateTime,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.created_at", param.add_value(created_at));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_is_deleted<'a>(
    is_deleted: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.is_deleted", param.add_value(is_deleted));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_image_url<'a>(
    image_url: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.image_url", param.add_value(image_url));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_item_type<'a>(
    item_type: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.item_type", param.add_value(item_type));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_link<'a>(
    link: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.link", param.add_value(link));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_source<'a>(
    source: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.source", param.add_value(source));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_journey_destiny<'a>(
    journey_destiny: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.journey_destiny", param.add_value(journey_destiny));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_script_content<'a>(
    script_content: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.script_content", param.add_value(script_content));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_play_list<'a>(
    play_list: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.play_list", param.add_value(play_list));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_douban_id<'a>(
    douban_id: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.douban_id", param.add_value(douban_id));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_douban_rank<'a>(
    douban_rank: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.douban_rank", param.add_value(douban_rank));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_address<'a>(
    address: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.address", param.add_value(address));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_shop_name<'a>(
    shop_name: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.shop_name", param.add_value(shop_name));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_province<'a>(
    province: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.province", param.add_value(province));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_city<'a>(
    city: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.city", param.add_value(city));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_district<'a>(
    district: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.district", param.add_value(district));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_user_id<'a>(
    user_id: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.user_id", param.add_value(user_id));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_title_crypto<'a>(
    title_crypto: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.title_crypto", param.add_value(title_crypto));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}

pub async fn get_my_adventures_by_fav_count<'a>(
    fav_count: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Option<MyAdventures>, Error> {
    let mut param = SqlParams::new();

    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));
    sql_builder
        .fields(MY_ADVENTURES_FIELDS)
        .and_where_eq("at.is_deleted", 0)
        .and_where_eq("at.fav_count", param.add_value(fav_count));

    let res = sql_builder.query_one_optinal(param, transaction).await?;
    debug!("get_favorite: {:?}", res);

    Ok(res)
}



pub async fn find_my_adventures_by_id<'a>(
    id: ID,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "id"), params.add_value(id));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_title<'a>(
    title: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "title"), params.add_value(title));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_created_at<'a>(
    created_at: DateTime,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "created_at"), params.add_value(created_at));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_is_deleted<'a>(
    is_deleted: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "is_deleted"), params.add_value(is_deleted));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_image_url<'a>(
    image_url: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "image_url"), params.add_value(image_url));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_item_type<'a>(
    item_type: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "item_type"), params.add_value(item_type));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_link<'a>(
    link: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "link"), params.add_value(link));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_source<'a>(
    source: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "source"), params.add_value(source));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_journey_destiny<'a>(
    journey_destiny: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "journey_destiny"), params.add_value(journey_destiny));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_script_content<'a>(
    script_content: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "script_content"), params.add_value(script_content));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_play_list<'a>(
    play_list: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "play_list"), params.add_value(play_list));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_douban_id<'a>(
    douban_id: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "douban_id"), params.add_value(douban_id));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_douban_rank<'a>(
    douban_rank: i16,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "douban_rank"), params.add_value(douban_rank));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_address<'a>(
    address: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "address"), params.add_value(address));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_shop_name<'a>(
    shop_name: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "shop_name"), params.add_value(shop_name));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_province<'a>(
    province: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "province"), params.add_value(province));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_city<'a>(
    city: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "city"), params.add_value(city));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_district<'a>(
    district: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "district"), params.add_value(district));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_user_id<'a>(
    user_id: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "user_id"), params.add_value(user_id));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_title_crypto<'a>(
    title_crypto: String,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "title_crypto"), params.add_value(title_crypto));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}

pub async fn find_my_adventures_by_fav_count<'a>(
    fav_count: i64,
    transaction: Option<&'a mut Transaction<'static, Postgres>>,
) -> Result<Vec<(MyAdventures,MyUsers)>, Error> {
    let mut params = SqlParams::new();
    let mut sql_builder = SqlBuilder::select_from(name!("my_adventures";"at"));

    sql_builder
        .fields(MY_ADVENTURES_STRUCT_FIELDS)
        .fields(MY_USERS_STRUCT_FIELDS)
        .left()
        
        .join(name!("my_users";"u"))
        .on("at.user_id = u.id and u.is_deleted = 0")
        
        .and_where_eq(name!("at", "is_deleted"), 0)
        .and_where_eq(name!("at", "fav_count"), params.add_value(fav_count));

    let list: Vec<RelationsResult> = sql_builder.query_list(params, transaction).await?;

    let c = list
        .into_iter()
        .map(|rs| {
            (
                rs.my_adventures.into(),
                
                rs.my_users.into(),
                
            )
        })
        .collect();
    Ok(c)
}
