let s1 = 23415;
let s2 = 23749;
let s3 = 1633;

const S1_MOD = 30269;
const S2_MOD = 30307;
const S3_MOD = 30323;

function WH() {
  s1 = (171 * s1) % S1_MOD;
  s2 = (172 * s2) % S2_MOD;
  s3 = (170 * s3) % S3_MOD;
  return (s1 / S1_MOD + s2 / S2_MOD + s3 / S3_MOD) % 1.0
}

function monte_carlo_pi(reps) {
  let count = 0;

  for (let i = 0; i < reps; i++) {
    const x = WH();
    const y = WH();
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

module.exports = function (context, req) {
  const almost_pi = approximate_pi(10000000);
  context.log(almost_pi);

  context.res = {
    status: 200,
    body: `${almost_pi}`
  };
  context.done();
};