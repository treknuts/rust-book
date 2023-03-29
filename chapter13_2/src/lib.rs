#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v = vec![1, 2, 3];

        let v_iter = v.iter();

        // the sum method takes ownership of the iterator
        let total: i32 = v_iter.sum();
        // any calls to v_iter after sum are invalid
        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map_collect() {
        let v = vec![1, 2, 3];
        let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn shoes_in_size_test() {
        let shoes = vec![
            Shoe {
                size: 12,
                style: String::from("high-top"),
            },
            Shoe {
                size: 10,
                style: String::from("croc"),
            },
            Shoe {
                size: 12,
                style: String::from("boot"),
            },
        ];

        let filtered_shoes = shoes_in_size(shoes, 12);

        assert_eq!(
            filtered_shoes,
            vec![
                Shoe {
                    size: 12,
                    style: String::from("high-top")
                },
                Shoe {
                    size: 12,
                    style: String::from("boot")
                }
            ]
        );
    }
}
