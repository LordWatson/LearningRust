use rand::Rng;
use std::io;
use std::collections::HashSet;

// races
#[derive(Debug, Clone, Copy)]
enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    Dragonborn,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
}

// classes
#[derive(Debug)]
enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

// all possible skills
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Skill {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

// stats
#[derive(Debug)]
struct AbilityScores {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

// character structure
#[derive(Debug)]
struct Character {
    name: String,
    race: Race,
    class: Class,
    level: u8,
    abilities: AbilityScores,
    hit_points: u8,
    skills: HashSet<Skill>,
}

impl Character {
    // character sheet
    fn display(&self) {
        println!("\n=== CHARACTER SHEET ===");
        println!("Name: {}", self.name);
        println!("Race: {:?}", self.race);
        println!("Class: {:?}", self.class);
        println!("Level: {}", self.level);
        println!("\nAbility Scores:");
        println!("Strength: {}", self.abilities.strength);
        println!("Dexterity: {}", self.abilities.dexterity);
        println!("Constitution: {}", self.abilities.constitution);
        println!("Intelligence: {}", self.abilities.intelligence);
        println!("Wisdom: {}", self.abilities.wisdom);
        println!("Charisma: {}", self.abilities.charisma);
        println!("\nHit Points: {}", self.hit_points);

        println!("\nSkills:");
        if self.skills.is_empty() {
            println!("None");
        } else {
            for skill in &self.skills {
                println!("{:?}", skill);
            }
        }
    }
}

// roll a die
fn roll_die(sides: u8) -> u8 {
    rand::thread_rng().gen_range(1..=sides)
}

// roll 4d6 and drop the lowest
fn roll_ability_score() -> u8 {
    let mut rolls = vec![
        roll_die(6),
        roll_die(6),
        roll_die(6),
        roll_die(6),
    ];

    // sort and drop the lowest
    rolls.sort();
    rolls[1..].iter().sum()
}

// ability scores
fn generate_ability_scores() -> AbilityScores {
    AbilityScores {
        strength: roll_ability_score(),
        dexterity: roll_ability_score(),
        constitution: roll_ability_score(),
        intelligence: roll_ability_score(),
        wisdom: roll_ability_score(),
        charisma: roll_ability_score(),
    }
}

// calculate hp based on class and constitution
fn calculate_hit_points(class: &Class, constitution: u8) -> u8 {
    let base_hp = match class {
        Class::Barbarian => 12,
        Class::Fighter | Class::Paladin | Class::Ranger => 10,
        Class::Bard | Class::Cleric | Class::Druid | Class::Monk | Class::Rogue | Class::Warlock => 8,
        Class::Sorcerer | Class::Wizard => 6,
    };

    let con_mod = (constitution as i32 - 10) / 2;
    std::cmp::max(1, base_hp + con_mod as u8)
}

// select race
fn select_race() -> Race {
    println!("\nSelect a race:");
    println!("1. Human");
    println!("2. Elf");
    println!("3. Dwarf");
    println!("4. Halfling");
    println!("5. Dragonborn");
    println!("6. Gnome");
    println!("7. Half-Elf");
    println!("8. Half-Orc");
    println!("9. Tiefling");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u8>() {
            Ok(1) => return Race::Human,
            Ok(2) => return Race::Elf,
            Ok(3) => return Race::Dwarf,
            Ok(4) => return Race::Halfling,
            Ok(5) => return Race::Dragonborn,
            Ok(6) => return Race::Gnome,
            Ok(7) => return Race::HalfElf,
            Ok(8) => return Race::HalfOrc,
            Ok(9) => return Race::Tiefling,
            _ => println!("Invalid selection. Please choose 1-9."),
        }
    }
}

// select a class
fn select_class() -> Class {
    println!("\nSelect a class:");
    println!("1. Barbarian");
    println!("2. Bard");
    println!("3. Cleric");
    println!("4. Druid");
    println!("5. Fighter");
    println!("6. Monk");
    println!("7. Paladin");
    println!("8. Ranger");
    println!("9. Rogue");
    println!("10. Sorcerer");
    println!("11. Warlock");
    println!("12. Wizard");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u8>() {
            Ok(1) => return Class::Barbarian,
            Ok(2) => return Class::Bard,
            Ok(3) => return Class::Cleric,
            Ok(4) => return Class::Druid,
            Ok(5) => return Class::Fighter,
            Ok(6) => return Class::Monk,
            Ok(7) => return Class::Paladin,
            Ok(8) => return Class::Ranger,
            Ok(9) => return Class::Rogue,
            Ok(10) => return Class::Sorcerer,
            Ok(11) => return Class::Warlock,
            Ok(12) => return Class::Wizard,
            _ => println!("Invalid selection. Please choose 1-12."),
        }
    }
}

