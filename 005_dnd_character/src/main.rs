use rand::Rng;
use std::io;
use std::collections::{HashSet, HashMap};

// race
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

impl Race {
    fn ability_bonuses(&self) -> HashMap<Ability, i8> {
        let mut bonuses = HashMap::new();

        match self {
            Race::Human => {
                bonuses.insert(Ability::Strength, 1);
                bonuses.insert(Ability::Dexterity, 1);
                bonuses.insert(Ability::Constitution, 1);
                bonuses.insert(Ability::Intelligence, 1);
                bonuses.insert(Ability::Wisdom, 1);
                bonuses.insert(Ability::Charisma, 1);
            },
            Race::Elf => {
                bonuses.insert(Ability::Dexterity, 2);
            },
            Race::Dwarf => {
                bonuses.insert(Ability::Constitution, 2);
            },
            Race::Halfling => {
                bonuses.insert(Ability::Dexterity, 2);
            },
            Race::Dragonborn => {
                bonuses.insert(Ability::Strength, 2);
                bonuses.insert(Ability::Charisma, 1);
            },
            Race::Gnome => {
                bonuses.insert(Ability::Intelligence, 2);
            },
            Race::HalfElf => {
                bonuses.insert(Ability::Charisma, 2);
                // Half-Elf gets +1 to two other abilities
            },
            Race::HalfOrc => {
                bonuses.insert(Ability::Strength, 2);
                bonuses.insert(Ability::Constitution, 1);
            },
            Race::Tiefling => {
                bonuses.insert(Ability::Intelligence, 1);
                bonuses.insert(Ability::Charisma, 2);
            },
        }

        bonuses
    }

    fn racial_skills(&self) -> Vec<Skill> {
        match self {
            Race::Elf => vec![Skill::Perception],
            Race::Dwarf => vec![Skill::History], // stonecunning
            Race::Halfling => vec![Skill::Acrobatics], // naturally Nimble
            Race::HalfOrc => vec![Skill::Intimidation],
            _ => vec![],
        }
    }
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

impl Class {
    fn hit_die(&self) -> u8 {
        match self {
            Class::Barbarian => 12,
            Class::Fighter | Class::Paladin | Class::Ranger => 10,
            Class::Bard | Class::Cleric | Class::Druid | Class::Monk | Class::Rogue | Class::Warlock => 8,
            Class::Sorcerer | Class::Wizard => 6,
        }
    }

