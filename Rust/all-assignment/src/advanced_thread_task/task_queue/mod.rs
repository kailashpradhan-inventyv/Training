use std::{collections::VecDeque, ops::Deref};

use super::{HASH_KEY, HASH_QUEUE, USER_SKILLS};
pub fn task_queue(){
    let time_level=vec!["L5","L4","L3","L2","L1"];
    let call_chat=vec!["Call","Chat"]; 
    let lang=vec!["English","Spanish"];
    for each_level in time_level{
        for each_lang in &lang{
            for each_skill in USER_SKILLS.deref(){
                for each_type in &call_chat{
                    let x=format!("{}-{}-{}-{}",each_type,each_skill,each_lang,each_level);
                    let key=&x;
                    HASH_QUEUE.write().unwrap().insert(key.to_string(), VecDeque::new()); 
                    HASH_KEY.write().unwrap().push(key.to_string());
                }
            }
        }
    }
    // println!("{:#?}",HASH_KEY.read().unwrap());
}