use rand::{thread_rng, Rng};

pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = thread_rng();

    rng.gen_range(min..max)
}

#[cfg(test)]
mod test {
    use super::random_range;

    #[test]
    fn test_random() {
        let r = random_range(0, 10);

        println!("{:?}", r);
    }
}