    fn saving_throws(&self) -> (Ability, Ability) {
        match self {
            Class::Barbarian => (Ability::Strength, Ability::Constitution),
            Class::Bard => (Ability::Dexterity, Ability::Charisma),
            Class::Cleric => (Ability::Wisdom, Ability::Charisma),
            Class::Druid => (Ability::Intelligence, Ability::Wisdom),
            Class::Fighter => (Ability::Strength, Ability::Constitution),
            Class::Monk => (Ability::Strength, Ability::Dexterity),
            Class::Paladin => (Ability::Wisdom, Ability::Charisma),
            Class::Ranger => (Ability::Strength, Ability::Dexterity),
            Class::Rogue => (Ability::Dexterity, Ability::Intelligence),
            Class::Sorcerer => (Ability::Constitution, Ability::Charisma),
            Class::Warlock => (Ability::Wisdom, Ability::Charisma),
            Class::Wizard => (Ability::Intelligence, Ability::Wisdom),
        }
    }
}

// abilities
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum Ability {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

// skills and their associated abilities
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
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

impl Skill {
    fn associated_ability(&self) -> Ability {
        match self {
            Skill::Acrobatics => Ability::Dexterity,
            Skill::AnimalHandling => Ability::Wisdom,
            Skill::Arcana => Ability::Intelligence,
            Skill::Athletics => Ability::Strength,
            Skill::Deception => Ability::Charisma,
            Skill::History => Ability::Intelligence,
            Skill::Insight => Ability::Wisdom,
            Skill::Intimidation => Ability::Charisma,
            Skill::Investigation => Ability::Intelligence,
            Skill::Medicine => Ability::Wisdom,
            Skill::Nature => Ability::Intelligence,
            Skill::Perception => Ability::Wisdom,
            Skill::Performance => Ability::Charisma,
            Skill::Persuasion => Ability::Charisma,
            Skill::Religion => Ability::Intelligence,
            Skill::SleightOfHand => Ability::Dexterity,
            Skill::Stealth => Ability::Dexterity,
            Skill::Survival => Ability::Wisdom,
        }
    }
}

// backgrounds
#[derive(Debug, Clone, Copy)]
enum Background {
    Acolyte,
    Charlatan,
    Criminal,
    Entertainer,
    FolkHero,
    GuildArtisan,
    Hermit,
    Noble,
    Outlander,
    Sage,
    Sailor,
    Soldier,
    Urchin,
}

impl Background {
    fn skills(&self) -> [Skill; 2] {
        match self {
            Background::Acolyte => [Skill::Insight, Skill::Religion],
            Background::Charlatan => [Skill::Deception, Skill::SleightOfHand],
            Background::Criminal => [Skill::Deception, Skill::Stealth],
            Background::Entertainer => [Skill::Acrobatics, Skill::Performance],
            Background::FolkHero => [Skill::AnimalHandling, Skill::Survival],
            Background::GuildArtisan => [Skill::Insight, Skill::Persuasion],
            Background::Hermit => [Skill::Medicine, Skill::Religion],
            Background::Noble => [Skill::History, Skill::Persuasion],
            Background::Outlander => [Skill::Athletics, Skill::Survival],
            Background::Sage => [Skill::Arcana, Skill::History],
            Background::Sailor => [Skill::Athletics, Skill::Perception],
            Background::Soldier => [Skill::Athletics, Skill::Intimidation],
            Background::Urchin => [Skill::SleightOfHand, Skill::Stealth],
        }
    }
}

// stats with modifiers
#[derive(Debug)]
struct AbilityScores {
    strength: u8,
    dexterity: u8,
    constitution: u8,
    intelligence: u8,
    wisdom: u8,
    charisma: u8,
}

impl AbilityScores {
    fn modifier(&self, ability: Ability) -> i8 {
        let score = match ability {
            Ability::Strength => self.strength,
            Ability::Dexterity => self.dexterity,
            Ability::Constitution => self.constitution,
            Ability::Intelligence => self.intelligence,
            Ability::Wisdom => self.wisdom,
            Ability::Charisma => self.charisma,
        };

        ((score as i8) - 10) / 2
    }

    fn apply_racial_bonuses(&mut self, race: Race) {
        let bonuses = race.ability_bonuses();

        for (ability, bonus) in bonuses {
            match ability {
                Ability::Strength => self.strength = (self.strength as i8 + bonus) as u8,
                Ability::Dexterity => self.dexterity = (self.dexterity as i8 + bonus) as u8,
                Ability::Constitution => self.constitution = (self.constitution as i8 + bonus) as u8,
                Ability::Intelligence => self.intelligence = (self.intelligence as i8 + bonus) as u8,
                Ability::Wisdom => self.wisdom = (self.wisdom as i8 + bonus) as u8,
                Ability::Charisma => self.charisma = (self.charisma as i8 + bonus) as u8,
            }
        }

        // handle Half-Elf's floating +1 bonuses
        if let Race::HalfElf = race {
            println!("\nHalf-Elves get +1 to two abilities of your choice (excluding Charisma)");
            for i in 0..2 {
                println!("\nChoose ability {} to receive +1:", i + 1);
                println!("1. Strength");
                println!("2. Dexterity");
                println!("3. Constitution");
                println!("4. Intelligence");
                println!("5. Wisdom");

                loop {
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).expect("Failed to read line");

                    match input.trim().parse::<u8>() {
                        Ok(1) => { self.strength += 1; break; },
                        Ok(2) => { self.dexterity += 1; break; },
                        Ok(3) => { self.constitution += 1; break; },
                        Ok(4) => { self.intelligence += 1; break; },
                        Ok(5) => { self.wisdom += 1; break; },
                        _ => println!("Invalid selection. Please choose 1-5."),
                    }
                }
            }
        }
    }
}

// character structure
#[derive(Debug)]
struct Character {
    name: String,
    race: Race,
    class: Class,
    background: Background,
    level: u8,
    abilities: AbilityScores,
    hit_points: u8,
    skills: HashSet<Skill>,
    expertise: HashSet<Skill>,
    saving_throws: HashSet<Ability>,
}

impl Character {
    // display character
    fn display(&self) {
        println!("\n=== CHARACTER SHEET ===");
        println!("Name: {}", self.name);
        println!("Race: {:?}", self.race);
        println!("Class: {:?}", self.class);
        println!("Background: {:?}", self.background);
        println!("Level: {}", self.level);

        println!("\nAbility Scores:");
        println!("Strength: {} ({:+})", self.abilities.strength, self.abilities.modifier(Ability::Strength));
        println!("Dexterity: {} ({:+})", self.abilities.dexterity, self.abilities.modifier(Ability::Dexterity));
        println!("Constitution: {} ({:+})", self.abilities.constitution, self.abilities.modifier(Ability::Constitution));
        println!("Intelligence: {} ({:+})", self.abilities.intelligence, self.abilities.modifier(Ability::Intelligence));
        println!("Wisdom: {} ({:+})", self.abilities.wisdom, self.abilities.modifier(Ability::Wisdom));
        println!("Charisma: {} ({:+})", self.abilities.charisma, self.abilities.modifier(Ability::Charisma));

        println!("\nHit Points: {}", self.hit_points);

        println!("\nSaving Throw Proficiencies:");
        for ability in &self.saving_throws {
            println!("{:?} ({:+})", ability, self.abilities.modifier(*ability) + 2); // +2 for proficiency
        }

        println!("\nSkill Proficiencies:");
        if self.skills.is_empty() {
            println!("None");
        } else {
            for skill in &self.skills {
                println!("{:?} ({:+}){}",
                         skill,
                         self.skill_modifier(*skill),
                         if self.expertise.contains(skill) { " (Expertise)" } else { "" });
            }
        }
    }

