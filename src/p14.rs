#![allow(dead_code)]

/// 入力：無向グラフG
/// 出力：Gの全張木全て
///
/// 最悪時多項式時間遅延アルゴリズム
pub fn comp(_g: Graph) {
    println!("Practice 14");
    // 1. 辺を添字の小さいものから見ていき，UF木を使って閉路判定しながら全張木Tを生成
    // 2. Tが構成できたならa(T)を呼び出す
    // 3. 停止
    unimplemented!()
}

/// 対象Tの親：E\E(T)から最小の添字のものを選びTに追加する．
/// この時できたサイクルから最大添字のものを除去した全張木
/// 根：添字を重みとしてみた最小全張木
pub fn a(_t: Graph, _g: Graph) {
    // 1. Tを出力
    // 2. m ← E\E(T)の最小値
    // 3. TODO: 4をO(1)でできるような前処理を考える
    // 4. reverse traverse: e \in E\E(T), eを追加してできたT'のサイクルを構成する最小添字の枝をe_1として除去: DFSでO(V + E)
    // 5. forward traverse: e_1がE\E(T')の中で最小添字: mとの比較でO(1)
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
