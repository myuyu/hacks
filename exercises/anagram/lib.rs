use std::collections::{HashMap, HashSet};

pub fn anagrams_for(word: &str, po: &[&str]) -> HashSet<String> {
    let possible_anagrams: Vec<_>= po.iter().map(|f|f.to_lowercase()).collect();
    let mut v: Vec<HashMap<String, Vec<char>>> = Vec::new();
    for i in possible_anagrams{
        let mut h= HashMap::new();
        let mut m = Vec::new();
        for c in i.chars(){
            m.push(c);
        }
        h.insert(i, m);
        v.push(h);
    }
    let mut s: Vec<_>= word.to_lowercase().chars().collect();
    let mut res = HashSet::new();
    for hashs in v{
        for (k,mut v) in hashs{
            if k.len() == word.len(){
                v.sort();s.sort();
                if v == s{
                    res.insert(k);
                }
            }
        }
    }
    res
}
