# subtratework
![image](https://github.com/enginefuture/subtratework/blob/master/class4/trafficlight/%E8%BF%90%E8%A1%8C%E6%88%AA%E5%9B%BE.png)
```
pub  trait  Trafficlighttime {
    fn lighttime(&self) -> u8;
}
enum Trafficlight {
    Red,
    Yellow,
    Green
}

impl Trafficlighttime for Trafficlight {
    fn lighttime(&self) -> u8 {
        match self {
            Self::Red => 10,
            Self::Yellow => 20,
            Self::Green => 30,
        }
    }
}


fn main() {
    let red = Trafficlight::Red.lighttime();
    let yellow = Trafficlight::Yellow.lighttime();
    let green = Trafficlight::Green.lighttime();

    println!("red time is : {:?}",red);
    println!("yellow time is : {:?}",yellow);
    println!("green time is : {:?}",green);
}

```
