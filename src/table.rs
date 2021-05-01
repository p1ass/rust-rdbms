use crate::btree::BTree;
use crate::buffer::BufferPoolManager;
use crate::disk::PageId;
use anyhow::Result;

pub struct SimpleTable {
    pub meta_page_id: PageId,
    pub num_key_elems: usize,
}

impl SimpleTable {
    pub fn create(&mut self, bufmgr: &mut BufferPoolManager) -> Result<()> {
        let btree = BTree::create(bufmgr)?;
        self.meta_page_id = btree.meta_page_id;
        Ok(())
    }
}
