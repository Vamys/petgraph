use crate::{prelude::UnGraph, algo::{dijkstra, min_spanning_tree}, Graph, graph::Node, Undirected, visit::{GraphBase, NodeCount, NodeIndexable, IntoNodeIdentifiers}};
use crate::dot::{Dot, Config};
use crate::data::FromElements;
use std::collections::HashSet;
use std::fmt::Debug;

use std::hash::Hash;

use petgraph::visit::{EdgeRef, IntoEdges, VisitMap, Visitable};
fn Lex_BFS_order<G>(graph:G)
where
    G: Visitable+ IntoEdges + NodeCount + NodeIndexable + IntoNodeIdentifiers,
    G::NodeId: Eq + Hash,
     <G as GraphBase>::NodeId: Debug //temporary
    {
    let mut list= vec![HashSet::new()];
    let mut order = Vec::new();
    for node in graph.node_identifiers(){
        list[0].insert(node);
    }
    println!("{:?}", list);
    while list.len() >0 {
        let curr_node= list[0].iter().next().unwrap().clone();
        list[0].remove(&curr_node);
        if list[0].len() == 0 {
            list.remove(0);
        }
        order.push(curr_node);
        println!("curr node: {:?}", curr_node);
        let mut i = 0;
        while i < list.len(){
            let neighbours = graph.neighbors(curr_node);
            let mut new_set = HashSet::new();
            neighbours.for_each(|neighbour|{
                if list[i].contains(&neighbour){
                    list[i].remove(&neighbour);
                    new_set.insert(neighbour);
                }
            });
            
            if new_set.len() > 0 {
                list.insert(i, new_set);
                i+=1;
            }
            if list[i].len() == 0 {
                list.remove(i);
                i-=1;
            }
            println!("after changes: {:?}", list);
            i+=1;
        }
    }
    println!("{:?}", list);
    println!("{:?}", order);
    
}
// fn main(){
//     let g = UnGraph::<i32, ()>::from_edges(&[
//         (0,5),(1,7),(2,5),(2,7),(3,7),(3,6),(4,6),(4,7),(5,6),(5,7)]);
//     Lex_BFS_order(&g);

//     // let node_map = dijkstra(&g, 1.into(), Some(4.into()), |_| 1);
//     // let mst = UnGraph::<_,_>::from_elements(min_spanning_tree(&g));
//     // println!("{:?}", Dot::with_config(&mst,&[Config::EdgeNoLabel]));  

// }