use std::{
    collections::{HashMap, HashSet},
    io::{self, Read},
};

use regex::Regex;

type Image = Vec<Vec<char>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Variant {
    id: i64,
    rot: usize,
    fliph: bool,
    flipv: bool,
    n: String,
    s: String,
    w: String,
    e: String,
}

impl Variant {
    fn new(id: i64, image: &Image) -> Variant {
        let dim = image.len();
        Variant {
            id,
            rot: 0,
            fliph: false,
            flipv: false,
            n: image.first().unwrap().iter().collect(),
            s: image.last().unwrap().iter().collect(),
            w: image.iter().map(|line| line[0]).collect(),
            e: image.iter().map(|line| line[dim - 1]).collect(),
        }
    }

    fn rotate(&self) -> Variant {
        Variant {
            id: self.id,
            rot: self.rot + 1,
            fliph: false,
            flipv: false,
            n: self.w.chars().rev().collect(),
            s: self.e.chars().rev().collect(),
            w: self.s.clone(),
            e: self.n.clone(),
        }
    }

    fn fliph(&self) -> Variant {
        Variant {
            id: self.id,
            rot: self.rot,
            fliph: !self.fliph,
            flipv: self.flipv,
            n: self.n.chars().rev().collect(),
            s: self.s.chars().rev().collect(),
            w: self.e.clone(),
            e: self.w.clone(),
        }
    }

    fn flipv(&self) -> Variant {
        Variant {
            id: self.id,
            rot: self.rot,
            fliph: self.fliph,
            flipv: !self.flipv,
            n: self.s.clone(),
            s: self.n.clone(),
            w: self.w.chars().rev().collect(),
            e: self.e.chars().rev().collect(),
        }
    }

    fn variants(&self) -> Vec<Variant> {
        let a = self.clone();
        let b = a.rotate();
        let c = b.rotate();
        let d = c.rotate();
        vec![
            a.clone(),
            a.fliph(), // same as c.flipv()
            a.flipv(), // same as c.fliph()
            b.clone(),
            b.fliph(), // same as d.flipv()
            b.flipv(), // same as d.fliph()
            c,
            d,
        ]
    }
}

#[derive(Debug)]
struct Index {
    n: HashMap<String, HashSet<Variant>>,
    w: HashMap<String, HashSet<Variant>>,
}

impl Index {
    fn new(variants: Vec<Variant>) -> Index {
        let mut index = Index {
            n: HashMap::new(),
            w: HashMap::new(),
        };

        for v in variants {
            index
                .n
                .entry(v.n.clone())
                .or_insert_with(|| HashSet::new())
                .insert(v.clone());
            index
                .w
                .entry(v.w.clone())
                .or_insert_with(|| HashSet::new())
                .insert(v);
        }

        index
    }
}

#[derive(Debug, Clone)]
struct Composite {
    dim: usize,
    tiles: Vec<Variant>,
}

impl Composite {
    fn new(dim: usize, v: Variant) -> Composite {
        Composite {
            dim,
            tiles: vec![v],
        }
    }

    fn is_complete(&self) -> bool {
        self.tiles.len() == self.dim * self.dim
    }

    fn eastern_candidates(&self, n: usize, index: &Index) -> HashSet<Variant> {
        index
            .w
            .get(&self.tiles[n].e)
            .map_or_else(|| HashSet::new(), |set| set.clone())
    }

    fn southern_candidates(&self, n: usize, index: &Index) -> HashSet<Variant> {
        index
            .n
            .get(&self.tiles[n].s)
            .map_or_else(|| HashSet::new(), |set| set.clone())
    }

    fn next_candidates(&self, index: &Index) -> Vec<Composite> {
        let candidates: HashSet<Variant> = if self.tiles.len() < self.dim {
            // First row; only check for eastern matches
            self.eastern_candidates(self.tiles.len() - 1, index)
        } else if self.tiles.len() % self.dim == 0 {
            // First item in new row; only check for southern matches
            self.southern_candidates(self.tiles.len() - self.dim, index)
        } else {
            // Check for both eastern and southern matches
            self.eastern_candidates(self.tiles.len() - 1, index)
                .intersection(&self.southern_candidates(self.tiles.len() - self.dim, index))
                .cloned()
                .collect()
        };

        let used: HashSet<i64> = self.tiles.iter().map(|v| v.id).collect();
        candidates
            .iter()
            .filter(|v| !used.contains(&v.id))
            .map(|v| {
                let mut next = (*self).clone();
                next.tiles.push(v.clone());
                next
            })
            .collect()
    }

    fn search(&self, index: &Index) -> Option<Composite> {
        for c in self.next_candidates(index) {
            if c.is_complete() {
                return Some(c);
            }

            if let Some(res) = c.search(index) {
                return Some(res);
            }
        }

        None
    }

