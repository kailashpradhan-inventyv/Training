use std::{collections::{HashMap, VecDeque}, fs::read_to_string, sync::{Arc, RwLock}, thread::{ sleep, spawn}, time::Duration};

use lazy_static::lazy_static;
use rand::Rng;
pub mod task_bifurcation;
pub mod random_task;
pub mod task_assign;
pub mod shuffle_customerdata;
pub mod task_queue;
pub mod bifurcation_task_hashmap;
pub mod bifurcation_level_hashmap;
pub mod hash_task_assign;



use crate::{advanced_thread_task::{task_bifurcation::task_bifurcation,random_task::random_task,task_assign::task_assign,shuffle_customerdata::shuffle_customerdata,task_queue::task_queue,bifurcation_task_hashmap::task_bifurcation_hashmap,bifurcation_level_hashmap::bifurcation_level,hash_task_assign::hash_task_assign}, common::{CustomerData,request_data,language,available}};
lazy_static!{ 
    #[derive(Debug)]
    pub static ref MASTER_DATA:Arc<RwLock<VecDeque<CustomerData>>>={
        let des_data=read_to_string("JSON-data/Master_Data.json").expect("Unable to Read");
        let master_data:VecDeque<CustomerData>=serde_json::from_str(&des_data).expect("Deserialize Fail");
        Arc::new(RwLock::new(master_data))
    };
    pub static ref REQUEST_DATA: Arc<RwLock<VecDeque<request_data>>>  =Arc::new(RwLock::new(VecDeque::new()));
    pub static ref  request_data_call: Arc<RwLock<VecDeque<request_data>>>  =Arc::new(RwLock::new(VecDeque::new()));
    pub static ref  request_data_chat: Arc<RwLock<VecDeque<request_data>>>  =Arc::new(RwLock::new(VecDeque::new()));
    pub static ref USER_SKILLS:Vec<String>={
        let mut user_skills:Vec<String>=Vec::new(); 
        user_skills.push("Customer Service".to_string());
        user_skills.push("Problem-solving".to_string());
        user_skills.push("Product Knowledge".to_string());
        user_skills.push("Effective Communication".to_string());
        user_skills.push("Time Management".to_string());
        user_skills.push("Adaptability".to_string());
        user_skills.push("Team Collaboration".to_string());
        user_skills.push("Feedback Analysis".to_string());
        user_skills.push("Proactive Engagement".to_string());
        user_skills.push("Technical Proficiency".to_string());
        user_skills.push("Cultural Sensitivity".to_string());
        user_skills.push("Documentation".to_string());
        user_skills
    };
    pub static ref HASH_QUEUE:Arc<RwLock<HashMap<String,VecDeque<request_data>>>>=Arc::new(RwLock::new(HashMap::new()));

    pub static ref HASH_KEY:Arc<RwLock<Vec<String>>>=Arc::new(RwLock::new(Vec::new()));
}

pub fn advanced_task(){
    let thread1=spawn(move||loop{
        sleep(Duration::from_secs(1));
        random_task();
   });

   //Created Hashmap<String,VecDequeu<request_task>>
   task_queue();

   //Task Bifurcation
   let thread2=spawn(move||loop{
      sleep(Duration::from_secs(3));
      task_bifurcation_hashmap();
   });


    let thread3=spawn(move||loop{
        sleep(Duration::from_secs(5));
        bifurcation_level();
    });

    let task_queue_hash=spawn(move||loop{
        sleep(Duration::from_secs(4));
        hash_task_assign();
    });

//    let task_queue_thread=spawn(move||{
//         task_queue();
//    });
    
    // let thread2=spawn(move||loop{
    //     sleep(Duration::from_secs(2));
    //     task_bifurcation();
        
    // });

    // let thread4=spawn(move||{
    //     // sleep(Duration::from_secs(5));
    //     task_assign();
        
    // });

    let thread5=spawn(move||loop{
        sleep(Duration::from_secs(15));
        shuffle_customerdata();
        // println!("{:#?}",MASTER_DATA.read().unwrap());
    });
   
    thread1.join();
    thread2.join();
    thread3.join();
    // thread4.join();
    task_queue_hash.join();
    thread5.join();
}

