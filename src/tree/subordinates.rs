use std::cell::RefCell;
use std::collections::HashMap;

fn traverse(employee_height: &mut Vec<u32>, graph: &HashMap<u32, RefCell<Vec<u32>>>, boss: u32) {
    if let Some(val) = graph.get(&boss) {
        let length = val.borrow().len();
        for i in 0..length {
            employee_height[boss as usize] += 1;
            let current_employee = val.borrow()[i];
            traverse(employee_height, graph, current_employee);
            employee_height[boss as usize] += employee_height[current_employee as usize];
        }
    }
}

pub fn main(n: u32, subordinates: Vec<u32>) {
    let mut graph: HashMap<u32, RefCell<Vec<u32>>> = HashMap::new();
    graph.insert(1, RefCell::new(Vec::new()));

    for i in 0..(n - 1) as usize {
        let employee_id = (i + 2) as u32;
        let boss_id = subordinates[i];

        match graph.get(&boss_id) {
            Some(val) => val.borrow_mut().push(employee_id),
            None => {
                graph.insert(boss_id, RefCell::new(vec![employee_id]));
            }
        }
    }

    let mut employee_height: Vec<u32> = vec![0; (n + 1) as usize];

    for i in 1..n {
        if employee_height[i as usize] == 0 {
            traverse(&mut employee_height, &graph, i)
        }
    }

    for i in 1..employee_height.len() {
        print!("{} ", employee_height[i]);
    }
}
