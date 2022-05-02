use ordered_map::OrderedMap;

//按照值 (V)的大小排列
fn to_comparable(t: &usize) -> usize {
    *t
}

fn main() {
    let mut channels = OrderedMap::new(to_comparable);
    channels.insert(&"1", 1);
    channels.insert(&"2", 2);
    channels.insert(&"3", 3);
    channels.insert(&"4", 4);
    println!("{:?}", channels);
}
