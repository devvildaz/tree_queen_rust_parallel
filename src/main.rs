use std::option;
use std::cell::RefCell;
use std::rc::Rc;
use std::time;
use std::thread;
use std::sync::mpsc;

struct TreeNode {
    previous: option::Option<Rc<RefCell<Box<TreeNode>>>>,
    col: i32,
    row: i32,
    children: Vec<Rc<RefCell<Box<TreeNode>>>>
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
}

fn check_position(node: Rc<RefCell<TreeNode>>, pair: (i32,i32)) -> bool{
    let mut pointer = node;
    let (col ,row) = pair;
    loop {
        let col_p : i32 = pointer.borrow().col;
        let row_p : i32 = pointer.borrow().row;
        
        if col_p == 0 || row_p == 0 {
            break;
        }

        if check_position_i32((col, row),(col_p, row_p)){
            let new_p = match pointer.borrow().previous {
                Some(ref x) => 
                    Rc::clone(x)
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

fn find_poss_values(node : Rc<RefCell<TreeNode>>) -> Vec<(i32, i32)> {
    let mut arr = Vec::new();
    let col = node.borrow_mut().col;
    let mut idx = 1;
    while idx <= 13 {
        if check_position(Rc::clone(&node), (col+1, idx)) {
            arr.push((col+1,idx));
        }
        idx+=1;
    }
    return arr;
}

fn find(node: Rc<RefCell<TreeNode>>) -> bool {
    let possible_values = find_poss_values(Rc::clone(&node));
    let mut bool_res = false;

    if node.borrow().col == 13 {
        return true;
    }

    for item in possible_values {
        let (x,y) = item;
        let new_node = Rc::new(RefCell::new(TreeNode::new(x,y)));
        new_node.borrow_mut().previous = Some(Rc::clone(&node));     
        let result = find(Rc::clone(&new_node));
        
        if result {
            node.borrow_mut().children.push(Rc::clone(&new_node));
            //println!("({},{})", new_node.borrow().col, new_node.borrow().row);
            bool_res = result;
        }
    }
    return bool_res;
}

fn solution(){
    let root = TreeNode::new(0,0);
    let rc_root = Rc::new(RefCell::new(root));
    let now = time::Instant::now();
    let possible_values = find_poss_values(Rc::clone(&rc_root));

    let mut handles = vec![];

    let (tx, rx) = mpsc::channel();

    for item in possible_values {
        let (x,y) = item;

        let handle = thread::spawn(move || { 
            let new_node = Rc::new(RefCell::new(TreeNode::new(x,y)));
            let result = find(Rc::clone(&new_node));
            if result {
                let res_to = Box::new(new_node.borrow_mut());
                tx.send(res_to).unwrap();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let elapsed_time = now.elapsed();
    println!("Running find() took {} ms", elapsed_time.as_millis());


}

fn main() {
    solution();
}
