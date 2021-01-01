#[derive(Copy, Clone, Debug)]
struct Moon {
    pos: [i64; 3],
    vel: [i64; 3],
}

impl Moon {
    fn potential(&self) -> i64 {
        self.pos.iter().fold(0, |acc, v| acc + v.abs())
    }

    fn kinetic(&self) -> i64 {
        self.vel.iter().fold(0, |acc, v| acc + v.abs())
    }

    fn energy(&self) -> i64 {
        self.potential() * self.kinetic()
    }
}

fn step(moons: &mut Vec<Moon>) {
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

fn main() {
    /*
    <x=1, y=2, z=-9>
    <x=-1, y=-9, z=-4>
    <x=17, y=6, z=8>
    <x=12, y=4, z=2>
    */
    let mut moons = vec![
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
    let steps = 1000;

    for _ in 0..steps {
        step(&mut moons);
    }

    println!("{}", moons.iter().fold(0, |acc, m| acc + m.energy()));
}
