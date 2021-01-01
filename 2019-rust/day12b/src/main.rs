use num::integer::lcm;

#[derive(Copy, Clone, Debug)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

fn step(moons: &mut [Moon; 4]) {
    // gravity
    for a in 0..moons.len() {
        for b in (a + 1)..moons.len() {
            for n in 0..3 {
                if moons[a].pos[n] < moons[b].pos[n] {
                    moons[a].vel[n] += 1;
                    moons[b].vel[n] += -1;
                } else if moons[b].pos[n] < moons[a].pos[n] {
                    moons[a].vel[n] += -1;
                    moons[b].vel[n] += 1;
                }
            }
        }
    }

    // position
    for mut moon in moons {
        for n in 0..3 {
            moon.pos[n] += moon.vel[n];
        }
    }
}

fn extract_dim(moons: &[Moon; 4], dim: usize) -> [i64; 8] {
    [
        moons[0].pos[dim],
        moons[0].vel[dim],
        moons[1].pos[dim],
        moons[1].vel[dim],
        moons[2].pos[dim],
        moons[2].vel[dim],
        moons[3].pos[dim],
        moons[3].vel[dim],
    ]
}

fn main() {
    /*
    <x=1, y=2, z=-9>
    <x=-1, y=-9, z=-4>
    <x=17, y=6, z=8>
    <x=12, y=4, z=2>
    */
    let mut moons = [
        Moon {
            pos: [1, 2, -9],
            vel: [0, 0, 0],
        },
        Moon {
            pos: [-1, -9, -4],
            vel: [0, 0, 0],
        },
        Moon {
            pos: [17, 6, 8],
            vel: [0, 0, 0],
        },
        Moon {
            pos: [12, 4, 2],
            vel: [0, 0, 0],
        },
    ];

    let start_by_dim: Vec<_> = (0..3).map(|n| extract_dim(&moons, n)).collect();
    let mut cycle_sizes = vec![-1, -1, -1];
    let mut i = 0;

    loop {
        step(&mut moons);
        i += 1;

        for n in 0..3 {
            if cycle_sizes[n] == -1 {
                if start_by_dim[n] == extract_dim(&moons, n) {
                    cycle_sizes[n] = i;
                }
            }
        }

        if cycle_sizes.iter().all(|s| *s > 0) {
            println!(
                "{}",
                cycle_sizes.iter().fold(1, |acc, s| lcm(acc, *s as i64))
            );
            return;
        }
    }
}
