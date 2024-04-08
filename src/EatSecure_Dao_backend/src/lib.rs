use candid::{CandidType, Deserialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Product {
    id: String,
    name: String,
    production_date: String,
    expiration_date: String,
    image: Vec<u8>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq)]
pub struct Info {
    desc: String,
    production_date: String,
    expiration_date: String,
    product_accuracy: bool,
}

type ProductInfo = Vec<Info>;

struct ProductRegistry {
    product_list: Arc<Mutex<HashMap<String, Product>>>,
    info_list: Arc<Mutex<HashMap<String, ProductInfo>>>,
}

impl ProductRegistry {
    pub fn new() -> Self {
        Self {
            product_list: Arc::new(Mutex::new(HashMap::new())),
            info_list: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn create_product(&self, id: String, product: Product, product_info: ProductInfo) -> bool {
        let mut product_list = self.product_list.lock().unwrap();
        let mut info_list = self.info_list.lock().unwrap();
        product_list.insert(id.clone(), product);
        info_list.insert(id, product_info);
        true
    }

    pub fn get_product_list(&self) -> Vec<Product> {
        let product_list = self.product_list.lock().unwrap();
        product_list.values().cloned().collect()
    }

    pub fn get_info_list(&self) -> Vec<ProductInfo> {
        let info_list = self.info_list.lock().unwrap();
        info_list.values().cloned().collect()
    }

    pub fn get_infos(&self, id: &String) -> Option<ProductInfo> {
        let info_list = self.info_list.lock().unwrap();
        info_list.get(id).cloned()
    }

    pub fn get_product(&self, id: &String) -> Option<Product> {
        let product_list = self.product_list.lock().unwrap();
        product_list.get(id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio; // tokio crate ekledik

    #[tokio::test]
    async fn test_create_and_get_product() {
        let registry = ProductRegistry::new();
        let product = Product {
            id: "1".to_string(),
            name: "Test Product".to_string(),
            production_date: "2024-01-01".to_string(),
            expiration_date: "2024-12-31".to_string(),
            image: Vec::new(),
        };
        let product_info = vec![
            Info {
                desc: "Test Description".to_string(),
                production_date: "2024-01-01".to_string(),
                expiration_date: "2024-12-31".to_string(),
                product_accuracy: true,
            }
        ];
        registry.create_product("1".to_string(), product.clone(), product_info.clone());
        let retrieved_product = registry.get_product(&"1".to_string());
        let retrieved_info = registry.get_infos(&"1".to_string());

        assert_eq!(retrieved_product, Some(product.clone()));
        assert_eq!(retrieved_info, Some(product_info));
    }
}



