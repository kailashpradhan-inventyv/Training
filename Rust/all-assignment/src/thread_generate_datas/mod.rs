use std::{fmt::Write, sync::{Arc, RwLock}, thread, time::{Duration, SystemTime}};
use rand::Rng;
use chrono::{prelude::*};

use crate::common::Data;


pub fn generate_datas(){
    let mut SID:i32=1 ;
    let v1: RwLock<Vec<Data>>  =RwLock::new(Vec::new());
    let arc=Arc::new(v1);
    let ref1=Arc::clone(&arc);
    let ref2=Arc::clone(&arc);
    let ref3=Arc::clone(&arc);

    
    let t1=thread::spawn(move||loop {
        thread::sleep(Duration::from_secs(5));
        println!("{}",ref1.read().unwrap().len());
    });
    let t2=thread::spawn(move||loop{
        thread::sleep(Duration::from_secs(1));
        let dtime=Utc::now();
        ref2.write().unwrap().push(Data{id:SID,name:randomString(),timestamp:dtime});
        SID=SID+1;
        println!("Data Updated");
        
    });

    let t3=thread::spawn(move||loop{
        thread::sleep(Duration::from_secs(10));
        let dtime=Utc::now();
        let mut data=ref3.write().unwrap();
        data.retain(|x| {
            let dur=dtime.signed_duration_since(x.timestamp);
            dur.num_seconds()<5
        });
        println!("Data Deleted");
    });
 
    t1.join();
    t2.join();
    t3.join();
}

fn randomString()->String{
    let secret_number = rand::thread_rng().gen_range(5..=15);
    let mut randomName:String="".to_string();
    for i in 0..secret_number{
        let random_char:u8=rand::thread_rng().gen_range(97..=122);
        randomName.write_char(random_char as char);
    }
    randomName
}