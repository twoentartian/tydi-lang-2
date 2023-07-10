use std::sync::atomic::AtomicUsize;

pub trait GetName {
    fn get_name(&self) -> String;
}

static mut generate_counter: AtomicUsize = AtomicUsize::new(0);

pub fn generate_random_str(length: usize) -> String {
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;
    let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(length)
            .map(char::from)
            .collect();
    let counter;
    unsafe {
        generate_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        counter = generate_counter.load(std::sync::atomic::Ordering::SeqCst);
    }
    return format!("{}_{}", rand_string, counter);
}

pub fn generate_init_name() -> String {
    return String::from("???");
}