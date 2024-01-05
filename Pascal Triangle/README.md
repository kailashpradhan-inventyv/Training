# Question : Create a JavaScript program that takes a user-input number, generates permutations and combinations based on that number, identifies prime numbers within these combinations, displays the primes found within Pascal's Triangle while highlighting them, and display also those prime numbers which are absent from the Pascal's Triangle, and calculates the error ratio. This ratio represents the proportion of prime numbers not present in Pascal's Triangle to the total number of prime numbers generated.

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
