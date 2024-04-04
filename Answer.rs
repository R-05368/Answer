use std::io;

// Function to check whether a given string is a palindrome or not
fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// Function to return the index of the first occurrence of a given number in a sorted array
fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

// Function to return the shortest word in a string of words
fn shortest_word(words: &str) -> Option<&str> {
    words.split_whitespace().min_by_key(|word| word.len())
}

// Function to check whether a given number is prime or not
fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Function to return the median of a sorted array of integers
fn median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        (arr[len / 2 - 1] + arr[len / 2]) as f64 / 2.0
    } else {
        arr[len / 2] as f64
    }
}

// Function to find the longest common prefix of a given set of strings
fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }
    let first_string = &strings[0];
    for (i, c) in first_string.chars().enumerate() {
        if strings.iter().any(|s| s.len() <= i || s.chars().nth(i) != Some(c)) {
            return first_string[..i].to_string();
        }
    }
    first_string.to_string()
}

// Function to return the kth smallest element in a given array
fn kth_smallest_element(arr: &[i32], k: usize) -> Option<i32> {
    if k > 0 && k <= arr.len() {
        let mut sorted_arr = arr.to_vec();
        sorted_arr.sort();
        Some(sorted_arr[k - 1])
    } else {
        None
    }
}

// Definition of a binary tree node
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Function to return the maximum depth of a binary tree
fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Prompt the user for input
    println!("Enter a string:");
    let mut input_string = String::new();
    io::stdin().read_line(&mut input_string).expect("Failed to read line");

    // Test cases
    println!("Is '{}' a palindrome? {}", input_string.trim(), is_palindrome(input_string.trim()));
    
    println!("Enter a sorted array of integers separated by spaces:");
    let mut input_array_string = String::new();
    io::stdin().read_line(&mut input_array_string).expect("Failed to read line");
    let input_array: Vec<i32> = input_array_string.trim().split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    println!("Enter a number to find its first occurrence:");
    let mut input_number = String::new();
    io::stdin().read_line(&mut input_number).expect("Failed to read line");
    let input_number: i32 = input_number.trim().parse().expect("Failed to parse integer");
    println!("Index of first occurrence of {}: {:?}", input_number, first_occurrence(&input_array, input_number));

    println!("Enter a string of words:");
    let mut input_words = String::new();
    io::stdin().read_line(&mut input_words).expect("Failed to read line");
    println!("Shortest word in '{}': {:?}", input_words.trim(), shortest_word(input_words.trim()));

    println!("Enter a number to check if it's prime:");
    let mut input_prime = String::new();
    io::stdin().read_line(&mut input_prime).expect("Failed to read line");
    let input_prime: u64 = input_prime.trim().parse().expect("Failed to parse integer");
    println!("Is {} prime? {}", input_prime, is_prime(input_prime));

    println!("Enter a sorted array of integers separated by spaces:");
    let mut input_median_array_string = String::new();
    io::stdin().read_line(&mut input_median_array_string).expect("Failed to read line");
    let input_median_array: Vec<i32> = input_median_array_string.trim().split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    println!("Median of {:?}: {}", input_median_array, median(&input_median_array));

    println!("Enter a set of strings separated by spaces:");
    let mut input_prefix_strings = String::new();
    io::stdin().read_line(&mut input_prefix_strings).expect("Failed to read line");
    let input_prefix_strings: Vec<String> = input_prefix_strings.trim().split_whitespace()
        .map(|s| s.to_string())
        .collect();
    println!("Longest common prefix of {:?}: {}", input_prefix_strings, longest_common_prefix(&input_prefix_strings));

    println!("Enter a sorted array of integers separated by spaces:");
    let mut input_kth_array_string = String::new();
    io::stdin().read_line(&mut input_kth_array_string).expect("Failed to read line");
    let input_kth_array: Vec<i32> = input_kth_array_string.trim().split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();
    println!("Enter the value of k:");
    let mut input_k = String::new();
    io::stdin().read_line(&mut input_k).expect("Failed to read line");
    let input_k: usize = input_k.trim().parse().expect("Failed to parse integer");
    println!("{}th smallest element in {:?}: {:?}", input_k, input_kth_array, kth_smallest_element(&input_kth_array, input_k));

    // Example of how to use max_depth function
    println!("Enter the binary tree structure in the following format:\nroot\nleft_child1\nright_child1\nleft_child2\nright_child2\n...\nType 'None' for empty nodes.");
    let mut input_tree_string = String::new();
    io::stdin().read_line(&mut input_tree_string).expect("Failed to read line");
    let input_tree_values: Vec<&str> = input_tree_string.trim().split_whitespace().collect();
    let root = construct_tree(&input_tree_values, 0);
    println!("Maximum depth of binary tree: {}", max_depth(root));
}

// Function to construct binary tree from input values
fn construct_tree(values: &[&str], index: usize) -> Option<Box<TreeNode>> {
    if index >= values.len() || values[index] == "None" {
        return None;
    }
    let val: i32 = values[index].parse().expect("Failed to parse integer");
    let left_child = construct_tree(values, 2 * index + 1);
    let right_child = construct_tree(values, 2 * index + 2);
    Some(Box::new(TreeNode {
        val,
        left: left_child,
        right: right_child,
    }))
}
