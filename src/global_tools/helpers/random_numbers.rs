

pub fn get_random<T>(min: T, max: T) -> T
    where T: num_traits::float::Float + num_traits::cast::FromPrimitive 
{
    let min = min.ceil();
    let max = max.floor();

    let random: T = num_traits::cast::FromPrimitive::from_f64(js_sys::Math::random()).unwrap();
    let one_number: T = num_traits::cast::FromPrimitive::from_f64(1.0).unwrap();

    ((random * (max - min + one_number)) + min).floor()
}