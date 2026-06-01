use std::collections::HashMap;
// Mr. Right tells truth
// Mr. Wrong tells lies
//
// one Mr.Wrong
// everyone else Mr.Right

//can I arrange the queue so that everyone except this person tells the truth, and this person's own
//statements are false?

//return mr. wrong

#[derive(Clone)]
enum KindOfStatement {
    //of statement
    Pos(usize),    //"I'm in 2nd position"
    Front(usize),  // "The man in front of me is Bob"
    Behind(usize), // "The man behind me is Bob"
}

#[derive(Clone)]
struct Claim {
    speaker: usize,
    kind_of_statement: KindOfStatement,
}

#[derive(Clone)]
struct Comp {
    nodes: Vec<usize>,
    fixed_start: Option<usize>,
}

fn num(s: &str) -> usize {
    s.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn id<'a>(name: &'a str, ids: &mut HashMap<&'a str, usize>, names: &mut Vec<&'a str>) -> usize {
    if let Some(&v) = ids.get(name) {
        v
    } else {
        let v = names.len();
        ids.insert(name, v);
        names.push(name);
        v
    }
}

pub fn find_out_mr_wrong<'a>(conversation: &[&'a str]) -> Option<&'a str> {
    let mut ids = HashMap::new();
    let mut names = Vec::new();
    let mut claims = Vec::new();

    for line in conversation {
        let (speaker, text) = line.split_once(':').unwrap();
        let speaker = id(speaker, &mut ids, &mut names);

        if text.starts_with("I'm in ") {
            claims.push(Claim {
                speaker,
                kind_of_statement: KindOfStatement::Pos(num(text)),
            });
        } else if text.contains("people in front of me") {
            let k = text
                .split_whitespace()
                .nth(2)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            claims.push(Claim {
                speaker,
                kind_of_statement: KindOfStatement::Pos(k + 1),
            });
        } else if text.contains("people behind me") {
            let k = text
                .split_whitespace()
                .nth(2)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            claims.push(Claim {
                speaker,
                kind_of_statement: KindOfStatement::Pos(usize::MAX - k),
            });
        } else if text.starts_with("The man behind me is ") {
            let other = text
                .trim_start_matches("The man behind me is ")
                .trim_end_matches('.');
            let other = id(other, &mut ids, &mut names);
            claims.push(Claim {
                speaker,
                kind_of_statement: KindOfStatement::Behind(other),
            });
        } else if text.starts_with("The man in front of me is ") {
            let other = text
                .trim_start_matches("The man in front of me is ")
                .trim_end_matches('.');
            let other = id(other, &mut ids, &mut names);
            claims.push(Claim {
                speaker,
                kind_of_statement: KindOfStatement::Front(other),
            });
        }
    }

    let n = names.len();
    let mut possible = Vec::new();

    for wrong in 0..n {
        if possible_with_wrong(wrong, n, &claims) {
            possible.push(wrong);
        }
    }

    if possible.len() == 1 {
        Some(names[possible[0]])
    } else {
        None
    }
}

fn real_pos(k: usize, n: usize) -> usize {
    if k > n { n - (usize::MAX - k) } else { k }
}

fn possible_with_wrong(wrong: usize, n: usize, claims: &[Claim]) -> bool {
    let mut fixed = vec![None::<usize>; n];
    let mut next = vec![None::<usize>; n];
    let mut prev = vec![None::<usize>; n];

    for c in claims.iter().filter(|c| c.speaker != wrong) {
        match c.kind_of_statement {
            KindOfStatement::Pos(p) => {
                let p = real_pos(p, n);

                if p == 0 || p > n {
                    return false;
                }

                if matches!(fixed[c.speaker], Some(x) if x != p) {
                    return false;
                }

                fixed[c.speaker] = Some(p);
            }

            KindOfStatement::Behind(b) => {
                if c.speaker == b {
                    return false;
                }

                if matches!(next[c.speaker], Some(x) if x != b)
                    || matches!(prev[b], Some(x) if x != c.speaker)
                {
                    return false;
                }

                next[c.speaker] = Some(b);
                prev[b] = Some(c.speaker);
            }

            KindOfStatement::Front(f) => {
                if c.speaker == f {
                    return false;
                }

                if matches!(prev[c.speaker], Some(x) if x != f)
                    || matches!(next[f], Some(x) if x != c.speaker)
                {
                    return false;
                }

                prev[c.speaker] = Some(f);
                next[f] = Some(c.speaker);
            }
        }
    }

    let mut seen = vec![false; n];
    let mut comps = Vec::<Comp>::new();

    for i in 0..n {
        if prev[i].is_none() && !seen[i] {
            let mut nodes = Vec::new();
            let mut cur = i;

            loop {
                if seen[cur] {
                    return false;
                }

                seen[cur] = true;
                nodes.push(cur);

                if let Some(nx) = next[cur] {
                    cur = nx;
                } else {
                    break;
                }
            }

            let mut start = None;

            for (off, &person) in nodes.iter().enumerate() {
                if let Some(p) = fixed[person] {
                    if p <= off {
                        return false;
                    }

                    let s = p - off;

                    if matches!(start, Some(x) if x != s) {
                        return false;
                    }

                    start = Some(s);
                }
            }

            if let Some(s) = start {
                if s == 0 || s + nodes.len() - 1 > n {
                    return false;
                }
            }

            comps.push(Comp {
                nodes,
                fixed_start: start,
            });
        }
    }

    if seen.iter().any(|&x| !x) {
        return false;
    }

    let mut person_comp = vec![0usize; n];
    let mut person_off = vec![0usize; n];

    for (ci, comp) in comps.iter().enumerate() {
        for (off, &p) in comp.nodes.iter().enumerate() {
            person_comp[p] = ci;
            person_off[p] = off;
        }
    }

    let mut starts = vec![0usize; comps.len()];
    let all = (1u64 << n) - 1;

    fn dfs(
        mask: u64,
        n: usize,
        comps: &[Comp],
        starts: &mut [usize],
        wrong: usize,
        claims: &[Claim],
        person_comp: &[usize],
        person_off: &[usize],
        all: u64,
    ) -> bool {
        if mask == all {
            return claims.iter().filter(|c| c.speaker == wrong).all(|c| {
                let pos = |p: usize| starts[person_comp[p]] + person_off[p];

                match c.kind_of_statement {
                    KindOfStatement::Pos(k) => pos(c.speaker) != real_pos(k, n),
                    KindOfStatement::Behind(b) => pos(b) != pos(c.speaker) + 1,
                    KindOfStatement::Front(f) => pos(f) + 1 != pos(c.speaker),
                }
            });
        }

        let first_free = (0..n).find(|&i| mask & (1u64 << i) == 0).unwrap();
        let start = first_free + 1;

        for ci in 0..comps.len() {
            if starts[ci] != 0 {
                continue;
            }

            let comp = &comps[ci];

            if matches!(comp.fixed_start, Some(s) if s != start) {
                continue;
            }

            let len = comp.nodes.len();

            if start + len - 1 > n {
                continue;
            }

            let bits = ((1u64 << len) - 1) << first_free;

            if mask & bits != 0 {
                continue;
            }

            starts[ci] = start;

            if dfs(
                mask | bits,
                n,
                comps,
                starts,
                wrong,
                claims,
                person_comp,
                person_off,
                all,
            ) {
                return true;
            }

            starts[ci] = 0;
        }

        false
    }

    dfs(
        0,
        n,
        &comps,
        &mut starts,
        wrong,
        claims,
        &person_comp,
        &person_off,
        all,
    )
}

fn main() {
    println!("Mr. Wrong problem...")
}
