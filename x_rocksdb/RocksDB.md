## rocksdb 相关的方法

### DB
* pub type DB = DBWithThreadMode<SingleThreaded>;
* pub type DBWithThreadMode<T> = DBCommon<T, DBWithThreadModeInner>;

#### DBWithThreadMode 
    open_default 使用默认选项打开数据库。
    open  使用指定的选项打开数据库。
    open_for_read_only 使用指定的选项打开只读数据库
    open_as_secondary 将数据库作为辅助数据库打开
    open_with_ttl 使用“生存时间”压缩筛选器打开数据库。
    open_cf_with_ttl 使用生存时间压缩筛选器和列系列名称打开数据库, 使用此功能打开的列族将使用默认选项创建
    open_cf_descriptors_with_ttl 使用具有生存时间压缩筛选器和列族描述符的给定数据库打开数据库
    open_cf 打开具有给定数据库选项和列系列名称的数据库。 使用此功能打开的列族将使用默认选项创建。
    open_cf_with_opts 打开具有给定数据库选项和列系列名称的数据库。 使用给定选项打开的列族。
    open_cf_for_read_only 使用给定的数据库选项和列系列名称打开只读数据库
    open_cf_with_opts_for_read_only 使用给定的数据库选项和列系列名称打开只读数据库
    open_cf_descriptors_read_only 仅使用给定的数据库选项和列系列描述符打开就绪数据库
    open_cf_as_secondary 使用给定的数据库选项和列系列名称将数据库作为辅助数据库打开
    open_cf_descriptors_as_secondary 使用给定的数据库选项和列族描述符将数据库作为辅助数据库打开。
    open_cf_descriptors 使用给定的数据库选项和列族描述符打开数据库。
    open_cf_descriptors_internal 私有的
    open_raw 私有的
    open_cf_raw 私有的
    delete_range_cf_opt 使用给定的写入选项删除范围内的数据库条目["from", "to")。
    delete_range_cf ["from", "to")使用默认写入选项删除范围内的数据库条目。
    write_opt 
    write
    write_without_wal

#### DBCommon -- impl<T: ThreadMode, D: DBInner> DBCommon<T, D>
    new
    list_cf
    destroy
    repair
    path
    flush_wal
    flush_opt
    flush
    flush_cf_opt
    flush_cf
    get_opt
    get
    get_cf_opt
    get_cf
    get_pinned_opt
    get_pinned
    get_pinned_cf_opt
    get_pinned_cf
    multi_get
    multi_get_opt
    multi_get_cf
    multi_get_cf_opt
    batched_multi_get_cf
    batched_multi_get_cf_opt
    key_may_exist
    key_may_exist_opt
    key_may_exist_cf
    key_may_exist_cf_opt
    create_inner_cf_handle
    iterator
    iterator_opt
    iterator_cf_opt
    full_iterator
    prefix_iterator
    iterator_cf
    full_iterator_cf
    prefix_iterator_cf
    raw_iterator
    raw_iterator_cf
    raw_iterator_opt
    raw_iterator_cf_opt
    snapshot
    put_opt
    put_cf_opt
    merge_opt
    merge_cf_opt
    delete_opt
    delete_cf_opt
    put
    put_cf
    merge
    merge_cf
    delete
    delete_cf
    compact_range
    compact_range_opt
    compact_range_cf
    compact_range_cf_opt
    set_options
    set_options_cf
    property_value_impl 私有的
    property_value
    property_value_cf
    parse_property_int_value 私有的
    property_int_value
    property_int_value_cf
    latest_sequence_number
    get_updates_since
    try_catch_up_with_primary
    ingest_external_file
    ingest_external_file_opts
    ingest_external_file_cf
    ingest_external_file_cf_opts
    ingest_external_file_raw 私有的
    ingest_external_file_raw_cf 私有的
    live_files
    delete_file_in_range
    delete_file_in_range_cf
    cancel_all_background_work
    drop_column_family 私有的

#### impl<I: DBInner> DBCommon<SingleThreaded, I>

#### impl<I: DBInner> DBCommon<MultiThreaded, I>