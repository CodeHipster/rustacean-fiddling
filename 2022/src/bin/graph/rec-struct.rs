use std::collections::HashMap;
#[derive(Debug)]
struct Node {
    val: u8,
}

fn main() {
    let mut n1 = Node { val: 55 };
    let mut n2 = Node { val: 66 };
    let mut n3 = Node { val: 99 };

    let mut node_map: HashMap<u8, Node> = HashMap::new();
    let mut relations: HashMap<u8, Vec<u8>> = HashMap::new();

    node_map.insert(1, n1);
    node_map.insert(2, n2);
    node_map.insert(3, n3);

    relations.insert(1, vec![2, 3]);
    relations.insert(2, vec![1, 3]);
    relations.insert(3, vec![1, 2]);

    for (k, node) in &node_map {
        let rels = relations.get(&k).unwrap();
        let things = rels
            .iter()
            .map(|rel| node_map.get(rel).unwrap())
            .collect::<Vec<_>>();
        println!("node: {:?} with id: {}, references: {:?}", node, k, things)
    }
}
