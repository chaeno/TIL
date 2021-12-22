class Person (var name:String, val birthYear:Int) {
    // 기본 생성자
    init {
        println("${this.birthYear}년생 ${this.name}님이 생성되었습니다.")
    }
    // 보조 생성자
    constructor(name:String) : this(name, 1997) {
        println("보조 생성자가 사용되었습니다.")
    }

    fun introduce() {
        println("안녕하세요, ${birthYear}년생 ${name}입니다.")
    }
}

// 상속의 2가지 규칙
// 1. 서브 클래스는 슈퍼 클래스에 존재하는 속성과 '같은 이름'의 속성을 가질 수 없음
// 2. 서브 클래스가 생성될 때는 반드시 슈퍼 클래스의 생성자까지 호출되어야 함
open class Animal (var name:String, var age:Int, var type:String) {
    fun introduce() {
        println("저는 ${type} ${name}이고, ${age}살 입니다.")
    }
    open fun eat() {
        println("음식을 먹습니다.")
    }
}

class Dog (name:String, age:Int) : Animal (name, age, "개") {
    fun bark() {
        println("멍멍")
    }
}

class Cat (name:String, age:Int) : Animal (name, age, "고양이") {
    fun meow() {
        println("야옹야옹")
    }
}

class Tiger() : Animal("호돌이", 10, "호랑이") {
    override fun eat() {
        println("고기를 먹습니다.")
    }
}

// 추상 클래스는 미완성 함수를 포함하고 단독으로 인스턴스를 생성할 수 없다.
abstract class Animal1 {
    abstract fun eat()
    fun sniff() {
        println("킁킁")
    }
}

class Rabbit : Animal1() {
    override fun eat() {
        println("당근을 먹습니다.")
    }
}

// 인터페이스에서
// 구현부가 있는 함수 -> open 함수로 간주
// 구현부가 없는 함수 -> abstract 함수로 간주
// 별도 키워드 없이 포함된 모든 함수를 서브 클래스에서 구현 및 재정의가 가능
// 한 번에 여러 인터페이스를 상속 받을 수 있음
interface Runner {
    fun run()
}

interface Eater {
    fun eat() {
        println("음식을 먹습니다.")
    }
}

// 여러 인터페이스 상속 시 같은 이름과 형태를 가진 함수가 있을 경우
// 서브 클래스에서는 혼선을 막기 위해 함수를 반드시 오버라이딩하여 재구현 필
class Dog1 : Runner, Eater {
    override fun run() {
        println("우다다다 뜁니다.")
    }
    override fun eat() {
        println("허겁지겁 먹습니다.")
    }
}

fun main(args: Array<String>) {
    var dog = Dog1()
    dog.run()
    dog.eat()

    var r = Rabbit()
    r.eat()
    r.sniff()

    var t = Tiger()
    t.eat()

    var aa = Animal("별이", 5, "개")
    var bb = Dog("별이", 5)

    aa.introduce()
    bb.introduce()
    bb.bark()

    var cc = Cat("루이", 1)
    cc.meow()

    var a = Person("박보영", 1990)
    var b = Person("전정국", 1997)
    var c = Person("장원영", 2004)

    var d = Person("이루다")
    var e = Person("차은우")
    var f = Person("류수정")

//    println("안녕하세요, ${a.birthYear}년생 ${a.name}입니다.")
    a.introduce()
    b.introduce()
    c.introduce()
}
