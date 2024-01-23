use std::{char, i32, usize};

pub fn string_frequency(){
    use crate::string_task::string_find;
    
    let str1="lorem ipsum".to_lowercase();
    let str2="ceaser cypher".to_lowercase();
    let mut unique_chars:String=String::new();

    // for i in str1.chars(){
    //     if !i.is_whitespace() && !unique_chars.contains(i){
    //         unique_chars.push(i);
    //     }
    // }
    // for i in str2.chars(){
    //     if !i.is_whitespace() && !unique_chars.contains(i){
    //         unique_chars.push(i);
    //     }
    // }

    for i in str1.chars().chain(str2.chars()){
        if !i.is_whitespace() && !unique_chars.contains(i){
            unique_chars.push(i);
        }
    }

    // println!("{:?}",unique_chars);

    let mut com:Vec<(char,usize)>=Vec::new();
    let mut uncom:Vec<(char,usize)>=Vec::new();
    let mut count_str1 = [0; 128]; 
    let mut count_str2 = [0; 128];

    // str1 char count
    for i in str1.chars(){
        if !i.is_whitespace() {
            count_str1[i as usize] += 1;
        }
    }

    for i in str2.chars(){
        if !i.is_whitespace() {
            count_str2[i as usize] += 1;
        }
    }

    for ch in unique_chars.chars(){
        let freq1=count_str1[ch as usize];
        let freq2=count_str2[ch as usize];
        if freq1>0 && freq2>0{
            com.push((ch,freq1+freq2));
        }
        else{
            uncom.push((ch,0));
        }
    }
    // com.sort();

    for i in 0..com.len(){
        for j in 0..com.len()-1-i{
            if com[j].0>com[j+1].0{
                let temp =com[j];
                com[j]=com[j+1];
                com[j+1]=temp;
            }
        }
    }

    println!("V1 : {:?}\n V2: {:?}",com,uncom);
    let mut index=0;
    let updated_string=string_find();
    let mut final_str=String::new();
    // println!("{:?}",final_str);

    for i in updated_string.chars() {
        if i == '_' {
            let mut found_char=false;
            for j in 0..com.len() {
                let (ch, freq) = &mut com[j];
                if *freq > 0 {
                    final_str.push(*ch);
                    *freq -= 1;
                    found_char=true;
                    break; 
                }
            }
            if !found_char{
                final_str.push('_');
            }
        } 
        else {
            final_str.push(i);
        }
    }
    println!("{:?}",final_str);
    // println!("{:?}",com);

    let mut all:Vec<(char,usize)>=Vec::new();
    all.extend(com);
    all.extend(uncom);
    // all.sort();
    for i in 0..all.len(){
        for j in 0..all.len()-1-i{
            if all[j].0>all[j+1].0{
                let temp =all[j];
                all[j]=all[j+1];
                all[j+1]=temp;
            }
        }
    }
    println!("{:?}",all);
}
