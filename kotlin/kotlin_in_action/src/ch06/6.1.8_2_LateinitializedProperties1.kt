package ch06.ex1_8_2_LateinitializedProperties1

import org.junit.Assert
import org.junit.Before
import org.junit.Test

class MyService {
    fun performAction(): String = "foo"
}

class MyTest {
    private lateinit var myService: MyService

    @Before
    fun setUp() {
        myService = MyService()
    }

    @Test
    fun testAction() {
        Assert.assertEquals("foo",
            myService.performAction())
    }
}