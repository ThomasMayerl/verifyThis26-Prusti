use prusti_contracts::*;

struct Lock {
    x: *mut i32
}

impl Lock {

    #[trusted]
    #[ensures(unsafe {acc(*(self.x))})]
    fn acquire(&self){}

    #[trusted]
    #[requires(unsafe {acc(*(self.x))})]
    fn release(&self){}

    #[trusted]
    #[requires(unsafe {acc(*(self.x))})]
    fn init(&self){}

}

fn non_determ() -> bool {
    true // irrelevant, not seen by caller
}

fn main() {
    let mut val = 5;
    let ptr = &raw mut val;
    let lock = Lock {x: ptr};

    // prove that we can create lock -> what parallel loop requires
    lock.init();
    
    // create parallel threads
    if non_determ()  {
        
    } else {
        // prove that we can use lock -> what happens in parallel loop
        lock.acquire();
        unsafe {
            *ptr = *ptr + 1;
        }
        lock.release();
    }

}