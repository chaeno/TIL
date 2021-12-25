import java.util.*

fun main() {
    var a = Drink()
    a.drink()

    var b: Drink = Cola()
    b.drink()
    // Drink로 선언했으므로 washDishes를 호출할 수 없다.
    // b.washDishes()

    // b는 조건문 안에서만 잠시 down-casting 된다.
    if(b is Cola) {
        b.washDishes()
    }

    var c = b as Cola
    c.washDishes()
    // 반환값 뿐만 아니라 변수 자체도 down-casting 된다.
    b.washDishes()

    UsingGeneric(A()).doShouting()
    UsingGeneric(B()).doShouting()
    UsingGeneric(C()).doShouting()
    NotUsingGeneric(A()).doShouting()
    NotUsingGeneric(B()).doShouting()
    NotUsingGeneric(C()).doShouting()
    doShouting(A())
    doShouting(B())
    doShouting(C())

    val aa = listOf("사과", "딸기", "배")
    println(aa[1])

    for(fruit in aa) {
        print("${fruit}:")
    }
    println()

    val bb = mutableListOf(6, 3, 1)
    println(bb)
    bb.add(4)
    println(bb)
    bb.add(2, 8)
    println(bb)
    bb.removeAt(1)
    println(bb)
    bb.shuffle()
    println(bb)
    bb.sort()
    println(bb)

    val test1 = "Test.Kotlin.String"
    println(test1.length)
    println(test1.lowercase())
    println(test1.uppercase())
    val test2 = test1.split(".")
    println(test2)
    println(test2.joinToString())
    println(test2.joinToString("-"))
    println(test1.substring(5..10))

    val nullString: String? = null
    val emptyString = ""
    val blankString = " "
    val normalString = "A"
    println(nullString.isNullOrEmpty())
    println(emptyString.isNullOrEmpty())
    println(blankString.isNullOrEmpty())
    println(normalString.isNullOrEmpty())
    println()
    println(nullString.isNullOrBlank())
    println(emptyString.isNullOrBlank())
    println(blankString.isNullOrBlank())
    println(normalString.isNullOrBlank())
    println()

    var test3 = "kotlin.kt"
    var test4 = "java.java"
    println(test3.startsWith("java"))
    println(test4.startsWith("java"))
    println(test3.endsWith(".kt"))
    println(test4.endsWith(".kt"))
    println(test3.contains("lin"))
    println(test4.contains("lin"))
    println()

//    var aaa: String? = null
    var aaa: String? = "Kotlin Exam"
    println(aaa?.uppercase())
    println(aaa?:"default".uppercase())
//    println(aaa!!.uppercase())
    aaa?.run {
        println(uppercase())
        println(lowercase())
    }
    println()

    val aaaa = Product("콜라", 1000)
    val bbbb = Product("콜라", 1000)
    val cccc = aaaa
    val dddd = Product("사이다", 1000)

    println(aaaa == bbbb)
    println(aaaa === bbbb)
    println(aaaa == cccc)
    println(aaaa === cccc)
    println(aaaa == dddd)
    println(aaaa === dddd)
    println()

    read(7)
    read("감사합니다")
    println()

    deliveryItem("짬뽕")
    deliveryItem("책", 1)
    deliveryItem("노트북", 30, "학교")
    deliveryItem("선물", destination = "친구집")
    println()

    sum(1, 2, 3, 4)
    println()

    println(6 multiply 4)
    println(6.multiply(4))
    println()
}

infix fun Int.multiply(x: Int): Int = this * x

// vararg가 다른 파라미터와 쓰일 경우는 항상 맨 마지막에 나와야한다.
fun sum(vararg numbers: Int) {
    var sum = 0
    for(n in numbers) {
        sum += n
    }
    println(sum)
}

fun deliveryItem(name: String, count: Int = 1, destination: String = "집") {
    println("${name}, ${count}개를 ${destination}에 배달하였습니다.")
}

fun read(x: Int) {
    println("숫자 $x 입니다.")
}

fun read(x: String) {
    println(x)
}

class Product(val name: String, val price: Int) {
    override fun equals(other: Any?): Boolean {
        return if(other is Product) {
            other.name == name && other.price == price
        } else {
            false
        }
    }
}

open class Drink {
    var name = "음료"

    open fun drink() {
        println("${name}를 마십니다")
    }
}

class Cola: Drink() {
   var type = "콜라"

    override fun drink() {
        println("${name}중에 ${type}를 마십니다")
    }
    fun washDishes() {
        println("${type}로 설거지를 합니다")
    }
}

fun <T: A> doShouting(t: T) {
    t.shout()
}

open class A {
   open fun shout() {
       println("A가 소리칩니다")
   }
}

class B : A() {
    override fun shout() {
        println("B가 소리칩니다")
    }
}

class C : A() {
    override fun shout() {
        println("C가 소리칩니다")
    }
}

class UsingGeneric<T: A> (val t: T) {
    fun doShouting() {
        t.shout()
    }
}

class NotUsingGeneric (val t: A) {
    fun doShouting() {
        t.shout()
    }
}

