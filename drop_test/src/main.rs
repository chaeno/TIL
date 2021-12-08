struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("CustomSmartPointer의 데이터 '{}'를 해제합니다!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("내 데이터"),
    };
    let d = CustomSmartPointer {
        data: String::from("다른 데이터"),
    };
    println!("CustomSmartPointer를 생성했습니다.");

    // c.drop();
    // explicit destructor calls not allowed
    // println!("CustomSmartPointer를 main 함수 끝에 도달하기 전에 해제합니다.")

    drop(c);
    // std::mem::drop
    // 프렐류드에 포함되어 있으므로 바로 호출 가능
    // println!("CustomSmartPointer를 main 함수 끝에 도달하기 전에 해제합니다.")
}
