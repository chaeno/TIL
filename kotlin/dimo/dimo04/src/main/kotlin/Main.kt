val a = "패키지 스코프"

class B {
    fun print() {
        println(a)
    }
}

// 패키지 스코프
// public (기본값): 어떤 패키지에서도 접근 가능
// internal: 같은 모듈 내에서만 접근 가능
// private: 같은 파일 내에서만 접근 가능

// 클래스 스코프
// puglic (기본값): 클래스 외부에서 늘 접근 가능
// private: 클래스 내부에서만 접근 가능
// protected: 클래스 자신과 상속받은 클래스에서 접근 가능


fun main() {
    val a = "함수 스코프"
    println(a)
    B().print()

    b(::a)

    val c:(String)->Unit = { str -> println("$str 람다함수") }
    b(c)

    val d = { str:String -> println("$str 람다함수") }
    b(d)
}

fun a (str: String) {
    println("$str 함수 a")
}

fun b (function: (String)->Unit) {
    function("b가 호출한")
}