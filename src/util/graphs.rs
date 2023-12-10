use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
    ops::Add,
};

// TODO: dijsktra for 2d/3d arrays, floyd-marshall

pub trait FromIsize {
    fn from_isize(n: isize) -> Self;
}
macro_rules! primitive_from_isize {
    ($($t:ty),*) => {
        $(
            impl FromIsize for $t {
                fn from_isize(n: isize) -> Self {
                    n as $t
                }
            }
        )*
    };
}
primitive_from_isize!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64);

// TODO: use grid

/// Dijkstra's algorithm on a 2d grid with diagonal movement, each can have a different length
pub fn dijkstra2d<C: PartialEq + Eq + PartialOrd + Ord + Clone + FromIsize, T: Clone>(
    grid: Vec<Vec<(C, T)>>,
    start: (isize, isize),
    start_cost: C,
    end: (isize, isize),
) -> Option<(C, Vec<(C, T)>)>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    _dijkstra2d(grid, start, start_cost, end, true)
}

/// Dijkstra's algorithm on a 2d grid without diagonal movement, each can have a different length
pub fn dijkstra2<C: PartialEq + Eq + PartialOrd + Ord + Clone + FromIsize, T: Clone>(
    grid: Vec<Vec<(C, T)>>,
    start: (isize, isize),
    start_cost: C,
    end: (isize, isize),
) -> Option<(C, Vec<(C, T)>)>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    _dijkstra2d(grid, start, start_cost, end, false)
}

/// Dijsktra's algorithm on a 2d grid with diagonal movement (if `diagonal`), each row can have a different length
fn _dijkstra2d<C: PartialEq + Eq + PartialOrd + Ord + Clone + FromIsize, T: Clone>(
    grid: Vec<Vec<(C, T)>>,
    start: (isize, isize),
    start_cost: C,
    end: (isize, isize),
    diagonal: bool,
) -> Option<(C, Vec<(C, T)>)>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    dijkstra(
        (start_cost, start),
        |c, (x, y)| {
            let mut adj = vec![];
            for dx in -1..=1 {
                for dy in -1..=1 {
                    if dx == 0 && dy == 0 || (diagonal && dx != 0 && dy != 0) {
                        continue;
                    }
                    let nx = x + dx;
                    let ny = y + dy;
                    if nx < 0 || ny < 0 {
                        continue;
                    }
                    if let Some(row) = grid.get(ny as usize) {
                        if let Some((cost, _)) = row.get(nx as usize) {
                            adj.push((c + cost, (nx, ny)));
                        }
                    }
                }
            }
            adj
        },
        |_, (x, y)| {
            C::from_isize(
                (((end.0 - x) * (end.0 - x) + (end.1 - y) * (end.1 - y)) as f64)
                    .sqrt()
                    .round() as isize,
            )
        },
        |_, (x, y)| (*x, *y) == end,
    )
    .map(|(c, v)| {
        (
            c,
            v.into_iter()
                .map(|(c, (x, y))| (c, grid[y as usize][x as usize].1.clone()))
                .collect(),
        )
    })
}

#[derive(Debug, Clone, Copy)]
struct Vertex<C, V> {
    cost: C,
    heuristic: C,
    value: V,
}

impl<C: PartialEq + Eq + PartialOrd + Ord, V> PartialEq for Vertex<C, V>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl<C: PartialEq + Eq + PartialOrd + Ord, V> Eq for Vertex<C, V> where
    for<'a> &'a C: Add<&'a C, Output = C>
{
}
impl<C: PartialEq + Eq + PartialOrd + Ord, V> PartialOrd for Vertex<C, V>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<C: PartialEq + Eq + PartialOrd + Ord, V> Ord for Vertex<C, V>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (&self.cost + &self.heuristic)
            .cmp(&(&other.cost + &other.heuristic))
            .reverse()
    }
}

// adjacent must be consistent
pub fn dijkstra<C: PartialEq + Eq + PartialOrd + Ord + Clone, V: PartialEq + Eq + Hash + Clone>(
    (start_cost, start_vertex): (C, V),
    adjacent: impl Fn(&C, &V) -> Vec<(C, V)>,
    heuristic: impl Fn(&C, &V) -> C,
    is_destination: impl Fn(&C, &V) -> bool,
) -> Option<(C, Vec<(C, V)>)>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Vertex {
        heuristic: heuristic(&start_cost, &start_vertex),
        cost: start_cost,
        value: start_vertex.clone(),
    });
    let mut predecessor = HashMap::new();
    while let Some(next) = queue.pop() {
        if visited.contains(&next.value) {
            continue;
        }
        visited.insert(next.value.clone());
        if is_destination(&next.cost, &next.value) {
            let mut current = (next.cost.clone(), next.value.clone());
            let mut path = vec![current.clone()];
            while let Some(predecessor) = predecessor.remove(&current.1) {
                current = predecessor;
                path.push(current.clone());
            }
            path.reverse();
            return Some((next.cost, path));
        }
        let adj = adjacent(&next.cost, &next.value);
        for (cost, vertex) in adj {
            if let Some((c, v)) = predecessor.get_mut(&vertex) {
                if &cost < c {
                    *c = cost.clone();
                    *v = next.value.clone();
                }
            } else {
                predecessor.insert(vertex.clone(), (cost.clone(), next.value.clone()));
            }
            queue.push(Vertex {
                heuristic: heuristic(&cost, &vertex),
                cost,
                value: vertex,
            });
        }
    }
    None
}

pub fn dijkstraa<
    C: PartialEq + Eq + PartialOrd + Ord + Clone + Hash,
    V: PartialEq + Eq + Hash + Clone,
>(
    (start_cost, start_vertex): (C, V),
    adjacent: impl Fn(&C, &V) -> Vec<(C, V)>,
    heuristic: impl Fn(&C, &V) -> C,
) -> HashSet<(C, V)>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    let mut distance_to = HashSet::<(C, V)>::new();
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Vertex {
        heuristic: heuristic(&start_cost, &start_vertex),
        cost: start_cost,
        value: start_vertex.clone(),
    });
    let mut predecessor = HashMap::new();
    while let Some(next) = queue.pop() {
        if visited.contains(&next.value) {
            continue;
        }
        distance_to.insert((next.cost.clone(), next.value.clone()));
        visited.insert(next.value.clone());
        let adj = adjacent(&next.cost, &next.value);
        for (cost, vertex) in adj {
            if let Some((c, v)) = predecessor.get_mut(&vertex) {
                if &cost < c {
                    *c = cost.clone();
                    *v = next.value.clone();
                }
            } else {
                predecessor.insert(vertex.clone(), (cost.clone(), next.value.clone()));
            }
            queue.push(Vertex {
                heuristic: heuristic(&cost, &vertex),
                cost,
                value: vertex,
            });
        }
    }
    distance_to
}
