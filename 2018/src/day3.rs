mod common;

struct Cut {
    id: usize,
    left: usize,
    top: usize,
    width: usize,
    height: usize,
}

impl Cut {
    fn parse(s: String) -> Cut {
        let e: Vec<usize> = s
            .split(['#', '@', ',', ':', 'x'].as_ref())
            .filter(|t| !t.is_empty())
            .map(|t| t.trim().parse::<usize>().expect("Could not parse number"))
            .collect();
        Cut {
            id: e[0],
            left: e[1],
            top: e[2],
            width: e[3],
            height: e[4],
        }
    }
}

#[allow(dead_code)]
fn main() {
    let cuts: Vec<Cut> = common::get_lines().into_iter().map(Cut::parse).collect();

    const N: usize = 1000;
    let mut fabric: [[u32; N]; N] = [[0; N]; N];

    for c in &cuts {
        for x in 0..c.width {
            for y in 0..c.height {
                fabric[c.left + x][c.top + y] += 1;
            }
        }
    }
    let overlapping = fabric
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&i| i >= 2)
        .count();
    println!("Overlapping inches: {}", overlapping);

    for c in &cuts {
        let mut non_overlapping = true;
        'cut_loop: for x in 0..c.width {
            for y in 0..c.height {
                if fabric[c.left + x][c.top + y] > 1 {
                    non_overlapping = false;
                    break 'cut_loop;
                }
            }
        }
        if non_overlapping {
            println!("ID of non-overlapping cut: {}", c.id);
        }
    }
}
