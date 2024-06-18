use std::{fs::{File, OpenOptions}, io::Write, thread, time::Duration};
// use std::io::prelude::*;

fn main() {

   let file = OpenOptions::new()
   .write(true)
   .append(true)
   .open("neko.txt")
   .expect("errrrrror");

    // let k:usize = 8;
    // for i in 3..k {
        
    
    let mut element = vec!['a','b','c','d','e','f','g','h','i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z','1','2','3','4', '5','6','7','8','9', '0'];
    let element_upper: Vec<_> = element.iter().map(|a| a.to_uppercase().next().unwrap()).collect();
    element.extend(element_upper);

    kombinasi(element, 8, file);
    // let kombinate = kombinasi(element, i + 1);
    
    // for kombinasi in kombinate {
    //     let mut text = String::new();
    //     for char  in kombinasi {
    //         text.push(char);
    //         thread::sleep(Duration::from_millis(250));
    //     }
    //     file.write_all(format!("{text} \n").as_bytes()).expect("errrorororo");
    //     thread::sleep(Duration::from_millis(250));
    // }
    // }

}


#[allow(unused)]
fn kombinasi(mut element:Vec<char>, k:usize,mut file:File){
    // let mut result: Vec<Vec<char>> = Vec::new();
    let mut data: Vec<char> = Vec::new();

    fn kombinasi_gen(data:&mut Vec<char>, k:usize, element:&mut Vec<char>, file:&mut File){
        if data.len() == k {
            let mut kombinasi = String::new();
            for char in data {
                kombinasi.push(*char);
            }
            file.write_all(format!("{kombinasi}\n").as_bytes()).expect("error write");
            println!("{:?}", kombinasi);
            return;
        }

        for i in  0..element.len() {
            data.push(element[i]);
            kombinasi_gen(data, k, element, file);
            data.pop();
        }
    }

    kombinasi_gen(&mut data, k, &mut element, &mut file);
    // result
}
