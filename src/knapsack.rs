use rand;

#[derive(Debug, Clone)]
pub struct Item {
    pub value: u32,
    pub weight: u32,
}

#[derive(Debug)]
pub struct Knapsack {
    pub items: Vec<Item>,
    pub capacity: u32,
}

pub fn total_value(items:&Vec<Item>) -> u32 {items.iter().map(|item| item.value).sum()}
pub fn total_weight(items:&Vec<Item>) -> u32 {items.iter().map(|item| item.weight).sum()}

pub fn generate_items(n: usize) -> Vec<Item> {
    (0..n).map(|_| Item {
        value: rand::random::<u32>() % 100 +1,
        weight: rand::random::<u32>() %50 + 1,
    }).collect()
}

pub fn calculate_capacity_by_average(items: &[Item], multiplier: u32) -> u32 {
    let total_weight: u32 = items.iter().map(|item|item.weight).sum();
    let avg_weight = total_weight / items.len() as u32;
    avg_weight * multiplier

}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_items_vec() -> Vec<Item> {
        let item1 = Item {
            value: 50,
            weight: 5
        };
        let item2 = Item {
            value: 30,
            weight: 15
        };
        let item3 = Item {
            value: 70,
            weight: 9
        };
        let item4 = Item {
            value: 10,
            weight: 1
        };
        let items: Vec<Item> = vec![item1, item2, item3, item4];
        items
    }

    #[test]
    fn test_total_value() {
        let items: Vec<Item> = setup_items_vec();
        let knapsack = Knapsack { items, capacity: 100 };
        let total_val = total_value(&knapsack.items);
        assert_eq!(total_val, 160)
    }

    #[test]
    fn test_total_weight() {
        let items: Vec<Item> = setup_items_vec();
        let knapsack = Knapsack { items, capacity: 100 };
        let total_weight =total_weight(&knapsack.items);
        assert_eq!(total_weight, 30)
    }

    #[test]
    fn test_items_upper_bound() {
        let items = generate_items(150);
        for item in items.iter() {
            assert!(item.weight < 51);
            assert!(item.value < 101);
        }
    }

    #[test]
    fn test_items_lower_bound() {
        let items = generate_items(150);
        for item in items.iter() {
            assert!(item.weight > 0);
            assert!(item.value > 0);
        }
    }

    #[test]
    fn test_calc_capacity() {
        let items = setup_items_vec();
        let capacity = calculate_capacity_by_average(&items, 2);
        assert_eq!(capacity, 14)
    }
}