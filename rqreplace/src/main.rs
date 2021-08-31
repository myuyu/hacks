use std::borrow::Cow;
use url::Url;
use clap::{Arg, App,AppSettings};

fn main(){
    let app = App::new("rqreplace").setting(AppSettings::ArgRequiredElseHelp).args_from_usage(
        "-v, --value=[VALUE] 'New value for params'
         -u, --url=[URL] 'URL'"
    ).get_matches();
    let inject = app.value_of("value").unwrap();
    let m = Url::parse(app.value_of("url").unwrap()).unwrap();
    let v: Vec<_> = m.query_pairs().map(|(k,v)|{(k,inject)}).collect();
    // build(, v);
    build(&m,v);
}

fn build(u: &Url,params: Vec<(Cow<str>, &str)>) {
    let url = format!("{}://{}{}",&u.scheme(),u.host_str().unwrap(),u.path());
    let u = Url::parse_with_params(&url, params).unwrap();
    println!("{}",u);
}
