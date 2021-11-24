mod front_of_house;

pub use crate::front_of_house::hosting;

pub mod hosting {
    pub fn add_to_waitlist() {}
    fn seat_at_table() {}
}

mod serving {
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {}
}

pub fn eat_at_restaurant() {
    // 절대 경로
    crate::front_of_house::hosting::add_to_waitlist();
    // 상대 경로
    front_of_house::hosting::add_to_waitlist();

    // 여름에 아침 식사로 호밀빵을 주문한다.
    let mut meal = back_of_house::Breakfast::summer("호밀빵");
    // 고객이 마음을 바꿔서 빵 종류를 바꾼다.
    meal.toast = String::from("밀빵");
    println!("{} 토스트로 주세요", meal.toast);

    // private field 직접 법근하면 에러가 발생한다.
    // meal.seasonal_fruit = String::from("블루베리");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
    // use를 사용하여 함수까지 경로를 가져올 수 있으나 이러면 해당 함수가 어디에 정의되어 있는지 찾기가 어려워진다.
    // 위 방식처럼 hosting:: 정도의 단일 경로 정도를 남겨주는 것이 바람직하다. 
    add_to_waitlist();
}