use std::collections::BinaryHeap;
use std::cmp::{max, Reverse};


// Binary Tree_Node
// --------------------------------------------------------------------------------------------------
struct Node{
    data:i32,
    left:Option<Box<Node>>,  // we want Box which can be None whereas Box is smart pointer for heap allocation
    right:Option<Box<Node>>  // and unqiue owner ship
}

impl Node{
    fn new() -> Self{
        Self{
            data:0,
            left:None,
            right:None
        }
    }
}

// --------------------------------------------------------------------------------------------------

// Implement a function that checks whether a given string is a palindrome or not.

fn palindrome(a:&str) -> bool{
    let a = a.as_bytes();
    let mut front = 0;
    let mut last = a.len() - 1;
    let mut flag = true;

    while front <= last {
        if a[front] != a[last]{
            flag = !flag;
            break;
        }
        front += 1;
        last -= 1;
    }

    flag
}

// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn first_occurence_sorted_array(arr: Vec<i32>, target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;
    let mut first_occ: Option<usize> = None;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            first_occ = Some(mid);
            right = mid - 1;
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    first_occ
}
// Given a string of words, implement a function that returns the shortest word in the string.

fn shortest_word(a:String) -> usize{
    let a:Vec<&str> = a.split_whitespace().collect();
    let mut min_size = i32::MAX;
    
    for i in a{
        if i.len() < max_size{
            min_size = i.len();
        }
    }
    
    min_size
}

// Implement a function that checks whether a given number is prime or not

fn is_prime(n:i32) -> bool{
    if n <= 1{
        return false;
    }
    if n <= 3{
        return true
    }
    if n % 2 == 0 || n % 3 == 0{
        return false
    }

    let mut i = 5;

    while i*i <= n {
        if n % i == 0 || n % (i + 2) == 0{
            return false;
        }
        i += 6;
    }
    
    true
}

// Given a sorted array of integers, implement a function that returns the median of the array.
fn median(a:Vec<i32>) -> i32{
    let n = a.len();

    if n % 2 == 0{
        let mid = n / 2;
        let rmid = mid - 1;   
        (a[mid] + a[rmid]) / 2
    }else{
        a[n / 2]
    }
}

// Implement a function that finds the longest common prefix of a given set of strings.
fn longest_common_prefix(mut a:Vec<String>) -> String{
    // taking Vec<String> because input can be very large and cannot fit inside the `&str` string literal can have an stack overflow

    let mut prefix = a[0].clone();

    // using closures as a helper function 
    let lcp_helper = |x:String,y:String| -> String{
        let mut i = 0;
        let mut j = 0;

        while i < x.len() && j < y.len() {
            if x.chars().nth(i).unwrap() != y.chars().nth(j).unwrap(){
                break;
            }
            i += 1;
            j += 1;
        }
        
        x[0..i].to_string()
    };

    for word in a{
        prefix = lcp_helper(prefix,word);
    }

    prefix
}


// implement a function that returns the kth smallest element in a given array.
// Time complexity is O(k * log(N))
fn kth_smallest_element(a:Vec<i32>,k:usize) -> i32{
    let mut bheap = BinaryHeap::new(); 
    
    for i in a{
        bheap.push(Reverse(i))
    }

    let mut i = 0;
    let mut result = -1;
    
    while i < k {
       match bheap.pop(){
        // using pattern matching to extract out the information in here
        Some(Reverse(i)) => {
            result = i;
        },
        None => {
            result = -1
        }
       }
        i += 1
    }
    result
}

fn max_depth_tree(tree:&mut Option<Box<Node>>) -> usize{
    
    // used match expression for evaluating the functions Optional type pattern
    match tree {
        Some(i) => {
            // explicitly return 
            1 + max(max_depth_tree(&mut i.left), max_depth_tree(&mut i.right))
        },
        None => {
            // explicitly return 
            0
        }
    }
}

