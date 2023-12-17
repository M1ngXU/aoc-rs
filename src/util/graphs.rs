use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Add,
};

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
    dijkstrao(
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
        |_, _| C::from_isize(0),
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

/// adjacent must be consistent
pub fn dijkstrao<C: PartialEq + Eq + PartialOrd + Ord + Clone, V: PartialEq + Eq + Hash + Clone>(
    (start_cost, start_vertex): (C, V),
    adjacent: impl Fn(&C, &V) -> Vec<(C, V)>,
    heuristic: impl Fn(&C, &V) -> C,
    is_destination: impl Fn(&C, &V) -> bool,
) -> Option<(C, Vec<(C, V)>)>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    dijkstra(
        vec![(start_cost, start_vertex)],
        adjacent,
        heuristic,
        is_destination,
    )
}
/// adjacent must be consistent, heuristic MUST BE the shortest path `cost + heuristic` is the same as `cost`
pub fn dijkstra<C: PartialEq + Eq + PartialOrd + Ord + Clone, V: PartialEq + Eq + Hash + Clone>(
    starts: Vec<(C, V)>,
    adjacent: impl Fn(&C, &V) -> Vec<(C, V)>,
    heuristic: impl Fn(&C, &V) -> C,
    is_destination: impl Fn(&C, &V) -> bool,
) -> Option<(C, Vec<(C, V)>)>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    let mut queue = BinaryHeap::new();
    let mut predecessor = HashMap::new();
    for (start_cost, start_vertex) in &starts {
        predecessor.insert(
            start_vertex.clone(),
            (start_cost.clone(), start_vertex.clone()),
        );
        queue.push(Vertex {
            heuristic: heuristic(&start_cost, &start_vertex),
            cost: start_cost.clone(),
            value: start_vertex.clone(),
        });
    }
    while let Some(next) = queue.pop() {
        if predecessor[&next.value].0 < next.cost {
            continue;
        }
        if is_destination(&next.cost, &next.value) {
            let mut current = (next.cost.clone(), next.value.clone());
            let mut path = vec![current.clone()];
            while let Some(predecessor) = predecessor.remove(&current.1) {
                current = predecessor;
                path.push(current.clone());
                if starts.contains(&current) {
                    break;
                }
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
                } else {
                    continue;
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

pub fn dijkstraao<
    C: PartialEq + Eq + PartialOrd + Ord + Clone + Hash,
    V: PartialEq + Eq + Hash + Clone,
>(
    (start_cost, start_vertex): (C, V),
    adjacent: impl Fn(&C, &V) -> Vec<(C, V)>,
    heuristic: impl Fn(&C, &V) -> C,
) -> HashMap<V, (C, V)>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    dijkstraa(vec![(start_cost, start_vertex)], adjacent, heuristic)
}
pub fn dijkstraa<
    C: PartialEq + Eq + PartialOrd + Ord + Clone + Hash,
    V: PartialEq + Eq + Hash + Clone,
>(
    starts: Vec<(C, V)>,
    adjacent: impl Fn(&C, &V) -> Vec<(C, V)>,
    heuristic: impl Fn(&C, &V) -> C,
) -> HashMap<V, (C, V)>
where
    for<'a> &'a C: Add<&'a C, Output = C>,
{
    let mut queue = BinaryHeap::new();
    let mut predecessor = HashMap::new();
    for (start_cost, start_vertex) in &starts {
        predecessor.insert(
            start_vertex.clone(),
            (start_cost.clone(), start_vertex.clone()),
        );
        queue.push(Vertex {
            heuristic: heuristic(&start_cost, &start_vertex),
            cost: start_cost.clone(),
            value: start_vertex.clone(),
        });
    }
    while let Some(next) = queue.pop() {
        if predecessor[&next.value].0 < next.cost {
            continue;
        }
        let adj = adjacent(&next.cost, &next.value);
        for (cost, vertex) in adj {
            if let Some((c, v)) = predecessor.get_mut(&vertex) {
                if &cost < c {
                    *c = cost.clone();
                    *v = next.value.clone();
                } else {
                    continue;
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

    predecessor
}
