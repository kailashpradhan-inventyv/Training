use std::sync::Arc;

// use super::ref1_request_data;
use crate::{advanced_thread_task::{REQUEST_DATA, USER_SKILLS}, common::{language,available,request_data}};
use chrono::Utc;
// use super::
use rand::Rng;
pub fn random_task(){
    let ref1_request_data = Arc::clone(&REQUEST_DATA);
    
    let random_num =rand::thread_rng().gen_range(1..13);
    let u_skill=match random_num{
       
        1=>&USER_SKILLS[0],
        2=>&USER_SKILLS[1],
        3=>&USER_SKILLS[2],
        4=>&USER_SKILLS[3],
        5=>&USER_SKILLS[4],
        6=>&USER_SKILLS[5],
        7=>&USER_SKILLS[6],
        8=>&USER_SKILLS[7],
        9=>&USER_SKILLS[8],
        10=>&USER_SKILLS[9],
        11=>&USER_SKILLS[10],
        12=>&USER_SKILLS[11],
        _=>todo!()
    };
    let random_num =rand::thread_rng().gen_range(1..3);
    let u_lang=match random_num{
        1=>language::English,
        2=>language::Spanish,
        _=>todo!()

        
    };
    let random_num =rand::thread_rng().gen_range(1..3);
    let u_avail=match random_num {
        1=>available::Call,
        2=>available::Chat,
        _=>todo!()

    };
    ref1_request_data.write().unwrap().push_back(request_data{skills:u_skill.to_string(),language:u_lang,available:u_avail,timestamp:Utc::now()});  
    println!("New Task Generated");
    
}