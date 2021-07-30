use isahc::prelude::*;
use clap::{Arg, App,AppSettings};
use serde_json::Value;
use std::process::{Command,Stdio};
use std::env;

const Url: &str = "http://api.whoxy.com/?reverse=whois";

fn main() {
    let matchs = App::new("Rwhoxy").setting(AppSettings::ArgRequiredElseHelp).version("0.1.0").author("by elmyuyu").about("Extract domains from whoxy").arg(Arg::with_name("set").short("set").required(false).help("Set whoxy token in env variable ").takes_value(true)).arg(Arg::with_name("email").short("e").help("Set Organization email").takes_value(true)).get_matches();
    match matchs.is_present("set"){
        true => {
            Command::new("sh").stdout(Stdio::null()).args(&["-c",&format!("echo 'export rkwhoxy={}' >> ~/.bashrc",matchs.value_of("set").unwrap())]).spawn();
            Command::new("source").stdout(Stdio::null()).arg("~/.bashrc").spawn();
            std::process::exit(0);
    },
    false => ()
}
    let URL = format!("{}&key={}&email={}&page=",Url,env::var("rkwhoxy").unwrap(),matchs.value_of("email").unwrap());
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
