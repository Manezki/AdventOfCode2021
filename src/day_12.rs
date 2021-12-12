use std::{fs, collections::{HashMap, HashSet}};

fn traverse_nodes(
    adjacency_mtx: &Vec<Vec<bool>>,
    node: usize,
    minor_nodes: HashMap<usize, usize>,
    end_node: usize
) -> Option<Vec<Vec<usize>>> {

    let neighbours = &adjacency_mtx[node];
    let mut passable_visited_minors = minor_nodes;

    if passable_visited_minors.contains_key(&node) {
        
        let updated_value = *passable_visited_minors.get(&node).unwrap() - (1 as usize);
        passable_visited_minors.insert(node, updated_value);
    }

    let mut paths = vec![];

    if node == end_node {
        paths.push(vec![node]);
        return Some(paths);
    }

    for (neighbour, access) in neighbours.iter().enumerate() {
        // Skip non adjacence nodes
        if !access {
            continue;
        } else if passable_visited_minors.contains_key(&neighbour) {
            if *passable_visited_minors.get(&neighbour).unwrap() == 0 {
                continue;
            }
        }

        let child_paths = traverse_nodes(adjacency_mtx, neighbour, passable_visited_minors.clone(), end_node);

        if child_paths.is_none() {
            continue;
        }

        for path in child_paths.unwrap() {
            let mut extended_path = vec![node];
            extended_path.extend(path);
            paths.push(extended_path);
        }        
    }

    if paths.len() == 0 {
        return None;
    }

    return Some(paths);
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

    let mut total_paths: Vec<Vec<usize>> = vec![];

    for key in node_names.keys() {
        // Rules prohibit start and end twice
        if (*key == "start") | (*key == "end") {
            continue
        // Unlimited visits to large caves
        } else if **key == key.to_uppercase() {
            continue;
        }

        let minor_nodes: HashMap<usize, usize> = node_names.iter()
            .filter(|t| **t.0 == t.0.to_lowercase())
            .map(|(k, v)| {
                if k == key {
                    return (*v, 2 as usize);
                } else {
                    return (*v, 1 as usize);
                }
        }).collect::<HashMap<usize, usize>>();

        let paths = traverse_nodes(&adjacency, *node_names.get("start").unwrap(), minor_nodes.clone(), *node_names.get("end").unwrap());

        if !paths.is_none() {
            total_paths.extend(paths.unwrap());
        }

        // break;
    }

    // adjacency.iter().for_each(|r| println!("{:?}", r));
    // println!("{:?}", minor_nodes);
    // println!("{:?}", node_names);
    let unique_total_paths: HashSet<&Vec<usize>> = total_paths.iter().collect(); 
    // total_paths.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());
    println!("{:?}", total_paths);
    println!("{:?}", unique_total_paths.len());
}
