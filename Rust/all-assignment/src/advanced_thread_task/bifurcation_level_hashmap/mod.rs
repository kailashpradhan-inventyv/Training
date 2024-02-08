use std::{collections::{hash_map, HashMap, VecDeque}, ops::Deref};

use chrono::{DurationRound, Utc};

use super::{HASH_QUEUE, REQUEST_DATA};

pub fn bifurcation_level(){
    let mut hash_temp=HashMap::new();
    for (each_key,each_queue) in HASH_QUEUE.write().unwrap().iter_mut(){
        let mut new_queue_name:Option<String>=None;
        let mut list_req=VecDeque::new();
        for mut each_req in each_queue.pop_front().into_iter(){
            let time_second_diff=Utc::now().signed_duration_since(each_req.timestamp).num_seconds();
            if time_second_diff>30{
                // each_req.timestamp=Utc::now();
                let queue_name:Vec<&str>=each_key.split("-").collect();
                let old_level=*queue_name.last().unwrap();
                let new_level=match old_level{
                    "L1"=>"L2",
                    "L2"=>"L3",
                    "L3"=>"L4",
                    "L4"=>"L5",
                    _=>old_level
                };
                let temp_name=queue_name.join("-");
                println!("Old level:{} and new level: {}", old_level,new_level);
                new_queue_name=Some(temp_name.replace(old_level, new_level));
            
                list_req.push_back(each_req);
            }
            
        }
        hash_temp.insert(new_queue_name, list_req);
    }
    
        for (each_key,mut each_req) in hash_temp{
            if each_key.is_some(){
                HASH_QUEUE.write().unwrap().entry(each_key.unwrap()).and_modify(|queue|{
                queue.append(&mut each_req);
        });
            // println!("{:#?}",HASH_QUEUE.read().unwrap());
    }
    }
    
}