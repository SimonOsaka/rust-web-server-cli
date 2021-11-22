use types::ID;

use types::DateTime;




















#[derive(Debug, Clone)]
pub struct MyAdventures {
    
    id: ID,
    title: String,
    created_at: DateTime,
    is_deleted: i16,
    image_url: String,
    item_type: i16,
    link: String,
    source: i16,
    journey_destiny: String,
    script_content: String,
    play_list: String,
    douban_id: i64,
    douban_rank: i16,
    address: String,
    shop_name: String,
    province: String,
    city: String,
    district: String,
    user_id: i64,
    title_crypto: String,
    fav_count: i64,
}


#[derive(Debug, Clone)]
pub struct RelationsResult {
    my_adventures: MyAdventures,
    
    my_users: MyUsers,
    
}
