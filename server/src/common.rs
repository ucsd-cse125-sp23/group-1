use rand::{thread_rng,seq::IteratorRandom};

pub fn get_rand_from_vec<T>(vec: &mut Vec<T>) -> T {
    vec.swap_remove((0..vec.len()).choose(&mut thread_rng()).unwrap())
}