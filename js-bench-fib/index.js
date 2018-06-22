function fib(n) {
  if (n === 1) return 1;
  if (n === 2) return 1;
  return fib(n - 1) + fib(n - 2);
}

module.exports = function (context, req) {
  const fibonacci = fib(43);
  console.log(fibonacci);

  context.res = {
    status: 200,
    body: `${fibonacci}`
  };
  context.done();
};