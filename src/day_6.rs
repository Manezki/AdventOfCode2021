use std::fs;
use cached::proc_macro::cached;

#[cached]
fn child_tree(days_left: u32) -> u64 {

    // Child itself
    let mut offspring = 1;

    // Does it have time to reproduce?
    if days_left > 9 {
        let children = (days_left - 2)/7;

        for child in 1..=children {
            offspring += child_tree(days_left - 2 - (child*7));
        }
    } else if days_left == 0 {
        offspring -= 1;
    }
    println!("{} - {}", days_left, offspring);
    return offspring;
}

fn main() {
    let file_content = fs::read_to_string("src\\day_6_input.txt").expect("Something went horribly wrong reading the file");
    let initial_fish_state = file_content.split(",").map(|f| u32::from_str_radix(f, 10).unwrap()).collect::<Vec<u32>>();

    let num_days: u32 = 256;

    let mut colony_size: u64 = initial_fish_state.len() as u64;

    for tt_first_doubling in initial_fish_state {

        // let children = (num_days - tt_first_doubling)/7;
        let children = (num_days - tt_first_doubling)/7 + 1;

        for child in 0..children {
            colony_size += child_tree(num_days - tt_first_doubling - (child*7));
        }

        // for child in 1..=children {
            
        //     let ancestral_summoning_sickness = (child - 1)*2;
        //     let ancestral_reproduction_time = ((child - 1)*7) + tt_first_doubling;
        //     let day_born = ancestral_reproduction_time + ancestral_summoning_sickness;
            
        //     if day_born >= num_days {
        //         // colony_size -= generation_size;
        //     } else {
        //         let time_left_to_reproduce = num_days - day_born;

        //         let doublings = time_left_to_reproduce/7;
        //         println!("{:?}", doublings);

        //         if doublings == 0 {
        //             colony_size += 1 as u64;
        //         } else if doublings == 1 {
        //             colony_size += 2 as u64;
        //         } else {
        //             let leafs = (2 as u64).pow(doublings-1);
        //             let nodes = (2 as u64).pow(doublings-1);
        //             colony_size += leafs + nodes;
        //         }
        //     }
        // }

        // for generation in 1..generations {

        //     let generation_size = (2 as u64).pow(generation - 1);
        //     colony_size += generation_size;

        //     let ancestral_summoning_sickness = (generation - 1)*2;
        //     let ancestral_reproduction_time = ((generation - 1)*7) + tt_first_doubling;
        //     let day_born = ancestral_reproduction_time + ancestral_summoning_sickness;

        //     if day_born > num_days {
        //         colony_size -= generation_size;
        //     } else {
        //         let time_left_to_reproduce = num_days - day_born;

        //         let doublings = time_left_to_reproduce/7;
        //         println!("{:?}", doublings);
        //         colony_size += generation_size*doublings as u64;
        //     }
        // }
    }

    // for tt_first_doubling in initial_fish_state {

    //     colony_size += 1 as u64;

    //     // Should floor automatically
    //     let generations = (num_days - tt_first_doubling)/7;
        
    //     // colony_size += generations as u64;

    //     println!("Gen {:?}, {:?} doublings", 0, generations);

    //     for generation in 1..=generations {

    //         let generation_size: u64 = (2 as u64).pow(generation - 1);

    //         // colony_size += generation_size;

    //         let generation_offset = generation*7;
    //         let summonings_sicknesses = generation * 2;

    //         if tt_first_doubling+summonings_sicknesses+generation_offset < num_days {
    //             let doublings = ((num_days-tt_first_doubling-summonings_sicknesses-generation_offset)/7) as u64;

    //             // println!("Gen {:?}, {:?} doublings", generation, doublings);

    //             colony_size += generation_size*doublings;
    //         } else {
    //             colony_size += generation_size;
    //             println!("Skipped gen {:?}, {:?} TTD", generation, tt_first_doubling+summonings_sicknesses+generation_offset);
    //         }
    //     }
    // }


    // for day in 0..num_days {

    //     println!("{:?}", day);    

    //     for fish in unprocessed_fish.iter() {
    //         if *fish == 0 {
    //             processed_fish.push(6);
    //             processed_fish.push(8);
    //         } else {
    //             processed_fish.push(*fish - 1);
    //         }
    //     }

    //     unprocessed_fish = processed_fish;
    //     processed_fish = Vec::new();
    // }

    println!("{:?}", colony_size);
    // With 80 days should be 362_346
}


#[test]
fn five_fish_given_example() {

    let initial_fishes = vec![3,4,3,1,2];
    let num_days = 18;
    let mut colony = initial_fishes.len() as u64;

    for tt in initial_fishes {
        println!("Fish {}, {}", tt, num_days-tt);
        let children = (num_days - tt)/7 + 1;

        for child in 0..children {
            colony += child_tree(num_days - tt - (child*7));
        }
        // colony += child_tree(18 - tt);
        println!("Colony {}", colony);
        // break;
    }
    assert_eq!(colony, 26);
}


#[test]
fn single_3_fish_given_example() {

    let initial_fishes = vec![3];
    let num_days = 18;
    let mut colony = 1;

    for tt in initial_fishes {
        println!("Fish {}, {}", tt, num_days-tt);
        let children = (num_days - tt)/7 + 1;

        for child in 0..children {
            colony += child_tree(num_days - tt - (child*7));
        }
        // colony += child_tree(18 - tt);
        println!("Colony {}", colony);
        break;
    }
    assert_eq!(colony, 5);
}

#[test]
fn single_4_fish_given_example() {

    let initial_fishes = vec![4];
    let num_days = 18;
    let mut colony = 1;

    for tt in initial_fishes {
        println!("Fish {}, {}", tt, num_days-tt);
        let children = (num_days - tt)/7 + 1;

        for child in 0..children {
            colony += child_tree(num_days - tt - (child*7));
        }
        // colony += child_tree(18 - tt);
        println!("Colony {}", colony);
        break;
    }
    assert_eq!(colony, 4);
}

