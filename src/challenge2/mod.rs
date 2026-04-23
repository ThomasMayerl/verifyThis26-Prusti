use core::panic;

use prusti_contracts::*;

const N:usize = 500;

pub struct Bag<'a> {
    vals: &'a mut[i32]
}

pub struct Lock<'a> {
    bag: *mut Bag<'a>
}

impl<'a> Bag<'a> {

    #[trusted]
    fn create() -> Bag<'a> {
        panic!("ABSTRACT");
    }

    #[trusted]
    fn push(&self, val: i32) {
        panic!("ABSTRACT");
    }

    #[trusted]
    fn transfer(&self, other: &Self) {
        panic!("ABSTRACT");
    }

    #[trusted]
    fn iter<F>(&self, fun: F) where
            F: Fn(i32) -> () {
        panic!("ABSTRACT");
    }

    #[trusted]
    fn clear(&self) {
        panic!("ABSTRACT");
    }
}

impl <'a> Lock<'a> {

    #[trusted]
    #[ensures(unsafe {acc(*(self.bag))})]
    fn acquire(&self){}

    #[trusted]
    #[requires(unsafe {acc(*(self.bag))})]
    fn release(&self){}

}

#[trusted]
//#[requires(to_lock.len() == N)]
fn lock_all<'a>(to_lock: [Bag; N]) -> [Lock<'a>; N] {
    panic!("ABSTRACT");
}

#[trusted]
#[requires(i < N)]
#[ensures(result < N)]
fn f(i: usize, x: usize) -> usize {
    panic!("ABSTRACT");
}

#[trusted]
//#[ensures(forall(|i: usize| (0 <= i && i < N) ==> (result[i].vals.is_empty())))]
fn create_n_bags<'a>() -> [Bag<'a>; N] {
    panic!("ABSTRACT");
}

//#[requires(cur.len() == N)]
fn compute(cur: &mut [Bag; N], k: usize) {
    let next_private = create_n_bags();
    let mut next_shared = create_n_bags();

    for _k_idx in 0..k {
        let locked_shared = lock_all(next_shared);
        next_shared = create_n_bags();
        for i in 0..N {
            cur[i].iter(|x| {
                let j = f(i, x.try_into().unwrap());
                if j == i {
                    next_private[i].push(x);
                } else {
                    unsafe {
                        locked_shared[j].acquire();
                        (*locked_shared[j].bag).push(x);
                        locked_shared[j].release();
                    }
                }
            })
        }
        for i in 0..N {
            cur[i].clear();
            next_private[i].transfer(&cur[i]);
            next_shared[i].transfer(&cur[i]);
        }
    }
}