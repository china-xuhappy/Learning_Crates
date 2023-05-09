// 这个文件不用管，由于 RocksDB 的设计，实现这种行为并不容易

// 模拟出现WAL 场景代码

// 在 RocksDB 中，WAL（WritAhead Log）文件用于确保在系统崩溃时能够恢复数据。
// 当一个写操作发生时，RocksDB 首先将其记录到 WAL 文件中，然后将数据写入内存表（memtable）。如果系统在数据被写入磁盘之前崩溃，WAL 文件可以在重启时用于恢复数据。
// 要模拟一个 WAL 存在的场景，可以在程e-序执行写操作的过程中中断程序，这样 WAL 文件将包含未完全写入磁盘的数据。以下是一个简单的示例：

use rocksdb::{DB, Options};
use std::thread;
use std::time::Duration;

// 通常以 LOG 为前缀，后跟一个递增的数字，如 LOG.old.1683618171081658
pub(crate) fn mock_main() {
    let path = "./db/learning/wal";
    let mut options = Options::default();
    options.create_if_missing(true);

    let db = DB::open(&options, path).expect("Failed to open database");

    // 写入数据
    for i in 0..10 {
        db.put(format!("key11111{}", i), format!("value22222222{}", i))
            .expect("Failed to write data");
        println!("Put key{}, value{}", i, i);

        // 暂停 1 秒
        thread::sleep(Duration::from_secs(1));
    }
}
