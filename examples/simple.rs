use const_map::ConstMap;

fn main() {
    let mut map: ConstMap<&'static str, usize, 3> = ConstMap::new(["Alice", "Bob", "Carol"], || 0);
    *map.get_mut("Alice").unwrap() = 1;
    *map.get_mut("Bob").unwrap() = 2;
    *map.get_mut("Carol").unwrap() = 3;
    println!("{:?}", map);
}
