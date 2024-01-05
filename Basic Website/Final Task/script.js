function generateUniquePermutations(number) {
    const numString = number.toString();
    const digits = numString.split('').map(Number);
    const permutations = new Set();
  
    function permute(remainingDigits, currentPerm = []) {
      if (currentPerm.length > 0 && currentPerm.length <= 3) {
        permutations.add(parseInt(currentPerm.join('')));
      }
      if (currentPerm.length === numString.length) {
        permutations.add(parseInt(currentPerm.join('')));
        return;
      }
  
      for (let i = 0; i < remainingDigits.length; i++) {
        if (i > 0 && remainingDigits[i] === remainingDigits[i - 1]) {
          continue; // Skip identical digits to avoid repetitions
        }
        const newRemaining = remainingDigits.slice(0, i).concat(remainingDigits.slice(i + 1));
        permute(newRemaining, [...currentPerm, remainingDigits[i]]);
      }
    }
  
    permute(digits);
  
    const result = [...permutations];
    console.log(result);
  }
  
  const inputNumber = 1111; // Replace this number with your desired number
  generateUniquePermutations(inputNumber);



  // Pascal Triangle

  