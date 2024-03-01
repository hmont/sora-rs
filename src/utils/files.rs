use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub async fn gen_filename(length: usize) -> String {
    let rng = thread_rng();
    let filename: String = rng.sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect();

    filename
}
