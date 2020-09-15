pub fun mergesort(i32 s, i32 e){
	if s < e { 
		let m = s+(e-s)/2;
		mergesort(arr, s, m);
		mergesort(arr, m+1, e);
		merge(arr, s, m, e);
	}
}

fn merge(){

}