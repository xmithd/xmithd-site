
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub name: String,
    pub is_admin: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostIdent {

    // Post ID
    pub id: u32,

    // Post Title
    pub title: String,

    // Timestamp when it was created
    // (ms since Unix epoch - but only accurate to the second)
    pub created: i64,
}

pub struct Post {
    // post ident info
    pub ident: PostIdent,

    // markdown content
    pub content: String,

    // Timestamp when it was last updated
    // (ms since Unix epoch - but only accurate to the second)
    pub updated: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InventoryItem {
    pub description: String,
    pub price: f64, //in Yen
    pub items_sold: Option<Vec<usize>>,
    pub total_price: Option<Vec<f64>>,
}

impl Clone for InventoryItem {
    fn clone(&self) -> Self {
        Self {
            items_sold: self.items_sold.clone(),
            price: self.price,
            description: String::from(&self.description),
            total_price: self.total_price.clone()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Category {
    pub name: String,
    pub items: Vec<InventoryItem>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InputSummary {
    pub num_items: usize,
    pub total_sale: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryResult {
    pub category: Category,
    pub summary: InputSummary,
}
