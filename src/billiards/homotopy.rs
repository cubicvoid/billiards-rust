use crate::billiards::BaseSingularity::{self, B0, B1};

use enum_map::EnumMap;


pub struct Turn {
	// The base vertex this turn rotates around.
	pub s: BaseSingularity,

	// degree is the exponent of this turn relative to a given triangle's
	// rotation coefficients. if 
	pub degree: i64,
}

impl Turn {
	pub fn new(degree: i64, around: BaseSingularity) -> Turn {
		return Turn{s: around, degree: degree};
	}
}

pub struct TurnPath {
}

impl TurnPath {
	pub fn from_turns(turns: &[Turn]) -> TurnPath {
		return TurnPath{};
	}
}

fn make_turn_path_or_something() -> TurnPath {
	let mut turns = Vec::new();
	turns.push(Turn::new(3, B0));
	turns.push(Turn::new(5, B1));
	return TurnPath::from_turns(&turns);
}