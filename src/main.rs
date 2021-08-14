fn main() {
    /* start point, the hardest test
     */
    multiply(20,20);

    /* readable and very clean for me, more than that is ask for much...
     */
    assert_eq!(solution(10), 23);

    /* okay this was very fun to try, and a trip to discover how to split a integer but
     * work and i'm satisfied with that
     */
    digital_root(1872);

    /* worked for the test below but in random test has failed so i just give up on the code
     * idk actually how to turn a ref into mut ref and already has some problems with borrowing
     */
    assert_eq!(sort_array(&[5, 3, 2, 8, 1, 4]), [1, 3, 2, 8, 5, 4]);

    /* :p
     */
    assert_eq!(square_sum(vec![1, 2]), 5);
}

//d1
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

//d2
pub fn solution(num: i32) -> i32 {
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
pub fn digital_root(n: i64) -> i64 {
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
pub fn spliting(x: i64) -> Vec<i64> {
    let vec = x.to_string().chars().map(|c| c as i64 - 0x30).collect();

    vec
}

//d4
pub fn sort_array(arr: &[i32]) -> Vec<i32> {
    let mut copy = arr.to_owned();
    //?????????????????????
    for num in arr {
        if (num % 2) != 0 {
            let index = arr.iter().position(|n| n == num).unwrap();

            for snum in arr {
                if (snum % 2) != 0 {
                    let sec_index = arr.iter().position(|n| n == snum).unwrap();
                    if num > snum {
                        copy.swap(index, sec_index);
                    }
                }
            }
        }
    }
    println!("{:?}", copy);
    copy
}

//d5
fn square_sum(vec: Vec<i32>) -> i32 {
    let mut sum: i32 = 0;

    for num in vec {
        sum += num * num;
    }

    sum
}