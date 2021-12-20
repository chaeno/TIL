fun main(args: Array<String>) {
    println("Hello World!")

    // Try adding program arguments via Run/Debug configuration.
    // Learn more about running applications: https://www.jetbrains.com/help/idea/running-applications.html.
    println("Program arguments: ${args.joinToString()}")

    // l2. 변수와 자료형

    var a: Int
    // 반드시 초기화 한 이후에 사용 가능
    a = 123
    println(a)

    // nullable 자료형
    var b: Int? = null
    println(b)

    // 정수형 자료형
    var intValue:Int = 1234
    var longValue:Long = 1234L
    var intValueByHex = 0x1af
    var intValueByBin:Int = 0b1010101
    // 8진수 미지원
    println("$intValue, $longValue, $intValueByHex, $intValueByBin")

    // 실수형 자료형
    var doubleValue:Double = 123.5
    var doubleValueWithExp:Double = 123.5e10
    var floatValue:Float = 123.5f
    println("$doubleValue, $doubleValueWithExp, $floatValue")

    // Char
    // 코틀린은 UTF-16 BE 인코딩 사용 1char = 2byte
    var charValue:Char = 'a'
    var koreanCharValue:Char = '가'
    println("$charValue, $koreanCharValue")

    // Boolean
    var booleanBalue:Boolean = true
    println(booleanBalue)

    // 문자열
    val stringValue = "one line string test"
    val multiLineStringValue = """multiline
string
test"""
    println(stringValue)
    println(multiLineStringValue)
}