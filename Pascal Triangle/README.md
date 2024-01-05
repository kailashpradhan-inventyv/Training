# Question : 

Create a JavaScript program that takes a user-input number, generates permutations and combinations based on that number, identifies prime numbers within these combinations, displays the primes found within Pascal's Triangle while highlighting them, and display also those prime numbers which are absent from the Pascal's Triangle, and calculates the error ratio. This ratio represents the proportion of prime numbers not present in Pascal's Triangle to the total number of prime numbers generated.

## PseudoCode : 

    function generatePascalTriangle(limit) {
        let triangle = [[1]];
        do {
          let lastRow = triangle[triangle.length - 1];
          let newRow = [1];
          for (let i = 1; i < lastRow.length; i++) {
                let nextValue = lastRow[i] + lastRow[i - 1];
                newRow.push(nextValue);
                if (nextValue > limit) {
                  return triangle;
                }
          }
          newRow.push(1);
          triangle.push(newRow);
        } while (true);
      }
----------------------------------------------------------------------
      function generateCombinations(number) {
        const numString = number.toString();
        const len = numString.length;
        const combinations = new Set();

        function permute(remainingDigits, currentPerm = "") {
          if (currentPerm.length > 0 && currentPerm.length <= len) {
            combinations.add(parseInt(currentPerm));
          }
          if (currentPerm.length === len) {
            combinations.add(parseInt(currentPerm));
            return;
          }

          for (let i = 0; i < remainingDigits.length; i++) {
            const newRemaining =
            remainingDigits.slice(0, i) + remainingDigits.slice(i + 1);
            permute(newRemaining, currentPerm + remainingDigits[i]);
          }
        }

        permute(numString);

        return [...combinations];
      }

### Explaination : 

The JavaScript code is designed to perform various operations based on user-inputted numbers. It begins by defining functions to check for prime numbers and generate Pascal's Triangle. The isPrime function determines if a number is prime, using basic divisibility rules and checks up to the square root of the number for efficiency. The generatePascalTriangle function constructs Pascal's Triangle, starting with the first row and iteratively generating subsequent rows until reaching a limit derived from the prime numbers generated later.

Upon receiving an input number, the program generates combinations using permutations and combinations. From these combinations, it identifies prime numbers using the isPrime function and constructs Pascal's Triangle based on the highest prime found. The program then displays the primes within the Pascal's Triangle, highlighting them visually. Additionally, it creates an array showcasing prime numbers absent from the Pascal's Triangle. Finally, the program calculates the error ratio by determining the proportion of prime numbers that are not present in the Pascal's Triangle to the total number of primes generated from the combinations. This comprehensive code provides a clear demonstration of prime number identification, Pascal's Triangle construction, visual highlighting, and error ratio calculation, offering an efficient solution for these mathematical operations in JavaScript.


#### Author Details :

Author Name : Kailash Pradhan
