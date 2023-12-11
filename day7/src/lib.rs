use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(PartialEq, PartialOrd, Debug, Eq, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug)]
struct CamelCard {
    hand: Vec<char>,
    bid: u32,
    hand_type: HandType,
}

#[derive(Debug)]
struct CamelCardJoker {
    hand: Vec<char>,
    bid: u32,
    hand_type: HandType,
}

impl CamelCard {
    fn get_card_rank(card: char) -> u32 {
        match card {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => panic!("Illegal card"),
        }
    }
}

impl CamelCardJoker {
    fn get_card_rank_with_joker(card: char) -> u32 {
        match card {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 1,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            ' ' => 0,
            _ => panic!("Illegal card"),
        }
    }
}

impl FromStr for CamelCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();

        let hand: Vec<char> = hand.chars().collect();
        let mut hand_cnt_map: HashMap<char, u8> = HashMap::new();

        for c in &hand {
            match hand_cnt_map.get(c) {
                Some(cnt) => hand_cnt_map.insert(*c, *cnt + 1),
                None => hand_cnt_map.insert(*c, 1),
            };
        }

        let hand_type = match hand_cnt_map.values().len() {
            1 => HandType::FiveKind,
            2 => {
                let mut hand_type = HandType::FullHouse;
                for (_, cnt) in hand_cnt_map.into_iter() {
                    // if card is joker then add to the highest count.

                    if cnt == 1 || cnt == 4 {
                        hand_type = HandType::FourKind;
                    }
                    break;
                }
                hand_type
            }
            3 => {
                let mut hand_type = HandType::TwoPair;
                for (card, cnt) in hand_cnt_map.into_iter() {
                    // if card is joker then add to the highest count.
                    if cnt == 3 {
                        hand_type = HandType::ThreeKind;
                        break;
                    }
                }
                hand_type
            }
            4 => HandType::Pair,
            5 => HandType::HighCard,
            _ => panic!("Can't determine hand type"),
        };

        Ok(CamelCard {
            hand,
            bid: bid.parse::<u32>().unwrap(),
            hand_type,
        })
    }
}

impl FromStr for CamelCardJoker {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();

        let hand: Vec<char> = hand.chars().collect();
        let mut hand_cnt_map: HashMap<char, u8> = HashMap::with_capacity(5);
        let mut joker_cnt: u8 = 0;
        let mut max_key_value = (' ', 0);

        for c in &hand {
            match hand_cnt_map.get(c) {
                Some(cnt) => {
                    if *c == 'J' {
                        joker_cnt = joker_cnt + 1;
                    } else {
                        let new_cnt = *cnt + 1;
                        hand_cnt_map.insert(*c, new_cnt);
                        if new_cnt >= max_key_value.1 {
                            max_key_value = (*c, new_cnt);
                        } else if new_cnt == max_key_value.1 {
                            // max card count is equal but we need to change to the max valued card
                            let curr_max_card =
                                CamelCardJoker::get_card_rank_with_joker(max_key_value.0);

                            if CamelCardJoker::get_card_rank_with_joker(*c) > curr_max_card {
                                max_key_value.0 = *c;
                            }
                        }
                    }
                }
                None => {
                    if *c == 'J' {
                        joker_cnt = joker_cnt + 1;
                    } else {
                        hand_cnt_map.insert(*c, 1);
                        let curr_max_card =
                            CamelCardJoker::get_card_rank_with_joker(max_key_value.0);

                        if CamelCardJoker::get_card_rank_with_joker(*c) > curr_max_card
                            && max_key_value.1 == 0
                        {
                            max_key_value = (*c, 1);
                        }
                    }
                }
            };
        }

        if max_key_value.0 != ' ' {
            hand_cnt_map.insert(max_key_value.0, max_key_value.1 + joker_cnt);
        } else if hand_cnt_map.len() == 0 {
            // all jokers
            hand_cnt_map.insert('A', 5);
        }

        let hand_type = match hand_cnt_map.values().len() {
            1 => HandType::FiveKind,
            2 => {
                let mut hand_type = HandType::FullHouse;
                for (_, cnt) in hand_cnt_map.into_iter() {
                    if cnt == 1 || cnt == 4 {
                        hand_type = HandType::FourKind;
                    }
                    break;
                }
                hand_type
            }
            3 => {
                let mut hand_type = HandType::TwoPair;
                for (_, cnt) in hand_cnt_map.into_iter() {
                    if cnt == 3 {
                        hand_type = HandType::ThreeKind;
                        break;
                    }
                }
                hand_type
            }
            4 => HandType::Pair,
            5 => HandType::HighCard,
            _ => panic!("Can't determine hand type"),
        };

        Ok(CamelCardJoker {
            hand,
            bid: bid.parse::<u32>().unwrap(),
            hand_type,
        })
    }
}

