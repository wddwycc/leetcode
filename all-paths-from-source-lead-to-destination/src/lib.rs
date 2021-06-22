#[derive(Clone, PartialEq, Eq, Debug)]
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
        // NOTE: Ensure destination has no outgoing edge
        if adjacency_list[destination].len() > 0 {
            return false;
        }

        let mut node_states = vec![NodeState::Idle; n];
        let mut stack = vec![];
        stack.push(source);
        while let Some(u) = stack.pop() {
            match node_states[u] {
                NodeState::Idle => {
                    node_states[u] = NodeState::Checking;
                    if u == destination {
                        for i in 0..n {
                            if node_states[i] == NodeState::Checking {
                                node_states[i] = NodeState::Checked;
                            }
                        }
                        continue;
                    }
                    let outging_edges = &adjacency_list[u];
                    if outging_edges.len() == 0 {
                        return false;
                    }
                    for &v in outging_edges {
                        if u == v {
                            return false;
                        }
                        stack.push(v);
                    }
                }
                NodeState::Checking => return false,
                NodeState::Checked => {
                    for i in 0..n {
                        if node_states[i] == NodeState::Checking {
                            node_states[i] = NodeState::Checked;
                        }
                    }
                }
            }
        }
        true
    }
}
