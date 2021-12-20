
fun main() {
    // 명시적 형변환 (explicit type casting)
    // 코틀린은 암시적 형변환은 지원하지 않는다.
    var a: Int = 54321
    var b: Long = a.toLong()
    println("$a, $b")

    var intArr = arrayOf(1, 2, 3, 4, 5)
    var nullArr = arrayOfNulls<Int>(5)
    intArr[2] = 8
    println(intArr[4])

    // int
    var c = 1234
    // long
    var d = 1234L
    // double
    var e = 12.45
    // float
    var f = 12.45f
    // int
    var g = 0xABCD
    var h = 0b0101010
    // boolean
    var i = true
    // char
    var j = 'c'

    println(add(5, 6, 7))
    println(add_oneline(5, 6, 7))

    mainIf()
    mainWhen()
    mainLoop()
    mainFlowControl()
}

// 코틀린에서 함수는 자료형이 결정된 변수라는 개념으로 접근하는 것이 좋음
// 그러면, 함수형 언어 특징을 이해하기 편함
fun add(a:Int, b:Int, c:Int): Int {
    return a + b + c
}

// 단일 표현식 함수 (반환형 타입 추론 가능하므로 생략 가능)
fun add_oneline(a:Int, b:Int, c:Int) = a + b + c

fun mainIf() {
    var a = 7

    if (a > 10) {
        println("a는 10보다 크다")
    } else {
        println("a는 10보다 작거나 같다")
    }
}

fun mainWhen() {
    doWhen(123f)
}

fun doWhen(a: Any) {
    // 등호나 부등호의 사용은 불가
    // 여러 조건이 부합할 경우 먼저 부합하는 조건이 실행
    when(a) {
        1 -> println("정수 1입니다.")
        "Dimo" -> println("디모의 코틀린 강좌입니다")
        is Long -> println("Long 타입 입니다")
        !is String -> println("String 타입이 아닙니다")
        else -> println("어떤 조건도 만족하지 않습니다")
    }

    val result = when(a) {
        1 -> "정수 1입니다."
        "Dimo" -> "디모의 코틀린 강좌입니다"
        is Long -> "Long 타입 입니다"
        !is String -> "String 타입이 아닙니다"
        else -> "어떤 조건도 만족하지 않습니다"
    }
    println(result)

}

fun mainLoop() {
    var a = 0
    while(a < 5) {
        println(++a)
    }

    // do while은 1번 수행 보장
    var b = 0
    do {
        println(b++)
    } while (b < 5)

    for (i in 0..9 step 3) {
        println(i)
    }

    for (i in 9 downTo 0 step 3)
        println(i)

    for (i in 'a'..'e' step 2) {
        println(i)
    }
}

fun mainFlowControl() {
    for (i in 1..10) {
//        if (i == 3) break
        if (i == 3) continue
        println(i)
    }

    for (i in 1..10) {
        for (j in 1..10) {
            if(i==1 && j==2) break
            println("$i $j")
        }
        if (i==1) break
    }

    loop@for (i in 1..10) {
        for (j in 1..10) {
            if(i==1 && j==2) break@loop
            println("$i $j")
        }
    }

    println(true && false)
    println(true || false)
    println(!true)
    println(!false)

    var a = 6
    var b = 4
    println(a > 5 && b > 5)
}