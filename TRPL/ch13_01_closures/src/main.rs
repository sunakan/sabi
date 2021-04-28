use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

// カスタマイズされたエクササイズのトレーニングプランを生成するアプリを作るスタートアップ
// 年齢、BMI、運動の好み、最近のトレーニング、指定された強弱値
// 計算に数秒かかるとする
fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly....");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Pushup!! do {}", expensive_result.value(intensity));
        println!("Situps!! do {}", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today!");
        } else {
            println!("Run!! {} minutes", expensive_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 100;
    let simulated_random_number = 3;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}
