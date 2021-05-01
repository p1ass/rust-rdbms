use anyhow::Result;
use rust_rdbms::buffer::{BufferPool, BufferPoolManager};
use rust_rdbms::disk::{DiskManager, PageId};
use rust_rdbms::table::SimpleTable;

fn main() -> Result<()> {
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);
    let mut table = SimpleTable {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
    };

    table.create(&mut bufmgr);

    table.insert(&mut bufmgr, &[b"z", b"Alice", b"Smith"])?;
    table.insert(&mut bufmgr, &[b"x", b"Bob", b"Johnson"])?;
    table.insert(&mut bufmgr, &[b"y", b"Charlie", b"Williams"])?;
    table.insert(&mut bufmgr, &[b"w", b"Dave", b"Miller"])?;
    table.insert(&mut bufmgr, &[b"v", b"Eve", b"Brown"])?;

    bufmgr.flush()?;

    Ok(())
}
