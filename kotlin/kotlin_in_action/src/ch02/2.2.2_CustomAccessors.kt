package ch02

class Rectangle(private val height: Int, private val width: Int) {
    var isSquare: Boolean = false
        get() = height == width
        set(value) {
            println("set value")
            field = value
        }
}

fun main(args: Array<String>) {
    val rectangle = Rectangle(41, 41)
    println(rectangle.isSquare)
    rectangle.isSquare = false
    println(rectangle.isSquare)
}