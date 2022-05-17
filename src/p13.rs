#![allow(dead_code)]

/// 入力：無向グラフ G
/// 出力：Gのクリーク全て
///
/// 最悪時線形時間遅延アルゴリズム
pub fn comp(_g: Graph) {
    println!("Practice 13");
    // Gを連結成分C_1, C_2, ..., C_kに分解する O(V + E)
    // 連結成分C_i毎にa(C_i, G)を呼び出して停止
    unimplemented!()
}

// 根：連結成分
// 対象Sの親：V\SのうちSと合わせてcliqueになるような頂点のうち最小のものをSに加えた集合
fn a(_s: Vec<usize>, _g: Graph) {
    // 1. Sの中でcut-vertexでないものを列挙：lowlink, O(V + E)
    // 2. v_1 := min{v \in V\S; Sと隣接する}：O(V + E)
    // 3. v_1が１頂点s_1とのみ隣接 → v_2 := min{v \in V\S; S\{s_1}と隣接する}：O(V + E)
    // 4. reverse traverse：s \in S がcut-vertexでない O(1)
    // 5. forward traverse：v_1が２頂点以上と隣接 → v_1以下，v_1が１頂点のみと隣接 → v_1以下またはs_1でs_1 <= v_2 O(1)
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
