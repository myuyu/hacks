use anagram::anagrams_for;

fn main(){
    let p = anagrams_for("ΑΒΓ",&["Βγα", "ΒΓΔ", "γβα"]);
    for v in p{
        println!("{}",v);
    }
}
