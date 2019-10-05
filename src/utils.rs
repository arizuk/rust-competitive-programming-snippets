#[snippet = "next_positions"]
pub fn next_positions(x: usize, y: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let moves = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut pos = vec![];
    for (dx, dy) in moves {
        let nx = dx + x as i64;
        let ny = dy + y as i64;
        if nx >= 0 && nx < h as i64 && ny >= 0 && ny < w as i64 {
            pos.push((nx as usize, ny as usize))
        }
    }
    pos
}

#[test]
fn test_next_positions() {
    assert_eq!(next_positions(0, 0, 2, 2), [(0, 1), (1, 0)]);
    assert_eq!(next_positions(1, 1, 2, 3), [(1, 2), (1, 0), (0, 1)]);
    assert_eq!(next_positions(1, 1, 3, 3), [(1, 2), (1, 0), (2, 1), (0, 1)]);
}
