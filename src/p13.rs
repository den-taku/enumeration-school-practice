#![allow(dead_code)]
/// 入力：無向グラフ G
/// 出力：Gのクリーク(完全部分グラフ)全て
///
/// 最悪時線形時間遅延アルゴリズム
pub fn comp(_g: Graph) {
    println!("Practice 13");
    // a(∅)を呼び出して停止
    unimplemented!()
}

// 根：∅
// 対象Sの親：Sのうち一番添字の小さいものを取り除いたもの（cliqueの単調性より）
fn a(_s: Vec<usize>, _g: Graph) {
    // 0. Sを出力
    // 1. V\Sに対し配列を用意する．全て0で初期化．: O(V)
    // 2. 辺集合Eに対し以下を行う : O(E)
    //  2-1. 辺の片方がS，もう片方がV\Sに接続する時，V\Sの配列の値をインクリメントする O(1)
    // 3. reverse traverse：V\Sのうちmin Sより小さいもの : 各遷移はO(1)
    // 4. forward traverse：配列の値が|S|（その頂点はSの全ての頂点に隣接することを意味する）: O(1)
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
