use std::collections::HashSet;

fn dfs(node: usize, visited: &mut HashSet<usize>, subgraph: &Vec<Vec<u8>>) {
    visited.insert(node);
    for (neighbor, &is_connected) in subgraph[node].iter().enumerate() {
        if is_connected == 1 && !visited.contains(&neighbor) {
            dfs(neighbor, visited, subgraph);
        }
    }
}

pub fn are_strokes_linked(strokes: &Vec<usize>, intersection_matrix: &Vec<Vec<u8>>) -> bool {
    if strokes.is_empty() {
        return false;
    }

    // Create a subgraph for only the strokes of interest
    let mut subgraph: Vec<Vec<u8>> = vec![vec![0; strokes.len()]; strokes.len()];
    for (i, &stroke1) in strokes.iter().enumerate() {
        for (j, &stroke2) in strokes.iter().enumerate() {
            subgraph[i][j] = intersection_matrix[stroke1][stroke2];
        }
    }

    let mut visited: HashSet<usize> = HashSet::new();
    dfs(0, &mut visited, &subgraph);

    visited.len() == strokes.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_strokes_linked_single() {
        assert!(are_strokes_linked(&vec![0], &vec![vec![1]]));
    }

    #[test]
    fn test_are_strokes_linked_pair_connected() {
        assert!(are_strokes_linked(
            &vec![0, 1],
            &vec![vec![1, 1], vec![1, 1]]
        ));
    }

    #[test]
    fn test_are_strokes_linked_pair_disconnected() {
        assert!(!are_strokes_linked(
            &vec![0, 1],
            &vec![vec![1, 0], vec![0, 1]]
        ));
    }

    #[test]
    fn test_are_strokes_linked_group_disconnected() {
        assert!(!are_strokes_linked(
            &vec![0, 1, 2, 3],
            &vec![
                vec![1, 1, 0, 0],
                vec![1, 1, 0, 0],
                vec![0, 0, 1, 1],
                vec![0, 0, 1, 1],
            ]
        ));
    }

    #[test]
    fn test_are_strokes_linked_specific_case() {
        assert!(!are_strokes_linked(
            &vec![0, 3],
            &vec![
                vec![1, 1, 1, 0],
                vec![1, 1, 0, 1],
                vec![1, 0, 1, 1],
                vec![0, 1, 1, 1],
            ]
        ));
    }
}
