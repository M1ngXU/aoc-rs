#![feature(iter_map_windows, iter_from_coroutine)]

use aoc_rs::prelude::*;

fn solve() {
    #[cfg(feature = "ex")]
    compile_error!("Example does not work.");
    let pi = pi!(id);
    let mut grid = parse_grid(
        pi,
        |_, _, c| c != '#',
        |_, _, c| c == 'S',
        |_, _, _| false,
        false,
    );
    grid.graph.grow(3, 1, grid.width - 1);
    let dist = dijkstra::<_, _, isize>(&grid.graph, grid.start.unwrap().0, None, ec);
    let [y64, y65, y195] =
        [64, 65, 195].map(|c| dist.values().filter(|&&x| x <= c && x % 2 == c % 2).count());
    println!("Part 1: {}", y64);
    let lhs = Matrix3::new(4, -2, 1, 1, -1, 1, 0, 0, 1).map(|x| x as f64);
    let rhs = Vector3::new(y195, y64, y65).map(|x| x as f64);
    let coef = lhs.full_piv_lu().solve(&rhs).unwrap();
    let x = ((26501365 - grid.start.unwrap().1) / grid.width) as f64;
    let p2 = x * x * coef[0] + x * coef[1] + coef[2];
    println!("Part 2: {p2}");
}

fn main() {
    solve();
}
