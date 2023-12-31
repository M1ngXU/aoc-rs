use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::{Debug, Display},
    hash::Hash,
    io::Write,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    process::{Command, Stdio},
};

use itertools::Itertools;
use linked_hash_map::LinkedHashMap;
use nalgebra::{Dyn, OMatrix};
use num::{One, Zero};
use petgraph::{
    dot::Dot,
    graph::{EdgeIndex, EdgeReference, NodeIndex},
    visit::{EdgeRef, GraphProp, IntoEdgeReferences, IntoNodeReferences, NodeIndexable},
    EdgeType, Graph, Undirected,
};
use rand::{seq::SliceRandom, thread_rng};

/// The edge cost, rustworkx sometimes needs this mapping
pub fn ec<T: Copy>(e: EdgeReference<T>) -> T {
    *e.weight()
}

pub trait GrowFactor {
    fn x(&self) -> usize;
    fn y(&self) -> usize;
}
impl GrowFactor for usize {
    fn x(&self) -> usize {
        *self
    }
    fn y(&self) -> usize {
        *self
    }
}
impl GrowFactor for (usize, usize) {
    fn x(&self) -> usize {
        self.0
    }
    fn y(&self) -> usize {
        self.1
    }
}

pub trait Grow {
    fn grow(&mut self, n: impl GrowFactor, w: isize, max: impl GrowFactor);
}
impl<T: Clone, D: EdgeType> Grow for Graph<(usize, usize, T), isize, D> {
    fn grow(&mut self, n: impl GrowFactor, w: isize, max: impl GrowFactor) {
        let (sx, sy) = (n.x(), n.y());
        let (maxx, maxy) = (max.x(), max.y());
        assert_eq!(sx, sy, "Only square grids are supported");
        assert!(sx % 2 == 1, "Only odd sizes are supported");
        assert!(!self.is_directed(), "Only undirected graphs are supported");
        let mut new = vec![HashMap::new(); sx * sy];
        let nodes = self.node_indices().collect_vec();
        let refs = self
            .node_references()
            .map(|(i, x)| (i, x.clone()))
            .collect_vec();
        for (i, w) in &refs {
            for j in 0..sx * sy {
                if j == (sx * sy) / 2 {
                    new[j].insert(*i, *i);
                } else {
                    new[j].insert(*i, self.add_node(w.clone()));
                }
            }
        }
        for i in nodes {
            for j in 0..sx * sy {
                for (to, w) in self
                    .edges(i)
                    .map(|x| (x.target(), *x.weight()))
                    .collect_vec()
                {
                    if j != (sx * sy) / 2 {
                        self.update_edge(new[j][&i], new[j][&to], w);
                    }
                }
            }
        }
        let mut added = 0;
        for x in 0..=maxx {
            for (y, to_y) in [(0, maxy), (maxy, 0)] {
                if let Some((idx_from, _)) = refs.iter().find(|(_, n)| n.0 == x && n.1 == y) {
                    let to_x = x;
                    if let Some((idx_to, _)) = refs.iter().find(|(_, n)| n.0 == to_x && n.1 == to_y)
                    {
                        for gy in 0..sy {
                            for gx in 0..sx {
                                if y == 0 && gy > 0 || y != 0 && gy < sy - 1 {
                                    let j = gy * sx + gx;
                                    let j2 = if y == 0 { gy - 1 } else { gy + 1 } * sx + gx;
                                    self.update_edge(new[j][&idx_from], new[j2][&idx_to], w);
                                    added += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        for y in 0..=maxy {
            for (x, to_x) in [(0, maxx), (maxx, 0)] {
                if let Some((idx_from, _)) = refs.iter().find(|(_, n)| n.0 == x && n.1 == y) {
                    let to_y = y;
                    if let Some((idx_to, _)) = refs.iter().find(|(_, n)| n.0 == to_x && n.1 == to_y)
                    {
                        for gy in 0..sy {
                            for gx in 0..sx {
                                if x == 0 && gx > 0 || x != 0 && gx < sx - 1 {
                                    let j = gy * sx + gx;
                                    let j2 = gy * sx + if x == 0 { gx - 1 } else { gx + 1 };
                                    self.update_edge(new[j][&idx_from], new[j2][&idx_to], w);
                                    added += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        println!("{added}");
    }
}

#[derive(Clone, Debug)]
pub struct Grid {
    pub graph: Graph<(usize, usize, char), isize, Undirected>,
    pub width: usize,
    pub height: usize,
    pub start: Option<(NodeIndex, usize, usize)>,
    pub end: Option<(NodeIndex, usize, usize)>,
    pub vertex_map: HashMap<(usize, usize), NodeIndex>,
    pub edge_map: HashMap<((usize, usize), (usize, usize)), EdgeIndex>,
}

pub fn parse_grid(
    s: &str,
    is_walkable: impl Fn(usize, usize, char) -> bool,
    is_start: impl Fn(usize, usize, char) -> bool,
    is_end: impl Fn(usize, usize, char) -> bool,
    diagonal: bool,
) -> Grid {
    let mut graph = Graph::new_undirected();
    let mut vertex_mapping = HashMap::new();
    let mut edge_mapping = HashMap::new();
    let mut start = None;
    let mut end = None;
    let mut w = 0;
    let mut h = 0;
    for (y, line) in s.trim().lines().enumerate() {
        h = y;
        for (x, c) in line.char_indices() {
            w = x;
            if is_walkable(x, y, c) {
                let idx = graph.add_node((x, y, c));
                if is_start(x, y, c) {
                    start = Some((idx, x, y));
                }
                if is_end(x, y, c) {
                    end = Some((idx, x, y));
                }
                vertex_mapping.insert((x, y), idx);
                if x > 0 {
                    if let Some(n) = vertex_mapping.get(&(x - 1, y)) {
                        edge_mapping.insert(((x - 1, y), (x, y)), graph.add_edge(*n, idx, 1));
                    }
                }
                if diagonal && x > 0 && y > 0 {
                    if let Some(n) = vertex_mapping.get(&(x - 1, y - 1)) {
                        edge_mapping.insert(((x - 1, y - 1), (x, y)), graph.add_edge(*n, idx, 1));
                    }
                }
                if y > 0 {
                    if let Some(n) = vertex_mapping.get(&(x, y - 1)) {
                        edge_mapping.insert(((x, y - 1), (x, y)), graph.add_edge(*n, idx, 1));
                    }
                }
            }
        }
    }
    Grid {
        graph,
        width: w + 1,
        height: h + 1,
        start,
        end,
        vertex_map: vertex_mapping,
        edge_map: edge_mapping,
    }
}

#[derive(Debug, Clone)]
/// Uses a linked hashmap for deterministic adjacency matrices.
pub struct FixedGraph<V: Hash + Eq> {
    pub adjacencies: LinkedHashMap<V, HashMap<V, isize>>,
}
impl<V: Hash + Eq + Clone> Default for FixedGraph<V> {
    fn default() -> Self {
        Self {
            adjacencies: LinkedHashMap::new(),
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
    pub fn add_edge(&mut self, from: V, to: V, weight: isize) {
        if !self.adjacencies.contains_key(&from) {
            self.add_vertex(from.clone());
        }
        if !self.adjacencies.contains_key(&to) {
            self.add_vertex(to.clone());
        }
        self.adjacencies.get_mut(&from).unwrap().insert(to, weight);
    }
    pub fn add_undirected_edge(&mut self, u: V, v: V, weight: isize) {
        self.add_edge(u.clone(), v.clone(), weight);
        self.add_edge(v.clone(), u.clone(), weight);
    }
    pub fn remove_edge(&mut self, from: &V, to: &V) -> Option<isize> {
        self.adjacencies
            .get_mut(from)
            .and_then(|edges| edges.remove(to))
    }
    pub fn remove_undirected_edge(&mut self, u: &V, v: &V) -> Option<isize> {
        self.adjacencies
            .get_mut(v)
            .and_then(|edges| edges.remove(u));
        self.adjacencies
            .get_mut(u)
            .and_then(|edges| edges.remove(v))
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
pub trait Render {
    fn render(self, name: &str) -> std::io::Result<()>;
}
impl<G: IntoEdgeReferences + IntoNodeReferences + NodeIndexable + GraphProp> Render for G
where
    <Self as petgraph::visit::Data>::EdgeWeight: Debug,
    <Self as petgraph::visit::Data>::NodeWeight: Debug,
{
    fn render(self, name: &str) -> std::io::Result<()> {
        let s = format!("{:?}", Dot::new(self));
        let mut dot = Command::new("dot")
            .args(&["-Tsvg", "-o", name])
            .stdin(Stdio::piped())
            .spawn()?;
        dot.stdin.as_mut().unwrap().write_all(s.as_bytes())?;
        let status = dot.wait()?;
        if !status.success() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "dot failed"));
        }
        Ok(())
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
struct UnionFind<T: Hash + Eq + Clone> {
    representants: HashMap<T, T>,
    members: HashMap<T, Vec<T>>,
}
impl<T: Hash + Eq + Clone> UnionFind<T> {
    fn new(v: Vec<T>) -> Self {
        let mut representants = HashMap::new();
        let mut members = HashMap::new();
        for x in v {
            representants.insert(x.clone(), x.clone());
            members.insert(x.clone(), vec![x]);
        }
        Self {
            representants,
            members,
        }
    }
    fn same(&self, a: &T, b: &T) -> bool {
        self.representants[a] == self.representants[b]
    }
    fn union(&mut self, a: &T, b: &T) {
        let a = self.representants[a].clone();
        let b = self.representants[b].clone();
        if a == b {
            return;
        }
        let (a, b) = if self.members[&a].len() < self.members[&b].len() {
            (b, a)
        } else {
            (a, b)
        };
        for x in self.members.remove(&b).unwrap() {
            self.representants.insert(x.clone(), a.clone());
            self.members.get_mut(&a).unwrap().push(x);
        }
    }
}
impl<V: Hash + Eq + Clone> FixedGraph<V> {
    pub fn mst(&self, shuffle: bool) -> Option<FixedGraph<V>> {
        let mut graph = Self::default();
        let mut added = 0;
        let mut edges = self
            .adjacencies
            .iter()
            .flat_map(|(from, edges)| {
                edges
                    .iter()
                    .map(|(to, weight)| (from.clone(), to.clone(), weight.clone()))
            })
            .collect_vec();
        if shuffle {
            edges.shuffle(&mut thread_rng());
        }
        edges.sort_unstable_by_key(|(_, _, w)| -*w);
        let mut uf = UnionFind::new(self.adjacencies.keys().cloned().collect());
        while added != self.adjacencies.len() - 1 {
            let (from, to, _) = edges.pop()?;
            if uf.same(&from, &to) {
                continue;
            }
            uf.union(&from, &to);
            let w = self.adjacencies[&from][&to];
            graph.add_edge(from, to, w);
            added += 1;
        }
        Some(graph)
    }
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

        #[derive(Debug, Clone)]
        struct OnlyFirst<V>(isize, V);
        impl<V> PartialEq for OnlyFirst<V> {
            fn eq(&self, other: &Self) -> bool {
                self.0 == other.0
            }
        }
        impl<V> Eq for OnlyFirst<V> {}
        impl<V> PartialOrd for OnlyFirst<V> {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl<V> Ord for OnlyFirst<V> {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.0.cmp(&other.0)
            }
        }
        let mut queue = BinaryHeap::new();
        queue.push(OnlyFirst(0, start.clone()));

        while let Some(OnlyFirst(cost, current)) = queue.pop() {
            if cost > distances[&current] {
                continue;
            }
            for (to, weight) in &self.adjacencies[&current] {
                let new_cost = cost + weight;
                if distances.get(to).is_some_and(|&d| new_cost >= d) {
                    continue;
                }
                distances.insert(to.clone(), new_cost);
                predecessors.insert(to.clone(), current.clone());
                queue.push(OnlyFirst(new_cost, to.clone()));
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
            new_graph.add_edge(unused.clone(), vertex.clone(), 0);
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

    pub fn matrix(&self) -> (OMatrix<Option<isize>, Dyn, Dyn>, HashMap<V, usize>, Vec<V>) {
        let n = self.adjacencies.len();

        let mut mapping_iv = Vec::with_capacity(n);
        let mut mapping_vi = HashMap::with_capacity(n);
        let mut matrix = OMatrix::<Option<isize>, Dyn, Dyn>::from_element(n, n, None);
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
pub struct MinAdd<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero>(pub Option<T>);
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> Add for MinAdd<T> {
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
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> AddAssign for MinAdd<T> {
    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> Mul for MinAdd<T> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.zip(rhs.0).map(|(x, y)| x + y))
    }
}
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> MulAssign for MinAdd<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> Div for MinAdd<T> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.zip(rhs.0).map(|(x, y)| x - y))
    }
}
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> DivAssign for MinAdd<T> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> Sub for MinAdd<T> {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> SubAssign for MinAdd<T> {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> Zero for MinAdd<T> {
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
impl<T: Copy + Ord + Add<Output = T> + Sub<Output = T> + Zero> One for MinAdd<T> {
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        self.0 == Some(T::zero())
    }
    fn one() -> Self {
        Self(Some(T::zero()))
    }
    fn set_one(&mut self) {
        self.0 = Some(T::zero());
    }
}
impl<T: Display + Ord + Copy + Add<Output = T> + Sub<Output = T> + Zero> Display for MinAdd<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            Some(x) => f.pad(&x.to_string()),
            None => f.pad("∞"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
/// Useful to get whether a path exists from `u` to `v` in `k` steps:
/// - add(x, y) = `x | y`
/// - mul(x, y) = `x & y`
/// - neutral element for addition: `false`
/// - neutral element for multiplication: `true`
pub struct Boolean(pub bool);
impl Add for Boolean {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}
impl AddAssign for Boolean {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}
impl Sub for Boolean {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl SubAssign for Boolean {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}
impl Mul for Boolean {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}
impl MulAssign for Boolean {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}
impl Div for Boolean {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn div(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}
impl DivAssign for Boolean {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}
impl Zero for Boolean {
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
impl One for Boolean {
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
impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&self.0.to_string())
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Matrix4;

    use super::*;

    #[test]
    fn test_dijkstra() {
        let mut graph = FixedGraph::new();
        graph.add_edge('A', 'B', 1);
        graph.add_edge('A', 'C', 2);
        graph.add_edge('B', 'C', 1);
        graph.add_edge('B', 'D', 2);
        graph.add_edge('C', 'D', 1);
        graph.add_edge('C', 'E', 2);
        graph.add_edge('D', 'E', 1);
        graph.add_edge('D', 'F', 2);
        graph.add_edge('E', 'F', 1);

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
        assert_eq!(vertex_to_other.path(&'D'), Some((3, vec!['A', 'C', 'D'])));
        assert_eq!(vertex_to_other.path(&'E'), Some((4, vec!['A', 'C', 'E'])));
        assert_eq!(
            vertex_to_other.path(&'F'),
            Some((5, vec!['A', 'C', 'E', 'F']))
        );
    }

    #[test]
    fn test_bf_fw_negative_cycle() {
        let mut graph = FixedGraph::new();
        graph.add_vertex('A');
        graph.add_vertex('B');
        graph.add_vertex('C');
        graph.add_edge('A', 'B', -1);
        graph.add_edge('B', 'C', 0);
        graph.add_edge('C', 'A', 0);

        assert!(graph.bellman_ford(&'A').is_none());
        assert!(graph.floyd_warshall().is_none());
    }

    #[test]
    fn test_bellman_ford_negative_weights() {
        let mut graph = FixedGraph::new();
        graph.add_edge('A', 'B', 1);
        graph.add_edge('A', 'C', 2);
        graph.add_edge('B', 'C', 1);
        graph.add_edge('B', 'D', -2);
        graph.add_edge('C', 'D', 1);
        graph.add_undirected_edge('C', 'E', 2);
        graph.add_edge('D', 'E', -1);

        let vertex_to_other = graph.bellman_ford(&'A').unwrap();
        assert_eq!(vertex_to_other.distance(&'A'), Some(0));
        assert_eq!(vertex_to_other.distance(&'B'), Some(1));
        assert_eq!(vertex_to_other.distance(&'C'), Some(0));
        assert_eq!(vertex_to_other.distance(&'D'), Some(-1));
        assert_eq!(vertex_to_other.distance(&'E'), Some(-2));
        assert_eq!(vertex_to_other.path(&'A'), Some((0, vec!['A'])));
        assert_eq!(vertex_to_other.path(&'B'), Some((1, vec!['A', 'B'])));
        assert_eq!(
            vertex_to_other.path(&'C'),
            Some((0, vec!['A', 'B', 'D', 'E', 'C']))
        );
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

        graph.add_edge('A', 'B', 1);
        graph.add_edge('A', 'C', 3);
        graph.add_edge('B', 'C', -1);
        graph.add_edge('B', 'D', 2);
        graph.add_edge('C', 'D', 4);
        graph.add_edge('D', 'A', 1);

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
        cut.add_vertex('D');
        cut.add_vertex('C');

        cut.add_edge('A', 'B', 1);
        cut.add_edge('A', 'A', 0);
        cut.add_edge('A', 'D', 2);
        cut.add_edge('B', 'B', 0);
        cut.add_edge('B', 'C', 4);
        cut.add_undirected_edge('B', 'D', 1);
        cut.add_edge('C', 'C', 0);
        cut.add_edge('C', 'D', -1);
        cut.add_edge('D', 'D', 0);

        let (matrix, mapping_vi, mapping_iv) = cut.matrix();
        for (v, i) in &mapping_vi {
            assert_eq!(*v, mapping_iv[*i]);
        }
        let mut expected = Matrix4::from_element(None);
        let a = 0;
        let b = 1;
        let c = 3;
        let d = 2;
        expected[(a, a)] = Some(0);
        expected[(a, b)] = Some(1);
        expected[(a, d)] = Some(2);
        expected[(b, b)] = Some(3);
        expected[(b, b)] = Some(0);
        expected[(b, c)] = Some(4);
        expected[(b, d)] = Some(1);
        expected[(c, c)] = Some(0);
        expected[(c, d)] = Some(-1);
        expected[(d, b)] = Some(1);
        expected[(d, d)] = Some(0);
        assert_eq!(expected, matrix);
    }

    #[test]
    fn test_boolean_ring() {
        let f = Boolean(false);
        let t = Boolean(true);
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

        assert_eq!(f, Boolean::zero());
        assert_eq!(t, Boolean::one());
    }

    #[test]
    fn test_minadd() {
        fn m(i: isize) -> MinAdd<isize> {
            MinAdd(Some(i))
        }
        assert_eq!(MinAdd::one() * m(3), m(3));
        assert_eq!(MinAdd::one() * m(-12), m(-12));
        assert_eq!(MinAdd::one() * m(0), m(0));
        assert_eq!(m(3) * MinAdd::one(), m(3));
        assert_eq!(m(-12) * MinAdd::one(), m(-12));
        assert_eq!(m(0) * MinAdd::one(), m(0));
        assert_eq!(MinAdd::<isize>::one() * MinAdd::one(), MinAdd::one());

        assert_eq!(m(1) + MinAdd::zero(), m(1));
        assert_eq!(m(0) + MinAdd::zero(), m(0));
        assert_eq!(m(-1) + MinAdd::zero(), m(-1));
        assert_eq!(MinAdd::zero() + m(1), m(1));
        assert_eq!(MinAdd::zero() + m(0), m(0));
        assert_eq!(MinAdd::zero() + m(-1), m(-1));
        assert_eq!(MinAdd::<isize>::zero() + MinAdd::zero(), MinAdd::zero());

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
