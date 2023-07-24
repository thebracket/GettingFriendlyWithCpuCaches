use std::{collections::HashMap, time::Instant};

pub type Image = Box<[u8; 128 * 128]>;

pub struct User {
    login: String,
    active: bool,
    icon: Image,
    country: String,
}

pub fn country_count(users: &[User]) -> HashMap<String, usize> {
    let mut result = HashMap::new();

    users
        .iter()
        .filter(|user| user.active)
        .for_each(|user| {
            let count = result.entry(user.country.clone()).or_insert(0);
            *count += 1;
        });

    result
}

pub fn init_users() -> Vec<User> {
    const SIZE: usize = 10_000;
    const COUNTRIES: [&str; 4] = ["AD", "BB", "CA", "DK"];
    let mut users = Vec::with_capacity(SIZE);
    for i in 0..SIZE {
        users.push(User {
            login: String::new(),
            active: i % 5 > 0, // 20 % non active
            icon: Box::new([0; 128 * 128]),
            country: COUNTRIES[i % COUNTRIES.len()].to_string(),
        });
    }
    users
}