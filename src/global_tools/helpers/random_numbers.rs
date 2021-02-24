use random_number::rand::distributions::uniform::SampleUniform;

pub fn get_random<T>(start: T, end: T) -> T
where
    T: SampleUniform,
{
    random!(start..=end)
}
