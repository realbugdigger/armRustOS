use std::alloc::{GlobalAlloc, Layout};
use std::io::BufRead;

pub struct BuddyAllocator {
    min_size: usize,
    size: usize,
    memory: *mut (),
    free_list: Vec<Vec<* mut()>>
}

impl BuddyAllocator {
    pub fn new() {

    }

    pub fn init() {

    }

    /// Convert an index in the free_list to a block size
    fn index_to_size(&self, index: usize) -> usize {
        2usize.pow((index as u32) + (self.min_size.trailing_zeros()))
    }

    /// Convert a requested size in bytes to an index in the free_list
    fn size_to_index(&self, size: usize) -> usize {
        ((size + self.min_size - 1) / self.min_size).next_power_of_two().trailing_zeros() as usize
    }

    /// Recursively split memory blocks until it matches the requested size
    fn split(&mut self, block: *mut (), curr_index: usize, end_index: usize) {
        if curr_index == end_index {
            self.free_list[curr_index].push(block);
            return;
        }

        let new_block = ((block as usize) + self.index_to_size(curr_index - 1)) as *mut ();
        self.split(block, curr_index - 1, end_index);
        self.split(new_block, curr_index - 1, end_index);
    }

    /// Merge two buddy blocks if both are free
    unsafe fn merge(&mut self, block: *mut (), index: usize) {
        let buddy;
        let combined_address;

        if (block as usize) & self.index_to_size(index) == 0 {
            buddy = ((block as usize) - self.index_to_size(index)) as *mut ();
            combined_address = block as usize;
        } else {
            buddy = ((block as usize) - self.index_to_size(index)) as *mut ();
            combined_address = buddy as usize;
        }

        if let Some(position) = self.free_list[index].iter().position(|&x| x == buddy) {
            self.free_list[index].remove(position);
            self.merge(combined_address as *mut (), index + 1);
        } else {
            self.free_list[index].push(block);
        }
    }
}

unsafe impl GlobalAlloc for BuddyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size().max(self.min_size);
        let index = self.size_to_index(size);

        let ptr = ptr as *mut ();

        self.merge(index, ptr);
    }
}