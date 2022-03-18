@file:JvmName("ProcessTheAnswer")
package ch08.ProcessTheAnswer

fun processTheAnswer(f: (Int) -> Int) {
    println(f(42))
}