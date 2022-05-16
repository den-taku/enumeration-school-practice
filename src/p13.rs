#![allow(dead_code)]

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

// 最悪時線形時間遅延アルゴリズム
pub fn comp(_g: Graph) {
    println!("Practice 13");
    // Gを連結成分C_1, C_2, ..., C_kに分解する
    // 連結成分C_i毎にa(C_i, G)を呼び出して停止
    unimplemented!()
}

// 根：連結成分
// 対象Sの親：V\SのうちSと合わせてcliqueになるような頂点のうち最小のものをSに加えた集合
fn a(_s: Vec<usize>, _g: Graph) {
    // 1. Sの中でcut-vertexでないものを列挙：lowlink，O(V + E)
    // 2. v := min{min{v \in Sで V\S の一つのみと隣接する}, max{v \in Sで vは隣接しないがv+1が2個以上と隣接する}}：O(V + E)
    // 3. reverse traverse：s \in S がcut-vertexでない O(1)
    // 4. forward traverse：s \in S が 2で定めたv以下
    unimplemented!()
}
