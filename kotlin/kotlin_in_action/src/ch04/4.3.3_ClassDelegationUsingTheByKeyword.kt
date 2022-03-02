package ch04.ex3_3_ClassDelegationUsingTheByKeyword

class CountingSet<T>(
    val innerSet: MutableCollection<T> = HashSet<T>()
) : MutableCollection<T> by innerSet {
    var objectsAdded = 0

    override fun add(element: T): Boolean {
        objectsAdded++
        return innerSet.add(element)
    }

    override fun addAll(elements: Collection<T>): Boolean {
        objectsAdded += elements.size
        return innerSet.addAll(elements)
    }
}

fun main(args: Array<String>) {
    val cset = CountingSet<Int>(innerSet = mutableListOf())
    cset.addAll(listOf(1, 1, 2))
    println("${cset.objectsAdded} objects were added, ${cset.size} remain")
    cset.addAll(listOf(1, 1, 2, 3))
    println("${cset.objectsAdded} objects were added, ${cset.size} remain")
    println(cset.innerSet)
}