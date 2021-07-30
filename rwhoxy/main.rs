use isahc::prelude::*;
use clap::{Arg, App};
use serde_json::Value;

const Url: &str = "http://api.whoxy.com/?reverse=whois";

fn main() {
    let matchs = App::new("Rwhoxy").version("1.0").author("elmyuyu").about("Extract domains from whoxy").arg(Arg::with_name("key").short("k").required(true).help("Set Whoxy Key value").takes_value(true)).arg(Arg::with_name("email").short("e").help("Set Organization email").takes_value(true)).get_matches();
    let URL = format!("{}&key={}&email={}&page=",Url,matchs.value_of("key").unwrap(),matchs.value_of("email").unwrap());
    let i: Value= serde_json::from_str(&isahc::get(&URL).unwrap().text().unwrap()).unwrap();
    let mut domains: Vec<String> = Vec::new();
    for s in 1..=i["total_pages"].as_u64().unwrap() {
        domains.append(&mut request(s,&URL));
    }
    domains.sort();domains.dedup();
    for i in domains {
        println!("{}",i);
    }
}


 fn request(page: u64, URL: &String) -> Vec<String> { 
    let resp = isahc::get(format!("{}{}",URL,page)).unwrap().text().unwrap();
    parse(resp)
    
} 

fn parse(resp: String) -> Vec<String>{
    let mut ve: Vec<String> = Vec::new();
    let s: Value= serde_json::from_str(&resp).unwrap();
    for i in s["search_result"].as_array().unwrap().iter(){
        for (k,v) in i.as_object().unwrap(){
            match k.as_str(){
                "domain_name" => ve.push(v.to_string()),
                _=> ()
            }
        }

}
    ve
}
