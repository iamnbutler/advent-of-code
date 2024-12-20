#![allow(dead_code)]

use std::collections::HashSet;
const INPUT: &str = include_str!("input.txt");

// Attempt 3

fn main() {
    let (towels, designs) = INPUT.split_once("\n").expect("Couldn't parse input");
    let mut possible_designs: Vec<&str> = Vec::new();

    let all_towels: HashSet<&str> = towels.split(", ").collect();
    let desired_designs: Vec<&str> = designs.split('\n').filter(|s| !s.is_empty()).collect();
    let desired_designs_len = desired_designs.clone().len();

    for design in desired_designs {
        let mut remaining_stripes = design.to_string();

        while let Some(towel) = all_towels
            .iter()
            .find(|&&towel| remaining_stripes.contains(towel))
        {
            remaining_stripes = remaining_stripes.replace(towel, "");
            println!("{}", remaining_stripes)
        }
        if remaining_stripes.is_empty() {
            possible_designs.push(design);
        }
    }

    println!(
        "{} of {} total designs are are possible!",
        possible_designs.len(),
        desired_designs_len
    )
}
// submitted anser: 342 of 400 total designs are are possible!
// ❌ incorrect
//
// Ran out of time to continue

// Attempt 2

// const VALID_STRIPE_COLORS: [char; 5] = ['w', 'u', 'b', 'r', 'g'];

// /// A stripe is a single color
// type Stripe = char;

// /// A towel is a pattern of [Stripe]s
// #[derive(Debug, Copy, Clone)]
// struct Towel(&'static str);

// /// An unparsed design is string of unresolved [Stripe]s.
// ///
// /// A UnparsedDesign can't become a [Design] until it is known
// /// if it is possible to create
// #[derive(Clone, Copy)]
// struct UnparsedDesign(&'static str);

// /// A design canidate contains all the possible [Towel] matches in a [Design].
// ///
// /// We don't know at this point if the design is possble yet, as it only contains
// /// the series of possible [Towel]s without considering order or uniquemess.
// #[derive(Debug, Clone)]
// struct DesignCanidate(&'static str, Vec<Towel>);

// /// A design is a ordered series of [Towel]s
// struct Design(Vec<Towel>);

// // Note: After working on attempt #1 I realized the terms in the prompt are
// // somewhat (intentionally?) tricky:
// //
// // I keep wanting to call a "Towel" a "Towel
// // Design", which is actually a pattern of towels, not the pattern ON the towel.

// // 1. Create a list of all towels avaiable

// // TODO: Probably can be done with filter, map
// fn all_towels() -> Vec<Towel> {
//     let (possible_towels, _) = INPUT.split_once("\n").unwrap();

//     let possible_towels = possible_towels.split(", ").into_iter();

//     let mut avaiable_towels = vec![];

//     for towel in possible_towels {
//         let is_valid = towel.chars().all(|c| VALID_STRIPE_COLORS.contains(&c));

//         if is_valid {
//             avaiable_towels.push(Towel(towel));
//         } else {
//             println!("{} contains invalid chars", towel);
//             continue;
//         }
//     }

//     avaiable_towels
// }

// // 2. Create a list of all desired designs

// fn unparsed_designs() -> Vec<UnparsedDesign> {
//     let (_, all_designs) = INPUT.split_once("\n").unwrap();

// all_designs
//     .split('\n')
//     .filter(|s| !s.is_empty())
//     .map(|s| UnparsedDesign(s))
//     .collect::<Vec<_>>()
// }

// fn design_canidate(all_towels: Vec<Towel>, design: UnparsedDesign) -> DesignCanidate {
//     let mut matched_towels: Vec<Towel> = vec![];

//     for towel in all_towels {
//         if design.0.contains(towel.0) {
//             matched_towels.push(towel)
//         } else {
//             continue;
//         }
//     }

//     DesignCanidate(design.0, matched_towels)
// }

// fn resolve_design(canidate: DesignCanidate) -> Option<Design> {
//     let design = canidate.0;
//     let possible_towels = canidate.1;
//     let mut towels = vec![];

//     let mut remaining_stripes = design;

//     while !remaining_stripes.is_empty() {
//         let possible_towels = possible_towels.clone();
//         let last_towel = possible_towels.last().unwrap().0;

//         for pt in possible_towels {
//             if remaining_stripes.starts_with(pt.0) {
//                 towels.push(pt.clone());
//                 remaining_stripes = remaining_stripes
//                     .strip_prefix(pt.0)
//                     .unwrap_or(remaining_stripes);
//             } else {
//                 if pt.0 == last_towel {
//                     break;
//                 }
//             }
//         }
//     }

//     if towels.is_empty() {
//         None
//     } else {
//         Some(Design(towels))
//     }
// }

// // 3. Create a list of all _potential_ towel matches in a given design

// fn main() {
//     let all_towels = all_towels();
//     let mut possible_designs = vec![];
//     let all_design_candidates: Vec<_> = unparsed_designs()
//         .into_iter()
//         .map(|d| design_canidate(all_towels.clone(), d))
//         .collect();
//     for candidate in all_design_candidates {
//         if let Some(design) = resolve_design(candidate) {
//             possible_designs.push(design);
//         }
//     }
//     println!("{} designs are possible!", possible_designs.len())
// }

// impl Display for DesignCanidate {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let towel_list: String = self.1.iter().map(|t| t.0).collect::<Vec<_>>().join(", ");
//         write!(f, "{}: Possibly contains {}", self.0, towel_list)
//     }
// }

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
