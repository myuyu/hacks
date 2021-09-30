fn re(i: &str){
    let m = i.chars().rev().collect::<String>();
    println!("{}",m);
}

fn main(){
    re("Mahdi");
}