    fn to_image(&self, src_images: &HashMap<i64, Image>) -> Image {
        // Get full images for each tile, correctly rotated, flipped, and
        // trimmed.
        let tiles: Vec<Image> = self
            .tiles
            .iter()
            .map(|v| {
                let mut image: Image = src_images.get(&v.id).unwrap().iter().cloned().collect();

                for _ in 0..v.rot {
                    image = image_rotate(&image);
                }

                if v.fliph {
                    image = image_fliph(&image);
                }

                if v.flipv {
                    image = image_flipv(&image);
                }

                image_trim(&image)
            })
            .collect();

        let tile_len = tiles[0].len();
        let res_len = self.dim * tile_len;
        let mut res = vec![vec!['0'; res_len]; res_len];

        for ti in 0..self.dim {
            for tj in 0..self.dim {
                let tile = &tiles[ti * self.dim + tj];
                let base_i = ti * tile_len;
                let base_j = tj * tile_len;
                for i in 0..tile_len {
                    for j in 0..tile_len {
                        res[base_i + i][base_j + j] = tile[i][j];
                    }
                }
            }
        }

        res
    }
}

fn image_rotate(image: &Image) -> Image {
    let dim = image.len();
    let mut res = vec![vec!['0'; dim]; dim];
    for i in 0..dim {
        for j in 0..dim {
            res[j][dim - i - 1] = image[i][j];
        }
    }
    res
}

fn image_fliph(image: &Image) -> Image {
    let dim = image.len();
    let mut res = vec![vec!['0'; dim]; dim];
    for i in 0..dim {
        for j in 0..dim {
            res[i][dim - j - 1] = image[i][j];
        }
    }
    res
}

fn image_flipv(image: &Image) -> Image {
    let dim = image.len();
    let mut res = vec![vec!['0'; dim]; dim];
    for i in 0..dim {
        for j in 0..dim {
            res[dim - i - 1][j] = image[i][j];
        }
    }
    res
}

fn image_trim(image: &Image) -> Image {
    let dim = image.len();
    let mut res = vec![vec!['0'; dim - 2]; dim - 2];
    for i in 1..(dim - 1) {
        for j in 1..(dim - 1) {
            res[i - 1][j - 1] = image[i][j];
        }
    }
    res
}

fn find_composite(variants: &Vec<Variant>, dim: usize, index: &Index) -> Composite {
    for v in variants.iter() {
        if let Some(res) = Composite::new(dim, v.clone()).search(&index) {
            return res;
        }
    }

    unreachable!();
}

fn monster_offsets() -> Vec<(usize, usize)> {
    let monster = "                  # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
    let mut offsets: Vec<(usize, usize)> = Vec::new();
    for (i, line) in monster.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                offsets.push((i, j));
            }
        }
    }
    offsets
}

fn monster_positions(image: &Image) -> Vec<(usize, usize)> {
    let image_len = image.len();
    let mut res: Vec<(usize, usize)> = Vec::new();

    for i in 0..image_len {
        for j in 0..image_len {
            let mut ok = true;
            for (di, dj) in monster_offsets() {
                if i + di >= image_len || j + dj >= image_len || image[i + di][j + dj] != '#' {
                    ok = false;
                    break;
                }
            }

            if ok {
                res.push((i, j));
            }
        }
    }

    res
}

fn image_variants(image: &Image) -> Vec<Image> {
    let rot1 = image_rotate(image);
    let rot2 = image_rotate(&rot1);
    let rot3 = image_rotate(&rot2);

    vec![
        image.clone(),
        image_fliph(image), // same as rot2 flipv
        image_flipv(image), // same as rot2 fliph
        rot1.clone(),
        image_fliph(&rot1), // same as rot3 flipv
        image_flipv(&rot1), // same as rot3 fliph
        rot2,
        rot3,
    ]
}

fn image_roughness(image: &Image) -> u64 {
    for mut v in image_variants(image) {
        let monsters = monster_positions(&v);
        if monsters.len() == 0 {
            continue;
        }

        for (i, j) in monsters.iter() {
            for (di, dj) in monster_offsets() {
                v[i + di][j + dj] = '.';
            }
        }

        return v.iter().fold(0, |accum, row| {
            row.iter()
                .fold(accum, |accum, c| if *c == '#' { accum + 1 } else { accum })
        });
    }

    unreachable!()
}

fn main() {
    let tile_id_re = Regex::new(r#"Tile (\d+):"#).unwrap();

    let mut src_images: HashMap<i64, Image> = HashMap::new();
    for chunk in get_input().split("\n\n") {
        let lines: Vec<&str> = chunk.lines().collect();
        let (id_s, image_s) = (lines[0], &lines[1..]);

        let id: i64 = tile_id_re.captures(id_s).unwrap()[1].parse().unwrap();
        let image: Image = image_s.iter().map(|line| line.chars().collect()).collect();
        src_images.insert(id, image);
    }

    let dim = (src_images.len() as f64).sqrt() as usize;
    let all_variants: Vec<Variant> = src_images
        .iter()
        .flat_map(|(id, image)| Variant::new(*id, image).variants())
        .collect();
    let index = Index::new(all_variants.clone());

    let composite = find_composite(&all_variants, dim, &index);
    println!("{}", image_roughness(&composite.to_image(&src_images)));
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input).unwrap();
    return input.trim().to_string();
}
