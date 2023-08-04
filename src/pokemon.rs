// Structs
#[derive(Debug)]
struct Character {
    name: String,
    health: i32,
}

#[derive(Debug)]
struct Pokemon {
    poke_type: String,
    game_object: Character,
    attack: i32,
}

// Structs impls
impl Character {
    fn new(name: String, health: i32) -> Self { Self {  name, health } }
    fn receive_damage(&mut self, damage: i32) { self.health -= damage }
    fn is_alive(&self) -> bool { self.health > 0 }
}

// Structs impls
impl Pokemon {
    fn new(poke_type: String, name: String, health: i32, attack: i32) -> Self {
        let game_object = Character::new(name, health);
        return Self { poke_type, game_object, attack }
    }
    fn say_type(&self) { println!("My name is {}, I'm a {} type pokemon", self.game_object.name, self.poke_type) }
}

// Trait
trait Aggressive {
    fn attack(&self) -> &i32;
}

impl Aggressive for Pokemon {
    fn attack(&self) -> &i32 {
        &self.attack
    }
}

pub fn run() {
    let juan = Pokemon::new(String::from("water"), String::from("Juan"), 20, 100);
    let mut nefrito = Pokemon::new(String::from("fire"), String::from("Nefrito"), 200, 1);

    juan.say_type();
    let &damage = juan.attack();
    nefrito.game_object.receive_damage(damage);

    println!("Is nefrito alive? {}", &nefrito.game_object.is_alive());

}
