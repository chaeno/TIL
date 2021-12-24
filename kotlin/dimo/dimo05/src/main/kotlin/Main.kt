fun main() {
    // 이렇게 중복된 이름의 변수가 있으면 이 변수가 출력됨
    // run 함수가 인스턴스 내의 속성보다 스코프의 변수를 더 우선한다.
    var price = 5000

    var a = Book("디모의 코틀린", 10000).apply {
        name = "[초특가]" + name
        discount()
    }

    a.run {
        println("a run  상품명: ${name}, 가격: ${price}원")
    }

    a.let {
        println("a let  삼풍명: ${it.name}, 가격: ${it.price}원")
    }

    // with은 인스턴스를 parameter로 받는다 차이 밖에 없음
    with(a) {
        println("a with 상품명: ${name}, 가격: ${price}원")
    }

    var b = Book("디모의 코틀린", 10000).also {
        it.name = "[초초특가]" + it.name
        it.discount()
        it.discount()
    }

    b.let {
        println("b run  상품명: ${it.name}, 가격: ${it.price}원")
    }

    // object 는 최초 사용 시 초기화 이후 재사용 (singleton pattern)
    println(CounterFood.count)  // print 0
    CounterFood.countUp()       // 1
    CounterFood.countUp()       // 2
    println(CounterFood.count)  // print 2
    CounterFood.clear()         // 0
    println(CounterFood.count)  // print 0

    var c = FoodPoll("짜장")
    var d = FoodPoll("짬뽕")
    c.vote()
    c.vote()
    d.vote()
    d.vote()
    d.vote()
    println("${c.name} : ${c.count}")
    println("${d.name} : ${d.count}")
    println("총계 : ${FoodPoll.total}")

    EventPrinter().start()
    println("")
    EventPrinter2().start()
}

// 처리가 끝나면 인스턴스를 반환
// apply / also

// 처리가 끝나면 최종값을 반환
// run / let

class Book(var name: String, var price: Int) {
    fun discount() {
        price -= 2000
    }
}

object CounterFood {
    var count = 0

    fun countUp() {
        count++
    }

    fun clear() {
        count = 0
    }
}

class FoodPoll (val name: String) {
    // 인스턴스가 모두 공유하는 값
    companion object {
        var total = 0
    }

    var count = 0

    fun vote() {
        total++
        count++
    }
}

interface EventListener {
    fun onEvent(count: Int)
}

class Counter(val listener: EventListener) {
    fun count() {
        for(i in 1..100) {
            if(i%5==0) listener.onEvent(i)
        }
    }
}

class EventPrinter: EventListener {
    override fun onEvent(count: Int) {
        print("${count}-")
    }
    fun start() {
        val counter = Counter(this)
        counter.count()
    }
}

class EventPrinter2 {
    fun start() {
        val counter = Counter(object: EventListener{
            override fun onEvent(count: Int) {
                print("${count}-")
            }
        })
        counter.count()
    }
}