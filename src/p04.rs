use std::collections::HashSet;

// 多分 最悪時線形時間遅延アルゴリズムになっているはず
pub fn comp(n: usize) {
    a(HashSet::new(), n + 1);
}

/// 対象Sの親：Sの小さいものから2つ取り除いた部分集合
pub fn a(parent: HashSet<usize>, min: usize) {
    println!("{parent:?}"); // O(n)
    for i in 1..min {
        for j in i + 1..min {
            // in real, this is O(n)
            let new_set = {
                let mut set = parent.clone();
                set.insert(i);
                set.insert(j);
                set
            };
            a(new_set, i);
        }
    }
}
