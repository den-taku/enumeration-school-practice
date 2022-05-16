// 簡単のため n >= 2 の実装
pub fn comp(n: usize, capital_n: usize) {
    println!("Practice 06");
    let mut v = vec![false; n];
    let mut sum = 0;
    for i in (1..=n).rev() {
        if sum + i <= capital_n {
            sum += i;
            v[i - 1] = true;
        }
        if sum == capital_n {
            break;
        }
    }
    if sum == capital_n {
        a(&mut v, n, capital_n);
    }
    println!()
}

// 根：nから降順にNを超えないように足していき，ちょうどNになるような部分集合
// 対象Sの親：Sの最小の要素から1を引き，
// ２番目に小さいものから昇順に見ていき足せる初めの要素に1を足したもの
fn a(parent: &mut [bool], n: usize, capital_n: usize) {
    print_set(parent);
    let indeces = parent
        .iter()
        .enumerate()
        .filter(|(_, e)| **e)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    // println!("indeces: {indeces:?}");
    // 1, a, a+2以上, ... -> a+1, a+2以上, ...
    if indeces[0] > 1 {
        parent[0] = true;
        parent[indeces[0]] = false;
        parent[indeces[0] - 1] = true;
        a(parent, n, capital_n);
        parent[0] = false;
        parent[indeces[0]] = true;
        parent[indeces[0] - 1] = false;
    }
    // 1, a, a+1, ..., a+k-1, a+k, a+k+2以上, ...
    // -> a, a+1, ..., a+k-1, a+k + 1, a+k+2以上, ...
    if indeces[0] > 0 {
        for i in 1..indeces.len() {
            if indeces[i] - indeces[i - 1] == 1 {
                continue;
            }
            if indeces[i] - indeces[i - 1] == 2 {
                parent[0] = true;
                parent[indeces[i]] = false;
                parent[indeces[i] - 1] = true;
                a(parent, n, capital_n);
                parent[0] = false;
                parent[indeces[i]] = true;
                parent[indeces[i] - 1] = false;
            }
            break;
        }
    }
    // b, a, a+2以上, ... -> b-1, a+1, a+2以上, ...
    if indeces.len() > 1 && indeces[1] - indeces[0] > 2 {
        parent[indeces[0]] = false;
        parent[indeces[0] + 1] = true;
        parent[indeces[1]] = false;
        parent[indeces[1] - 1] = true;
        a(parent, n, capital_n);
        parent[indeces[0]] = true;
        parent[indeces[0] + 1] = false;
        parent[indeces[1]] = true;
        parent[indeces[1] - 1] = false;
    }
    // b, a, a+1, ..., a+k-1, a+k, a+k+2以上, ...
    // -> b - 1, a, a+1, ..., a+k-1, a+k + 1, a+k+2以上, ...
    if indeces.len() > 1 && indeces[1] - indeces[0] > 1 {
        for i in 2..indeces.len() {
            if indeces[i] - indeces[i - 1] == 1 {
                continue;
            }
            if indeces[i] - indeces[i - 1] == 2 {
                parent[indeces[0]] = false;
                parent[indeces[0] + 1] = true;
                parent[indeces[i]] = false;
                parent[indeces[i] - 1] = true;
                a(parent, n, capital_n);
                parent[indeces[0]] = true;
                parent[indeces[0] + 1] = false;
                parent[indeces[i]] = true;
                parent[indeces[i] - 1] = false;
            }
            break;
        }
    }
}

fn print_set(v: &[bool]) {
    print!("[");
    for (i, e) in v.iter().enumerate() {
        if *e {
            print!("{}, ", i + 1);
        }
    }
    println!("]")
}
