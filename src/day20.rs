use utils;
use utils::Part;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Clone)]
struct Particle {
    x:(i64, i64, i64),
    v:(i64, i64, i64),
    a:(i64, i64, i64),
}

impl Particle {
    fn distance(&self) -> i64 {
        self.x.0.abs() + self.x.1.abs() + self.x.2.abs()
    }

    fn update(&mut self) {
        self.v.0 += self.a.0;
        self.v.1 += self.a.1;
        self.v.2 += self.a.2;
        self.x.0 += self.v.0;
        self.x.1 += self.v.1;
        self.x.2 += self.v.2;
    }
}

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 20).unwrap();

    let mut out = 0;

    let mut particles = HashMap::new();

    let mut cleaned_input = input
        .replace(|c| {c=='p' || c=='=' || c=='<' || c=='>' || c=='v' || c=='a'}, "");

    for (i, line) in cleaned_input.lines().enumerate() {
        let tokens:Vec<i64> = line.split(',')
            .map(|s| s.trim().parse().unwrap()).collect();
        particles.insert(i, Particle{x:(tokens[0], tokens[1], tokens[2]),
                                        v:(tokens[3], tokens[4], tokens[5]),
                                        a:(tokens[6], tokens[7], tokens[8])});
    }

    for _ in 0..1_000 { // there's probably a better way to estimate how many iterations you need
        for particle in particles.values_mut() {
            particle.update();
        }

        match part {
            Part::PartOne => {},
            Part::PartTwo => {
                let mut remove_set = Vec::new();
                for i in particles.clone().keys() {
                    for j in particles.keys() {
                        if *i==*j {continue;}
                        if particles.get(&i).unwrap().x == particles.get(&j).unwrap().x {
                            remove_set.push(*i);
                            remove_set.push(*j);
                        }
                    }
                }
                remove_set.sort();
                remove_set.dedup_by(|a, b| a==b);
                for particle in remove_set {
                    particles.remove(&particle);
                }
            },
        }
    }

    match part {
        Part::PartOne => {
            let distances = particles.iter()
                .map(|(k, v)| (k, v.distance()));

            let mut min = <i64>::max_value();
            let mut closest: usize = 0;

            for (i, d) in distances {
                if d < min {
                    min = d;
                    closest = *i;
                }
            }

            out = closest as i32;
        },
        Part::PartTwo => {
            out = particles.len() as i32;
        },
    }

    out
}