use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_random_string(length: usize) -> String {
    let rng = thread_rng();
    let random_string: String = rng
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    random_string
}
