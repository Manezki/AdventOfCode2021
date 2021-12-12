use std::{fs, collections::{HashMap, HashSet}};

fn traverse_nodes(
    adjacency_mtx: &Vec<Vec<bool>>,
    node: usize,
    visited_minor_nodes: &HashSet<usize>,
    minor_nodes: HashSet<usize>,
    end_node: usize
) -> Vec<Vec<usize>> {

    let neighbours = &adjacency_mtx[node];
    let mut passable_visited_minors = visited_minor_nodes.clone();

    if minor_nodes.contains(&node) {
        passable_visited_minors.insert(node);
    }

    let mut paths = vec![];

    if node == end_node {
        paths.push(vec![node]);
        return paths;
    }

    for (neighbour, access) in neighbours.iter().enumerate() {
        // Skip non adjacence nodes
        if !access {
            continue;
        } else if passable_visited_minors.contains(&neighbour) {
            continue;
        }

        let child_paths = traverse_nodes(adjacency_mtx, neighbour, &passable_visited_minors, minor_nodes.clone(), end_node);

        for path in child_paths {
            let mut extended_path = vec![node];
            extended_path.extend(path);
            paths.push(extended_path);
        }        
    }

    return paths;
}

fn main() {
    let file_content = fs::read_to_string("src\\day_12_input.txt").expect("Something went wrong reading the file");
    let edges = file_content.split("\r\n").collect::<Vec<&str>>();

    let mut node_names: HashMap<&str, usize> = HashMap::new();

    // Add all node node_names
    for edge in &edges {
        let mut splitted = edge.split("-");
        let (left, right) = (splitted.next().unwrap(), splitted.next().unwrap());

        if !node_names.contains_key(left) {
            node_names.insert(left, node_names.len());
        }

        if !node_names.contains_key(right) {
            node_names.insert(right, node_names.len());
        }
    }

    let mut adjacency: Vec<Vec<bool>> = vec![vec![false; node_names.len()]; node_names.len()];

    for edge in edges {
        let mut splitted = edge.split("-");
        let (left, right) = (splitted.next().unwrap(), splitted.next().unwrap());

        adjacency[node_names.get(left).unwrap().clone()][node_names.get(right).unwrap().clone()] = true;
        adjacency[node_names.get(right).unwrap().clone()][node_names.get(left).unwrap().clone()] = true;
    }

    let minor_nodes: HashSet<usize> = node_names.iter().filter(|t| **t.0 == t.0.to_lowercase()).map(|(_, v)| *v).collect();

    let paths = traverse_nodes(&adjacency, *node_names.get("start").unwrap(), &HashSet::new(), minor_nodes.clone(), *node_names.get("end").unwrap());

    // let mut recoded_paths = vec![];
    // let node_name_lookup: HashMap<usize, &str> = HashMap::from(node_names.iter().map(|(k, v)| (*v, *k)).collect());

    // for path in paths {

    // }

    // adjacency.iter().for_each(|r| println!("{:?}", r));
    // println!("{:?}", minor_nodes);
    // println!("{:?}", node_names);
    println!("{:?}", paths.len());
}
