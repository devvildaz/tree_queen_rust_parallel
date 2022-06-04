use std::option::*;
use std::time;
use std::thread;
use std::sync::{ Arc, Mutex };
use std::sync::mpsc;

struct TreeNode {
    previous: Option<Arc<Mutex<TreeNode>>>,
    col: i32,
    row: i32,
    children: Vec<Arc<Mutex<TreeNode>>>
}

impl TreeNode {
    pub fn new(col: i32, row: i32) -> TreeNode{
        return TreeNode {
            previous: None,
            col: col,
            row: row,
            children: Vec::new()
        };
    }
    pub fn add_child(&mut self ,node: Arc<Mutex<TreeNode>>) {
        self.children.push(node);
    }
}

fn check_position(node: Arc<Mutex<TreeNode>>, pair: (i32,i32)) -> bool{
    let mut pointer = node;
    let (col ,row) = pair;
    loop {
        let col_p : i32 = pointer.lock().unwrap().col;
        let row_p : i32 = pointer.lock().unwrap().row;
        
        if col_p == 0 || row_p == 0 {
            break;
        }

        if check_position_i32((col, row),(col_p, row_p)){
            let new_p = match pointer.lock().unwrap().previous {
                Some(ref x) => 
                    Arc::clone(x)
                ,
                None => break
            };
            pointer = new_p;
        } else {
            return false;
        }

    }

    return true;
}

fn check_position_i32(pair : (i32, i32), pair_p : (i32, i32)) -> bool {
        let (col, row) = pair;
        let (col_p , row_p) = pair_p;
        if col_p == col || row_p == row {
            return false;
        }
        let diff_col : i32  = col - col_p;
        let diff_row: i32 = row - row_p;
        if diff_col.abs() == diff_row.abs() {
            return false;
        }
        return true;
}

fn find_poss_values(node : Arc<Mutex<TreeNode>>) -> Vec<(i32, i32)> {
    let mut arr = Vec::new();
    let col = node.lock().unwrap().col;
    let mut idx = 1;
    while idx <= 13 {
        if check_position(Arc::clone(&node), (col+1, idx)) {
            arr.push((col+1,idx));
        }
        idx+=1;
    }
    return arr;
}

fn find(node: Arc<Mutex<TreeNode>>) -> bool {
    let possible_values = find_poss_values(Arc::clone(&node));
    let mut bool_res = false;

    if node.lock().unwrap().col == 13 {
        return true;
    }

    for item in possible_values {
        let (x,y) = item;
        let new_node = Arc::new(Mutex::new(TreeNode::new(x,y)));
        new_node.lock().unwrap().previous = Some(Arc::clone(&node));     
        let result = find(Arc::clone(&new_node));
        if result {
            node.lock().unwrap().children.push(Arc::clone(&new_node));
            //println!("({},{})", node.lock().unwrap().col, node.lock().unwrap().row);
            bool_res = result;
        }
    }
    return bool_res;
}

fn print_solutions(node : Arc<Mutex<TreeNode>> ) {
    let unlocked_node = node.lock().unwrap();
    let (col, row) = (unlocked_node.col, unlocked_node.row);
    let children = &unlocked_node.children;
    let padding = "-".repeat(1+ (col as usize));
    println!("{} ({},{})",padding, col, row);
    for node_item in children {
        print_solutions(Arc::clone(&node_item));
    }
}

fn solution(){
    let root = Arc::new(Mutex::new(TreeNode::new(0,0)));
    let now = time::Instant::now();
    let possible_values = find_poss_values(Arc::clone(&root));

    let mut handles = vec![];

    let (tx, rx) = mpsc::channel();
    for item in possible_values {
        let (x,y) = item;
        let tx1 = tx.clone();
        let handle = thread::spawn(move || { 
            let new_node = Arc::new(Mutex::new(TreeNode::new(x,y)));
            let result = find(Arc::clone(&new_node));
            if result {
                tx1.send(new_node).unwrap();
            }
            println!("Thread Finished");
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    drop(tx);
    for item in rx {
        root.lock().unwrap().children.push(item);
    }
    
    let elapsed_time = now.elapsed();
    print_solutions(root);

    println!("Running find() took {} ms", elapsed_time.as_millis());
}

fn main() {
    solution();
}