    // calculate skill modifier
    fn skill_modifier(&self, skill: Skill) -> i8 {
        let base = self.abilities.modifier(skill.associated_ability());
        let proficiency = if self.skills.contains(&skill) { 2 } else { 0 };
        let expertise = if self.expertise.contains(&skill) { 2 } else { 0 };

        base + proficiency + expertise
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

// select a race
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

// select a background
fn select_background() -> Background {
    println!("\nSelect a background:");
    println!("1. Acolyte");
    println!("2. Charlatan");
    println!("3. Criminal");
    println!("4. Entertainer");
    println!("5. Folk Hero");
    println!("6. Guild Artisan");
    println!("7. Hermit");
    println!("8. Noble");
    println!("9. Outlander");
    println!("10. Sage");
    println!("11. Sailor");
    println!("12. Soldier");
    println!("13. Urchin");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u8>() {
            Ok(1) => return Background::Acolyte,
            Ok(2) => return Background::Charlatan,
            Ok(3) => return Background::Criminal,
            Ok(4) => return Background::Entertainer,
            Ok(5) => return Background::FolkHero,
            Ok(6) => return Background::GuildArtisan,
            Ok(7) => return Background::Hermit,
            Ok(8) => return Background::Noble,
            Ok(9) => return Background::Outlander,
            Ok(10) => return Background::Sage,
            Ok(11) => return Background::Sailor,
            Ok(12) => return Background::Soldier,
            Ok(13) => return Background::Urchin,
            _ => println!("Invalid selection. Please choose 1-13."),
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
fn select_skills(class: &Class, background: Background, race: Race) -> HashSet<Skill> {
    let mut selected_skills = HashSet::new();

    // add background skills
    let [bg_skill1, bg_skill2] = background.skills();
    selected_skills.insert(bg_skill1);
    selected_skills.insert(bg_skill2);

    // add racial skills
    for skill in race.racial_skills() {
        selected_skills.insert(skill);
    }

    // get class skills and count
    let class_skills = get_class_skills(class);
    let skill_count = get_skill_proficiency_count(class);

    // if we already have enough skills from background / race, return
    if selected_skills.len() >= skill_count {
        return selected_skills;
    }

    println!("\nSelect {} additional skill proficiencies from:", skill_count - selected_skills.len());

    // display available skills with numbers
    let mut available_skills = Vec::new();
    for skill in class_skills {
        if !selected_skills.contains(&skill) {
            available_skills.push(skill);
        }
    }

    for (i, skill) in available_skills.iter().enumerate() {
        println!("{}. {:?}", i + 1, skill);
    }

    while selected_skills.len() < skill_count {
        println!("\nChoose skill {} (1-{}):", selected_skills.len() + 1, available_skills.len());

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= available_skills.len() => {
                let skill = available_skills[n - 1];
                selected_skills.insert(skill);
                println!("Skill added.");
            },
            _ => println!("Invalid selection. Please choose a number between 1 and {}.", available_skills.len()),
        }
    }

    selected_skills
}

// select expertise skills (for Rogues and Bards)
fn select_expertise(class: &Class, skills: &HashSet<Skill>) -> HashSet<Skill> {
    let expertise_count = match class {
        Class::Rogue => 2,
        Class::Bard => 2,
        _ => return HashSet::new(),
    };

    if expertise_count == 0 {
        return HashSet::new();
    }

    let mut expertise = HashSet::new();

    println!("\nSelect {} skill(s) for Expertise:", expertise_count);

    // display available skills with numbers
    let available_skills: Vec<Skill> = skills.iter().copied().collect();
    for (i, skill) in available_skills.iter().enumerate() {
        println!("{}. {:?}", i + 1, skill);
    }

    while expertise.len() < expertise_count {
        println!("\nChoose expertise skill {} (1-{}):", expertise.len() + 1, available_skills.len());

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(n) if n >= 1 && n <= available_skills.len() => {
                let skill = available_skills[n - 1];
                expertise.insert(skill);
                println!("Expertise added.");
            },
            _ => println!("Invalid selection. Please choose a number between 1 and {}.", available_skills.len()),
        }
    }

    expertise
}

fn main() {
    println!("D&D 5e Character Creator");

    // get character name
    println!("\nEnter your character's name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    // select race, class, and background
    let race = select_race();
    let class = select_class();
    let background = select_background();

    // generate ability scores and apply racial bonuses
    println!("\nRolling ability scores (4d6 drop lowest)...");
    let mut abilities = generate_ability_scores();
    abilities.apply_racial_bonuses(race);

    // calculate hit points
    let hit_points = class.hit_die() + ((abilities.constitution as i8 - 10) / 2) as u8;

    // select skills
    let skills = select_skills(&class, background, race);

    // select expertise if applicable
    let expertise = select_expertise(&class, &skills);

    // get saving throw proficiencies
    let (saving_throw1, saving_throw2) = class.saving_throws();
    let mut saving_throws = HashSet::new();
    saving_throws.insert(saving_throw1);
    saving_throws.insert(saving_throw2);

    // create level 1 character
    let character = Character {
        name,
        race,
        class,
        background,
        level: 1,
        abilities,
        hit_points,
        skills,
        expertise,
        saving_throws,
    };

    // display character sheet
    character.display();
}