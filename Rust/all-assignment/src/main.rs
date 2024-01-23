//! There are 6 modules in this Assignment

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

fn main() {
    employee::emp_sorting();
    student::students_result();
    string_task::string_find();
    string_freq::string_frequency();
    table::table_update();
}
