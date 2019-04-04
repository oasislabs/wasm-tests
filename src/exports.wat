(module
    (import "env" "fetch_input" (func $fetch_input (param i32)))
    (import "env" "input_length" (func $input_length (result i32)))
    (import "env" "ret" (func $ret (param i32) (param i32)))
    (import "env" "memory" (memory 1 65535))

    (func $call
        ;; Assert that input_length is exactly 1 byte long.
        (if
            (i32.ne
                (call $input_length)
                (i32.const 1)
            )
            (unreachable)
        )

        ;; Load the input data at address 0
        (call $fetch_input
            (i32.const 0)
        )

        ;; Set the return to be the input
        (call $ret
            (i32.const 0)
            (i32.const 1)
        )
    )

    (func $func
        ;; Assert that input_length is exactly 2 bytes long.
        (if
            (i32.ne
                (call $input_length)
                (i32.const 2)
            )
            (unreachable)
        )

        ;; Store 2 at address 0
        (i32.store
            (i32.const 0)
            (i32.const 2)
        )

        ;; Set the return to be 2
        (call $ret
            (i32.const 0)
            (i32.const 1)
        )
    )

    (table 0 funcref)
    (export "call" (func $call))
    (export "func" (func $func))
)
