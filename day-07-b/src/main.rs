fn get_hand(mut cards: String) -> u32 {
    let mut cards_nr = vec![0; 14];

    let nr_jokers = cards.chars().filter(|&c| c == '1').count();
    cards = cards.replace("1", "");

    for card in cards.chars() {
        let index = match card {
            'W'..='Z' => card as u32 - 'W' as u32 + 10,
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

    max += nr_jokers;

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
    let mut input: String = include_str!("../input.txt").to_string();

    let replacements = [('A', 'Z'), ('K', 'Y'), ('Q', 'X'), ('T', 'W'), ('J', '1')];

    for (from, to) in replacements.iter() {
        input = input.replace(*from, &to.to_string());
    }

    let mut hands: Vec<(&str, u32)> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let cards = iter.next().unwrap();
        let nr: u32 = iter.next().unwrap().parse().unwrap();
        hands.push((cards, nr));
    }

    hands.sort_by(|(cards1, _), (cards2, _)| {
        let hand1 = get_hand(cards1.to_string());
        let hand2 = get_hand(cards2.to_string());

        hand1.cmp(&hand2).then_with(|| cards1.cmp(&cards2))
    });

    let sum: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, (_, nr))| nr * (i as u32 + 1))
        .sum();

    println!("{:?}", sum); // 253473930
}
