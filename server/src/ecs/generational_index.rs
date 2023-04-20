// Generational index template from https://kyren.github.io/2018/09/14/rustconf-talk.html

// You can use other types that usize / u64 if these are too large
#[derive(Eq, PartialEq)]
pub struct GenerationalIndex {
    index: usize,
    generation: u64,
}

impl GenerationalIndex {
    pub fn index(&self) -> usize {
        self.index
    }
}

struct AllocatorEntry {
    is_live: bool,
    generation: u64,
}

pub struct GenerationalIndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl GenerationalIndexAllocator {
    pub fn allocate(&mut self) -> GenerationalIndex {
        if self.free.is_empty() {
            self.entries.push(AllocatorEntry { is_live: true, generation: 0 });
            let index = self.entries.len() - 1;
            GenerationalIndex { index, generation: 0 }
        } else {
            let index = self.free.pop().unwrap();
            assert!(!self.entries[index].is_live);
            self.entries[index].is_live = true;
            let generation = self.entries[index].generation;
            GenerationalIndex { index, generation }
        }
    }

    // Returns true if the index was allocated before and is now deallocated
    pub fn deallocate(&mut self, index: GenerationalIndex) -> bool {
        if self.entries[index.index].is_live && self.entries[index.index].generation == index.generation {
            self.entries[index.index].is_live = false;
            self.entries[index.index].generation += 1;
            self.free.push(index.index);
            true
        } else {
            false
        }
    }
    
    pub fn is_live(&self, index: GenerationalIndex) -> bool {
        self.entries[index.index].is_live && self.entries[index.index].generation == index.generation
    }
}

struct ArrayEntry<T> {
    value: T,
    generation: u64,
}

// An associative array from GenerationalIndex to some Value T.
pub struct GenerationalIndexArray<T>(Vec<Option<ArrayEntry<T>>>);

impl<T> GenerationalIndexArray<T> {
    // Set the value for some generational index.  May overwrite past generation
    // values.
    pub fn set(&mut self, index: GenerationalIndex, value: T) {
        self.0[index.index] = Some(ArrayEntry { value, generation: index.generation });
    }

    // Gets the value for some generational index, the generation must match.
    pub fn get(&self, index: GenerationalIndex) -> Option<&T> {
        // if self.0[index.index].map(|e| e.generation) == Some(index.generation) {
        //     Some(self.0[index.index].unwrap().value).as_ref()
        // } else {
        //     None
        // }
        // Some(& self.0[index.index].unwrap().value)
        
    }
    pub fn get_mut(&mut self, index: GenerationalIndex) -> Option<&mut T> {
        // if self.0[index.index].unwrap().generation == index.generation {
        //     Some(&mut self.0[index.index].unwrap().value)
        // } else {
        //     None
        // }
    }
}