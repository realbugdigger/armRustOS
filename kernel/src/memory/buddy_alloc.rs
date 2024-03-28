use alloc::vec::Vec;
use core::alloc::{GlobalAlloc, Layout};
use core::cmp;
use core::fmt::Display;
use core::ptr::NonNull;
use crate::info;

pub struct BuddyAllocator {
    start_address: u64,
    end_address: u64,
    num_levels: u8,
    block_size: u16,
    free_list: Vec<Vec<u32>>
}

impl BuddyAllocator {

    pub fn new() -> BuddyAllocator {
        // number of levels excluding the leaf level
        let mut num_levels: u8 = 18;
        // vector of free lists
        let mut free_lists: Vec<Vec<u32>> = Vec::with_capacity((num_levels + 1) as usize);
        // Initialize each free list with a small capacity (in order to use the current allocator
        // at least for the first few items and not the one that will be in use when we're actually
        // using this as the allocator as this might lead to this allocator using itself and locking)
        for _ in 0..(num_levels + 1) {
            free_lists.push(Vec::with_capacity(4));
        }
        // The top-most block is (the only) free for now!
        free_lists[0].push(0);

        BuddyAllocator {
            start_address: 0,
            end_address: 0,
            num_levels,
            block_size: 4096,
            free_list: free_lists,
        }
    }
    
    pub fn init(&mut self, start_addr: *mut u8, heap_size: usize) {
        self.start_address = start_addr as u64;
        self.end_address = self.start_address + (heap_size as u64);
    }

    fn max_size(&self) -> usize {
        // max size that can be supported by this buddy alloc
        (self.block_size as usize) << (self.num_levels as usize)
    }

    // /// Convert an index in the free_list to a block size
    // fn index_to_size(&self, index: usize) -> usize {
    //     2usize.pow((index as u32) + (self.block_size.trailing_zeros()))
    // }

    // /// Convert a requested size in bytes to an index in the free_list
    // fn size_to_index(&self, size: usize) -> usize {
    //     ((size + self.min_size - 1) / self.min_size).next_power_of_two().trailing_zeros() as usize
    // }

    /// Find the level that can accommodate the required memory size.
    fn size_to_level(&self, size: usize) -> usize {
        let max_size = self.max_size();

        // find the largest block level that can support this size
        let mut next_level = 1;
        while (max_size >> next_level) >= size {
            next_level += 1;
        }

        let req_level = cmp::min(next_level - 1, self.num_levels as usize);
        req_level
    }

    // get_free_block gives us the index of the block in the given level
    fn get_free_block(&mut self, level: usize) -> Option<u32> {
        // Get a block from the free list at this level or split a block above and
        // return one of the splitted blocks.
        self.free_list[level]
            .pop()
            .or_else(|| self.split_level(level))
    }

    fn split_level(&mut self, level: usize) -> Option<u32> {
        // We reached the maximum level, we can't split anymore! We can't support this allocation.
        if level == 0 {
            None
        } else {
            self.get_free_block(level - 1).map(|block| {
                // Get a block from 1 level above us and split it.
                // We push the second of the splitted blocks to the current free list
                // and we return the other one as we now have a block for this allocation!
                self.free_list[level].push(block * 2 + 1);
                block * 2
            })
        }
    }

    // /// Merge two buddy blocks if both are free
    // unsafe fn merge(&mut self, block: *mut (), index: usize) {
    //     let buddy;
    //     let combined_address;
    //
    //     if (block as usize) & self.index_to_size(index) == 0 {
    //         buddy = ((block as usize) + self.index_to_size(index)) as *mut ();
    //         combined_address = block as usize;
    //     } else {
    //         buddy = ((block as usize) - self.index_to_size(index)) as *mut ();
    //         combined_address = buddy as usize;
    //     }
    //
    //     if let Some(position) = self.free_list[index].iter().position(|&x| x == buddy) {
    //         self.free_list[index].remove(position);
    //         self.merge(combined_address as *mut (), index + 1);
    //     } else {
    //         self.free_list[index].push(block);
    //     }
    // }

    fn merge_buddies(&mut self, level: usize, block_num: u32) {
        // toggle last bit to get buddy block number
        let buddy_block = block_num ^ 1;
        // if buddy block in free list
        if let Some(buddy_idx) = self.free_list[level]
            .iter()
            .position(|blk| *blk == buddy_block)
        {
            // remove current block (in last place)
            self.free_list[level].pop();
            // remove buddy block
            self.free_list[level].remove(buddy_idx);
            // add free block to free list 1 level above
            self.free_list[level - 1].push(block_num / 2);
            // repeat the process!
            self.merge_buddies(level - 1, block_num / 2)
        }
    }
}

impl BuddyAllocator {
    pub unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, ()> {
        info!("[!!!!!!!!!!!!] ALOCATING !!!!");
        // We should always be aligned due to how the buddy alloc works
        // (everything will be aligned to block_size bytes).
        // If we need in some case that we are aligned to a greater size,
        // allocate a memory block of (alignment) bytes.
        let size = cmp::max(layout.size(), layout.align());

        // find which level of this alloc can accommodate this amount of memory
        let level = self.size_to_level(size);

        // Now to check if we actually have / can make a free block
        // let block = self.get_free_block(level).map(|block| {
        //     // We got a free block!
        //     // get_free_block gives us the index of the block in the given level,
        //     // so we need to find the size of each block in that level and multiply by the index
        //     // to get the offset of the memory that was allocated.
        //     let addr= self.start_address + block as u64 * (self.max_size() >> level);
        //     // Add the base address of this buddy allocator's block and return
        //     addr as *mut u8
        // })

        // let addr= self.start_address + block as u64 * (self.max_size() >> level);

        // let block_num = ((ptr as u64 - self.start_address) / (self.max_size() >> level)) as u32;

        // addr as *mut u8

        let block = self.get_free_block(level).unwrap();
        let addr = self.start_address + block as u64 * (self.max_size() >> level) as u64;
        Ok(addr as *mut u8)
    }

    pub unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        info!("[!!!!!!!!!!!!] DEALOCATING !!!!");
        // As above, find which size was used for this allocation so that we can find the level
        // that gave us this memory block.
        let size = cmp::max(layout.size(), layout.align());

        let level = self.size_to_level(size); // find which level of this alloc was used for this memory request

        // find size of each block at this level
        let level_block_size = self.max_size() >> level;

        let block_num = ((ptr as u64 - self.start_address) / level_block_size as u64) as u32;

        // push freed block to the free list, so we can reuse it
        self.free_list[level].push(block_num);

        // try merging buddy blocks now that we might have some to merge
        self.merge_buddies(level, block_num);
    }
}

impl Display for BuddyAllocator {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut res = writeln!(
            f,
            "  Start: {:x?} / End: {:x?} / Levels: {} / Block size: {} / Max alloc: {}",
            self.start_address,
            self.end_address,
            self.num_levels + 1,
            self.block_size,
            (self.block_size as usize) << (self.num_levels as usize),
        );
        res = res.and_then(|_| write!(f, "  Free lists: "));
        for i in 0usize..(self.num_levels as usize + 1) {
            res = res.and_then(|_| write!(f, "{} in L{} / ", self.free_list[i].len(), i));
        }
        res
    }
}