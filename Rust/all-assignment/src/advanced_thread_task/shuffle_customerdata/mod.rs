use crate::common::{available, language};

use super::{random_task::random_task, MASTER_DATA, USER_SKILLS};
use rand::Rng;
pub fn shuffle_customerdata(){
    for i in 0..25{
        let skill_count=rand::thread_rng().gen_range(2..=3);
        MASTER_DATA.write().unwrap()[i].skills.clear();
        for j in 0..skill_count{
            let rand_skill=rand::thread_rng().gen_range(0..=11);
            if !MASTER_DATA.read().unwrap()[i].skills.contains(&USER_SKILLS[rand_skill].to_string()){
                MASTER_DATA.write().unwrap()[i].skills.push(USER_SKILLS[rand_skill].to_string());
            }
            if MASTER_DATA.read().unwrap()[i].language==language::English{
                MASTER_DATA.write().unwrap()[i].language=language::Spanish;
            }
            else{
                MASTER_DATA.write().unwrap()[i].language=language::English;
            }
            if MASTER_DATA.read().unwrap()[i].status=="Online"{
                MASTER_DATA.write().unwrap()[i].status="Offline".to_string();
            }
            else{
                MASTER_DATA.write().unwrap()[i].status="Online".to_string();
            }
        }

    }
    println!("Data Shuffled Success");
}