pub fn part1(input: &str) -> u32 {
    let mut camel_cards: Vec<CamelCard> = input
        .lines()
        .map(|line| line.parse::<CamelCard>().unwrap())
        .collect();

    camel_cards.sort_by(|c1, c2| match c1.hand_type.cmp(&c2.hand_type) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            let card_zip = c1.hand.iter().zip(c2.hand.iter());

            for (card1, card2) in card_zip {
                let card1_val = CamelCard::get_card_rank(*card1);
                let card2_val = CamelCard::get_card_rank(*card2);

                if card1_val > card2_val {
                    return Ordering::Greater;
                }

                if card1_val < card2_val {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        }
    });

    let mut total = 0;
    let mut multiplier = 1;

    for camel_card in &camel_cards {
        total = total + (camel_card.bid * multiplier);
        multiplier = multiplier + 1;
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let mut camel_cards: Vec<CamelCardJoker> = input
        .lines()
        .map(|line| line.parse::<CamelCardJoker>().unwrap())
        .collect();

    camel_cards.sort_by(|c1, c2| match c1.hand_type.cmp(&c2.hand_type) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => {
            let card_zip = c1.hand.iter().zip(c2.hand.iter());

            for (card1, card2) in card_zip {
                let card1_val = CamelCardJoker::get_card_rank_with_joker(*card1);
                let card2_val = CamelCardJoker::get_card_rank_with_joker(*card2);

                if card1_val > card2_val {
                    return Ordering::Greater;
                }

                if card1_val < card2_val {
                    return Ordering::Less;
                }
            }

            Ordering::Equal
        }
    });
    println!("{}", camel_cards.len());

    let mut total = 0;
    let mut multiplier = 1;

    for camel_card in &camel_cards {
        println!("{:?}", camel_card);
        total = total + (camel_card.bid * multiplier);
        multiplier = multiplier + 1;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::HandType::*;
    use super::*;

    const INPUT: &'static str = include_str!("../input.txt");

    #[test]
    fn hand_type_test() {
        assert!(Pair > HighCard);
        assert!(ThreeKind > Pair);
        assert!(FullHouse > ThreeKind);
        assert!(FourKind > FullHouse);
        assert!(FiveKind > FourKind);
    }

    #[test]
    fn camel_card_parse_test() {
        let three_kind_line = "QQQJA 483";
        let four_kind_line = "QQQQA 483";
        let full_house_line = "QQQAA 483";

        let three_kind_hand = three_kind_line.parse::<CamelCard>().unwrap();
        let four_kind_hand = four_kind_line.parse::<CamelCard>().unwrap();
        let full_house_hand = full_house_line.parse::<CamelCard>().unwrap();

        assert_eq!(three_kind_hand.bid, 483);
        assert_eq!(three_kind_hand.hand_type, ThreeKind);
        assert_eq!(four_kind_hand.hand_type, FourKind);
        assert_eq!(full_house_hand.hand_type, FullHouse);
    }

    #[test]
    fn camel_card_joker_parse_test() {
        let four_kind_with_jack_hand = "QQQJA 483".parse::<CamelCardJoker>().unwrap();
        let four_kind_hand = "QQQQA 483".parse::<CamelCardJoker>().unwrap();
        let full_house_hand = "QQJAA 483".parse::<CamelCardJoker>().unwrap();
        let high_card_hand = "TAK97 148".parse::<CamelCardJoker>().unwrap();
        let all_joker_hand = "JJJJJ 171".parse::<CamelCardJoker>().unwrap();
        let pair_kings = "TK2J7 116".parse::<CamelCardJoker>().unwrap();
        let four_nines = "99992 721".parse::<CamelCardJoker>().unwrap();
        let four_sixes_with_jack = "AJ666 222".parse::<CamelCardJoker>().unwrap();

        assert_eq!(four_kind_with_jack_hand.bid, 483);
        assert_eq!(four_kind_with_jack_hand.hand_type, FourKind);
        assert_eq!(four_kind_hand.hand_type, FourKind);
        assert_eq!(full_house_hand.hand_type, FullHouse);
        assert_eq!(high_card_hand.hand_type, HighCard);
        assert_eq!(all_joker_hand.hand_type, FiveKind);
        assert_eq!(pair_kings.hand_type, Pair);
        assert_eq!(four_nines.hand_type, FourKind);
        assert_eq!(four_sixes_with_jack.hand_type, FourKind);
    }

    #[test]
    fn part1_test() {
        let result = part1(INPUT);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part2_mini_test() {
        let input = "AAAJJ 1\nAJAAA 2";
        let result = part2(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn part2_test() {
        let result = part2(INPUT);
        assert_eq!(result, 5905);
    }
}
