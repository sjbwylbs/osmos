pub trait Fitness {
    fn fitness(&self) -> isize;
}

pub fn selection(rng: &mut rand::rngs::ThreadRng, object_list: &[impl Fitness]) -> usize {
    let total_fitness = object_list.iter().map(|o| o.fitness()).sum::<isize>();
    loop {
        let random_index = rand::Rng::gen_range(rng, 0..object_list.len());
        let fitness = object_list[random_index].fitness();
        let probability = fitness as f64 / total_fitness as f64;
        println!("{probability}");
        let win = rand::Rng::gen_bool(rng, probability);
        if win {
            return random_index;
        }
    }
}

pub fn crossover() {
    todo!()
}

pub fn mutation() {
    todo!()
}

pub fn evole() {
    todo!()
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test() {
    // }
}
