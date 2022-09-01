use core::panic;
use regex::{Error, Regex};

#[allow(dead_code)]
struct Dice {
    sides: i32,
    count: i32,
    is_dice: bool,
}

enum QueryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[allow(dead_code)]
impl QueryOperator {
    const fn as_str(&self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
        }
    }
}

#[allow(dead_code)]
struct DiceQuery<'a> {
    pub dice: Vec<Dice>,
    pub operations: Vec<QueryOperator>,
    pub result: i32,
    query: &'a str,
}

impl DiceQuery<'_> {
    pub const fn new(query: &str) -> DiceQuery<'_> {
        DiceQuery {
            dice: Vec::<Dice>::new(),
            operations: Vec::<QueryOperator>::new(),
            result: -1,
            query,
        }
    }

    pub fn parse_query(&mut self) -> Result<&DiceQuery<'_>, Error> {
        let num_regex = Regex::new(r"\d*d?\d+")?;
        let oper_regex = Regex::new("([\\*/\\-\\+])")?;

        self.dice = num_regex
            .find_iter(self.query)
            .map(|x| parse_dice(x.as_str()))
            .collect();

        self.operations = oper_regex
            .find_iter(self.query)
            .map(|o| match o.as_str() {
                "*" => QueryOperator::Multiply,
                "/" => QueryOperator::Divide,
                "-" => QueryOperator::Subtract,
                "+" => QueryOperator::Add,
                _ => panic!("invalid math operator, aborting"),
            })
            .collect();

        Ok(self)
    }
}

fn parse_dice(dice: &str) -> Dice {
    let parts: Vec<&str> = dice.split('d').collect();
    let count = match parts[0].parse::<i32>() {
        Ok(c) => c,
        Err(_) => i32::MIN,
    };
    let sides = match parts[1].parse::<i32>() {
        Ok(s) => s,
        Err(_) => i32::MIN,
    };

    assert!(
        !(count == i32::MIN && sides == i32::MIN),
        "INVALID DICE FORMAT"
    );

    if count == i32::MIN && sides != i32::MIN {
        return Dice {
            sides: count,
            count: 1,
            is_dice: false,
        };
    }

    if sides == i32::MIN && count != i32::MIN {
        return Dice {
            sides,
            count: 1,
            is_dice: true,
        };
    }

    Dice {
        count,
        sides,
        is_dice: true,
    }
}

fn main() {
    let mut query = DiceQuery::new("1d6 + 3d4 * d20");
    match query.parse_query() {
        Ok(_) => {}
        Err(error) => panic!("{}", error),
    };
}
