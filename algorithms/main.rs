mod mergesort;

use rand::Rng;

fn main(){
	let mut rng = rand::thread_rng();

	let vec: Vec<i32> = Vec::new();

	for x in 0..100 {
		let num = rng.gen_range(0, 10000);
		vec.push(num);
	}
	println!("{:?}", numbers);

}