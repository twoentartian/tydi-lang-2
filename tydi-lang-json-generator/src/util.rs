pub trait GetName {
    fn get_name(&self) -> String;
}

pub fn generate_random_str(length: usize) -> String {
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
    return rand_string;
}

pub fn generate_init_name() -> String {
    return String::from("???");
}