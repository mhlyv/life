use std::collections::HashSet;

// vector type, represents coordinates in N dimensions
type Vector<const N: usize> = [i32; N];

// vector addition
fn vec_add<const N: usize>(a: &Vector<N>, b: &Vector<N>) -> Vector<N> {
    let mut res = *a;

    for (i, val) in b.iter().enumerate() {
        res[i] += val;
    }

    res
}

// N dimensional Game of Life representation
struct Life<const N: usize> {
    cells: HashSet<Vector<N>>,
    neighbors: Vec<Vector<N>>, // cache for neighbor offsets
}

impl<const N: usize> Life<N> {
    // generate all offsets from a point in N dimensions
    // it's not fast but it doesn't need to be as it is only run once
    fn gen_offsets() -> Vec<Vector<N>> {
        // 1D
        let mut ns = vec![vec![-1], vec![0], vec![1]];

        for _ in 1..N {
            let mut new = Vec::new();

            for n in ns.iter_mut() {
                // generate all permutations
                for d in -1..=1 {
                    let mut nn = n.clone();
                    nn.push(d);
                    new.push(nn);
                }
            }

            ns = new;
        }

        // convert and remove the center point
        ns.into_iter()
            .map(|x| x.try_into().unwrap())
            .filter(|v: &Vector<N>| *v != [0; N])
            .collect()
    }

    fn new() -> Self {
        let cells = HashSet::<Vector<N>>::new();
        let neighbors = Self::gen_offsets();

        Life { cells, neighbors }
    }

    // return true if there is a live cell at the position
    fn get(&self, pos: &Vector<N>) -> bool {
        self.cells.contains(pos)
    }

    // kill the cell at the position
    fn kill(&mut self, pos: &Vector<N>) {
        self.cells.remove(pos);
    }

    // create a live cell at the position
    fn create(&mut self, pos: Vector<N>) {
        self.cells.insert(pos);
    }

    // count the live neighbors of the position
    fn count_neighbors(&self, pos: &Vector<N>) -> usize {
        self.neighbors
            .iter()
            .filter(|&d| self.get(&vec_add(d, pos)))
            .count()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_offsets() {
        let os = Life::<2>::gen_offsets();
        assert_eq!(os.len(), 3usize.pow(2) - 1);

        let os = Life::<5>::gen_offsets();
        assert_eq!(os.len(), 3usize.pow(5) - 1);
    }

    #[test]
    fn count_neighbors() {
        let mut l = Life::<3>::new();
        l.create([0, 0, 0]);
        l.create([1, 0, 0]);
        l.create([0, -1, -1]);

        assert_eq!(l.count_neighbors(&[0, 0, 0]), 2);
    }
}
