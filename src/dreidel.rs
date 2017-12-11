use rand::Rng;
use rand::sample;

#[derive(Debug, Clone, Copy)]
pub enum Dreidel {
    Nun,
    Gimel,
    He,
    Shin,
}

impl Dreidel {
    pub fn spin(mut rng: &mut Rng, mut stake: &mut usize, mut pot: &mut usize) -> Dreidel {
        let result = sample(
            &mut rng,
            vec![Dreidel::Nun, Dreidel::Gimel, Dreidel::He, Dreidel::Shin],
            1,
        )[0];
        result.handle(&mut stake, &mut pot);
        result
    }

    fn handle(&self, stake: &mut usize, pot: &mut usize) {
        match *self {
            Dreidel::Nun => (),
            Dreidel::Gimel => {
                *stake += *pot;
                *pot = 0;
            }
            Dreidel::He => if *pot > 0 {
                let amount_to_transfer = (*pot + 1) / 2;
                *stake += amount_to_transfer;
                *pot -= amount_to_transfer;
            },
            Dreidel::Shin => if *stake > 0 {
                *stake -= 1;
                *pot += 1;
            },
        }
    }
}

#[test]
fn test_gimel() {
    let mut stake = 9;
    let mut pot = 9;
    Dreidel::Gimel.handle(&mut stake, &mut pot);
    assert_eq!(stake, 18);
    assert_eq!(pot, 0);

    stake = 5;
    pot = 0;
    Dreidel::Gimel.handle(&mut stake, &mut pot);
    assert_eq!(stake, 5);
    assert_eq!(pot, 0);
}

#[test]
fn test_shin() {
    let mut stake = 9;
    let mut pot = 9;
    Dreidel::Shin.handle(&mut stake, &mut pot);
    assert_eq!(stake, 8);
    assert_eq!(pot, 10);

    stake = 0;
    pot = 9;
    Dreidel::Shin.handle(&mut stake, &mut pot);
    assert_eq!(stake, 0);
    assert_eq!(pot, 9);
}

#[test]
fn test_nun() {
    let mut stake = 9;
    let mut pot = 9;
    Dreidel::Nun.handle(&mut stake, &mut pot);
    assert_eq!(stake, 9);
    assert_eq!(pot, 9);
}

#[test]
fn test_he() {
    let mut stake = 9;
    let mut pot = 9;
    Dreidel::He.handle(&mut stake, &mut pot);
    assert_eq!(stake, 14);
    assert_eq!(pot, 4);

    pot = 0;
    stake = 5;
    Dreidel::He.handle(&mut stake, &mut pot);
    assert_eq!(stake, 5);
    assert_eq!(pot, 0);

    pot = 1;
    stake = 5;
    Dreidel::He.handle(&mut stake, &mut pot);
    assert_eq!(stake, 6);
    assert_eq!(pot, 0);
}
