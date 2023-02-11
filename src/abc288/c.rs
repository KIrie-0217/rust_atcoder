use proconio::input;

pub struct UnionFind {
    par: Vec<i64>,
    siz: Vec<u64>,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            par: vec![-1; n],
            siz: vec![1; n],
        }
    }

    // 根を求める
    pub fn root(&mut self, x: usize) -> usize {
        if self.par[x] == -1 {
            return x; // 自分自身が根の場合は自身を返す
        } else {
            self.par[x] = self.root(self.par[x] as usize) as i64;
            return self.par[x] as usize;
        }
    }

    // xとｙの根が一致するかどうか
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    // xとyのグループを併合する
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        // x と y を根まで移動する
        let mut x = self.root(x);
        let mut y = self.root(y);

        if x == y {
            return false;
        }

        // union by size ( y側のサイズが小さくなるようにする)
        if self.siz[x] < self.siz[y] {
            //swap(x y)
            let tmp = y;
            y = x;
            x = tmp;
        }

        // ｙをxの子にする
        self.par[y] = x as i64;
        self.siz[x] += self.siz[y];
        return true;
    }

    pub fn size(&mut self, x: usize) -> u64 {
        let r = self.root(x);
        return self.siz[r as usize];
    }
}

fn main() {
    input! {
        n:usize,m:usize,
        ab_vec:[ [ usize;2];m]
    }

    let mut union_find = UnionFind::new(n);

    let mut ans: usize = 0;
    for ab in ab_vec {
        if union_find.is_same(ab[0] - 1, ab[1] - 1) {
            ans += 1;
        } else {
            union_find.unite(ab[0] - 1, ab[1] - 1);
        }
    }

    println!("{}", ans);
}
