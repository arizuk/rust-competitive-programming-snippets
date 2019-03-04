#[derive(Debug)]
struct UnionFind {
    par: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        let mut vec = vec![0; n];
        for i in 0..n {
            vec[i] = i;
        }
        UnionFind { par: vec }
    }

    fn find(&mut self, x: usize) -> usize {
        if x == self.par[x] {
            x
        } else {
            let par = self.par[x];
            let res = self.find(par);
            self.par[x] = res;
            res
        }
    }

    fn same(&mut self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }

    fn unite(&mut self, a: usize, b: usize) {
        let apar = self.find(a);
        let bpar = self.find(b);
        self.par[apar] = bpar;
    }
}

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new(5);
    uf.unite(0, 1);
    assert_eq!(uf.same(0, 1), true);
    assert_eq!(uf.find(0), 1);
    assert_eq!(uf.find(1), 1);
    assert_eq!(uf.same(0, 2), false);

    uf.unite(1, 3);
    assert_eq!(uf.same(1, 3), true);
    assert_eq!(uf.same(0, 3), true);
    assert_eq!(uf.same(2, 3), false);
}