// get class skills based on the selected class
fn get_class_skills(class: &Class) -> Vec<Skill> {
    match class {
        Class::Barbarian => vec![
            Skill::AnimalHandling,
            Skill::Athletics,
            Skill::Intimidation,
            Skill::Nature,
            Skill::Perception,
            Skill::Survival,
        ],
        Class::Bard => vec![
            Skill::Acrobatics,
            Skill::AnimalHandling,
            Skill::Arcana,
            Skill::Athletics,
            Skill::Deception,
            Skill::History,
            Skill::Insight,
            Skill::Intimidation,
            Skill::Investigation,
            Skill::Medicine,
            Skill::Nature,
            Skill::Perception,
            Skill::Performance,
            Skill::Persuasion,
            Skill::Religion,
            Skill::SleightOfHand,
            Skill::Stealth,
            Skill::Survival,
        ],
        Class::Cleric => vec![
            Skill::History,
            Skill::Insight,
            Skill::Medicine,
            Skill::Persuasion,
            Skill::Religion,
        ],
        Class::Druid => vec![
            Skill::Arcana,
            Skill::AnimalHandling,
            Skill::Insight,
            Skill::Medicine,
            Skill::Nature,
            Skill::Perception,
            Skill::Religion,
            Skill::Survival,
        ],
        Class::Fighter => vec![
            Skill::Acrobatics,
            Skill::AnimalHandling,
            Skill::Athletics,
            Skill::History,
            Skill::Insight,
            Skill::Intimidation,
            Skill::Perception,
            Skill::Survival,
        ],
        Class::Monk => vec![
            Skill::Acrobatics,
            Skill::Athletics,
            Skill::History,
            Skill::Insight,
            Skill::Religion,
            Skill::Stealth,
        ],
        Class::Paladin => vec![
            Skill::Athletics,
            Skill::Insight,
            Skill::Intimidation,
            Skill::Medicine,
            Skill::Persuasion,
            Skill::Religion,
        ],
        Class::Ranger => vec![
            Skill::AnimalHandling,
            Skill::Athletics,
            Skill::Insight,
            Skill::Investigation,
            Skill::Nature,
            Skill::Perception,
            Skill::Stealth,
            Skill::Survival,
        ],
        Class::Rogue => vec![
            Skill::Acrobatics,
            Skill::Athletics,
            Skill::Deception,
            Skill::Insight,
            Skill::Intimidation,
            Skill::Investigation,
            Skill::Perception,
            Skill::Performance,
            Skill::Persuasion,
            Skill::SleightOfHand,
            Skill::Stealth,
        ],
        Class::Sorcerer => vec![
            Skill::Arcana,
            Skill::Deception,
            Skill::Insight,
            Skill::Intimidation,
            Skill::Persuasion,
            Skill::Religion,
        ],
        Class::Warlock => vec![
            Skill::Arcana,
            Skill::Deception,
            Skill::History,
            Skill::Intimidation,
            Skill::Investigation,
            Skill::Nature,
            Skill::Religion,
        ],
        Class::Wizard => vec![
            Skill::Arcana,
            Skill::History,
            Skill::Insight,
            Skill::Investigation,
            Skill::Medicine,
            Skill::Religion,
        ],
    }
}

// get the number of skill proficiencies based on class
fn get_skill_proficiency_count(class: &Class) -> usize {
    match class {
        Class::Barbarian => 2,
        Class::Bard => 3,
        Class::Cleric => 2,
        Class::Druid => 2,
        Class::Fighter => 2,
        Class::Monk => 2,
        Class::Paladin => 2,
        Class::Ranger => 3,
        Class::Rogue => 4,
        Class::Sorcerer => 2,
        Class::Warlock => 2,
        Class::Wizard => 2,
    }
}

// select skills from the available class skills
fn select_skills(class: &Class) -> HashSet<Skill> {
    let class_skills = get_class_skills(class);
    let skill_count = get_skill_proficiency_count(class);
    let mut selected_skills = HashSet::new();

    println!("\nSelect {} skill proficiencies from:", skill_count);

    // display available skills with numbers
    for (i, skill) in class_skills.iter().enumerate() {
        println!("{}. {:?}", i + 1, skill);
    }

    while selected_skills.len() < skill_count {
        println!("\nChoose skill {} (1-{}):", selected_skills.len() + 1, class_skills.len());

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= class_skills.len() => {
                let skill = class_skills[n - 1].clone();
                if selected_skills.contains(&skill) {
                    println!("You've already selected that skill.");
                } else {
                    selected_skills.insert(skill);
                    println!("Skill added.");
                }
            },
            _ => println!("Invalid selection. Please choose a number between 1 and {}.", class_skills.len()),
        }
    }

    selected_skills
}

fn main() {
    println!("D&D Character Creator");

    // character name
    println!("\nEnter your character's name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    // select race and class
    let race = select_race();
    let class = select_class();

    // ability scores
    println!("\nRolling ability scores (4d6 drop lowest)...");
    let abilities = generate_ability_scores();

    // calculate hp
    let hit_points = calculate_hit_points(&class, abilities.constitution);

    // select skills
    let skills = select_skills(&class);

    // create level 1 character
    let character = Character {
        name,
        race,
        class,
        level: 1,
        abilities,
        hit_points,
        skills,
    };

    // display character sheet
    character.display();
}