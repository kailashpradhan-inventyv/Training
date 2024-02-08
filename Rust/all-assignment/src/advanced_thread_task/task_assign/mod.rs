use std::{fs::read, sync::Arc, thread::{sleep, spawn}, time::Duration};

use super::{request_data_call, request_data_chat, MASTER_DATA};
pub fn task_assign(){
    let chat_data=Arc::clone(&request_data_chat);
    let call_data=Arc::clone(&request_data_call);
    let chat_thread=spawn(move||loop{
        sleep(Duration::from_secs(10));
        let chat_one_data=chat_data.write().unwrap().pop_front().unwrap();
        let mut flag=0;
        for i in 0..MASTER_DATA.read().unwrap().len(){
            flag=0;
            if MASTER_DATA.read().unwrap()[i].status=="Online"{
                if MASTER_DATA.read().unwrap()[i].skills.contains(&chat_one_data.skills) && MASTER_DATA.read().unwrap()[i].language==chat_one_data.language{
                    flag=1;
                    println!("One Chat Task Assigned By ID {:?}",MASTER_DATA.read().unwrap()[i].id);
                    break;
                }
            }
        }
        if flag==0{
            chat_data.write().unwrap().push_back(chat_one_data);
        }
    });
    let call_thread=spawn(move||loop{
        sleep(Duration::from_secs(10));
        let call_one_data=call_data.write().unwrap().pop_front().unwrap();
        let mut flag=0;
        for i in 0..MASTER_DATA.read().unwrap().len(){
            flag=0;
            if MASTER_DATA.read().unwrap()[i].status=="Online"{
                if MASTER_DATA.read().unwrap()[i].skills.contains(&call_one_data.skills) && MASTER_DATA.read().unwrap()[i].language==call_one_data.language{
                    flag=1;
                    println!("One Call Task Assigned By ID {:?}",MASTER_DATA.read().unwrap()[i].id);
                    break;
                }
            }
        }
        if flag==0{
            call_data.write().unwrap().push_back(call_one_data);
        }
    });

    chat_thread.join();
    call_thread.join();
}