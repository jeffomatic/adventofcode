fn main() {
    let card_pubkey = 14082811;
    let door_pubkey = 5249543;
    // let card_pubkey: i64 = 5764801;
    // let door_pubkey: i64 = 17807724;

    let mut v: i64 = 1;
    let mut card_loop_size = 0;
    while v != card_pubkey {
        card_loop_size += 1;
        v = (v * 7) % 20201227;
    }

    println!("a loop size: {}", card_loop_size);

    v = 1;
    let mut door_loop_size = 0;
    while v != door_pubkey {
        door_loop_size += 1;
        v = (v * 7) % 20201227;
    }

    println!("b loop size: {}", door_loop_size);

    v = 1;
    for _ in 0..card_loop_size {
        v = (v * door_pubkey) % 20201227;
    }

    println!("key calculated from card: {}", v);

    v = 1;
    for _ in 0..door_loop_size {
        v = (v * card_pubkey) % 20201227;
    }

    println!("key calculated from door: {}", v);
}
