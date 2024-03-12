//等差数列
(0..n).iter().step_by(d).take_while(|&x| x < b + d)

//繰り返し適用
itertools::iterate(a, |&x| x + d).take_while(|&x| x < b + d)

