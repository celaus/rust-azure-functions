function monte_carlo_pi(reps) {
    let count = 0;
  
    for (let i = 0; i < reps; i++){
      const x = Math.random();
      const y = Math.random(); 
      if (in_unit_circle(x, y)) {
          count += 1;
      }
    }
    return count;
  }
  
  function in_unit_circle(x, y) {
    return (x * x + y * y) < 1.0;
  }
  
  function approximate_pi(n) {
    let hits = monte_carlo_pi(n);
    return hits / n * 4.0;
  } 

const almost_pi = approximate_pi(10000000);
console.log(almost_pi);