## rust-web-server-cli
Generate rust web server code via mustache template [rust-web-server-mustache](https://github.com/SimonOsaka/rust-web-server-mustache).

[rust-web-server-cli](https://github.com/SimonOsaka/rust-web-server-cli) ╋ [rust-web-server-mustache](https://github.com/SimonOsaka/rust-web-server-mustache) 〓 [rust-web-server-example](https://github.com/SimonOsaka/rust-web-server-example)

- cli: generate code
- mapper: generate sql and model

### Source to use

1. `git clone https://github.com/SimonOsaka/rust-web-server-cli.git`
2. `git clone https://github.com/SimonOsaka/rust-web-server-mustache.git`
3. `cd rust-web-server-cli`
4. `vi mustache.config.toml`
5. modify mustache path and example path.
```toml
mustache_path = "<input 'rust-web-server-mustache' path>"

example_path = "<input path to generate>"
```
6. Everything is `ok`, run command `cargo run`, code will be generate in example_path.

### Binary to use
Run `cargo build` or `cargo build --release`, after that open folder `target/debug` or `target/release`, open it in terminal and input command 
```shell
./gen -m <mustache-config-path> -d # -d will output gen detail
```
for example
```shell
./gen -m /path/to/mustache.config.toml
```
output below
```shell
Gen...
Gen success!
```
**Code generation is done.**

### SQL
Database *postgresql*, SQL structure => `https://github.com/SimonOsaka/my_adventures_api#postgresql-13`.

Database *mysql*, 
```sql
CREATE TABLE `my_adventures` (
  `id` bigint(20) unsigned NOT NULL AUTO_INCREMENT,
  `title` varchar(40) CHARACTER SET utf8mb4 NOT NULL DEFAULT '',
  `created_at` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP,
  `is_deleted` tinyint(4) unsigned NOT NULL DEFAULT '0' COMMENT '0:删除，1:正常',
  `image_url` varchar(255) NOT NULL DEFAULT '',
  `item_type` tinyint(4) unsigned NOT NULL DEFAULT '1' COMMENT '类型',
  `link` varchar(255) NOT NULL DEFAULT '' COMMENT '外链',
  `source` tinyint(4) unsigned NOT NULL DEFAULT '0' COMMENT '来源',
  `journey_destiny` varchar(12) NOT NULL DEFAULT '' COMMENT '旅游目的地',
  `script_content` varchar(140) CHARACTER SET utf8mb4 NOT NULL DEFAULT '' COMMENT '剧本内容',
  `play_list` varchar(16) NOT NULL DEFAULT '' COMMENT '播放列表',
  `douban_id` bigint(20) unsigned NOT NULL DEFAULT '0' COMMENT 'douban subject id',
  `douban_rank` tinyint(3) unsigned NOT NULL DEFAULT '0' COMMENT '豆瓣电影排名',
  `address` varchar(100) NOT NULL DEFAULT '' COMMENT '地址',
  `shop_name` varchar(20) NOT NULL DEFAULT '' COMMENT '商店名称',
  `province` varchar(7) NOT NULL DEFAULT '' COMMENT '省',
  `city` varchar(10) NOT NULL DEFAULT '' COMMENT '市',
  `district` varchar(10) NOT NULL DEFAULT '' COMMENT '区',
  PRIMARY KEY (`id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=1 DEFAULT CHARSET=utf8 ROW_FORMAT=DYNAMIC;
```