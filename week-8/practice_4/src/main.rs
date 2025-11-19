fn main() {
    let name= vec!["mary", "samad","sally","greg","ade","mark"];

    let age =vec![20,21,19,22,23,20];

    for i in 0..name.len(){
        println!("{} is {} years old",name[i],age[i]);

    }
}