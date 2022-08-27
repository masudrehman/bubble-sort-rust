fn main() {
    let mut  v = vec![10, 30 ,5, 3, 99, 1, 4, 3, 2, 2, 165];
    bubble_sort(&mut v);    
}

fn bubble_sort(v: &mut Vec<i32>) {
    for i in 0..v.len()  {
        for j in 0..(v.len() - i - 1) {
            if v[j] > v[j+1] {
                let temp = v[j];
                let temp2 = v[j + 1];
                v[j] = temp2;
                v[j+1] = temp;
            }
        }
    }

    println!("{:#?}", v);
}
