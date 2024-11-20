use rand::{self, distributions, Rng};
use rand::distributions::{Bernoulli, Distribution};

pub struct Chromosome {
    pub genes:Vec<bool>
}

impl Chromosome {
    pub fn count_genes(&self) -> usize {
        self.genes.len()
    }
    pub fn new(distribution:&Bernoulli, rng:&mut impl Rng, genes_num: usize) -> Self{
        let genes: Vec<bool> = (0..genes_num).map(|_| distribution.sample(rng)).collect();
        Self{genes}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_genes(){
        let test_chrom = Chromosome{
            genes: vec![true, false, true,true,false]
        };
        assert_eq!(5, test_chrom.count_genes())
    }

    #[test]
    fn test_new_chromosome(){
        let dist = distributions::Bernoulli::new(0.5).unwrap();
        let mut rng = rand::thread_rng();
        let chromosome = Chromosome::new(&dist, & mut rng, 5);
        assert_eq!(chromosome.genes.len(), 5)
    }

    #[test]
    fn test_empty_chromosome(){
        let dist = distributions::Bernoulli::new(0.5).unwrap();
        let mut rng = rand::thread_rng();
        let chromosome = Chromosome::new(&dist, & mut rng, 0);
        assert_eq!(chromosome.genes.len(), 0)

    }
    #[test]
    fn test_chromosome_p(){
        let dist = distributions::Bernoulli::new(0.5).unwrap();
        let mut rng = rand::thread_rng();
        let genes_num = 500000;
        let chromosome = Chromosome::new(&dist, & mut rng, genes_num);
        let count_true = chromosome.genes.into_iter().filter(|i| *i == true).count();
        let freq_rounded = ((count_true as f64/genes_num as f64) * 100.0).round();
        assert_eq!(freq_rounded as i32, 50);

    }

}