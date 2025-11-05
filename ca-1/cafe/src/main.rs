use std::io;

fn main() {
    let response="no";

    println!("THE CAFE MENU");
    println!("CODE   ITEM   PRICE");
    println!("T   TEA   800");
    println!("C   COFFEE   1200");
    println!("S   SANDWICH   2000");
    println!("J   JUICE   1500");
while response.trim() !="exit"{
    println!("Input the code of the item you want: ");
    let mut code= String::new();
    io::stdin().read_line(&mut code).expect("There was an error");
    println!("Quantity of the item: ");
    let mut quan= String::new();
    io::stdin().read_line(&mut quan).expect("There was an error");
    let quan:f32=quan.trim().parse().expect("There was an error");

    let mut price:f32=0.0;

        println!("The total amount is {}",price);

    if code=="T"{
        price=quan*800.0;
        println!("The total amount is {}",price);

    }else if code=="C"{
        price=quan*1200.0;
        println!("The total amount is {}",price);
    }else if code=="S"{
        price=quan*2000.0;
        println!("The total amount is {}",price);
    }else if code=="J"{
        price=quan*1500.0;
        println!("The total amount is {}",price);
    }else{
        println!("Choose a code from (T,C,S, or J)");
    }

    if price>5000.00{
        price= 5000.00-(5000.00*0.05);
        println!("The total amount is {}",price);
    }
     

     println!("Do you want to exit (no/exit)");
     let mut response=String::new();
     io::stdin().read_line(&mut response).expect("There was an error");
}


}
