# Rust Challenge
## Resume

Given a string that consists of left and right parentheses, '(' and ')', balance the parentheses by inserting parentheses as necessary. Determine the minimum number of characters that must be inserted.  

Example

s = '(()))'

Insert 1 left parenthesis at the left end of the string to get '((()))'. The string is balanced after 1 insertion.

Constraints

    1 ≤ length of s ≤ 105

Input Format For Custom Testing

The first line contains a string, s, the initial parentheses sequence.
Sample Case 0

Sample Input

STDIN     Function
-----     -----
()))   →  s = '()))'

Sample Output

2

Explanation

Insert a '(' 2 times at the beginning of the string to make it valid: '((()))'.
Sample Case 1

Sample Input

STDIN     Function
-----     -----
()()   →  s = '()()'

Sample Output

0

Explanation

The sequence is already valid, so no insertions are needed.

## Solution

The code modify the original input adding the '(' or ')' characters to balance the parentheses. Of course, the code return the balance integer.
I'm added some images with the outputs using differents inputs.

The time and space complexity are O(n)
