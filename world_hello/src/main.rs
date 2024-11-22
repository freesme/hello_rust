fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions: [&str; 3] = [southern_germany, chinese, english];
    for ele in regions {
        println!("{}", ele)
    }
}


fn start() {
    let penguin_data = "\
   common name,length (cm)
   Little penguin,33
   Yellow-eyed penguin,65
   Fiordland penguin,60
   Invalid,data";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        // 声明一个 fields 变量，类型是 Vec
        // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
        // <_>表示 Vec 中的元素类型由编译器自行推断
        let fields: Vec<_> = record
            .split(",")
            .map(|field| field.trim())
            .collect();

        if cfg!(debug_assertons) {
            // 输出到标准错误输出
            eprint!("debug: {:} -> {:?}",
                    record, fields)
        }

        let name: &str = fields[0];

        // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
        //
        // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
        //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
        //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
        //
        // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{},{}cm", name, length)
        } else if let Err(e) = fields[1].parse::<f32>() {
            println!("[ERROR] source: {} , err: {}", fields[1], e)
        }
    }
}


fn main() {
    greet_world();
    start();
    let circle = Circle::new(0.0, 0.0, 1.0);
    let x = circle.area();
    println!("area: {}", x);

    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());
}


struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn new(x: f64, y: f64, radius: f64) -> Circle {
        Circle { x, y, radius }
    }

    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

mod trait_demo;
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}


// 为Post实现Summary特征
impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{} by {} : {}", self.title, self.author, self.content)
    }
}


pub struct Weibo {
    pub username: String,
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}