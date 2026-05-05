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

#[pure]
#[trusted]
#[ensures(result == (forall(|i: usize, j: usize| (i < a.len() && j < a.len() && i < j) ==> a[i] >= a[j])))]
fn sorted_rev(a: &[usize]) -> bool {
    panic!("ABSTRACT");
}

// recursive because of Prusti loop limitations
#[requires(a.len() == n)]
#[requires(sorted_rev(a))]
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

#[requires(a.len() == n)]
#[requires(sorted_rev(a))]
// #[ensures(is_h_idx(a, result))]
// NOTE: We cannot currently verify termination since Prusti does not yet support decreases annotations
fn compute(a: &[usize], n: usize) -> usize {
    compute_rec(a, n, 0)
}

#[requires(a.len() == n)]
#[requires(sorted_rev(a))]
#[requires(low <= n && high <= n)]
fn compute_opt_rec(a: &[usize], n: usize, low: usize, high: usize) -> usize {
    if low >= high {
        low
    } else {
        let mid = low + (high - low) / 2;
        if mid < a[mid] {
            compute_opt_rec(a, n, mid + 1, high)
        } else {
            compute_opt_rec(a, n, low, mid)
        }
    }
}

#[requires(a.len() == n)]
#[requires(sorted_rev(a))]
fn compute_opt(a: &[usize], n: usize) -> usize {
    compute_opt_rec(a, n, 0, n)
}

// #[requires(is_h_idx(a, h))]
#[requires(sorted_rev(a))]
#[requires(i < a.len())]
#[requires(x == a[i])]
fn update_rec(a: &mut [usize], h: usize, i: usize, low: usize, high: usize, x: usize) -> usize {
    if low < high {
        let mid = low + (high - low) / 2;
        if a[mid] == x {
            update_rec(a, h, i, low, mid, x)
        } else {
            update_rec(a, h, i, mid + 1, high, x)
        }
    } else {
        a[low] += 1;
        if low == h && a[low] == h + 1 {
            h + 1
        } else {
            h
        }
    }
}

// #[requires(is_h_idx(a, h))]
#[requires(sorted_rev(a))]
#[requires(i < a.len())]
#[requires(x == a[i])] // Note we cannot do this assignment in the program since otherwise Prusti cannot verify that a hasn't changed
fn update(a: &mut [usize], h: usize, i: usize, x: usize) -> usize {
    let lo = 0_usize;
    let hi = i;
    update_rec(a, h, i, lo, hi, x)
}