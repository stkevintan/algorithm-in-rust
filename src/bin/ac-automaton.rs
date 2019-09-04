// https://www.luogu.org/problem/P3808
use std::collections::LinkedList;
use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

const N:usize = 1000_005;

struct Tree {
    queue:LinkedList<usize>,
    tr: Vec<[usize; 26]>,
    fail: Vec<usize>,
    tot: i32,
    count: Vec<i32>,
}

impl Tree {
    fn new(n: usize) -> Self {
        Tree {
            queue: LinkedList::new(),
            tr: vec![[0;26];n],
            fail: vec![0;n],
            count: vec![0;n],
            tot: 0,
        }
    }

    fn get_index(&c: &u8) -> usize {
        (c - b'a') as usize
    }

    fn insert(&mut self, s: &str) {
       let mut u = 0usize;
       for c in s.as_bytes().iter() {
           let index = Self::get_index(c);
           if self.tr[u][index] == 0 {
               self.tot += 1;
               self.tr[u][index] = self.tot as usize;
           }
           u = self.tr[u][index];
       }
       self.count[u] += 1;
    }

    fn build(&mut self) {
        for i in 0..26 {
            if self.tr[0][i] != 0 {
                self.queue.push_back(self.tr[0][i]);
            }
        }
        while let Some(u) = self.queue.pop_front() {
            for i in 0..26 {
                let cur = self.tr[u][i];
                let pre = self.tr[self.fail[u]][i];
                if cur != 0 {
                    self.fail[cur] = pre;
                    self.queue.push_back(cur)
                }else {
                    self.tr[u][i] = pre 
                }
            }
        }
    }

    fn query(&mut self, s: &str) -> i32{
        let mut u = 0usize;
        let mut res = 0i32;
        for c in s.as_bytes().iter() {
            u = self.tr[u][Self::get_index(c)];
            u = self.tr[u][Self::get_index(c)];
            let mut i = u;
            while i != 0 && self.count[i] != -1 {
                res += self.count[i];
                self.count[i] = -1;
                i = self.fail[i];
            }
        }
        res
    }
}

fn main() {
   let mut tree = Tree::new(N);
   let mut scan = Scanner::default();
   let out = &mut BufWriter::new(stdout());
   let n = scan.next();
   for _ in 0..n {
       let str = scan.next::<String>();
       tree.insert(&str);
   }
   tree.build();
   let str = scan.next::<String>();
   let result = tree.query(&str);
   writeln!(out, "{}", result).ok();
}

