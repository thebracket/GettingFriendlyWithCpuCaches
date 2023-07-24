use itertools::Itertools;

const COUNTRIES: [&str; 4] = ["AD", "BB", "CA", "DK"];

pub type Image = Box<[u8; 128 * 128]>;

pub struct User {
    login: String,
    active: bool,
    icon: Image,
    country: usize,
}

pub fn country_count(users: &[User]) -> Vec<(usize, usize)> {
    let mut result: Vec<usize> = users
        .iter()
        .filter(|user| user.active)
        .map(|user| user.country)
        .collect();
    result.sort_unstable();
    result.into_iter()
        .dedup_with_count()
        .collect()
}

pub fn init_users() -> Vec<User> {
    const SIZE: usize = 10_000;
    let mut users = Vec::with_capacity(SIZE);
    for i in 0..SIZE {
        users.push(User {
            login: String::new(),
            active: i % 5 > 0, // 20 % non active
            icon: Box::new([0; 128 * 128]),
            country: i % COUNTRIES.len(),
        });
    }
    users
}

/*fn main() {
    println!("Size of User: {} bytes", std::mem::size_of::<User>());
    let users = init_users();
    let now = Instant::now();
    let counts = country_count(&users);
    let elapsed = now.elapsed();
    println!("Completed in: {} nanos", elapsed.as_nanos());
    println!("Result: {:?}", counts);
}*/