#![allow(dead_code)]

use std::fmt::Display;
const INPUT: &str = include_str!("input.txt");
const VALID_STRIPE_COLORS: [char; 5] = ['w', 'u', 'b', 'r', 'g'];

/// A stripe is a single color
type Stripe = char;

/// A towel is a pattern of [Stripe]s
#[derive(Debug)]
struct Towel(&'static str);

/// An unparsed design is string of unresolved [Stripe]s.
///
/// A UnparsedDesign can't become a [Design] until it is known
/// if it is possible to create
#[derive(Clone, Copy)]
struct UnparsedDesign(&'static str);

/// A design canidate contains all the possible [Towel] matches in a [Design].
///
/// We don't know at this point if the design is possble yet, as it only contains
/// the series of possible [Towel]s without considering order or uniquemess.
struct DesignCanidate(&'static str, Vec<Towel>);

/// A design is a ordered series of [Towel]s
struct Design(Vec<Towel>);

// Note: After working on attempt #1 I realized the terms in the prompt are
// somewhat (intentionally?) tricky:
//
// I keep wanting to call a "Towel" a "Towel
// Design", which is actually a pattern of towels, not the pattern ON the towel.

// 1. Create a list of all towels avaiable

// TODO: Probably can be done with filter, map
fn all_towels() -> Vec<Towel> {
    let (possible_towels, _) = INPUT.split_once("\n").unwrap();

    let possible_towels = possible_towels.split(", ").into_iter();

    let mut avaiable_towels = vec![];

    for towel in possible_towels {
        let is_valid = towel.chars().all(|c| VALID_STRIPE_COLORS.contains(&c));

        if is_valid {
            avaiable_towels.push(Towel(towel));
        } else {
            println!("{} contains invalid chars", towel);
            continue;
        }
    }

    avaiable_towels
}

// 2. Create a list of all desired designs

fn unparsed_designs() -> Vec<UnparsedDesign> {
    let (_, all_designs) = INPUT.split_once("\n").unwrap();

    all_designs
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| UnparsedDesign(s))
        .collect::<Vec<_>>()
}

fn design_canidate(design: UnparsedDesign) -> Option<DesignCanidate> {
    let all_towels = all_towels();
    let mut matched_towels: Vec<Towel> = vec![];

    for towel in all_towels {
        if design.0.contains(towel.0) {
            matched_towels.push(towel)
        } else {
            continue;
        }
    }

    if matched_towels.is_empty() {
        None
    } else {
        Some(DesignCanidate(design.0, matched_towels))
    }
}

// 3. Create a list of all _potential_ towel matches in a given design

fn main() {
    let all_design_candidates: Vec<_> = unparsed_designs()
        .into_iter()
        .filter_map(|d| design_canidate(d))
        .collect();
    for candidate in all_design_candidates {
        println!("{}", candidate);
    }
}

impl Display for DesignCanidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let towel_list: String = self.1.iter().map(|t| t.0).collect::<Vec<_>>().join(", ");
        write!(f, "{}: Possibly contains {}", self.0, towel_list)
    }
}

// --- Attempt 1 ---

// fn parse_towel_stripes(pattern_string: &str) -> TowelDesign {
//     pattern_string
//         .chars()
//         .map(|color_code| match color_code {
//             'w' => Stripe::White,
//             'u' => Stripe::Blue,
//             'b' => Stripe::Black,
//             'r' => Stripe::Red,
//             'g' => Stripe::Green,
//             _ => panic!("Invalid color code: {}", color_code),
//         })
//         .collect()
// }

// type TowelDesign = Vec<Stripe>;

// fn parse_towel_patterns(input: &str) -> Vec<TowelDesign> {
//     let patterns = input.split(", ").filter(|s| !s.is_empty()).collect();
// }

// fn avaiable_towel_patterns() -> Vec<TowelDesign> {
//     let (available_towels, _) = INPUT.split_once("\n").unwrap();
//     let available_towels = parse_towel_patterns(available_towels);
//     available_towels
//         .iter()
//         .map(|t| parse_towel_stripes(t))
//         .collect()
// }

// fn possible_towels_for_pattern(pattern_string: &str) -> Vec<TowelDesign> {
//     vec![]
// }

// fn pattern_is_possible(pattern_string: &str) -> bool {
//     false
// }

// fn main() {
//     // let (available_towels, desired_patterns) = INPUT.split_once("\n").unwrap();
//     // let available_towels = available_towels.split(", ");
//     // let desired_patterns = desired_patterns
//     //     .split('\n')
//     //     .filter(|s| !s.is_empty())
//     //     .collect::<Vec<_>>();
//     // println!("Available towels: {:?}", available_towels);
//     // println!("Available towels: {:?}", desired_patterns);
//     // println!("Desired patterns: {}", desired_patterns);

//     println!("{:?}", all_towels())
// }

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_parse_towel_stripes() {
    //     let pattern = "wubrgwubrg";
    //     let expected = vec![
    //         Stripe::White,
    //         Stripe::Blue,
    //         Stripe::Black,
    //         Stripe::Red,
    //         Stripe::Green,
    //         Stripe::White,
    //         Stripe::Blue,
    //         Stripe::Black,
    //         Stripe::Red,
    //         Stripe::Green,
    //     ];
    //     assert_eq!(parse_towel_stripes(pattern), expected);
    // }

    // #[test]
    // #[should_panic(expected = "Invalid color code: x")]
    // fn test_parse_towel_stripes_invalid_code() {
    //     parse_towel_stripes("wxubrg");
    // }

    // #[test]
    // fn test_avaiable_towel_patterns() {
    //     let patterns = parse_towel_patterns("ububwuu, ub, gbwbgr, rrur, ggbuwg, uuubrwb");

    //     assert_eq!(
    //         parse_towel_patterns("ububwuu, ub, gbwbgr, rrur, ggbuwg, uuubrwb"),
    //         vec!["ububwuu", "ub", "gbwbgr", "rrur", "ggbuwg", "uuubrwb"]
    //     )
    // }
}
