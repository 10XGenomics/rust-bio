
use std::iter::{repeat, Enumerate};
use std::slice;
use std::collections::VecMap;

use alphabets::Alphabet;


type LPS = Vec<usize>;


#[derive(Copy)]
pub struct KMP {
    m: usize
}


fn get_lps(pattern: &[u8]) -> LPS {
    let (m, mut q) = (pattern.len(), 0us);
    let mut lps: LPS = repeat(0).take(m).collect();
    for i in 1..m {
        while q > 0 && pattern[q] != pattern[i] {
            q = lps[q];
        }
        if pattern[q] == pattern[i] {
            q += 1;
        }
        lps[i] = q;
    }

    lps
}


struct Delta {
    table: Vec<VecMap<usize>>
}


impl Delta {
    fn new(pattern: &[u8], alphabet: Alphabet) -> Self {
        //assert!(alphabet.is_word(pattern));
        let k = alphabet.max_symbol()
            .expect("Expecting non-empty alphabet.") as usize + 1;
        let m = pattern.len();

        let mut init = VecMap::with_capacity(k);
        for c in alphabet.symbols.iter() {
            init.insert(c, 0);
        }
        *init.get_mut(&(pattern[0] as usize)).unwrap() = 1;

        let lps = get_lps(pattern);

        let mut table = Vec::with_capacity(m + 1);
        table.push(init);
        for q in 1..m+1 {
            let mut dq = VecMap::with_capacity(k);
            for c in alphabet.symbols.iter() {
                dq.insert(c, *table[lps[q - 1]].get(&c).unwrap());
            }
            if q < m {
                *dq.get_mut(&(pattern[q] as usize)).unwrap() = q;
            }
            table.push(dq);
        }

        Delta { table: table }
    }
}




pub struct FindAll<'a> {
    kmp: KMP,
    q: usize,
    text: Enumerate<slice::Iter<'a, u8>>
}


impl<'a> Iterator for FindAll<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        for (i, &c) in self.text {
            // TODO self.q = self.kmp.delta(self.q, c);
            if self.q == self.kmp.m {
                return Some(i - self.kmp.m + 1);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::{get_lps, Delta};
    use alphabets::Alphabet;

    #[test]
    fn test_get_lps() {
        let pattern = b"ababaca";
        let lps = get_lps(pattern);
        assert_eq!(lps, [0, 0, 1, 2, 3, 0, 1]);
    }

    #[test]
    fn test_delta() {
        let pattern = b"ababaca";
        let alphabet = Alphabet::new(pattern);
        let delta = Delta::new(pattern, alphabet);
        
    }
}
