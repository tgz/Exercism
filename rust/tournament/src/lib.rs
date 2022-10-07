use std::collections::HashMap;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";

#[derive(Default)]
struct Team {
    name: String,
    wins: u8,
    draws: u8,
    losses: u8,
}

enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl Team {
    fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    fn matches(&self) -> i32 {
        self.wins as i32 + self.draws as i32 + self.losses as i32
    }

    fn points(&self) -> i32 {
        self.wins as i32 * 3 + self.draws as i32
    }

    fn win(&mut self) {
        self.wins += 1;
    }

    fn loss(&mut self) {
        self.losses += 1;
    }

    fn draw(&mut self) {
        self.draws += 1;
    }

    fn add_match(&mut self, result: &MatchResult) {
        match result {
            MatchResult::Win => self.win(),
            MatchResult::Draw => self.draw(),
            MatchResult::Loss => self.loss(),
        }
    }
}

impl From<&Team> for String {
    fn from(team: &Team) -> Self {
        format!(
            "{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team.name,
            team.matches(),
            team.wins,
            team.draws,
            team.losses,
            team.points()
        )
    }
}

impl From<&str> for MatchResult {
    fn from(origin: &str) -> Self {
        match origin {
            "win" => MatchResult::Win,
            "draw" => MatchResult::Draw,
            "loss" => MatchResult::Loss,
            _ => panic!("can not parse result flag!"),
        }
    }
}

impl MatchResult {
    fn reverse(&self) -> Self {
        match self {
            MatchResult::Win => MatchResult::Loss,
            MatchResult::Draw => MatchResult::Draw,
            MatchResult::Loss => MatchResult::Win,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut scores: HashMap<String, Team> = HashMap::new();
    match_results.lines().for_each(|line| {
        let parse: Vec<&str> = line.split(";").collect();
        let team = parse[0];
        let target = parse[1];
        let result = parse[2].into();

        scores
            .entry(team.into())
            .or_insert(Team::new(team.into()))
            .add_match(&result);

        scores
            .entry(target.into())
            .or_insert(Team::new(target.into()))
            .add_match(&result.reverse());
    });

    let mut score_values: Vec<&Team> = scores.values().collect();
    score_values.sort_by(|x, y| {
        y.points()
            .cmp(&x.points())
            .then_with(|| x.name.cmp(&y.name))
    });

    vec![String::from(HEADER)]
        .into_iter()
        .chain(score_values.into_iter().map(|t| t.into()))
        .collect::<Vec<String>>()
        .join("\n")
}
