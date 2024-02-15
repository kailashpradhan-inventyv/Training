//! There are 6 modules in this Assignmentx

/// Sort the json file of employee according to our need
pub mod employee;
/// All the strcutures and enums are define here
pub mod common;
/// Calculate the grade and percentage of student and update the json file
pub mod student;
/// Replace the particular character with _(underscore)
pub mod string_task;
/// Find the frequency of two string and update in string task according to its frequency
pub mod string_freq;
/// Table has Row and Column in that update Cell width and height according to our need.
pub mod table;
/// Completed the Employee Task using HashMap
pub mod emp_by_hashmap;
///Completed the Student Result Task using HashMap
pub mod stud_by_hashmap;
///Completed the Table Task with Hashmap having their content font value and calculate height of cell accordingly.
pub mod table_by_map;
///Completed the Thread Task having data i.e., ID, Name and Timestamp.
pub mod thread_generate_datas;
///Advance Task of Thread
pub mod advanced_thread_task;
///CRUD Operation on Employee,Student,MasterData
pub mod axum_crud_task;
#[tokio::main]
async fn main() {
    // employee::emp_sorting();
    // student::students_result();
    // string_task::string_find();
    // string_freq::string_frequency();
    // table::table_update();
    // emp_by_hashmap::emp_update_hashmap();
    // stud_by_hashmap::students_result_hashmap();
    // table_by_map::table_value_update();
    // thread_generate_datas::generate_datas();
    // advanced_thread_task::advanced_task();
    axum_crud_task::axum_server().await;
    
    

}



