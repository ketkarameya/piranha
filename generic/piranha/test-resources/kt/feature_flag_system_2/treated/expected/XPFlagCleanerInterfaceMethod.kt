/**
 * Copyright (c) 2022 Uber Technologies, Inc.
 *
 *
 * Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file
 * except in compliance with the License. You may obtain a copy of the License at
 *
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 *
 * Unless required by applicable law or agreed to in writing, software distributed under the
 * License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either
 * express or implied. See the License for the specific language governing permissions and
 * limitations under the License.
 */
package com.uber.input

internal class XPFlagCleanerPositiveCases {
    private val experimentation: SomeOtherParameter = SomeOtherParameter.create()
    private var ftBool1 = true
    private var ftBool2 = true
    fun conditional_contains_stale_flag() {
        println("Hello World")
    }

    fun conditional_with_else_contains_stale_flag() {
        println("Hello World")
    }

    fun conditional_with_else_contains_stale_flag_tbool() {
        println("Hello World")
    }

    fun conditional_with_else_contains_stale_flag_tbool(a: Int) {
        println("Hello World")
    }

    fun conditional_with_else_contains_stale_flag_tbool(a: Int, abc: Boolean) {
        println("Hi world")
    }

    fun conditional_with_else_contains_stale_flag_tbool_reassigned(a: Int, z: Int) {
        // Currently if there is another assignment, variable will not be inlined.
        var tBool = true
        tBool = abc() && tBool
        if (!tBool && true) {
            println("Hello World")
        } else {
            println("Hi world")
        }
    }

    fun abc(): Boolean {
        return false
    }

    fun conditional_with_else_contains_stale_flag_tbool_reassigned_to_same_val(a: Int, z: Int) {
        println("Hi world")
    }

    fun conditional_with_else_contains_stale_flag_ftbool(a: Int) {
        println("Hello World")
    }

    fun conditional_with_else_contains_stale_flag_tbool_reassigned_ftbool(a: Int, z: Int) {
        // Currently if there is another assignment, variable will not be inlined.
        println("Hi world")
    }

    fun conditional_with_else_contains_stale_flag_tbool_reassigned_ftbool_1(a: Int, z: Int) {
        // Currently if there is another assignment, variable will not be inlined.
        println("Hi world")
    }

    fun conditional_with_else_contains_stale_flag_tbool_reassigned_ftbool_2(a: Int, z: Int) {
        // Currently if there is another assignment, variable will not be inlined.
        ftBool1 = false
        if (!ftBool1 && true) {
            println("Hello World")
        } else {
            println("Hi world")
        }
    }

    fun conditional_with_else_contains_stale_flag_tbool_reassigned_ftbool_3(a: Int, z: Int) {
        // Currently if there is another assignment, variable will not be inlined.
        ftBool2 = true
        ftBool2 = false
        if (!ftBool2 && true) {
            println("Hello World")
        } else {
            println("Hi world")
        }
    }
}