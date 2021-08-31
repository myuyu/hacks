use std::{borrow::Cow, io::{BufRead, BufReader}};
use url::Url;
use clap::{App,AppSettings};

fn main(){
    let app = App::new("rqreplace").setting(AppSettings::ArgRequiredElseHelp).args_from_usage(
        "-v, --value=[VALUE] 'New value for params'
         -f, --file=[URL] 'File contain list of urls'"
    ).get_matches();
    let inject = app.value_of("value").unwrap();
    let v = std::fs::File::open(app.value_of("file").unwrap()).expect("Soemthing wrong with file or path");
    for h in BufReader::new(v).lines(){
        qrs(&h.unwrap(), inject)
    }
}
fn qrs(url: &str,inject: &str){
    let m = Url::parse(url).unwrap();
    let v: Vec<_> = m.query_pairs().map(|(k,v)|{(k,inject)}).collect();
    build(&m,v);
}

fn build(u: &Url,params: Vec<(Cow<str>, &str)>) {
    let url = format!("{}://{}{}",&u.scheme(),u.host_str().unwrap(),u.path());
    let u = Url::parse_with_params(&url, params).unwrap();
    println!("{}",u);
}
