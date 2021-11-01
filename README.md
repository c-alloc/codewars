## Code Wars archives

This is a repository to storage my code wars solved problems, mostly using Rust and some other languages maybe? well, i really like rust :p.

[My codewars profile](https://www.codewars.com/users/dertera)

### Multiples of 3 or 5

If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.

Finish the solution so that it returns the sum of all the multiples of 3 or 5 below the number passed in. Additionally, if the number is negative, return 0 (for languages that do have them).

_Note: If the number is a multiple of both 3 and 5, only count it once._

**rep:** [Multiples of 3 or 5](https://github.com/dertera/codewars-archives/tree/main/Multiples-3-or-5)

### Sum of Digits / Digital Root

Given n, take the sum of the digits of n. If that value has more than one digit, continue reducing in this way until a single-digit number is produced. The input will be a non-negative integer. 

_Examples:_

```
    16  -->  1 + 6 = 7
    942  -->  9 + 4 + 2 = 15  -->  1 + 5 = 6
    132189  -->  1 + 3 + 2 + 1 + 8 + 9 = 24  -->  2 + 4 = 6
    493193  -->  4 + 9 + 3 + 1 + 9 + 3 = 29  -->  2 + 9 = 11  -->  1 + 1 = 2
```

**rep:** [Sum of Digits / Digital Root](https://github.com/dertera/codewars-archives/tree/main/Digital-Root)

### Square(n) Sum

Complete the square sum function so that it squares each number passed into it and then sums the results together.

_For example, for [1, 2, 2] it should return 9 because 1^2 + 2^2 + 2^2 = 9._

**rep:** [Square(n) Sum](https://github.com/dertera/codewars-archives/tree/main/Square-Sum)


### Help the Bookseller!

A bookseller has lots of books classified in 26 categories labeled A, B, ... Z. Each book has a code c of 3, 4, 5 or more characters. The 1st character of a code is a capital letter which defines the book category.

In the bookseller's stocklist each code c is followed by a space and by a positive integer n (int n >= 0) which indicates the quantity of books of this code in stock.

For example an extract of a stocklist could be:


```
L = {"ABART 20", "CDXEF 50", "BKWRK 25", "BTSQZ 89", "DRTYM 60"}.
or
L = ["ABART 20", "CDXEF 50", "BKWRK 25", "BTSQZ 89", "DRTYM 60"] or ....
```

You will be given a stocklist (e.g. : L) and a list of categories in capital letters e.g :

```
M = {"A", "B", "C", "W"} 
or
M = ["A", "B", "C", "W"] or ...
```

and your task is to find all the books of L with codes belonging to each category of M and to sum their quantity according to each category.

For the lists L and M of example you have to return the string (in Haskell/Clojure/Racket a list of pairs):

```
(A : 20) - (B : 114) - (C : 50) - (W : 0)
```

where A, B, C, W are the categories, 20 is the sum of the unique book of category A, 114 the sum corresponding to "BKWRK" and "BTSQZ", 50 corresponding to "CDXEF" and 0 to category 'W' since there are no code beginning with W.

If L or M are empty return string is "" (Clojure and Racket should return an empty array/list instead).

**rep:** [Help the bookseller!](https://github.com/dertera/codewars-archives/tree/main/Help_the_bookseller)

### Create Phone Number

Write a function that accepts an array of 10 integers (between 0 and 9), that returns a string of those numbers in the form of a phone number.

_Example_
```
create_phone_number(&[1,2,3,4,5,6,7,8,9,0]); // returns "(123) 456-7890"
```

The returned format must be correct in order to complete this challenge.
Don't forget the space after the closing parentheses!

**rep:** [Create Phone number](https://github.com/dertera/codewars-archives/tree/main/create_phone_number)