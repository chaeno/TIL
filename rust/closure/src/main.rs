use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("시간이 오래 걸리는 계산을 수행 중");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let simulated_user_specified_value = 12;
    let simulated_random_number = 4;

    generate_workout (
        simulated_user_specified_value,
        simulated_random_number
    );

    fn add_one_v1 (x: u32) -> u32 {x + 1}
    let add_one_v2 = |x: u32| -> u32 {x + 1};
    let add_one_v3 = |x| {x+1};
    let add_one_v4 = |x| x+1;
    let result = add_one_v3(1);
    let result = add_one_v4(1);

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // 위에서 추론된 closure의 매개변수 타입과 미일치하여 에러
    // let n = example_closure(5);

    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // 소유권이 closure에게 넘어갔기 때문에 x를 사용할 수 없다.
    // println!("변수 x를 사용할 수 없습니다: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

struct Cacher<T>
    where T: Fn(u32) -> u32 
{
    calculation: T,
    value: Option<u32>
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None
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

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| -> u32 {
        println!("시간이 오래 걸리는 계산을 수행 중...");
        // thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("오늘은 {}번의 팔 굽혀펴기를 하세요!", expensive_result.value(intensity));
        println!("다음에는 {}번의 윗몸 일으키기를 하세요!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("오늘은 수분을 충분히 섭취하며 쉬세요!");
        } else {
            println!("오늘은 {}분간 달리기를 하세요!", expensive_result.value(intensity));
        }
    }
}
