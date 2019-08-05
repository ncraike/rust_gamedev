
struct Player {
    health: i64,
    nutrition: i64,
    oxygen: i64,
}

fn main() {
    let mut player = Player{100, 100, 100};

    while playerIsAlive(player) {
        playerStatus(player)
        playerTick(player);
    }
    playerDeath(player);
}

fn playerTick(player: &Player) {
    if player.nutrition < 0 {
        player.health -= 1;
    }
    if player.oxygen < 0 {
        player.health -= 10;
    }
    player.nutrition -= 1;
}

fn playerIsAlive(player: &Player) -> bool {
    return player.health > 0;
}

fn playerStatus(player: &Player) {
    println!("Health: {}", player.health);
    println!("Nutrition: {}", player.nutrition);
    println!("Oxygen: {}", player.oxygen);
}

fn playerDeath(player: &Player) {
    println!("Player is dead");
}
