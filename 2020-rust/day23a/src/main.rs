fn run(ring: &Vec<u32>) -> Vec<u32> {
    let mut next: Vec<u32> = Vec::new();

    let cur_label = ring[0];
    let removed = &ring[1..4];

    let mut dest_label = cur_label;
    while removed.contains(&dest_label) || dest_label == cur_label {
        dest_label -= 1;
        if dest_label == 0 {
            dest_label = ring.len() as u32;
        }
    }

    for &label in ring.iter().skip(4) {
        next.push(label);
        if label == dest_label {
            for &r in removed {
                next.push(r);
            }
        }
    }

    next.push(cur_label);

    next
}

fn main() {
    // let input = "389125467"; // test
    let input = "562893147"; // actual
    let mut ring: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for _ in 0..100 {
        ring = run(&ring);
    }

    while ring[0] != 1 {
        ring.rotate_left(1);
    }

    println!(
        "{}",
        ring.iter()
            .skip(1)
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("")
    );
}
