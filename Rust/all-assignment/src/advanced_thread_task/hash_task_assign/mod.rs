
use std::sync::Arc;

use super::{HASH_KEY, HASH_QUEUE, MASTER_DATA, USER_SKILLS};


pub fn hash_task_assign(){
    // let level=vec!["L5","L4","L3","L2","L1"];
    // let avail_type=vec!["Chat","Call"];
    // let lang=vec!["English","Spanish"];
    // let read_hash=Arc::clone(&HASH_QUEUE);
    let binding = Arc::clone(&HASH_QUEUE);
    let mut write_hash=binding.write().unwrap();
    let vec_bind=Arc::clone(&HASH_KEY);
    let hash_key =vec_bind.read().unwrap();
    for each_key in hash_key.iter(){
        let master_read=MASTER_DATA.read().unwrap();
        for i in 0..master_read.len(){
            let mut flag: i32=0;
            if !write_hash.get(&*each_key).unwrap().is_empty(){
                if master_read[i].status=="Online"{
                    if master_read[i].skills.contains(&write_hash.get(&*each_key).unwrap().front().unwrap().skills) && master_read[i].language==write_hash.get(&*each_key).unwrap().front().unwrap().language{
                        flag=1;
                        write_hash.get_mut(&*each_key).unwrap().remove(0).unwrap();
                        println!("One Task Assigned By ID {:?}",MASTER_DATA.read().unwrap()[i].id);
                        break;
                    }
                }
            }
            
        }
                        
    }
}       
        
    


