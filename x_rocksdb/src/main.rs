use rocksdb::{DB, Options, WriteOptions};

mod x_DBWithThreadMode;
mod mock_wal;

fn open_or_create_db(path: &str) -> DB {
    // 打开 或者 创建
    let mut opts = Options::default();
    opts.create_if_missing(true);
    DB::open(&opts, path).expect("Unable to open database")
}

fn put_data(db: &DB, key: &[u8], value: &[u8]) {
    // 写入数据
    let mut write_options = WriteOptions::default();
    db.put_opt(key, value, &write_options).expect("Unable to put data");
}

fn get_data(db: &DB, key: &[u8]) -> Option<Vec<u8>> {
    match db.get(key) {
        Ok(Some(value)) => Some(value.to_vec()),
        Ok(None) => None,
        Err(e) => {
            println!("Unable to get data {:?}", e);
            None
        }
    }
}

fn delete_data(db: &DB, key: &[u8]) {
    // 删除数据
    let mut write_options = WriteOptions::default();
    db.delete_opt(key, &write_options).expect("Unable to delete data");
}

fn main() {
    // let path = "./db/test";
    // let db = open_or_create_db(path);
    // put_data(&db, b"name", b"xuhappy");
    //
    // if let Some(v) = get_data(&db, b"name") {
    //     println!("The value for key {:?}", v);
    // }else {
    //     println!("Key 'name' not found");
    // }
    //
    // delete_data(&db, b"name");

    x_DBWithThreadMode::learning_open();

    // mock_wal::mock_main();
}


