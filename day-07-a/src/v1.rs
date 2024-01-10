fn get_hand(cards: &str) -> u32 {
    let mut cards_nr = vec![0; 15];

    for card in cards.chars() {
        let index = match card {
            'V' => 10, // T
            'W' => 11, // J
            'X' => 12, // Q
            'Y' => 13, // K
            'Z' => 14, // A
            _ => card.to_digit(10).unwrap(),
        };

        cards_nr[index as usize] += 1;
    }

    let mut max = 0;
    let mut max_second = 0;

    for card in cards_nr {
        if card > max {
            max_second = max;
            max = card;
        } else if card > max_second {
            max_second = card;
        }
    }

    match max {
        5 => 7,
        4 => 6,
        3 => match max_second {
            2 => 5,
            _ => 4,
        },
        2 => match max_second {
            2 => 3,
            _ => 2,
        },
        _ => 1,
    }
}

fn main() {
    let mut input:String = include_str!("../input.txt").to_string();

    input = input.replace("A", "Z");
    input = input.replace("K", "Y");
    input = input.replace("Q", "X");
    input = input.replace("J", "W");
    input = input.replace("T", "V");

    let mut hands: Vec<(&str, u32)> = Vec::new();

    for line in input.lines() {
        let cards = line.split_whitespace().nth(0).unwrap();

        let nr: u32 = line.split_whitespace().nth(1).unwrap().parse().unwrap();

        hands.push((cards, nr));
    }

    hands.sort_by(|(cards1, _), (cards2, _)| {
        let hand1 = get_hand(cards1);
        let hand2 = get_hand(cards2);

        if hand1.cmp(&hand2).is_ne() {
            return hand1.cmp(&hand2);
        } else {
            return cards1.cmp(&cards2);
        }
    });

    let sum = hands
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, (_, nr))| nr * (i as u32 + 1))
        .sum::<u32>();

    for (i, (cards, _)) in hands.into_iter().enumerate() {
        println!("{}: {} {}", i + 1, cards, get_hand(cards));
    }

    println!("{:?}", sum); // 253530450 - old 
                           // 253933213 - new
}
