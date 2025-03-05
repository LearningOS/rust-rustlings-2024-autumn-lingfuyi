/*
    dfs
    This problem requires you to implement a basic DFS traversal
    实现的图的深度优先搜索（DFS）的示例
*/

use std::collections::HashSet;

struct Graph {
    adj: Vec<Vec<usize>>,
}
//adj是一个二维向量，用于存储图的邻接列表。邻接列表中的每个元素是一个向量，包含与该节点相连的所有节点的列表。
impl Graph {
    fn new(n: usize) -> Self {
        Graph {
            adj: vec![vec![]; n],
        }
    }
    //在图中添加一条边，边连接src和dest两个节点，这里假设图是无向的，所以需要在两个节点的邻接列表中都添加对方。
    fn add_edge(&mut self, src: usize, dest: usize) {
        self.adj[src].push(dest);
        self.adj[dest].push(src);
    }
    //递归辅助函数，用于执行实际的深度优先搜索。它接受一个起始节点v，一个可变引用的HashSet用于记录访问过的节点，一个可变引用的Vec用于记录访问节点的顺序。
    fn dfs_util(&self, v: usize, visited: &mut HashSet<usize>, visit_order: &mut Vec<usize>) {
        // 将当前节点标记为已访问
        visited.insert(v);
        // 将当前节点添加到访问顺序列表中
        visit_order.push(v);
        // 递归地访问所有未访问过的相邻节点
        for &neighbor in &self.adj[v] {
            if !visited.contains(&neighbor) {
                self.dfs_util(neighbor, visited, visit_order);
            }
        }
    }

    // Perform a depth-first search on the graph, return the order of visited nodes
    fn dfs(&self, start: usize) -> Vec<usize> {
        let mut visited = HashSet::new();
        let mut visit_order = Vec::new();
        self.dfs_util(start, &mut visited, &mut visit_order);
        visit_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 测试一个简单的无环图。
    fn test_dfs_simple() {
        let mut graph = Graph::new(3);
        graph.add_edge(0, 1);
        graph.add_edge(1, 2);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
    }

    #[test]
    // 测试一个包含环的无向图
    fn test_dfs_with_cycle() {
        let mut graph = Graph::new(4);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(1, 2);
        graph.add_edge(2, 3);
        graph.add_edge(3, 3);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2, 3]);
    }

    #[test]
    //测试一个由2个不连通子图组成的图
    fn test_dfs_disconnected_graph() {
        let mut graph = Graph::new(5);
        graph.add_edge(0, 1);
        graph.add_edge(0, 2);
        graph.add_edge(3, 4);

        let visit_order = graph.dfs(0);
        assert_eq!(visit_order, vec![0, 1, 2]);
        let visit_order_disconnected = graph.dfs(3);
        assert_eq!(visit_order_disconnected, vec![3, 4]);
    }
}
