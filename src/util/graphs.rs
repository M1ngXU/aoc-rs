use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Clone, Copy)]
struct Vertex<C, V> {
    cost: C,
    value: V,
}

impl<C: PartialEq + Eq + PartialOrd + Ord, V> PartialEq for Vertex<C, V> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}
impl<C: PartialEq + Eq + PartialOrd + Ord, V> Eq for Vertex<C, V> {}
impl<C: PartialEq + Eq + PartialOrd + Ord, V> PartialOrd for Vertex<C, V> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<C: PartialEq + Eq + PartialOrd + Ord, V> Ord for Vertex<C, V> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

// adjacent must be consistent
pub fn dijkstra<C: PartialEq + Eq + PartialOrd + Ord + Clone, V: PartialEq + Eq + Hash + Clone>(
    (start_cost, start_vertex): (C, V),
    adjacent: impl Fn(&C, &V) -> Vec<(C, V)>,
    is_destination: impl Fn((&C, &V)) -> bool,
) -> Option<Vec<V>> {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    queue.push(Vertex {
        cost: start_cost,
        value: start_vertex.clone(),
    });
    let mut predecessor = HashMap::new();
    while let Some(next) = queue.pop() {
        if visited.contains(&next.value) {
            continue;
        }
        visited.insert(next.value.clone());
        if is_destination((&next.cost, &next.value)) {
            let mut path = vec![];
            let mut current = next.value;
            while let Some((_, predecessor)) = predecessor.remove(&current) {
                if predecessor != start_vertex {
                    path.push(current.clone());
                }
                current = predecessor;
            }
            path.reverse();
            return Some(path);
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
                cost,
                value: vertex,
            });
        }
    }
    None
}
