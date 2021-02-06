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
        // put all solutions in this vector:
        let mut solutions: Vec<Vec<u32>> = vec![];
        loop {
            let guess = combo.next();
            match guess {
                None => {
                    // reached the end of
                    break
                },
                Some(guess) => {
                    let soln = guess.clone();
                    // sum of (item.price * item.items_sold) = i.summary.total_sale
                    let matches = i.category.items.clone().into_iter()
                        .map( |item| item.price)
                        .zip(guess.into_iter())
                        .map(| (x, y) | x*(y as f64))
                        .sum::<f64>() == i.summary.total_sale;
                    if matches {
                        trace!("Found solution {:?}", soln);
                        solutions.push(soln);
                    }
                }
            }
        }
        solutions.into_iter().for_each( | soln: Vec<u32> | {
            trace!("Combo {:?} matched!", soln);
            for x in 0..i.category.items.len() {
                let copy = soln.clone();
                let curr = i.category.items[x].items_sold.clone();
                let mut replacement: Vec<usize> = vec![];
                match curr {
                    None => {
                        //i.category.items[x].items_sold =
                    }
                    Some(sold) => {
                        replacement = sold.clone();
                        //sold.push(copy[x] as usize);
                    }
                }
                replacement.push(copy[x] as usize);
                i.category.items[x].items_sold = Some(replacement);
            }
        });
        i.category
    }).collect()
}

// TODO: write more tests
#[cfg(test)]
mod tests {

    use super::{Category, CategoryResult, compute};
    use super::super::super::entity::{InventoryItem, InputSummary};

    fn helper_get_sample() -> Vec<CategoryResult> {
        vec![CategoryResult {
            category: Category {
                name: "Vinegar".to_string(),
                items: vec![ InventoryItem {
                    description: "1L".to_string(),
                    price: 290.0,
                    items_sold: None,
                }]
            },
            summary: InputSummary {
                num_items: 1,
                total_sale: 290.0
            }
        }, CategoryResult {
            category: Category {
                name: "soy sauce".to_string(),
                items: vec![ InventoryItem {
                    description: "Dashi 1L".to_string(),
                    price: 905.0,
                    items_sold: None,
                }, InventoryItem {
                    description: "Silver 1L".to_string(),
                    price: 540.0,
                    items_sold: None,
                }]
            },
            summary: InputSummary {
                num_items: 22,
                total_sale: 14070.0
            }
        }, CategoryResult {
            category: Category {
                name: "Sashimi sauce".to_string(),
                items: vec![ InventoryItem {
                    description: "0.153L".to_string(),
                    price: 260.0,
                    items_sold: None,
                }, InventoryItem {
                    description: "0.36L".to_string(),
                    price: 450.0,
                    items_sold: None,
                }, InventoryItem {
                    description: "1L".to_string(),
                    price: 940.0,
                    items_sold: None,
                }]
            },
            summary: InputSummary {
                num_items: 16,
                total_sale: 7420.0,
            }
        }]
    }
    #[test]
    fn test_common_case() {
        let input: Vec<CategoryResult> = helper_get_sample();
        let res = compute(input);
        assert_eq!(res.len(), 3);
        assert_eq!(res[0].items[0].items_sold, Some(vec![1]));
        assert_eq!(res[1].items[0].items_sold, Some(vec![6]));
        assert_eq!(res[1].items[1].items_sold, Some(vec![16]));
        assert_eq!(res[2].items[0].items_sold, Some(vec![4]));
        assert_eq!(res[2].items[1].items_sold, Some(vec![10]));
        assert_eq!(res[2].items[2].items_sold, Some(vec![2]));
    }

    #[test]
    fn test_with_zero_sales() {
        let mut input = helper_get_sample();
        input[0].summary.num_items = 0;
        input[0].summary.total_sale = 0.0;
        let res = compute(input);
        assert_eq!(res[0].items[0].items_sold, Some(vec![0]));
        assert_eq!(res[1].items[0].items_sold, Some(vec![6]));
        assert_eq!(res[1].items[1].items_sold, Some(vec![16]));
        assert_eq!(res[2].items[0].items_sold, Some(vec![4]));
        assert_eq!(res[2].items[1].items_sold, Some(vec![10]));
        assert_eq!(res[2].items[2].items_sold, Some(vec![2]));
    }
}