fn main() {
    let element = vec!['a','b','c'];
    let k:usize = 3;

    let kombinate = kombinasi(element, k);
    
    for kombinasi in kombinate {
        let mut text = String::new();
        for char in kombinasi {
            text.push(char);
        }
        println!("{}", text);
    }


}


#[allow(unused)]
fn kombinasi(mut element:Vec<char>, k:usize) -> Vec<Vec<char>>{
    let mut result: Vec<Vec<char>> = Vec::new();
    let mut data: Vec<char> = Vec::new();

    fn kombinasi_gen(result:&mut  Vec<Vec<char>>, data:&mut Vec<char>, k:usize, element:&mut Vec<char>){
        if data.len() == k {
            result.push(data.clone());
            return;
        }

        for i in  0..element.len() {
            data.push(element[i]);
            kombinasi_gen(result,data, k, element);
            data.pop();
        }
    }

    kombinasi_gen(&mut result, &mut data, k, &mut element);
    result

}
