#[derive(Copy, Clone)]
enum PokemonType {
    ELECTRIC,
    FIRE,
    WATER,
    GRASS,
}

impl From<PokemonType> for usize {
    fn from(value: PokemonType) -> Self {
        value as usize
    }
}

fn get_effectiveness(my_type: PokemonType, opponent_type: PokemonType) -> f64 {
    println!("{} {}", my_type as usize, opponent_type as usize);

    const EFFECTIVENESS: [[f64; 4]; 4] = [
        // ELECTRIC, FIRE, WATER, GRASS
        [1.0, 1.0, 2.0, 1.0], // ELECTRIC
        [1.0, 1.0, 0.5, 2.0], // FIRE
        [0.5, 2.0, 1.0, 0.5], // WATER
        [1.0, 0.5, 2.0, 1.0], // GRASS
    ];

    EFFECTIVENESS[my_type as usize][opponent_type as usize]
}

fn calculate_damage(
    my_type: PokemonType,
    opponent_type: PokemonType,
    attack_power: f64,
    opponent_defence: f64,
) -> f64 {
    return 50.0 * (attack_power / opponent_defence) * get_effectiveness(my_type, opponent_type)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_damage() {
        assert_eq!(calculate_damage(PokemonType::FIRE, PokemonType::WATER, 100.0, 100.0), 25.0);
        assert_eq!(calculate_damage(PokemonType::GRASS, PokemonType::WATER, 100.0, 100.0), 100.0);
        assert_eq!(calculate_damage(PokemonType::ELECTRIC, PokemonType::FIRE, 100.0, 100.0), 50.0);
        assert_eq!(calculate_damage(PokemonType::GRASS, PokemonType::ELECTRIC, 57.0, 19.0), 150.0);
        assert_eq!(calculate_damage(PokemonType::GRASS, PokemonType::WATER, 40.0, 40.0), 100.0);
        assert_eq!(calculate_damage(PokemonType::GRASS, PokemonType::FIRE, 35.0, 5.0), 175.0);
        assert_eq!(calculate_damage(PokemonType::FIRE, PokemonType::ELECTRIC, 10.0, 2.0), 250.0);
    }
}

