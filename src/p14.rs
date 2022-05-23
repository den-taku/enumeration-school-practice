#![allow(dead_code)]

/// 入力：無向グラフG
/// 出力：Gの全張木全て
///
/// 最悪時線形時間遅延アルゴリズム
/// c.f. T. MASTUI, "An Algorithm for Finding All the Spanning Tree in Undirected Graphs" (1993)
pub fn comp(_g: Graph) {
    println!("Practice 14");
    // 1. T ← DFSでspanning treeを構成 O(N + M)
    // 2. Tを{1, 2, ..., N-1}でラベル付けし，残りに{N, ..., M}とラベル付け O(M)
    // 3. a(T, G)を呼び出して終了
    unimplemented!()
}

/// 対象Tの親：(T\{e})∪{f}, e:=max(T), f:=min{uとvのカット; e=uv}
/// 根：{1, 2, ..., N-1}
pub fn a(_t: Graph, _g: Graph) {
    // 1. x := max(T)
    // 2. DFSでT\{1,2,..,N-1}の連結成分を構成する頂点たちに同じ値をラベル付け O(N + M)
    // 3. 各 (e ∈ (E(G)\T))に対し以下を考える O(M)
    //  3-1. e > x か (otherwise continue loop)
    //  3-2. e = uvとしてuとvのラベルが異なるか (otherwise continue loop)
    //  3-3. f := min{eを追加してできる基本サイクル} O(N); a(T\{f})∪{e}, G)
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