fn reverse_string(a:String) -> String{
    // using rust inbuild itereators and functional programming technique to reverse the strings in rust

    a.chars().rev().collect()    // using collect to make or transform it to string type  
}

fn merge_sorted_vectors(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> Vec<i32> {
    let mut mm = Vec::with_capacity(arr1.len() + arr2.len());

    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] <= arr2[j] {
            mm.push(arr1[i]);
            i += 1;
        } else {
            mm.push(arr2[j]);
            j += 1;
        }
    }

    // Append remaining elements from arr1
    while i < arr1.len() {
        mm.push(arr1[i]);
        i += 1;
    }

    // Append remaining elements from arr2
    while j < arr2.len() {
        mm.push(arr2[j]);
        j += 1;
    }

    mm
}


fn max_subarray_sum(arr: Vec<i32>) -> i32 {
    let mut x = arr[0];
    let mut y = arr[0];

    for num in arr {
        x = num.max(x + num);
        y = y.max(x);
    }

    y
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn palin_test(){
        assert_eq!(palindrome("nitin"),true);
        assert_eq!(palindrome("racecr"),false);
        assert_eq!(palindrome("aaaa"),true);
    }   

    #[test]
    fn first_occurence_test(){
        // given sorted array finding first occurence of the given integer if not available Option type will return None
        assert_eq!(first_occurence_sorted_array(vec![1,2,2,2,4,4,56], 2),Some(1));
        assert_eq!(first_occurence_sorted_array(vec![1,2,2,2,4,4,56], 4),Some(4));
        assert_eq!(first_occurence_sorted_array(vec![1,2,2,2,4,4,56,454,45443,45443], 3),None);
    }

    #[test]
    fn shortest_word_test(){
        // finding the max_size of words in the given string 
        assert_eq!(shortest_word("which word is very very great in here".to_string()),5)
    }

    #[test]
    fn is_prime_test(){
        // finding pime 
        assert_eq!(is_prime(12121),false)
    }

    #[test]
    fn median_test(){
        // finding the median of a sorted array considering return type as integers 

        assert_eq!(median(vec![1,2,3,34,343,1211]),18)
    }

    #[test]
    fn lcp_test(){
        assert_eq!(longest_common_prefix(vec!["abcd".to_string(),"ab".to_string(),"abck".to_string()]),"ab".to_string());
        assert_eq!(longest_common_prefix(vec!["abcc".to_string(),"abc".to_string(),"abcfgr".to_string(),"abcn".to_string()]),"abc".to_string());
    }

    #[test]
    fn kth_smallest_element_test(){
        assert_eq!(kth_smallest_element(vec![1,23,2422,1,23,32121], 3),23);
        assert_eq!(kth_smallest_element(vec![1,23,2422,10,23,32121], 5),2422);
    }

    #[test]
    fn max_depth_tree_test(){

        //this given tree has a max-depth of 4 
        let mut trees = Box::new(Node{
            data:12,
            left:Some(Box::new(Node{
                data:121,
                left:Some(Box::new(Node{
                    data:22,
                    left:None,
                    right:Some(Box::new(Node{
                        data:222,
                        left:None,
                        right:None
                    })),
                })),
                right:None
            })),
            right:Some(Box::new(Node{
                data:32,
                left:None,
                right:None
            }))
        });

        assert_eq!(max_depth_tree(&mut Some(trees)),4)
    }

    #[test]

    fn reverse_string_test(){
        assert_eq!(reverse_string("HelloWorld".to_string()),"dlroWolleH");
    }

    #[test]
    fn merge_sorted_vectors_test(){
        assert_eq!(merge_sorted_vectors(vec![1,3,4,54], vec![2,5,7,67]),vec![1,2,3,4,5,7,54,67]);
    }

    #[test]
    fn max_subarray_sum_test(){
        assert_eq!(max_subarray_sum(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),6);
    }
}

fn main(){

}
