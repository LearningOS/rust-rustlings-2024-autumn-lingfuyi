/*
    bfs
    This problem requires you to implement a basic BFS algorithm
*/

use std::collections::VecDeque;

// Define a graph
struct Graph {
    adj: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with n vertices
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }

    // Add an edge to the graph
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }

    // 在图上执行广度优先搜索，返回访问节点的顺序
    /*
    adj: Vec<Vec<usize>> 表示图的邻接表存储，每个索引位置对应一个节点，存储该节点的邻接节点列表
    queue: VecDeque 作为BFS的先进先出队列
    visited: Vec<bool> 记录节点是否被访问过
    */
    fn bfs_with_return(&self, start: usize) -> Vec<usize> {
        let mut visit_order = vec![]; // 记录访问顺序
        let mut queue = VecDeque::new(); // BFS队列
        let mut visited = vec![false; self.adj.len()]; // 访问标记数组

        // 初始化队列和访问标记
        queue.push_back(start);
        visited[start] = true;

        while let Some(node) = queue.pop_front() {
            visit_order.push(node); // 记录当前节点

            // 遍历邻接节点
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back(neighbor);
                }
            }
        }

        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //完整图的遍历顺序
    fn test_bfs_all_nodes_visited() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 4);
        graph.add_edge(1, 2);
        graph.add_edge(1, 3);
        graph.add_edge(1, 4);
        graph.add_edge(2, 3);
        graph.add_edge(3, 4);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 4, 2, 3]);
    }

    #[test]
    //不同起始点的遍历顺序
    fn test_bfs_different_start() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visited_order = graph.bfs_with_return(2);
        assert_eq!(visited_order, vec![2, 1, 0]);
    }

    #[test]
    //有环的图的遍历顺序
    fn test_bfs_with_cycle() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);
        graph.add_edge(2, 0);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0, 1, 2]);
    }

    #[test]
    //单点图的边界情况
    fn test_bfs_single_node() {
        let mut graph = Graph::new(1);

        let visited_order = graph.bfs_with_return(0);
        assert_eq!(visited_order, vec![0]);
    }
}
