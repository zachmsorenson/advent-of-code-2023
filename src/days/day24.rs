const LOW: f32 = 200000000000000f32;
const HIGH: f32 = 400000000000000f32;

// at + bs + c = 0
struct Polynomial {
    a: f32,
    b: f32,
    c: f32,
}

#[derive(Debug)]
pub enum Error {
    Past,
}

pub struct PosVel3 {
    posx: f32,
    posy: f32,
    posz: f32,
    velx: f32,
    vely: f32,
    velz: f32,
}

pub struct Input {
    data: Vec<PosVel3>,
    low_bound: f32,
    high_bound: f32,
}

pub fn solve_system_part1(pv1: &PosVel3, pv2: &PosVel3) -> Result<(f32, f32), Error> {
    // at - bs + c = 0
    let x_sys = Polynomial {
        a: pv1.velx,
        b: -pv2.velx,
        c: pv1.posx - pv2.posx,
    };
    let y_sys = Polynomial {
        a: pv1.vely,
        b: -pv2.vely,
        c: pv1.posy - pv2.posy,
    };

    // solve for t using the x system of equations
    // t = bs + c
    let t_eq = Polynomial {
        a: 0f32,
        b: -x_sys.b / x_sys.a,
        c: -x_sys.c / x_sys.a,
    };

    // plug above into y equation to solve for s
    // at - bs + c = 0
    // a(b's + c') + bs + c = 0
    // ab's + ac' + bs + c = 0
    // (ab' + b)s = -(ac' + c)
    // s = -(ac' + c) / (ab' + b)
    let s = -(y_sys.a * t_eq.c + y_sys.c) / (y_sys.a * t_eq.b + y_sys.b);

    // backsolve for t using new s
    // t = bs + c
    let t = t_eq.b * s + t_eq.c;

    if s < 0f32 || t < 0f32 {
        return Err(Error::Past);
    }

    // println!("Collision at times (t, s) = ({}, {})", t, s);

    // now backsolve for the intersection point (x, y) using (t, s)
    let x = pv2.posx + pv2.velx * s;
    let y = pv2.posy + pv2.vely * s;

    Ok((x, y))
}

pub fn parse_input(input: &str) -> Input {
    let data = input
        .lines()
        .map(|line| {
            let mut split = line.split(&[',', '@']).map(|s| s.trim().parse().unwrap());

            let posx = split.next().unwrap();
            let posy = split.next().unwrap();
            let posz = split.next().unwrap();
            let velx = split.next().unwrap();
            let vely = split.next().unwrap();
            let velz = split.next().unwrap();

            PosVel3 {
                posx,
                posy,
                posz,
                velx,
                vely,
                velz,
            }
        })
        .collect();

    let mut input = Input {
        data,
        low_bound: LOW,
        high_bound: HIGH,
    };

    if input.data.len() < 10 {
        input.low_bound = 7f32;
        input.high_bound = 27f32;
    }
    input
}

#[allow(unused_variables)]
pub fn part1(input: &Input) -> Option<u64> {
    let mut count = 0;
    let bounds = input.low_bound..=input.high_bound;
    // println!("bounds: {:?}", bounds);
    for (i, pv1) in input.data.iter().enumerate() {
        for pv2 in input.data.iter().skip(i + 1) {
            let out = solve_system_part1(pv1, pv2);
            // println!("Out: {:?}", out);
            if let Ok((x, y)) = out {
                if bounds.contains(&x) && bounds.contains(&y) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

#[allow(unused_variables)]
pub fn part2(input: &Input) -> Option<u64> {
    None
}

#[cfg(test)]
pub mod tests {
    use super::*;
    const TEST_INPUT: &str = include_str!("../../input/day24/test.txt");
    #[test]
    fn test_day24_part1() {
        let input = parse_input(TEST_INPUT);

        let resp = part1(&input);

        assert_eq!(resp, Some(2));
    }

    #[test]
    fn test_day24_part2() {
        let input = parse_input(TEST_INPUT);

        let resp = part2(&input);

        assert_eq!(resp, None);
    }
}
