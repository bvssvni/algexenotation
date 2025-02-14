/*
# Counting Primbix Levels

This example counts primbix levels.

A primbix level `k` is a set of numbers that have some primbix value `k` or higher.

The benefit of using a primbix level is that a level `k+1` is a subset of level `k`.

Counting primbix levels up to some N natural number is hard,
but one can make it easier by precounting records that can be used as offset for targeted counting.
It does not matter where these records are exactly, because one can use them to generate more
detailed counting for various purposes.
This program is designed to output candidates for records that is inserted as comments.
The comments can be copy & pasted and reformatted for various purposes.
Usually, only the last record is inserted as a comment.

Instead of printing out the counts of primbix levels at regular intervals,
one checks whether some relative ratio of offset logarithmic integral has produced some new maximum.
It helps reminding to reset the program and record the counts.

*/

use algexenotation::primbix;

fn main() {
    let mut primbixes = // [0; 16];
    // [13452916, 876776, 132997, 41191, 14975, 4659, 1539, 477, 115, 25, 0, 0, 0, 0, 0, 0];
    // [23146929, 1456884, 215440, 65754, 23837, 7425, 2482, 781, 193, 43, 2, 0, 0, 0, 0, 0];
    // [28488528, 1769896, 259362, 78927, 28578, 8861, 2935, 911, 234, 59, 2, 0, 0, 0, 0, 0];
    // [30763498, 1902294, 277670, 84390, 30562, 9445, 3127, 952, 246, 60, 3, 0, 0, 0, 0, 0];
    // [43531283, 2634779, 378678, 114597, 41423, 12819, 4253, 1286, 345, 93, 8, 0, 0, 0, 0, 0];
    // [46069454, 2779001, 398358, 120506, 43562, 13489, 4471, 1353, 365, 94, 9, 0, 0, 0, 0, 0];
    // [80926471, 4720353, 660175, 197765, 70898, 21969, 7173, 2173, 596, 155, 20, 0, 0, 0, 0, 0];
    // [82132549, 4786735, 669054, 200457, 71855, 22260, 7259, 2210, 604, 156, 20, 0, 0, 0, 0, 0];
    // [89120306, 5169123, 720016, 215500, 77212, 23900, 7763, 2358, 662, 175, 21, 0, 0, 0, 0, 0];
    // [100036958, 5763463, 798721, 238486, 85366, 26343, 8559, 2574, 724, 197, 26, 0, 0, 0, 0, 0];
    // [105890363, 6080856, 840898, 250907, 89743, 27726, 9024, 2697, 761, 208, 32, 0, 0, 0, 0, 0];
    // [111395068, 6378506, 880148, 262478, 93840, 28972, 9430, 2820, 807, 217, 37, 0, 0, 0, 0, 0];
    // [113548449, 6494679, 895492, 266964, 95464, 29461, 9576, 2870, 821, 219, 38, 1, 0, 0, 0, 0];
    // [142854465, 8064258, 1101028, 327074, 116569, 35891, 11681, 3534, 1025, 274, 47, 3, 0, 0, 0, 0];
    // [146777657, 8273189, 1128339, 335099, 119394, 36766, 11952, 3623, 1050, 279, 49, 3, 0, 0, 0, 0];
    // [149093659, 8396279, 1144413, 339738, 121003, 37243, 12111, 3672, 1061, 280, 49, 3, 0, 0, 0, 0];
    // [156839871, 8807191, 1197682, 355353, 126494, 38964, 12651, 3823, 1103, 292, 53, 3, 0, 0, 0, 0];
    // [179446726, 10001152, 1352366, 400275, 142225, 43736, 14228, 4321, 1251, 328, 60, 3, 0, 0, 0, 0];
    // [399983943, 21335489, 2793307, 817495, 287819, 87967, 28372, 8586, 2527, 688, 143, 14, 0, 0, 0, 0];
    // [405687407, 21623302, 2829234, 827702, 291413, 89087, 28719, 8694, 2562, 696, 147, 16, 0, 0, 0, 0];
    // [439641731, 23332707, 3043938, 889720, 313055, 95670, 30779, 9310, 2729, 738, 158, 21, 0, 0, 0, 0];
    // [448971481, 23801420, 3102470, 906522, 318836, 97464, 31337, 9475, 2776, 760, 161, 23, 0, 0, 0, 0];
    // [451543297, 23930476, 3118836, 911232, 320429, 97968, 31508, 9524, 2793, 764, 161, 23, 0, 0, 0, 0];
    // [631495704, 32881076, 4229607, 1230406, 431023, 131283, 42278, 12743, 3791, 1056, 253, 41, 1, 0, 0, 0];
    // [801833361, 41235768, 5254877, 1522702, 531987, 161909, 52158, 15636, 4686, 1323, 312, 50, 1, 0, 0, 0];
    // [910248037, 46506592, 5899357, 1706325, 595212, 180950, 58263, 17449, 5221, 1480, 348, 55, 1, 0, 0, 0];
    // [991361207, 50430616, 6375188, 1841869, 641930, 194928, 62791, 18812, 5616, 1587, 378, 61, 1, 0, 0, 0];
    // [1072276540, 54329692, 6847940, 1975893, 688115, 208945, 67161, 20140, 6036, 1706, 405, 73, 3, 0, 0, 0];
    [1099473570, 55637157, 7005824, 2020807, 703573, 213620, 68673, 20617, 6186, 1750, 417, 74, 3, 0, 0, 0];

    let debug_log_int_ratios = false;
    // Use some minimum primbix value for relative reference.
    let debug_frequency = 49459;
    let n: u64 = primbixes[0] + 1_000_000_000_000;
    let mut x = primbixes[0];
    let mut max_k = [0.0; 16];
    loop {
        if x >= n {break}

        let y = primbix(x);
        for k in 0..primbixes.len() {
            if y >= k as u64 {
                primbixes[k] += 1;
            }
        }

        if primbixes[0] % debug_frequency == 0 {

            let mut new_max = false;
            for k in 1..primbixes.len() {
                let pi = log_int(primbixes[k-1]);
                let ck = primbixes[k] as f64 / pi;
                if max_k[k] < ck {
                    max_k[k] = ck;
                    new_max = true;
                }
            }

            if new_max {
                if debug_log_int_ratios {
                    for k in 1..primbixes.len() {
                        let pi = log_int(primbixes[k-1]);
                        let ck = primbixes[k] as f64 / pi;
                        println!("  {}\t\t{}", ck, max_k[k]);
                    }
                }
                println!("{:?};", primbixes);
            }

        }

        x += 1;
    }

    println!("{:?};", primbixes);
}

/// Approximate offset logarithmic integral.
///
/// For more information see
/// [wikipedia article](https://en.wikipedia.org/wiki/Logarithmic_integral_function).
pub fn log_int(x: u64) -> f64 {
    let x = x as f64;
    let ln_x = x.ln();
    x / ln_x +
    x / ln_x.powi(2) +
    2.0 * x / ln_x.powi(3) +
    6.0 * x / ln_x.powi(4) +
    24.0 * x / ln_x.powi(5) +
    120.0 * x / ln_x.powi(6) +
    720.0 * x / ln_x.powi(7) +
    5040.0 * x / ln_x.powi(8)
}
