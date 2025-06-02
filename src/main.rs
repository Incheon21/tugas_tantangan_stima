use std::time::Instant;

mod test;
use test::get_test_matrices;

const INF: i32 = i32::MAX / 2;

pub fn solve_tsp(matrix: &Vec<Vec<i32>>) -> (i32, Vec<usize>, f64) {
    let start_time = Instant::now();
    let n = matrix.len();
    
    let mut dp = vec![vec![INF; n]; 1 << n];
    let mut pre = vec![vec![-1i32; n]; 1 << n];
    
    dp[1][0] = 0;
    
    for mask in 0..(1 << n) {
        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                let old_mask = mask ^ (1 << i);
                for j in 0..n {
                    if (mask & (1 << j)) != 0 {
                        let cost = dp[old_mask][j] + matrix[j][i];
                        if cost < dp[mask][i] {
                            dp[mask][i] = cost;
                            pre[mask][i] = j as i32;
                        }
                    }
                }
            }
        }
    }
    
    let mut min_cost = INF;
    let mut last = 0;
    
    for i in 0..n {
        let cost = dp[(1 << n) - 1][i] + matrix[i][0];
        if cost < min_cost {
            min_cost = cost;
            last = i;
        }
    }
    
    let mut mask = (1 << n) - 1;
    let mut path = Vec::new();
    
    while mask != 1 {
        path.push(last);
        let prev = pre[mask][last] as usize;
        mask ^= 1 << last;
        last = prev;
    }
    path.push(0);
    
    path.reverse();
    path.push(0);
    
    let elapsed_time = start_time.elapsed().as_micros() as f64 / 1000.0;
    
    (min_cost, path, elapsed_time)
}

fn print_matrix(matrix: &Vec<Vec<i32>>, test_num: usize) {
    println!("\nINPUT MATRIX - Test Case {}", test_num + 1);
    println!("Size: {} x {} cities", matrix.len(), matrix.len());
    println!("{}", "─".repeat(50));
    
    print!("     ");
    for i in 0..matrix.len() {
        print!("{:>8}", i);
    }
    println!();
    
    for (i, row) in matrix.iter().enumerate() {
        print!("{:>3}: ", i);
        for &cost in row {
            if cost == INF {
                print!("{:>8}", "∞");
            } else {
                print!("{:>8}", cost);
            }
        }
        println!();
    }
}

fn print_solution(cost: i32, path: &Vec<usize>, time: f64, matrix: &Vec<Vec<i32>>) {
    println!("\nOUTPUT SOLUTION:");
    println!("{}", "─".repeat(50));
    println!("Minimum Cost: {}", cost);
    println!("Optimal Path: {:?}", path);
    println!("Execution Time: {:.3} ms", time);
    
    println!("\nPATH BREAKDOWN:");
    let mut total_verified = 0;
    for i in 0..path.len()-1 {
        let from = path[i];
        let to = path[i+1];
        let edge_cost = matrix[from][to];
        total_verified += edge_cost;
        println!("Step {:2}: City {} → City {} (Cost: {})", 
                 i+1, from, to, edge_cost);
    }
    println!("Total path cost: {} = {}", total_verified, cost);
}

fn main() {
    println!("{}", "═".repeat(80));
    
    let test_matrices = get_test_matrices();
    
    println!("\nAvailable Test Cases: {} matrices", test_matrices.len());
    for (i, matrix) in test_matrices.iter().enumerate() {
        println!("{:2}. {} cities", i+1, matrix.len());
    }
    
    let selected_cases = vec![0, 1, 2, 3, 4, 5, 6]; // Select which test cases to run
    
    println!("\nRunning {} selected test cases...", selected_cases.len());
    
    for &case_idx in &selected_cases {
        if case_idx < test_matrices.len() {
            
            let matrix = &test_matrices[case_idx];
            print_matrix(matrix, case_idx);
            
            println!("\nSolving TSP...");
            let (cost, path, time) = solve_tsp(matrix);
            
            print_solution(cost, &path, time, matrix);
        }
    }
}