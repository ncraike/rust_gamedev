
struct Player {
    health: i64,
    nutrition: i64,
    oxygen: i64,
}

fn main() {
    let mut player = Player {
        health: 100,
        nutrition: 100,
        oxygen: 100,
    };

    while player_is_alive(&player) {
        player_status(&player);
        player_tick(&mut player);
    }
    player_death();
}

fn player_tick(player: &mut Player) {
    if player.nutrition < 0 {
        player.health -= 1;
    }
    if player.oxygen < 0 {
        player.health -= 10;
    }
    player.nutrition -= 1;
}

fn player_is_alive(player: &Player) -> bool {
    player.health > 0
}

fn player_status(player: &Player) {
    println!("Health: {}", player.health);
    println!("Nutrition: {}", player.nutrition);
    println!("Oxygen: {}", player.oxygen);
}

fn player_death() {
    println!("Player is dead");
}
