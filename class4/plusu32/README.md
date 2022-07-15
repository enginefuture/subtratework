# subtratework
![image](https://github.com/enginefuture/subtratework/blob/master/class4/plusu32/%E8%BF%90%E8%A1%8C%E6%88%AA%E5%9B%BE.png)

fn plusu32(s :&[u32]) -> Option<u32>{
    let sum = s.iter().try_fold(0u32, |sum,x|sum.checked_add(*x));
    return sum
}
fn main() {
    let maxu32 = u32::MAX;
    // print!("u32 max is {}------",maxu32);
    let s = [maxu32,1];
    print!("arr is {:?}",s);
    // let sum = plusu32(&s);
    match plusu32(&s) {
        Some(x) => print!("-------sum is {}\n",x),
        None =>print!("-------sum is None \n"),
    }
    let a = [19u32,1];
    print!("arr is {:?}",a);
    match plusu32(&a) {
        Some(x) => print!("-------sum is {} \n",x),
        None =>print!("-------sum is None \n"),
    }
}

