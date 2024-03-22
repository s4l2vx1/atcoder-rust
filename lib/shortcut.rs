//等差数列
(0..n).iter().step_by(d).take_while(|&x| x < b)

//繰り返し適用
itertools::iterate(a, |&x| x + d).take_while(|&x| x < b)

//累積和
a.iter().scan(0, |s, &x| {*s += x; Some(*s)}).collect::<Vec<_>>()
