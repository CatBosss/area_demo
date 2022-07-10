//实现一个打印图形面积的函数，它接收一个可以计算面积的类型作为参数，比如圆形，三角形，正方形，需要用到泛型和泛型约束。

use std::f64::consts::PI;

//定义一个特征 有一个打印面积函数
trait AreaType{
    fn area_types_print(&self);
}

//定义一个长方形的结构体
struct LongBox {
    width:u64,
    long:u64,
}

//定义一个正方形形的结构体
struct SquareBox {
    width:u64,
    long:u64,
}

//定义一个三角形结构体
struct TriangleBox{
    high:u64,  //高
    long:u64,   //底
}

//定义一个圆形结构体
struct CircleBox{
    r:f64,   //半径
}

//能打印面积的都是写areaType的特征
impl AreaType for LongBox{
    fn area_types_print(&self){
        println!("area is（长方形的面积是） {}",self.long*self.width);
    }
}
impl AreaType for SquareBox{
    fn area_types_print(&self){
        println!("area is（正方形的面积是） {}",self.long*self.width);
    }
}

impl AreaType for TriangleBox{
    fn area_types_print(&self){
        println!("area is（三角形面积是） {}",self.long*self.high/2);
    }
}
impl AreaType for CircleBox{
    fn area_types_print(&self){
        println!("area is（圆形面积是） {}",self.r*self.r*PI);
    }
}


//打印函数 T限定为实现了可打印面积特征areaType
fn area_ptint<T:AreaType>(item:T){
    item.area_types_print();
}

fn main() {
    let long_box = LongBox {width:60,long:20};
    let square_box = SquareBox{width:60,long:60};
    let triangle_box = TriangleBox{high:60,long:60};
    let circle_box = CircleBox{r:60.0};

    area_ptint(long_box);
    area_ptint(square_box);
    area_ptint(triangle_box);
    area_ptint(circle_box);
}
