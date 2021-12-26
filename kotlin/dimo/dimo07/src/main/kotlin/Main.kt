fun main() {
    Outer.Nested().introduce()

    val outer = Outer()
    val inner = outer.Inner()

    inner.introduceInner()
    inner.introduceOuter()

    outer.text = "Change Outer Class"
    inner.introduceOuter()
    println()

    val a = General("보영", 212)
    println(a == General("보영", 212))
    println(a.hashCode())
    println(a)
    println()

    val b = Data("루다", 306)
    println(b == Data("루다", 306))
    println(b.hashCode())
    println(b)
    println(b.copy())
    println(b.copy(name="아린"))
    println(b.copy(id=618))
    println()

    val list = listOf(Data("보영", 212),
                        Data("루다", 306),
                        Data("아린",618))
    for((c1, c2) in list) {
        println("$c1, $c2")
    }
    println()

    var state = State.SING
    println(state)
    state = State.SLEEP
    println(state.isSleeping())
    state = State.EAT
    println(state.message)
    println()

    example01()
    example02()
    example03()
    example04()
    example05()
    example06()
    example07()
    example08()
    example09()
}

fun example01() {
    var a = mutableSetOf("귤", "바나나", "키위")
    for(item in a) {
        println(item)
    }
    a.add("자몽")
    println(a)
    a.remove("바나나")
    println(a)
    println(a.contains("귤"))
    println()
}

fun example02() {
    val a = mutableMapOf("레드벨벳" to "음파음파",
                        "트와이스" to "FANCY",
                        "ITZY" to "ICY")

    for(entry in a) {
        println("${entry.key} : ${entry.value}")
    }
    a["오마이걸"] = "반지"
    println(a)
    a.remove("ITZY")
    println(a)
    println(a["레드벨벳"])
    println()
}

fun example03() {
    val nameList = listOf("박수영", "김지수", "김다현", "신유나", "김지우")
    nameList.forEach{print("$it ")}
    println()
    println(nameList.filter{it.startsWith("김")})
    println(nameList.map{"이름: $it"})
    println(nameList.any{it == "김지연"})
    println(nameList.all{it.length == 3})
    println(nameList.none{it.startsWith("이")})
    println(nameList.first{it.startsWith("김")})
    println(nameList.last{it.startsWith("김")})
    println(nameList.count{it.contains("지")})
    println()
}

fun example04() {
    data class Person(val name: String, val birthYear: Int)
    var personList = listOf(Person("유나", 1992),
                            Person("조이", 1996),
                            Person("츄", 1999),
                            Person("유나", 2003))
    println(personList.associateBy { it.birthYear })
    println(personList.groupBy { it.name })
    var (over98, under98) = personList.partition { it.birthYear > 1998 }
    println(over98)
    println(under98)
    println()
}

fun example05() {
    val numbers = listOf(-3, 7, 2, -10, 1)
    println(numbers.flatMap { listOf(it * 10, it + 10) })
    println(numbers.getOrElse(1) { 50 })
    println(numbers.getOrElse(10) { 50 })
    val names = listOf("A", "B", "C", "D")
    println(names zip numbers)
    println()
}

fun example06() {
    val foodCourt = FoodCourt()
    foodCourt.searchPrice(FoodCourt.FOOD_CREAM_PASTA)
    foodCourt.searchPrice(FoodCourt.FOOD_STEAK)
    foodCourt.searchPrice(FoodCourt.FOOD_PIZZA)
    println()
}

class FoodCourt {
    fun searchPrice(foodName: String) {
        val price = when(foodName) {
            FOOD_CREAM_PASTA -> 13_000
            FOOD_STEAK -> 25_000
            FOOD_PIZZA -> 15_000
            else -> 0
        }

        println("${foodName}의 가격은 ${price}원 입니다")
    }

    companion object {
        const val FOOD_CREAM_PASTA = "크림파스타"
        const val FOOD_STEAK = "스테이크"
        const val FOOD_PIZZA = "피자"
    }
}

fun example07() {
    val a = LateInitSample()
    println(a.getLateInitText())
    a.text = "새로 할당한 값"
    println(a.getLateInitText())
    println()
}

class LateInitSample() {
    lateinit var text: String

    fun getLateInitText(): String {
        return if(::text.isInitialized) {
            text
        } else {
            "기본값"
        }
    }
}

fun example08() {
    val number: Int by lazy {
        println("초기화를 합니다")
        7
    }
    println("코드를 시작합니다.")
    println(number)
    println(number)
    println()
}

fun example09() {
    var bitData: Int = 0b10000
    bitData = bitData or (1 shl 2)
    println(bitData.toString(2))

    var result = bitData and (1 shl 4)
    println(result.toString(2))

    println(result shr 4)

    bitData = bitData and((1 shl 4).inv())
    println(bitData.toString(2))

    println((bitData xor(0b10100)).toString(2))
    println()
}

enum class State(val message: String) {
    SING("노래를 부릅니다."),
    EAT("밥을 먹습니다."),
    SLEEP("잠을 잡니다.");

    fun isSleeping() = this == State.SLEEP
}

class General(val name: String, val id: Int)

data class Data(val name: String, val id: Int)

class Outer {
    var text = "Outer Class"

    class Nested {
        fun introduce() {
            println("Nested Class")
        }
    }

    inner class Inner {
        var text = "Inner Class"

        fun introduceInner() {
            println(text)
        }

        fun introduceOuter() {
            println(this@Outer.text)
        }
    }
}