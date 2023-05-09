use std::time::Duration;
// pub type DBWithThreadMode<T> = DBCommon<T, DBWithThreadModeInner>;
// impl<T: ThreadMode> DBWithThreadMode<T> {
use rocksdb::{DB, Options, WriteBatch, WriteOptions, ColumnFamilyDescriptor};


pub(crate) fn learning_open() {
    let path = "./db/learning/open";
    let mut options = Options::default();
    options.create_if_missing(true); // 当数据库不存在且 create_if_missing 选项为 false 时，打开数据库会失败。
    options.create_missing_column_families(true);

    // DB::open(&options, path); // 使用指定的选项打开数据库。
    // DB::open_default(path); // 使用默认选项打开数据库。


    // error_if_log_file_exist 是一个布尔值，用于指定在打开数据库时是否检查 write-ahead log (WAL) 文件的存在。当设置为 true 时，
    // 如果在打开数据库时检测到存在 WAL 文件，open_for_read_only 方法将返回错误。如果设置为 false，即使存在 WAL 文件，该方法仍将正常打开数据库。
    // WAL 文件用于在系统崩溃时恢复数据库。当一个写操作发生时，RocksDB 首先将其记录到 WAL 文件中，然后将数据写入内存表（memtable）。
    // 如果系统在数据被写入磁盘之前崩溃，WAL 文件可以在重启时用于恢复数据。在某些情况下，例如当你只想访问稳定的数据，而不是可能存在未完成写入的数据时，你可能希望在存在 WAL 文件时返回错误。
    DB::open_for_read_only(&options, "./db/learning/open", true).unwrap(); // 使用指定的选项打开只读数据库,  如果日志文件存在，则出现错误


    // 此方法用于打开一个从属（secondary）数据库，从属数据库会自动与主数据库保持同步。从属数据库允许以只读模式访问主数据库的数据，
    // 这在某些情况下很有用，比如当你需要为多个读取任务提供并发访问时。
    // DB::open_as_secondary(&options, path, "./db/learning/secondary"); // 将数据库作为辅助数据库打开


    // 此方法用于打开具有生存时间（TTL，Time to Live）功能的数据库。数据库中的数据在到达生存时间后会自动过期并被删除。
    // 这在某些情况下很有用，比如当你需要在一定时间内删除数据时。
    // DB::open_with_ttl(&options, path, Duration::from_secs(10)); // 使用“生存时间”压缩筛选器打开数据库。


    // 列族（Column Family）是 RocksDB 中的一个概念，它允许您在同一个数据库中存储具有不同设置和选项的数据。您可以将列族看作是 MySQL 中的表（table），
    // 但它们在实现和存储结构上有所不同。RocksDB 是一个键值存储，而不是关系型数据库，所以它没有像关系型数据库中那样的严格结构。
    // let db = DB::open_cf_with_ttl(&options, path, vec!["xu_cf", "happy_cf"], Duration::from_secs(100)).unwrap(); // 使用生存时间压缩筛选器和列系列名称打开数据库, 使用此功能打开的列组将使用默认选项创建
    // let cf_handle_1 = db.cf_handle("xu_cf").unwrap();
    // let cf_handle_2 = db.cf_handle("happy_cf").unwrap();
    // db.put_cf(cf_handle_1, b"key_1", b"value_1").unwrap();
    // db.put_cf(cf_handle_2, b"key_1", b"value_1").unwrap();

    // 和上面类似，ColumnFamilyDescriptor 配置用对象
    // DB::open_cf_descriptors_with_ttl(&options, path, vec![ColumnFamilyDescriptor::new("x_cf", Options::default())], Duration::from_secs(100));

    // 打开时 要打开所有列族， 不然会报错 message: "Invalid argument: Column families not opened: happy_cf, xu_cf"
    // DB::open_cf(&options, path, vec!["happy_cf"]).unwrap(); // 用于打开具有多个列族的数据库
    // println!("{:#?}", DB::list_cf(&options, path).unwrap()); // ["default", "xu_cf", "happy_cf", ]

    // 方法允许您为每个列族提供单独的选项。这意味着您可以为每个列族配置特定的设置，而不是将相同的设置应用于所有列族。
    // DB::open_cf_with_opts(&options, path, vec![("xu_cf", Options::default()), ("happy_cf", Options::default())]).unwrap();

    // 方法允许您以只读模式打开数据库的特定列族。这意味着您将无法修改数据库中的数据，但可以安全地读取数据，即使在多个线程同时访问时。
    // DB::open_cf_for_read_only(&options, path, vec!["xu_cf", "happy_cf"], true). unwrap();

    // 方法结合了上述两种方法的功能。它允许您为每个列族提供单独的选项，同时以只读模式打开数据库。 --- IntoIterator<Item = (N, Options)>
    // DB::open_cf_with_opts_for_read_only();

    // 方法允许您以只读模式打开数据库，并为每个列族提供单独的选项 --- ColumnFamilyDescriptor
    // DB::open_cf_descriptors_read_only();

    // 这个方法允许您以只读模式打开数据库的特定列族作为 Secondary 实例。Secondary 实例能够在不阻塞写入操作的情况下提供强一致性的读取，因为它可以并行读取主实例的更改。
    // DB::open_cf_as_secondary(&options, path, "./db/learning/secondary", vec!["xu_cf", "happy_cf"]).unwrap();

    // 与 open_cf_as_secondary 类似，这个方法也允许您打开数据库的特定列族作为 Secondary 实例。然而，它还允许您为每个列族提供单独的选项。 --- ColumnFamilyDescriptor
    // DB::open_cf_descriptors_as_secondary(&options, path, "./db/learning/secondary", vec![ColumnFamilyDescriptor::new("")]);

    // 此方法允许您在打开数据库时为每个列族提供单独的选项。它类似于 open_cf_with_opts  --- ColumnFamilyDescriptor
    // DB::open_cf_descriptors();


    let db = DB::open_cf(&options, path, vec!["happy_cf", "xu_cf"]).unwrap();
    let cf = db.cf_handle("xu_cf").unwrap();
    // 在给定列族中删除指定范围内的键值对。此方法允许您传入WriteOptions，以便对写入操作进行更细粒度的控制
    db.delete_range_cf_opt(&cf, b"key_1", b"key_2", &WriteOptions::default()).unwrap();

    // 但使用默认的WriteOptions
    db.delete_range_cf(&cf, b"key_1", b"key_2").unwrap();

    // 此方法允许您执行具有自定义WriteOptions的批量写入操作
    let mut batch = WriteBatch::default();
    batch.put_cf(cf, b"key_11", b"value_1");
    batch.put_cf(cf, b"key_22", b"value_2");
    // db.write_opt(batch, &WriteOptions::default());


    // 但使用默认的WriteOptions。
    // db.write(batch);

    // 此方法允许您执行批量写入操作，同时禁用 Write-Ahead Logging（WAL）。这可以提高性能，但在系统崩溃时可能导致数据丢失。
    db.write_without_wal(batch);
    // let mut write_options = WriteOptions::default();
    // db.put_opt(b"a", b"b", &write_options).expect("Unable to put data");

    println!("over")
}