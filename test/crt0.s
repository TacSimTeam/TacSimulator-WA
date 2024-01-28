start   ld  sp,#0xe000
  call    _main
  halt

_in
    ld  g1,2,sp
    in  g0,g1
    ret


_out
    ld  g0,2,sp
    ld  g1,4,sp
    out g1,g0
    ret
