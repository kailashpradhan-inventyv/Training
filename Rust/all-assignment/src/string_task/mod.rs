

pub fn string_find()->String{
    let mut string="Welcome to Inventyv Software Services".to_lowercase();
    let mut updated_string:String=String::new();
    let ch="e";
    for i in 0..string.len(){
       let s=&string[i..i+1];
       if s==ch{
        updated_string.push_str("_");
       }
       else{
        updated_string.push_str(s);
       }
    }
    println!("String : {:?}\nUpdated String {:?}",string,updated_string);

    updated_string
}