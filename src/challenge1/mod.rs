use prusti_contracts::*;

/* 
#[pure]
#[ensures(i >= a.len() ==> result == 0)]
#[ensures(i < a.len() ==> result == (if a[i] >= h { 1 } else { 0 }) + cnt_at_least(&a, h, i + 1))]
fn cnt_at_least(a: &[usize], h: usize, i: usize) -> usize {
    if i >= a.len() {
        0
    } else {
        (if a[0] >= h { 1 } else { 0 }) + cnt_at_least(&a, h, i + 1)
    }
}

#[pure]
fn is_h_idx(a: &[usize], h: usize) -> bool {
    cnt_at_least(a, h, 0) >= h && !is_h_idx(a, h+1)
}*/

#[requires(n > 0)]
#[requires(a.len() == n)]
fn compute(a: &mut [usize], n: usize) -> usize {
    let mut h: usize = 0;
    while h < n && h < a[h] {
        body_invariant!(n > 0);
        body_invariant!(a.len() == n);
        body_invariant!(0 <= h);
        body_invariant!(h < n);
        // body_invariant!(a === old(a));
        h = h + 1;
    }
    h
}