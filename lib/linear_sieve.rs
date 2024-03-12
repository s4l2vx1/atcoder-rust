mod linear_sieve {
    //lpf, exp, pow, rem
    #[derive(Clone, Copy)]
    struct LpfT (u32, u32, u32, u32);

    impl LpfT {
        fn new(lpf: usize, exp: usize, pow: usize, rem: usize) -> Self {
            LpfT(lpf as u32, exp as u32, pow as u32, rem as u32)
        }

        fn lpf(&self) -> usize { self.0 as usize }
        fn exp(&self) -> usize { self.1 as usize }
        fn pow(&self) -> usize { self.2 as usize }
        fn rem(&self) -> usize { self.3 as usize }
    }

    pub struct LinearSieve {
        lpf_t: Vec<LpfT>,
        pub primes: Vec<usize>,
    }

    impl LinearSieve {
        pub fn new(n: usize) -> Self {
            let mut lpf_t = vec![LpfT(0, 0, 0, 0); n + 1];
            let mut primes = vec![];
            for i in 2..=n {
                if lpf_t[i].0 < 2 {
                    lpf_t[i] = LpfT::new(i, 1, i, 1);
                    primes.push(i);
                }
                let lpf = lpf_t[i].0 as usize;
                for j in primes.iter().cloned().take_while(|&j| j <= lpf && i * j <= n) {
                    let mut exp = 1;
                    let mut rem = i * (j - 1);
                    while rem % j == 0 {
                        exp += 1;
                        rem /= j;
                    }
                    lpf_t[i * j] = LpfT::new(j, exp, i * j / rem, rem);
                }
            }
            LinearSieve { lpf_t, primes }
        }

        pub fn is_prime(&self, n: usize) -> bool { n >= 2 && self.lpf_t[n].0 == n as u32 }

        pub fn lpf(&self, n: usize) -> usize { self.lpf_t[n].lpf() }
        pub fn lpf_exp(&self, n: usize) -> usize { self.lpf_t[n].exp() }
        pub fn lpf_pow(&self, n: usize) -> usize { self.lpf_t[n].pow() }
        pub fn lpf_rem(&self, n: usize) -> usize { self.lpf_t[n].rem() }

        fn factorize_with_lpft(&self, n: usize) -> impl Iterator<Item = LpfT> + '_ {
            std::iter::successors(
                Some(self.lpf_t[n]),
                |lpft| { if lpft.3 == 1 { None } else { Some(self.lpf_t[lpft.3 as usize]) } }
            )
        }

        pub fn factorize_with_exp(&self, n: usize) -> impl Iterator<Item = (usize, usize)> + '_ { 
            self.factorize_with_lpft(n).map(|lpft| (lpft.0 as usize, lpft.1 as usize))
        }

        pub fn factorize(&self, n: usize) -> impl Iterator<Item = usize> + '_ {
            self.factorize_with_lpft(n).flat_map(|x| itertools::repeat_n(x.0 as usize, x.1 as usize))
        }
    }
}
use linear_sieve::*;