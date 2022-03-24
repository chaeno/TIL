package ch09.CopyDataIn

fun <T> copyData(source: MutableList<T>,
                 destination: MutableList<in T>) {
    for (item in source) {
        destination.add(item)
    }
}

fun main(args: Array<String>) {
    val ints = mutableListOf(1, 2, 3)
    val anyTimes = mutableListOf<Any>()
    copyData(ints, anyTimes)
    println(anyTimes)
}