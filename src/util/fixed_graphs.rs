use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    hash::Hash,
    ops::{Add, AddAssign, Mul, MulAssign},
};

use itertools::Itertools;
use nalgebra::DMatrix;
use num::{One, Zero};

#[derive(Debug, Clone)]
pub struct FixedGraph<V: Hash> {
    adjacencies: HashMap<V, HashMap<V, isize>>,
}
impl<V: Hash + Eq + Clone> Default for FixedGraph<V> {
    fn default() -> Self {
        Self {
            adjacencies: HashMap::new(),
        }
    }
}
impl<V: Hash + Eq + Clone> FixedGraph<V> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn add_vertex(&mut self, from: V) {
        self.adjacencies.insert(from, HashMap::new());
    }
    pub fn add_edge(&mut self, from: &V, to: V, weight: isize) {
        assert!(
            self.adjacencies.contains_key(from),
            "Vertex `from` not found"
        );
        assert!(self.adjacencies.contains_key(&to), "Vertex `to` not found");
        self.adjacencies.get_mut(from).unwrap().insert(to, weight);
    }
    pub fn remove_edge(&mut self, from: &V, to: &V) -> Option<isize> {
        self.adjacencies
            .get_mut(from)
            .and_then(|edges| edges.remove(to))
    }
    pub fn remove_vertex(&mut self, vertex: &V) -> Option<HashMap<V, isize>> {
        self.adjacencies.remove(vertex)
    }
    pub fn get_edges(&self, from: &V) -> Option<&HashMap<V, isize>> {
        self.adjacencies.get(from)
    }
    pub fn get_edges_mut(&mut self, from: &V) -> Option<&mut HashMap<V, isize>> {
        self.adjacencies.get_mut(from)
    }
    pub fn get_vertices(&self) -> HashSet<V> {
        self.adjacencies.keys().cloned().collect()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VertexToOther<V: Eq + Hash + Clone> {
    pub from: V,
    pub distances: HashMap<V, isize>,
    pub predecessors: HashMap<V, V>,
}
impl<V: Eq + Hash + Clone> VertexToOther<V> {
    pub fn distance(&self, to: &V) -> Option<isize> {
        self.distances.get(to).copied()
    }
    pub fn path(&self, to: &V) -> Option<(isize, Vec<V>)> {
        let mut current = to.clone();
        let mut path = VecDeque::from([current.clone()]);
        while current != self.from {
            current = self.predecessors.get(&current)?.clone();
            path.push_front(current.clone());
        }
        self.distance(to)
            .map(|d| (d, path.into_iter().collect_vec()))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AllToAll<V: Eq + Hash + Clone> {
    pub distances: HashMap<(V, V), isize>,
    pub predecessors: HashMap<(V, V), V>,
}
impl<V: Eq + Hash + Clone> AllToAll<V> {
    pub fn distance(&self, from: &V, to: &V) -> Option<isize> {
        self.distances.get(&(from.clone(), to.clone())).copied()
    }
    pub fn distances_from(&self, from: &V) -> HashMap<V, isize> {
        self.distances
            .iter()
            .filter(|((f, _), _)| f == from)
            .map(|((_, to), d)| (to.clone(), *d))
            .collect()
    }
    pub fn distances_to(&self, to: &V) -> HashMap<V, isize> {
        self.distances
            .iter()
            .filter(|((_, t), _)| t == to)
            .map(|((from, _), d)| (from.clone(), *d))
            .collect()
    }
    pub fn path(&self, from: &V, to: &V) -> Option<(isize, Vec<V>)> {
        let mut current = to.clone();
        let mut path = VecDeque::from([current.clone()]);
        while &current != from {
            current = self.predecessors.get(&(from.clone(), current))?.clone();
            path.push_front(current.clone());
        }
        self.distance(from, to)
            .map(|d| (d, path.into_iter().collect_vec()))
    }
    pub fn paths_from(&self, from: &V) -> HashMap<V, (isize, Vec<V>)> {
        self.predecessors
            .iter()
            .filter(|((f, _), _)| f == from)
            .filter_map(|((_, to), _)| self.path(from, to).map(|r| (to.clone(), r)))
            .collect()
    }
    pub fn paths_to(&self, to: &V) -> HashMap<V, (isize, Vec<V>)> {
        self.predecessors
            .iter()
            .filter(|((_, t), _)| t == to)
            .filter_map(|((from, _), _)| self.path(from, to).map(|r| (from.clone(), r)))
            .collect()
    }
    pub fn paths(&self) -> HashMap<(V, V), (isize, Vec<V>)> {
        self.predecessors
            .iter()
            .filter_map(|((from, to), _)| {
                self.path(from, to).map(|r| ((from.clone(), to.clone()), r))
            })
            .collect()
    }
}

impl<V: Hash + Eq + Clone> FixedGraph<V> {
    pub fn dijkstra(&self, start: &V) -> VertexToOther<V> {
        debug_assert!(
            self.adjacencies
                .iter()
                .all(|(_, e)| e.iter().all(|(_, w)| w >= &0)),
            "Negative edge weight"
        );
        assert!(
            self.adjacencies.contains_key(start),
            "Start vertex not found"
        );

        let mut distances = HashMap::new();
        distances.insert(start.clone(), 0);
        let mut predecessors = HashMap::new();
        predecessors.insert(start.clone(), start.clone());

        let mut queue = BTreeMap::new();
        queue.insert(0, start.clone());

        while let Some((cost, current)) = queue.pop_first() {
            if distances[&current] > cost {
                continue;
            }
            for (to, weight) in &self.adjacencies[&current] {
                let new_cost = cost + weight;
                if distances.get(to).is_some_and(|&d| new_cost >= d) {
                    continue;
                }
                distances.insert(to.clone(), new_cost);
                predecessors.insert(to.clone(), current.clone());
                queue.insert(new_cost, to.clone());
            }
        }
        VertexToOther {
            from: start.clone(),
            distances,
            predecessors,
        }
    }

    pub fn bellman_ford(&self, start: &V) -> Option<VertexToOther<V>> {
        assert!(
            self.adjacencies.contains_key(start),
            "Start vertex not found"
        );

        let mut distances: HashMap<V, isize> = self
            .adjacencies
            .keys()
            .map(|f| (f.clone(), isize::MAX))
            .collect();
        *distances.get_mut(start).unwrap() = 0;
        let mut predecessors = HashMap::new();
        predecessors.insert(start.clone(), start.clone());

        let mut changed = false;
        for _ in 0..self.adjacencies.len() {
            changed = false;

            for (current, edges) in &self.adjacencies {
                for (to, weight) in edges {
                    let new_cost = distances[&current].saturating_add(*weight);
                    if distances[to] <= new_cost {
                        continue;
                    }
                    changed = true;
                    *distances.get_mut(to).unwrap() = new_cost;
                    predecessors.insert(to.clone(), current.clone());
                }
            }

            if !changed {
                break;
            }
        }
        if changed {
            // negative cycle
            return None;
        }

        Some(VertexToOther {
            from: start.clone(),
            distances,
            predecessors,
        })
    }

    pub fn floyd_warshall(&self) -> Option<AllToAll<V>> {
        let mut distances = HashMap::new();
        let mut predecessors = HashMap::new();

        for e1 in self.adjacencies.keys() {
            for e2 in self.adjacencies.keys() {
                distances.insert((e1.clone(), e2.clone()), isize::MAX);
            }
        }

        for (from, edges) in &self.adjacencies {
            distances.insert((from.clone(), from.clone()), 0);
            predecessors.insert((from.clone(), from.clone()), from.clone());
            for (to, weight) in edges {
                distances.insert((from.clone(), to.clone()), *weight);
                predecessors.insert((from.clone(), to.clone()), from.clone());
            }
        }

        for k in self.adjacencies.keys() {
            for i in self.adjacencies.keys() {
                for j in self.adjacencies.keys() {
                    let cost_over_k = distances[&(i.clone(), k.clone())]
                        .saturating_add(distances[&(k.clone(), j.clone())]);
                    if distances[&(i.clone(), j.clone())] > cost_over_k {
                        *distances.get_mut(&(i.clone(), j.clone())).unwrap() = cost_over_k;
                        if let Some(p) = predecessors.get(&(k.clone(), j.clone())) {
                            predecessors.insert((i.clone(), j.clone()), p.clone());
                        }
                    }
                }
            }
        }

        if distances.iter().any(|((a, b), d)| a == b && *d < 0) {
            // negative cycle
            None
        } else {
            Some(AllToAll {
                distances,
                predecessors,
            })
        }
    }

    pub fn johnson(&self, unused: V) -> Option<AllToAll<V>> {
        assert!(!self.adjacencies.contains_key(&unused));

        let mut new_graph = self.clone();
        new_graph.add_vertex(unused.clone());
        for vertex in self.adjacencies.keys() {
            new_graph.add_edge(&unused, vertex.clone(), 0);
        }
        let level = new_graph.bellman_ford(&unused)?.distances;
        for (vertex, edges) in &mut new_graph.adjacencies {
            for (to, weight) in edges {
                *weight += level[vertex] - level[to];
            }
        }

        let mut distances = HashMap::new();
        let mut predecessors = HashMap::new();

        new_graph.remove_vertex(&unused);

        for vertex in new_graph.adjacencies.keys() {
            let VertexToOther {
                distances: d,
                predecessors: p,
                ..
            } = new_graph.dijkstra(vertex);
            distances.extend(d.into_iter().map(|(t, d)| {
                let cost = d - level[vertex] + level[&t];
                ((vertex.clone(), t), cost)
            }));
            predecessors.extend(p.into_iter().map(|(t, p)| ((vertex.clone(), t), p)));
        }

        Some(AllToAll {
            distances,
            predecessors,
        })
    }

    pub fn matrix(&self) -> (DMatrix<Option<isize>>, HashMap<V, usize>, Vec<V>) {
        let n = self.adjacencies.len();

        let mut mapping_iv = Vec::with_capacity(n);
        let mut mapping_vi = HashMap::with_capacity(n);
        let mut matrix = DMatrix::from_element(n, n, None);
        for (i, v) in self.adjacencies.keys().enumerate() {
            mapping_iv.push(v.clone());
            mapping_vi.insert(v.clone(), i);
        }
        for (u, edges) in &self.adjacencies {
            let ui = mapping_vi[u];
            for (v, w) in edges {
                let vi = mapping_vi[v];
                matrix[(ui, vi)] = Some(*w);
            }
        }

        (matrix, mapping_vi, mapping_iv)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
/// Useful to get shortest path from `u` to `v` with `k` steps:
/// - add(x, y) = min(x, y)
/// - mul(x, y) = x + y
/// - neutral element for addition: `None`, "infinite"
/// - neutral element for multiplication: `0`
///
/// This is a "semi-ring" (?)
pub struct MinAdd(Option<isize>);
impl Add for MinAdd {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(match (self.0, rhs.0) {
            (Some(x), Some(y)) => Some(x.min(y)),
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (None, None) => None,
        })
    }
}
impl AddAssign for MinAdd {
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Mul for MinAdd {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.zip(rhs.0).map(|(x, y)| x + y))
    }
}
impl MulAssign for MinAdd {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Zero for MinAdd {
    fn is_zero(&self) -> bool {
        self.0.is_none()
    }
    fn set_zero(&mut self) {
        self.0 = None;
    }
    fn zero() -> Self {
        Self(None)
    }
}
impl One for MinAdd {
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.0 == Some(0)
    }
    fn one() -> Self {
        Self(Some(0))
    }
    fn set_one(&mut self) {
        self.0 = Some(0);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
/// Useful to get whether a path exists from `u` to `v` in `k` steps:
/// - add(x, y) = `x | y`
/// - mul(x, y) = `x & y`
/// - neutral element for addition: `false`
/// - neutral element for multiplication: `true`
pub struct BooleanRing(bool);
impl Add for BooleanRing {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl AddAssign for BooleanRing {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Mul for BooleanRing {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl MulAssign for BooleanRing {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Zero for BooleanRing {
    fn is_zero(&self) -> bool {
        !self.0
    }
    fn set_zero(&mut self) {
        self.0 = false;
    }
    fn zero() -> Self {
        Self(false)
    }
}
impl One for BooleanRing {
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.0
    }
    fn one() -> Self {
        Self(true)
    }
    fn set_one(&mut self) {
        self.0 = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut graph = FixedGraph::new();
        graph.add_vertex('A');
        graph.add_vertex('B');
        graph.add_vertex('C');
        graph.add_vertex('D');
        graph.add_vertex('E');
        graph.add_vertex('F');
        graph.add_edge(&'A', 'B', 1);
        graph.add_edge(&'A', 'C', 2);
        graph.add_edge(&'B', 'C', 1);
        graph.add_edge(&'B', 'D', 2);
        graph.add_edge(&'C', 'D', 1);
        graph.add_edge(&'C', 'E', 2);
        graph.add_edge(&'D', 'E', 1);
        graph.add_edge(&'D', 'F', 2);
        graph.add_edge(&'E', 'F', 1);

        let vertex_to_other = graph.dijkstra(&'A');
        assert_eq!(vertex_to_other.distance(&'A'), Some(0));
        assert_eq!(vertex_to_other.distance(&'B'), Some(1));
        assert_eq!(vertex_to_other.distance(&'C'), Some(2));
        assert_eq!(vertex_to_other.distance(&'D'), Some(3));
        assert_eq!(vertex_to_other.distance(&'E'), Some(4));
        assert_eq!(vertex_to_other.distance(&'F'), Some(5));
        assert_eq!(vertex_to_other.path(&'A'), Some((0, vec!['A'])));
        assert_eq!(vertex_to_other.path(&'B'), Some((1, vec!['A', 'B'])));
        assert_eq!(vertex_to_other.path(&'C'), Some((2, vec!['A', 'C'])));
        assert_eq!(vertex_to_other.path(&'D'), Some((3, vec!['A', 'B', 'D'])));
        assert_eq!(vertex_to_other.path(&'E'), Some((4, vec!['A', 'C', 'E'])));
        assert_eq!(
            vertex_to_other.path(&'F'),
            Some((5, vec!['A', 'B', 'D', 'F']))
        );
    }

    #[test]
    fn test_bf_fw_negative_cycle() {
        let mut graph = FixedGraph::new();
        graph.add_vertex('A');
        graph.add_vertex('B');
        graph.add_vertex('C');
        graph.add_edge(&'A', 'B', -1);
        graph.add_edge(&'B', 'C', 0);
        graph.add_edge(&'C', 'A', 0);

        assert!(graph.bellman_ford(&'A').is_none());
        assert!(graph.floyd_warshall().is_none());
    }

    #[test]
    fn test_bellman_ford_negative_weights() {
        // build graph without negative cycle:
        let mut graph = FixedGraph::new();
        graph.add_vertex('A');
        graph.add_vertex('B');
        graph.add_vertex('C');
        graph.add_vertex('D');
        graph.add_vertex('E');
        graph.add_edge(&'A', 'B', 1);
        graph.add_edge(&'A', 'C', 2);
        graph.add_edge(&'B', 'C', 1);
        graph.add_edge(&'B', 'D', -2);
        graph.add_edge(&'C', 'D', 1);
        graph.add_edge(&'C', 'E', 2);
        graph.add_edge(&'D', 'E', -1);

        let vertex_to_other = graph.bellman_ford(&'A').unwrap();
        assert_eq!(vertex_to_other.distance(&'A'), Some(0));
        assert_eq!(vertex_to_other.distance(&'B'), Some(1));
        assert_eq!(vertex_to_other.distance(&'C'), Some(2));
        assert_eq!(vertex_to_other.distance(&'D'), Some(-1));
        assert_eq!(vertex_to_other.distance(&'E'), Some(-2));
        assert_eq!(vertex_to_other.path(&'A'), Some((0, vec!['A'])));
        assert_eq!(vertex_to_other.path(&'B'), Some((1, vec!['A', 'B'])));
        assert_eq!(vertex_to_other.path(&'C'), Some((2, vec!['A', 'C'])));
        assert_eq!(vertex_to_other.path(&'D'), Some((-1, vec!['A', 'B', 'D'])));
        assert_eq!(
            vertex_to_other.path(&'E'),
            Some((-2, vec!['A', 'B', 'D', 'E']))
        );
    }

    #[test]
    fn test_floyd_warshall_johnson() {
        let mut graph = FixedGraph::new();
        graph.add_vertex('A');
        graph.add_vertex('B');
        graph.add_vertex('C');
        graph.add_vertex('D');

        graph.add_edge(&'A', 'B', 1);
        graph.add_edge(&'A', 'C', 3);
        graph.add_edge(&'B', 'C', -1);
        graph.add_edge(&'B', 'D', 2);
        graph.add_edge(&'C', 'D', 4);
        graph.add_edge(&'D', 'A', 1);

        let ata = graph.floyd_warshall().unwrap();
        let ata2 = graph.johnson('Z').unwrap();
        assert_eq!(ata, ata2);
        assert_eq!(ata.distance(&'A', &'A'), Some(0));
        assert_eq!(ata.distance(&'A', &'B'), Some(1));
        assert_eq!(ata.distance(&'A', &'C'), Some(0));
        assert_eq!(ata.distance(&'A', &'D'), Some(3));
        assert_eq!(ata.distance(&'B', &'A'), Some(3));
        assert_eq!(ata.distance(&'B', &'B'), Some(0));
        assert_eq!(ata.distance(&'B', &'C'), Some(-1));
        assert_eq!(ata.distance(&'B', &'D'), Some(2));
        assert_eq!(ata.distance(&'C', &'A'), Some(5));
        assert_eq!(ata.distance(&'C', &'B'), Some(6));
        assert_eq!(ata.distance(&'C', &'C'), Some(0));
        assert_eq!(ata.distance(&'C', &'D'), Some(4));
        assert_eq!(ata.distance(&'D', &'A'), Some(1));
        assert_eq!(ata.distance(&'D', &'B'), Some(2));
        assert_eq!(ata.distance(&'D', &'C'), Some(1));
        assert_eq!(ata.distance(&'D', &'D'), Some(0));
        assert_eq!(
            ata.distances_from(&'A'),
            HashMap::from([('A', 0), ('B', 1), ('C', 0), ('D', 3)])
        );
        assert_eq!(
            ata.distances_to(&'A'),
            HashMap::from([('A', 0), ('B', 3), ('C', 5), ('D', 1)])
        );
        assert_eq!(
            ata.paths_from(&'B'),
            HashMap::from([
                ('A', (3, vec!['B', 'D', 'A'])),
                ('B', (0, vec!['B'])),
                ('C', (-1, vec!['B', 'C'])),
                ('D', (2, vec!['B', 'D']))
            ])
        );
        assert_eq!(
            ata.paths_to(&'A'),
            HashMap::from([
                ('A', (0, vec!['A'])),
                ('B', (3, vec!['B', 'D', 'A'])),
                ('C', (5, vec!['C', 'D', 'A'])),
                ('D', (1, vec!['D', 'A']))
            ])
        );
    }

    #[test]
    fn test_adj_matrix() {
        let mut cut = FixedGraph::new();
        cut.add_vertex('A');
        cut.add_vertex('B');
        cut.add_vertex('C');
        cut.add_vertex('D');

        cut.add_edge(&'A', 'B', 1);
        cut.add_edge(&'A', 'A', 0);
        cut.add_edge(&'A', 'D', 2);
        cut.add_edge(&'B', 'B', 0);
        cut.add_edge(&'B', 'C', 4);
        cut.add_edge(&'B', 'D', 1);
        cut.add_edge(&'C', 'C', 0);
        cut.add_edge(&'C', 'D', -1);
        cut.add_edge(&'D', 'B', -2);
        cut.add_edge(&'D', 'D', 0);

        let (matrix, mapping_vi, mapping_iv) = cut.matrix();
        for (v, i) in &mapping_vi {
            assert_eq!(*v, mapping_iv[*i]);
        }
        let mut expected = DMatrix::from_element(4, 4, None);
        let a = mapping_vi[&'A'];
        let b = mapping_vi[&'B'];
        let c = mapping_vi[&'C'];
        let d = mapping_vi[&'D'];
        expected[(a, a)] = Some(0);
        expected[(a, b)] = Some(1);
        expected[(a, d)] = Some(2);
        expected[(b, b)] = Some(3);
        expected[(b, b)] = Some(0);
        expected[(b, c)] = Some(4);
        expected[(b, d)] = Some(1);
        expected[(c, c)] = Some(0);
        expected[(c, d)] = Some(-1);
        expected[(d, b)] = Some(-2);
        expected[(d, d)] = Some(0);
        assert_eq!(expected, matrix);
    }

    #[test]
    fn test_boolean_ring() {
        let f = BooleanRing(false);
        let t = BooleanRing(true);
        assert_eq!(f + f, f);
        assert_eq!(f + t, t);
        assert_eq!(t + f, t);
        assert_eq!(t + t, t);

        let mut cut = f;
        cut += f;
        assert_eq!(cut, f);
        cut += t;
        assert_eq!(cut, t);
        cut *= t;
        assert_eq!(cut, t);
        cut = f;
        cut *= t;
        assert_eq!(cut, f);

        assert_eq!(f * f, f);
        assert_eq!(f * t, f);
        assert_eq!(t * f, f);
        assert_eq!(t * t, t);

        assert!(f.is_zero());
        assert!(!f.is_one());
        assert!(!t.is_zero());
        assert!(t.is_one());

        assert_eq!(f, BooleanRing::zero());
        assert_eq!(t, BooleanRing::one());
    }

    #[test]
    fn test_minadd() {
        fn m(i: isize) -> MinAdd {
            MinAdd(Some(i))
        }
        assert_eq!(MinAdd::one() * m(3), m(3));
        assert_eq!(MinAdd::one() * m(-12), m(-12));
        assert_eq!(MinAdd::one() * m(0), m(0));
        assert_eq!(m(3) * MinAdd::one(), m(3));
        assert_eq!(m(-12) * MinAdd::one(), m(-12));
        assert_eq!(m(0) * MinAdd::one(), m(0));
        assert_eq!(MinAdd::one() * MinAdd::one(), MinAdd::one());

        assert_eq!(m(1) + MinAdd::zero(), m(1));
        assert_eq!(m(0) + MinAdd::zero(), m(0));
        assert_eq!(m(-1) + MinAdd::zero(), m(-1));
        assert_eq!(MinAdd::zero() + m(1), m(1));
        assert_eq!(MinAdd::zero() + m(0), m(0));
        assert_eq!(MinAdd::zero() + m(-1), m(-1));
        assert_eq!(MinAdd::zero() + MinAdd::zero(), MinAdd::zero());

        let mut cut = MinAdd::zero();
        cut += m(10);
        assert_eq!(cut, m(10));
        cut += MinAdd::one();
        assert_eq!(cut, MinAdd::one());
        cut *= m(15);
        assert_eq!(cut, m(15));
        cut *= MinAdd::zero();
        assert_eq!(cut, MinAdd::zero());

        assert_eq!(m(1) * m(2), m(3));
        assert_eq!(m(3) * m(-4), m(-1));
        assert_eq!(m(-15) * m(124), m(109));
        assert_eq!(m(7) * m(-7), m(0));
        assert_eq!(m(7) * MinAdd::zero(), MinAdd::zero());

        assert_eq!(m(1) + m(2), m(1));
        assert_eq!(m(2) + m(15), m(2));
        assert_eq!(m(-3) + m(12), m(-3));
        assert_eq!(m(1) + MinAdd::one(), m(0));
    }
}
