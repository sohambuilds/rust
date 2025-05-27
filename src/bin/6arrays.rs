fn main(){

let arr: [i32; 5] = [1, 2, 3, 4, 5];

println!("arr: {:?}", arr);

println!("arr[0]: {}", arr[0]);

let slice: &[i32] = &arr[1..4];
println!("slice: {:?}", slice);

//Arays with all zeros
let zeros: [i32; 5] = [0; 5];
println!("zeros: {:?}", zeros);

}