fn main() {
  let v =vec![1,2,3,4,5,6];
  let x=vec![7,8,9,10,11,12];

  for index in 0..6{
    let sum =v[index]+ x[index];
    println!("The sum of {} and {} is {}",v[index],x[index],sum);
  }  
}
