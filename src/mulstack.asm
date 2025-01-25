; dougal assembly only stack no registers

push 5
push 10
call mul
halt

; multiply subroutine
mul:
push 0
push 0
    mloop:
        pop
        pick 1
        call add
        pick 2
        dec
        pick 0
        poke 3
        jnz mloop
    pop
    poke 1
    pop
    ret

; addition subroutine
add:
    aloop:
        dec
        swap
        inc
        swap
        jnz aloop
    pop
    ret
