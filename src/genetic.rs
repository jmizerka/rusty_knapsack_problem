
use rand::{self, Rng};
use rand::distributions::{Bernoulli, Distribution};
use crate::knapsack;

pub struct Chromosome {
    pub genes:Vec<bool>,
    pub fitness: u32
}

impl Chromosome {
    pub fn count_genes(&self) -> usize {
        self.genes.len()
    }
    pub fn new(distribution: &Bernoulli, rng: &mut impl Rng, genes_num: usize) -> Self {
        let genes: Vec<bool> = (0..genes_num).map(|_| distribution.sample(rng)).collect();
        Self { genes, fitness: 0 }
    }
    pub fn calculate_weight(genes: &Vec<bool>, items: &Vec<knapsack::Item>) -> u32 {
        let mut weight: u32 = 0;
        for idx in 0..items.len() {
            if genes[idx] == true {
                weight += items[idx].weight;
            }
        }
        weight
    }

    pub fn calculate_fitness(genes:&Vec<bool>, items: &Vec<knapsack::Item>, capacity: u32) -> u32{
        let weight = Self::calculate_weight(genes, items);
        let mut fitness:u32 = 0;
        if weight <= capacity {
            for idx in 0..items.len() {
                if genes[idx] == true {
                    fitness += items[idx].value;
                }
            }
        }
        fitness
    }

     }
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_genes(){
        let test_chrom = Chromosome{
            genes: vec![true, false, true,true,false], fitness:0
        };
        assert_eq!(5, test_chrom.count_genes())
    }

    #[test]
    fn test_new_chromosome(){
        let dist = Bernoulli::new(0.5).unwrap();
        let mut rng = rand::thread_rng();
        let chromosome = Chromosome::new(&dist, & mut rng, 5);
        assert_eq!(chromosome.genes.len(), 5)
    }

    #[test]
    fn test_empty_chromosome(){
        let dist = Bernoulli::new(0.5).unwrap();
        let mut rng = rand::thread_rng();
        let chromosome = Chromosome::new(&dist, & mut rng, 0);
        assert_eq!(chromosome.genes.len(), 0)

    }
    #[test]
    fn test_chromosome_p(){
        let dist = Bernoulli::new(0.5).unwrap();
        let mut rng = rand::thread_rng();
        let genes_num = 500000;
        let chromosome = Chromosome::new(&dist, & mut rng, genes_num);
        let count_true = chromosome.genes.into_iter().filter(|i| *i == true).count();
        let freq_rounded = ((count_true as f64/genes_num as f64) * 100.0).round();
        assert_eq!(freq_rounded as i32, 50);
    }

    #[test]
    fn test_chromosome_weight(){
        let chromosome = Chromosome{genes: vec![true, false, true], fitness: 0};
        let item1 = knapsack::Item{value:1,weight:50};
        let item2 = knapsack::Item{value:1,weight:51};
        let item3 = knapsack::Item{value:1,weight:5};
        let items = vec![item1,item2,item3];
        let weight = Chromosome::calculate_weight(&chromosome.genes, &items);
        assert_eq!(weight, 55)
    }

    #[test]
    fn test_fitness_right_weight(){
        let chromosome = Chromosome{genes: vec![true, false, true], fitness: 0};
        let item1 = knapsack::Item{value:1,weight:50};
        let item2 = knapsack::Item{value:21,weight:51};
        let item3 = knapsack::Item{value:13,weight:5};
        let items = vec![item1,item2,item3];
        let fitness = Chromosome::calculate_fitness(&chromosome.genes, &items, 55);
        assert_eq!(fitness, 14)
    }

    #[test]
    fn test_fitness_wrong_weight(){
        let chromosome = Chromosome{genes: vec![true, false, true], fitness: 0};
        let item1 = knapsack::Item{value:1,weight:50};
        let item2 = knapsack::Item{value:21,weight:51};
        let item3 = knapsack::Item{value:13,weight:1};
        let items = vec![item1,item2,item3];
        let fitness = Chromosome::calculate_fitness(&chromosome.genes, &items, 50);
        assert_eq!(fitness, 0)
    }

}