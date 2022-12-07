#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    name: String,
    size: u32,
    node_type: String,
    children: Vec<Node>,
}

fn parse_input(text: String) -> Node {
       let mut stack: Vec<Node> = Vec::new();

       let line_it = text.split("\n");

       for line in line_it {
           if line.len() == 0 {
               continue;
           } else if line == "$ cd .." {
               let current = stack.pop().unwrap();
               stack.last_mut().unwrap().children.push(current);
           } else if line.starts_with("$ cd ") {
               let name: String = line.chars().skip(5).collect();
               stack.push(Node{
                   name: name,
                   size: 0,
                   node_type: String::from("dir"),
                   children: Vec::new()
               });
           } else if line == "$ ls" || line.starts_with("dir ") {
           } else {
               let mut file_record_it = line.split(" ");
               let size: u32 = file_record_it.next().unwrap().parse().unwrap();
               let name = String::from(file_record_it.next().unwrap());
               let new_node = Node{
                   name: name,
                   size: size,
                   node_type: String::from("file"),
                   children: Vec::new()
               };
               stack.last_mut().unwrap().children.push(new_node);
           }
       }

       while stack.len() > 1 {
           let current = stack.pop().unwrap();
           stack.last_mut().unwrap().children.push(current);
       }

       stack.pop().unwrap()
}

fn compute_sum_size(node: &mut Node) {
    for mut child in &mut node.children {
        compute_sum_size(&mut child);
    }
    for child in &node.children {
        node.size += child.size;
    }
}

fn find_smallest_child_bigger_or_equal(node: &Node, limit: u32) -> u32 {
    let mut smallest_child_size = u32::MAX;

    for child in &node.children {
        let subtree_result = find_smallest_child_bigger_or_equal(&child, limit);
        if subtree_result < smallest_child_size {
            smallest_child_size = subtree_result;
        } else if child.size < smallest_child_size && child.node_type == "dir" && child.size >= limit {
            smallest_child_size = child.size;
        }
    }

    smallest_child_size
}

fn main() {
    let input = std::fs::read_to_string("../input.txt").expect("cannot open input file");
    let mut tree = parse_input(input);
    compute_sum_size(&mut tree);
    let total_disk_size: u32 = 70000000;
    let update_size: u32 = 30000000;
    let required_freed_space = tree.size - (total_disk_size - update_size);
    let result = find_smallest_child_bigger_or_equal(&tree, required_freed_space);
    println!("size of smallest child to delete: {}", result);
}

