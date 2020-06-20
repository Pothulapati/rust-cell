use std::cell::UnsafeCell;
pub struct Cell<T> {
    // UnsafeCell because there is no allowed way to convert shared reference to a exlusive reference.
    value: UnsafeCell<T>,
}

// Cell can't be used accross thread boundaries, because UnsafeCell does not implement Sync
// unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    // SAFETY: we know noone else can concurrently modify because !Sync, and in a single threaded programs
    // SAFETY: only either set or get can be executed at a time.
    pub fn new(value: T) -> Self {
        Cell {
         value: UnsafeCell::new(value)
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T : Copy 
    {
        unsafe { *self.value.get() }
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
