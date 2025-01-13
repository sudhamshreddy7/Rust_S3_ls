use aws_sdk_s3::{Client, Error};
use aws_config::meta::region::RegionProviderChain;
use futures::StreamExt;
use std::cell::RefCell;
use std::{env, io};
use std::rc::Rc;

fn help(){
    // rusty_s3_ls --r <region> --b <bucket_name>
    println!("  RRR   U   U  SSS  TTTTT ");
    println!(" R   R  U   U S       T   ");
    println!(" RRRR   U   U  SSS    T   ");
    println!(" R  R   U   U     S   T   ");
    println!(" R   R  UUUUU  SSS    T   ");
    println!();
    println!("usage: rusty_s3_ls [OPERATION] 🦀");
    println!();
    println!("OPERATION can be one of the following:");
    println!("rusty_s3_ls --r <region> --b <bucket_name>");
    println!("for help\n rusty_s3_ls --h");
}
fn invalid(){
    println!("Invalid commands\nplease follow below format:");
    help();

}
#[tokio::main]
async fn main() -> Result<(), Error> {
    // Set up AWS configuration
    let mut region_provider = RegionProviderChain::default_provider().or_else("us-east-1");


    // Create S3 client


    // Specify the bucket name
    // let bucket_name = "input-bucket-code-editor";
    let args: Vec<String> = env::args().collect();
    if args.len()==2 && args[1]=="--h"{
        help();
        return Ok(());
    }
    if args.len()!=5 || args[1]!="--r" || args[3]!="--b" {
        invalid();
        return Ok(());
    }
    let region: &'static str = Box::leak(args[2].clone().into_boxed_str());
    region_provider = RegionProviderChain::first_try(region).or_else("us-east-1");
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let bucket_name = args[4].to_string().clone();
    let client = Client::new(&shared_config);

    // List objects in the bucket

    let mut paginator = client
        .list_objects_v2()
        .bucket(bucket_name.clone())
        .into_paginator()
        .send();

    println!("Objects in bucket '{}':", bucket_name);
    let mut files = vec![];
    while let Some(page) = paginator.next().await {
        match page {
            Ok(output) => {
                if let Some(objects) = output.contents() {
                    for object in objects {
                        // Collect keys while ensuring no extra whitespace
                        files.push(object.key().unwrap_or("<no key>").trim().to_string());
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

#[derive(Debug)]
struct Node {
    value: String,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            value,
            children: vec![],
        }))
    }

    fn print_tree(&self, indent: usize, is_last: bool) {
        // Print the node with consistent formatting
        let prefix = if is_last { "└── " } else { "├── " };
        println!("{:indent$}{}{}", "", prefix, self.value, indent = indent);

        // Recursively print child nodes
        for (i, child) in self.children.iter().enumerate() {
            child.borrow().print_tree(indent + 4, i == self.children.len() - 1);
        }
    }
}

fn tree(files: Vec<String>) {
    let root = Node::new("root".to_string());

    for path in files {
        let path_parts: Vec<String> = path
            .split('/')
            .map(|s| s.trim().to_string())
            .collect();
        add_node(&root, &path_parts, 0);
    }

    println!("\nDirectory Structure:");
    root.borrow().print_tree(0, true);
}

fn add_node(node: &Rc<RefCell<Node>>, path: &[String], i: usize) {
    if i == path.len() {
        return;
    }

    let obj = &path[i];
    {
        let children = &node.borrow().children;
        for child in children.iter() {
            if &child.borrow().value == obj {
                add_node(child, path, i + 1);
                return;
            }
        }
    }

    let current = Node::new(obj.clone());
    node.borrow_mut().children.push(current.clone());
    add_node(&current, path, i + 1);
}
