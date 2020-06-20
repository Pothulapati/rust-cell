use std::cell::UnsafeCell;
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

// Cell can't be used accross thread boundaries, because UnsafeCell does not implement Sync

unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
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
    use super::Cell;

    #[test]
    fn bad() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(0));
        let x1 = Arc::clone(&x);
        let t1 = std::thread::spawn(move || {
            for i in 1..10000 {
                let val = x1.get();
                x1.set(val+1);
            }
        });

        let x2 = Arc::clone(&x);
        let t2 = std::thread::spawn(move || {
            for i in 1..10000 {
                let val = x2.get();
                x2.set(val+1);
            }
        });

        t1.join().unwrap();
        t2.join().unwrap();

        assert_eq!(x.get(), 20000);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
