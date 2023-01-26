// use crate::resource_loader;

pub fn part_one() {

    // Todo: Change basic implementation => Moved value on root.


    // let lines = resource_loader::load_resource("day07-data.txt");
    // let root = Node {
    //     name : "/".to_string(),
    //     size : 0,
    //     childs : vec![],
    //     parent : None
    // };

    // let mut current_node = root;

    // for line in lines {

    //     let tokens: Vec<&str> = line.split(' ').collect();

    //     if tokens[0] == "$" {

    //         if tokens[1] == "cd" {

    //             if tokens[2] == "/" {

    //                 current_node = root;

    //             } else if tokens[2] == ".." {

    //                 current_node = *current_node.parent.unwrap();

    //             } else {
                    
    //                 for child in current_node.childs {

    //                     if child.name == tokens[2] {

    //                         current_node = child;
    //                         break;

    //                     }

    //                 }

    //             }

    //         } else if tokens[1] == "ls" {
    //         }

    //     } else if tokens[0] == "dir" {

    //         current_node.childs.push(Node {
    //             name : tokens[1].to_string(),
    //             size : 0,
    //             childs : Vec::new(),
    //             parent : Option::Some(Box::new(current_node))
    //         })

    //     } else {

    //         // Check if child already exist

    //         current_node.childs.push(Node {
    //             name :  tokens[1].to_string(),
    //             size :  tokens[0].parse::<i32>().unwrap(),
    //             childs : Vec::new(),
    //             parent : Option::Some(Box::new(current_node))
    //         })
    //     }
        

    // }

}

// struct Node {
//    name: String,
//    size: i32,
//    childs: Vec<Node>,
//    parent: Option<Box<Node>> 
// }


pub fn part_two() {

}