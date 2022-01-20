use duplicate::duplicate;

struct GameObject {
    name: String,
    position: i32,
    speed: i32,
}

struct Dog {
    game_object: GameObject,
}

struct Cat {
    game_object: GameObject,
}

struct CleaningRobot {
    game_object: GameObject,
}

struct MurderRobot {
    game_object: GameObject,
}

struct MurderRobotDog {
    game_object: GameObject,
}


trait Barker {
    fn bark(&self);
}

#[duplicate(structName; [Dog]; [MurderRobotDog])]
impl Barker for structName {
    fn bark(&self) {
        println!("{} is barking", self.game_object.name);
    }
}

trait Meower {
    fn meow(&self);
}

impl Meower for Cat {
    fn meow(&self) {
        println!("{} is meowing", self.game_object.name);
    }
}

trait Driver {
    fn drive(&mut self);
}

#[duplicate(structName; [CleaningRobot]; [MurderRobot]; [MurderRobotDog])]
impl Driver for structName {
    fn drive(&mut self) {
        self.game_object.position += self.game_object.speed;
        println!("{} is now at position {}", self.game_object.name, self.game_object.position);
    }
}

trait Pooper {
    fn poop(&self);
}

#[duplicate(structName; [Dog]; [Cat])]
impl Pooper for structName {
    fn poop(&self) {
        println!("{} is pooping", self.game_object.name);
    }
}


trait Cleaner {
    fn clean(&self);
}

impl Cleaner for CleaningRobot {
    fn clean(&self) {
        println!("{} is cleaning", self.game_object.name);
    }
}

trait Killer {
    fn kill(&self);
}

#[duplicate(structName; [MurderRobot]; [MurderRobotDog])]
impl Killer for structName {
    fn kill(&self) {
        println!("{} is killing", self.game_object.name);
    }
}

fn main() {
    println!("Hello, world!");
    let fido = Dog { game_object: GameObject { name: "Fido".to_string(), position: 0, speed: 100 }};
    fido.poop();

    let mut ralphKillah = MurderRobotDog { game_object: GameObject { name: "Ralph da killah".to_string(), position: 0, speed: 100 }};
    ralphKillah.drive();
    ralphKillah.drive();
}
