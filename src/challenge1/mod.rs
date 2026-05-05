use prusti_contracts::*;

// #[pure]
// #[ensures(i >= a.len() ==> result == 0)]
// #[ensures(i < a.len() ==> result == (if a[i] >= h { 1 } else { 0 }) + cnt_at_least(&a, h, i + 1))]
// #[decreases(a.len() - i)]
// fn cnt_at_least(a: &[usize], h: usize, i: usize) -> usize {
//     if i >= a.len() {
//         0
//     } else {
//         (if a[i] >= h { 1 } else { 0 }) + cnt_at_least(&a, h, i + 1)
//     }
// }

// #[pure]
// fn is_h_idx(a: &[usize], h: usize) -> bool {
//     cnt_at_least(a, h, 0) >= h && !is_h_idx(a, h+1)
// }

// recursive because of Prusti loop limitations
#[requires(n > 0)]
#[requires(a.len() == n)]
// #[ensures(is_h_idx(a, result))]
fn compute_rec(a: &[usize], n: usize, curr_h: usize) -> usize {
    assert!(a.len() == n);
    if curr_h < n && curr_h < a[curr_h] {
        // NOTE: This fails if a is mut
        // assert!(a.len() == n);
        let new_h = curr_h + 1;
        compute_rec(a, n, new_h)
    } else {
        curr_h
    }
}

#[requires(n > 0)]
#[requires(a.len() == n)]
// #[ensures(is_h_idx(a, result))]
// NOTE: We cannot currently verify termination since Prusti does not yet support decreases annotations
fn compute(a: &[usize], n: usize) -> usize {
    compute_rec(a, n, 0)
}