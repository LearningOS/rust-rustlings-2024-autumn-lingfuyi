/*
    graph
    This problem requires you to implement a basic graph functio
实现了一个基本的无向图数据结构，主要功能包括：

1、节点管理：支持添加节点、检查节点是否存在、获取所有节点。
2、边管理：支持添加无向边、获取所有边。
3、邻接表存储：使用 HashMap 存储图的邻接表，每个节点对应一个邻居列表，列表中的元素是元组 (邻居节点, 边的权重)。
4、错误处理：定义了 NodeNotInGraph 错误类型，用于处理访问不存在的节点的情况。
###通过 Graph trait 和 UndirectedGraph 结构体的实现，代码提供了一个灵活且可扩展的图数据结构，适用于需要处理无向图的场景。
*/

use std::collections::{HashMap, HashSet};
use std::fmt; //模块用于格式化输出。
#[derive(Debug, Clone)]
pub struct NodeNotInGraph; //自定义错误类型，用于表示访问图中不存在的节点时发生的错误
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
//无向图结构，使用 HashMap 来存储邻接表。
pub struct UndirectedGraph {
    //hash键值是节点名称（String），值是该节点的邻居列表，每个邻居是一个元组 (String, i32)，表示邻居节点名称和边的权重。
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    //返回可变的邻接表
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    //返回不可变的邻接表
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    //向图中添加一条边，边的两个端点是 node1 和 node2，权重为 weight。
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        let (node1, node2, weight) = edge;
        // 确保节点存在
        self.add_node(node1);
        self.add_node(node2);

        let adj = self.adjacency_table_mutable();
        // 添加双向边
        adj.get_mut(node1)
            .unwrap()
            .push((node2.to_string(), weight));
        adj.get_mut(node2)
            .unwrap()
            .push((node1.to_string(), weight));
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    //增加节点，返回是否新增了节点。
    fn add_node(&mut self, node: &str) -> bool {
        let exists = self.contains(node);
        if !exists {
            let adj = self.adjacency_table_mutable();
            adj.insert(node.to_string(), Vec::new());
        }
        !exists // 返回是否新增了节点
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {}
    //检查是否存在某个节点。
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    //返回图中所有边的列表。
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    //测试用例用于验证 add_edge() 方法的正确性。它创建了一个无向图，添加了几条边，然后检查所有预期的边是否都存在于图中。
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}
