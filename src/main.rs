use aws_sdk_s3::{Client, Error};
use aws_config::meta::region::RegionProviderChain;
use futures::StreamExt;
use std::cell::RefCell;
use std::io;
use std::rc::Rc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up AWS configuration
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;

    // Create S3 client
    let client = Client::new(&shared_config);

    // Specify the bucket name
    let bucket_name = "input-bucket-code-editor";

    // List objects in the bucket
    let mut paginator = client
        .list_objects_v2()
        .bucket(bucket_name)
        .into_paginator()
        .send();

    println!("Objects in bucket '{}':", bucket_name);
    let mut files = vec![];
    // Iterate through the pages of objects in the S3 bucket
    while let Some(page) = paginator.next().await {
        match page {
            Ok(output) => {
                // Check if there are objects and print them
                if let Some(objects) = output.contents() {
                    for object in objects {
                        // let key =
                        files.push(object.key().unwrap_or("<no key>").to_string().clone());
                        // println!("key: {}", key);
                        // files.push(key);
                    }
                } else {
                    println!("No objects found.");
                }
            }
            Err(err) => {
                eprintln!("Error listing objects: {}", err);
            }
        }
    }
    tree(files);
    Ok(())
}



// Define a tree node
#[derive(Debug)]
struct Node {
    value: String,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    // Create a new node with a given value
    fn new(value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            children: vec![],
        }))
    }
    // Print the tree recursively
    fn print_tree(&self, indent: usize, is_last: bool) {
        // Print the current node's value with the appropriate indentation and prefix
        if is_last {
            println!("{:indent$}└── {}", "", self.value, indent = indent);
        } else {
            println!("{:indent$}├── {}", "", self.value, indent = indent);
        }

        // Recursively print each child node with updated indent
        for (i, child) in self.children.iter().enumerate() {
            child.borrow().print_tree(indent + 4, i == self.children.len() - 1);
        }
    }
}

fn tree(files: Vec<String>) {


    // Parse the first line as the number of paths
    let len = files.len();

    // Create the root node
    let root = Node::new("root".to_string());

    // Loop to read each path
    for i in 0..len {

        // Split input by '/' and create a path as a vector of strings
        let path: Vec<String> = files[i]
            .trim() // Trim any newlines or spaces
            .split('/')
            .map(|s| s.to_string()) // Convert each &str to String
            .collect();

        // Call add_node to add the path to the tree
        add_node(&root, &path, 0);
    }

    // Print the root node for verification (just its value in this case)
    println!("Root node: {}", root.borrow().value);
    root.borrow().print_tree(0,true);

    // You can print the whole tree if needed (not implemented here)
}

fn add_node(
    node: &Rc<RefCell<Node>>,
    path: &[String],  // Using a slice instead of Vec
    i: usize,         // Use usize instead of u32 for proper indexing
) {
    if path.len() == i {
        return; // Base case: we have reached the end of the path
    }

    let obj = &path[i]; // We already know that path[i] is valid because of the check above

    {
        // Borrow the node immutably to check its children
        let children = &node.borrow().children;
        for child in children.iter() {
            if obj == &child.borrow().value {
                // If a matching child is found, recurse into it
                add_node(child, path, i + 1);
                return;
            }
        }
    } // Immutable borrow ends here, allowing mutable borrow after this block.

    // If no matching child was found, create a new node and add it to the children
    let current = Node::new(obj.clone());
    node.borrow_mut().children.push(current.clone());

    // Recursively add the rest of the path into the new node
    add_node(&current, path, i + 1);
}
