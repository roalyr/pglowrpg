// [Revision: v0.1] [Path: libs/lib_moves/src/moves/fate_moves.rs] [Date: 2025-12-13]
// Description: Fate and oracle moves for solo play

use lib_pseudo_rng::Rng;

/// Ask the Oracle: Simple yes/no with likelihood.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OracleAnswer {
    No,
    Yes,
}

/// Ask a yes/no question with a specific likelihood.
pub fn ask_oracle(rng: &mut Rng, likelihood: i8) -> OracleAnswer {
    let roll = rng.d10();
    let threshold = match likelihood {
        -2 => 3,  // Almost Certain (No)
        -1 => 4,  // Likely (No)
        0 => 5,   // 50/50
        1 => 6,   // Likely (Yes)
        2 => 7,   // Almost Certain (Yes)
        _ => 5,   // Default to 50/50
    };
    
    if roll >= threshold {
        OracleAnswer::Yes
    } else {
        OracleAnswer::No
    }
}

/// Pay the Price: Determine what happens when you fail.
pub fn pay_price(rng: &mut Rng) -> &'static str {
    let roll = rng.d10();
    match roll {
        1 => "A person or community you trusted loses faith in you, or acts against you.",
        2 => "A person or community you care about is exposed to danger.",
        3 => "You encounter signs of a looming threat.",
        4 => "You create an opportunity for an enemy.",
        5 => "You face a tough choice.",
        6 => "You face the consequences of an earlier choice.",
        7 => "A surprising development complicates your quest.",
        8 => "You are separated from something or someone.",
        9 => "Your action causes collateral damage or has an unintended effect.",
        10 => "Something of value is lost or destroyed.",
        _ => "An unexpected complication occurs.",
    }
}
