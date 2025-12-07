fn main() {
    let v = vec![10,20,30];

    let v1=v;

    display(v1.clone());

    println!("In main {:?}", v1);

}

fn display(v:Vec<i32>){

    println!("Inside display {:?}",v);
}
