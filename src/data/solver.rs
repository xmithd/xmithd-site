use super::super::entity::CategoryResult;
use super::super::entity::Category;

use log::{trace};

struct Combinatorial {
    total: u32, // n
    current: Vec<u32>, // has size m
    current_n: u32,
}

// iterate through all possible combinations where sum of vector entries = total
// inefficient algorithm complexity: total^size or O(n^m)
impl Combinatorial {
    fn new(size: usize, total: u32) -> Combinatorial {
        let mut current : Vec<u32> = Vec::new();
        for _ in 0..size {
            current.push(0);
        }
        Self {
            total,
            current,
            current_n: 0,
        }
    }
}

impl Iterator for Combinatorial {
    type Item = Vec<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        let base = self.total+1;
        let len = self.current.len();
        let max_n = base.pow(len as u32);
        for n in self.current_n..max_n {
            // set all the numbers in the vector
            for i in 0..len {
                let x = len - 1 - i;
                let curr_base = base.pow((i) as u32);
                let shifted = (n - (n % curr_base)) / curr_base;
                self.current[x] = shifted % base;
            }
            // try
            if self.current.clone().into_iter().sum::<u32>() == self.total {
                self.current_n = n + 1;
                //println!("Eureka!");
                return Some(self.current.clone());
            }
        }
        return None;
    }
}


pub fn compute(input: Vec<CategoryResult>) -> Vec<Category> {
    input.into_iter().map(|mut i| {
        // for each category, we must satisfy
        // sum of items_sold = i.summary.num_items
        trace!("Trying combo for {:?} items and total number sold: {:?}",i.category.items.len(), i.summary.num_items);
        let mut combo  = Combinatorial::new(i.category.items.len(), i.summary.num_items as u32);
        let mut matches = false;
        while !matches {
            let guess = combo.next();
            match guess {
                None => {
                    trace!("No match");
                    break
                },
                Some(guess) => {
                    trace!("Trying guess {:?}", guess);
                    // sum of (item.price * item.items_sold) = i.summary.total_sale
                    matches = i.category.items.clone().into_iter()
                        .map( |item| item.price)
                        .zip(guess.into_iter())
                        .map(| (x, y) | x*(y as f64))
                        .sum::<f64>() == i.summary.total_sale
                }
            }
        }
        if matches {
            trace!("Combo {:?} matched!", combo.current);
            for x in 0..i.category.items.len() {
                let copy = combo.current.clone();
                i.category.items[x].items_sold = Some(copy[x] as usize)
            }
        }
        i.category
    }).collect()
}
