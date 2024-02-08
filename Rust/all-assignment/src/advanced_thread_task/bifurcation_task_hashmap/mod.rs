
use std::{collections::VecDeque, ops::Deref, vec};

use super::{HASH_QUEUE, REQUEST_DATA};


pub fn task_bifurcation_hashmap(){
    for each_task in REQUEST_DATA.write().unwrap().pop_front(){
        let skill=&each_task.skills;
        let lang=&each_task.language;
        let avail_type=&each_task.available;
        // let time_stamp=&each_skill.timestamp;
        let each_data=format!("{:?}-{}-{:?}-L1",avail_type,skill,lang);
        HASH_QUEUE.write().unwrap().entry(each_data).or_insert_with(VecDeque::new).push_front(each_task);
        // println!("HASHMAP : {:#?}",HASH_QUEUE.read().unwrap());
    } 

}