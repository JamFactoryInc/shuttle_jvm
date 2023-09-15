use std::mem::size_of;

pub struct Heap {
    heap: Vec<Line>,
    offset: usize
}
impl Heap {
    pub fn new(megabytes: usize) -> Heap {
        let heap = Vec::with_capacity(megabytes / size_of::<Line>());
        Heap {
            offset: heap.as_ptr() as usize,
            heap,
        }
    }
}

pub struct MemTrackerNode {
    available_values: u32,
    block_size: u32,
    child_indices: [u32; 8]
}

pub struct MemTrackerOctree {
    nodes: Vec<MemTrackerNode>
}
impl MemTrackerOctree {
    pub fn new(megabytes: usize) -> MemTrackerOctree {
        //let
        //let vec =
        todo!()
    }
}

/// a block of memory that fits in a cache line
pub struct Line {
    block: [u64; 8]
}