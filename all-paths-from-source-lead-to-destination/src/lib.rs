#[derive(Clone)]
pub enum NodeState {
    Idle,
    Checking,
    Checked,
}

pub struct Solution;
impl Solution {
    pub fn leads_to_destination(
        n: i32,
        edges: Vec<Vec<i32>>,
        source: i32,
        destination: i32,
    ) -> bool {
        let n = n as usize;
        let source = source as usize;
        let destination = destination as usize;
        let adjacency_list = {
            let mut res = vec![vec![]; n];
            for edge in &edges {
                let u = edge[0] as usize;
                let v = edge[1] as usize;
                res[u].push(v);
            }
            res
        };
        let mut states = vec![NodeState::Idle; n];
        Self::dfs(&adjacency_list, destination, source, &mut states)
    }

    fn dfs(
        adjacency_list: &[Vec<usize>],
        destination: usize,
        u: usize,
        states: &mut Vec<NodeState>,
    ) -> bool {
        match states[u] {
            NodeState::Idle => {
                if adjacency_list[u].len() == 0 {
                    return u == destination;
                }
                states[u] = NodeState::Checking;
                for &v in &adjacency_list[u] {
                    if !Self::dfs(adjacency_list, destination, v, states) {
                        return false;
                    }
                }
                states[u] = NodeState::Checked;
                return true;
            }
            NodeState::Checking => false,
            NodeState::Checked => true,
        }
    }
}
