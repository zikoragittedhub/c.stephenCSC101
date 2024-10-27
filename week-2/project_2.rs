fn main(){
	let tq: f64 = 2.0;
	let mq: f64 = 1.0;
	let hq: f64 = 3.0;
	let dq: f64 = 3.0;
	let aq: f64 = 1.0;
    let t:f64 = tq * 450000.0;
    let m:f64 = mq * 1500000.0;
    let h:f64 = hq * 750000.0;
    let d:f64 = dq * 2850000.0;
    let a:f64 = aq * 250000.0;

    let sum:f64 = t + m + h + d + a;
    let avg: f64 = sum / (tq + mq + hq + dq + aq);
    println!("sum: {}, avg: {}", sum, avg);
}