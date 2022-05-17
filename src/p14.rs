#![allow(dead_code)]

/// 入力：無向グラフG
/// 出力：Gの全張木全て
///
/// 最悪時線形時間遅延アルゴリズムになっているはず
pub fn comp(_g: Graph) {
    println!("Practice 14");
    unimplemented!()
}

/// 対象Sの親：
pub fn a() {
    unimplemented!()
}

/// G = (V, E)
pub struct Graph {
    vertices: Vertices,
    edges: Edges,
}

/// V = {1, ..., n}
pub struct Vertices {
    n: usize,
}

pub struct Edges {
    edges: Vec<(usize, usize)>,
}
