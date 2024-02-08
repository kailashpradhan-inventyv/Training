use std::{collections::VecDeque, sync::{Arc, RwLock}};
use crate::common::{available, request_data};

use super::{request_data_call, request_data_chat, REQUEST_DATA};
pub fn task_bifurcation(){
    let ref2_request_data = Arc::clone(&REQUEST_DATA);
    let ref_call_data=Arc::clone(&request_data_call);
    let ref_chat_data=Arc::clone(&request_data_chat);
    let data1=ref2_request_data.write().unwrap().pop_front().unwrap();
    let status=&data1.available;
    if *status==available::Call{
        ref_call_data.write().unwrap().push_back(data1);
        
        // println!("Call : {:?} ",request_data_call.read().unwrap());
    }
    else{
        ref_chat_data.write().unwrap().push_back(data1);    
     // println!("Chat : {:?} ",request_data_chat.read().unwrap());
    }  
}