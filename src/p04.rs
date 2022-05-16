use std::collections::HashSet;

/// 入力：自然数 n
/// 出力：{1,..,n}の部分集合で要素数が偶数のもの全て
///
/// 多分 最悪時線形時間遅延アルゴリズムになっているはず
pub fn comp(n: usize) {
    println!("Practice 04");
    a(HashSet::new(), n + 1);
    println!()
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
