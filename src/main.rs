fn main() {
    println!("Hello, world!");
    //multiply(20,20);
    //assert_eq!(solution(10), 23);
    //digital_root(1872);
    assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);
    assert_eq!(sort_array(&[5, 3, 1, 8, 0]), [1, 3, 5, 8, 0]);
    assert_eq!(sort_array(&[]), []);
}

//d1
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

//d2
fn solution(num: i32) -> i32 {
    let nums = [3, 5];
    let mut mult = Vec::new();
    let mut res;
    for i in 0..2 {
        for x in 1..num {
            res = nums[i] * x;
            if !mult.contains(&res) && res < num {
                mult.push(res);
            }
        }
    }
    //sum the entire vector
    let sum: i32 = mult.iter().sum();
    sum
}

//d3
fn digital_root(n: i64) -> i64 {
    let mut rng = 0;
    let mut number = n;
    while rng != 1 {
        let arr = spliting(number);
        rng = arr.len();
        number = 0;

        for num in 0..rng {
            number = number + arr[num];
        }

        println!("{:?}", arr);
        println!("{}", number);
    }
    number
}

//function to split a integer into a array
fn spliting(x: i64) -> Vec<i64> {
    let vec = x.to_string().chars().map(|c| c as i64 - 0x30).collect();

    vec
}

//d4

fn sort_array(arr: &[i32]) -> Vec<i32> {
    for i in arr {
        if (i % 2) != 0 {
            let index = arr.iter().position(|n| n == i).unwrap();
            println!("{}", index);
        }
    }
    todo!();
}

//d5
