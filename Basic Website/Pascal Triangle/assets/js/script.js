function isPrime(num) {
    if (num <= 1) return false;
    if (num === 2) return true;
    if (num % 2 === 0) return false;

    for (let i = 3; i <= Math.sqrt(num); i += 2) {
      if (num % i === 0) {
        return false;
      }
    }
    return true;
  }

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

  function calculate() {
    const inputNumber = document.getElementById('inputNumber').value;
    const combinations = generateCombinations(inputNumber);
    const primes = combinations.filter(isPrime);
    const sortedArr = combinations.sort((a, b) => a - b);
    const lastNum = sortedArr[sortedArr.length - 1];
    const pascalTriangle = generatePascalTriangle(lastNum);

    displayPascal(pascalTriangle,primes);
    
    displayPrimesNotInPascal(primes, pascalTriangle);
    displayAllPrimes(primes);
  }

  function displayAllPrimes(primes){
    // const primesNotInPascalDiv = document.createElement('div');
    // primesNotInPascalDiv.textContent = primesNotInPascal.join(', ');
    // primeOutput.appendChild(primesNotInPascalDiv);

    const primeOutput = document.getElementById('primeOutput');
    // primeOutput.innerHTML = '<h2>All Primes </h2>';

    const allPrimes=document.createElement('div');
    const heading=document.createElement('h2');
    heading.textContent='All Prime Numbers';
    primeOutput.appendChild(heading);
    allPrimes.textContent=primes.join(', ');
    primeOutput.appendChild(allPrimes);

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


  function displayPascal(pascalTriangle, primes) {
    const pascalOutput = document.getElementById('pascalOutput');
    pascalOutput.innerHTML = '<h2>Pascal\'s Triangle</h2>';

    for (let i = 0; i < pascalTriangle.length; i++) {
      const rowDiv = document.createElement('div');
      rowDiv.classList.add('row');
      for (let j = 0; j < pascalTriangle[i].length; j++) {
        const cellDiv = document.createElement('span');
        cellDiv.textContent = pascalTriangle[i][j] + ' ';
        if (primes.includes(pascalTriangle[i][j])) {
          cellDiv.classList.add('highlight');
        }else{
          cellDiv.classList.add('normal');

        }
        
        rowDiv.appendChild(cellDiv);
      }
      pascalOutput.appendChild(rowDiv);
    }
  }

  function displayPrimesNotInPascal(primes, pascalTriangle) {
    const primeOutput = document.getElementById('primeOutput');
    primeOutput.innerHTML = '<h2>Primes Not in Pascal\'s Triangle</h2>';
    const primeCount=primes.length;

    const primesNotInPascal = primes.filter(prime => !isPrimeInPascal(prime, pascalTriangle));
    const notInPascalCount=primesNotInPascal.length;
    const err_ratio=notInPascalCount/primeCount*100;
    const primesNotInPascalDiv = document.createElement('div');
    primesNotInPascalDiv.textContent = primesNotInPascal.join(', ');
    primeOutput.appendChild(primesNotInPascalDiv);

    const errorRatioDiv = document.createElement('h2');
    errorRatioDiv.textContent = `Error Ratio: ${err_ratio.toFixed(2)}%`;
    primeOutput.appendChild(errorRatioDiv);
    
    
  }

  function isPrimeInPascal(prime, pascalTriangle) {
    for (let row of pascalTriangle) {
      for (let num of row) {
        if (prime === num) {
          return true;
        }
      }
    }
    return false;
  }
