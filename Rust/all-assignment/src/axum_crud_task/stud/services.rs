use axum::{extract::Path, response::{IntoResponse, Response}, Json};

use crate::{axum_crud_task::STUD_DATA, common::{Message, StudData}};

pub async fn delete_data_by_id(Path(url_id):Path<i32>)->Response{
    let mut stud_data=STUD_DATA.write().unwrap();
    if let Some(deleted_data)=stud_data.iter_mut().position(|x|url_id==x.id){
        stud_data.remove(deleted_data);
        return {
            Json(Message{
                status:2000,
                message_key:String::from("Success"),
                data:String::from("Data Deleted")
            })
            .into_response()
        };
    }
    else{
        return {
            Json(Message{
                status:3000,
                message_key:String::from("Invalid ID"),
                data:String::from("Data Not Found")
            })
            .into_response()
        };
    }
}

pub async fn get_all_data()->Response{
    let stud_data=STUD_DATA.read().unwrap();
    
    return{
        Json(Message{
            status:2000,
            message_key:String::from("Success"),
            data:stud_data.clone()
        })
    }
    .into_response();
    
}

pub async fn get_data_by_id(Path(url_id):Path<i32>)->Response{
    let stud_data=crate::axum_crud_task::STUD_DATA.read().unwrap().clone();
    let mut final_data=&stud_data[0];
    let mut flag=0;
    for each_data in &stud_data{
        if each_data.id==url_id{
            final_data=each_data;
            flag=1;
            break;
        }
    }
    if flag==1{
        return {
            Json(Message{
                status:2000,
                message_key:String::from("Success"),
                data:final_data
            })
            .into_response()
        };
    }
    else{
        return {
            Json(Message{
                status:3000,
                message_key:String::from("Invalid ID"),
                data:String::from("Data Not Found")
            })
            .into_response()
        };
    }
}

pub async fn new_data(Json(new_data):Json<StudData>)->Response{
    let mut stud_data=STUD_DATA.write().unwrap();
    let index=stud_data.iter().position(|x|x.id==new_data.id);
    if index.is_some(){
        return{
            Json(Message{
                status:3000,
                message_key:String::from("Fail"),
                data:String::from("Data Already Exist")
            })
            .into_response()
        };
    }
    else{
        let sum:i32=new_data.marks.iter().sum();
        let per=sum as f32/new_data.marks.iter().count() as f32;
        let grade=match per{
            90.0..=100.0=>{"A1".to_string()},
            80.0..=90.0=>{"A2".to_string()},
            70.0..=80.0=>{"B1".to_string()},
            60.0..=70.0=>{"B2".to_string()},
            50.0..=60.0=>{"C1".to_string()},
            40.0..=50.0=>{"C2".to_string()}
            _=>{"F".to_string()}
        };
        let n_data=StudData{
            id:new_data.id,
            name:new_data.name,
            phone:new_data.phone,
            email:new_data.email,
            city:new_data.city,
            address:new_data.address,
            marks:new_data.marks,
            percentage:Some(per),
            grade:Some(grade)
        };
        stud_data.push(n_data.clone());
        return{
            Json(Message{
                status:2000,
                message_key:String::from("Success"),
                data:n_data
            })
            .into_response()
        };

    }
   
    
}

pub async fn update_data(Json(one_data):Json<StudData>)->Response{
    let mut stud_data=STUD_DATA.write().unwrap();
    if let Some(data) = stud_data
        .iter_mut()
        .find(|data| data.id == one_data.id)
    {
        let sum:i32=one_data.marks.iter().sum();
        let per=sum as f32/one_data.marks.iter().count() as f32;
        let grade=match per{
            90.0..=100.0=>{"A1".to_string()},
            80.0..=90.0=>{"A2".to_string()},
            70.0..=80.0=>{"B1".to_string()},
            60.0..=70.0=>{"B2".to_string()},
            50.0..=60.0=>{"C1".to_string()},
            40.0..=50.0=>{"C2".to_string()}
            _=>{"F".to_string()}
        };
            
            data.id=one_data.id;
            data.name=one_data.name;
            data.phone=one_data.phone;
            data.email=one_data.email;
            data.city=one_data.city;
            data.address=one_data.address;
            data.marks=one_data.marks;
            data.percentage=Some(per);
            data.grade=Some(grade);


        return{ 
            Json(Message{
                status:2000,
                message_key:String::from("Success"),
                data:String::from("Data Updated")
            })
            .into_response()
        };
    }
    else{
        stud_data.push(one_data.clone());
        return{
            Json(Message{
                status:2000,
                message_key:String::from("Success"),
                data:String::from("New Data Inserted")
            })
            .into_response()
        };

    }
}