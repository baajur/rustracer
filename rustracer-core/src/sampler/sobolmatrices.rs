pub const NUM_SOBOL_DIMENSIONS: usize = 1024;
pub const SOBOL_MATRIX_SIZE: usize = 52;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const VdCSobolMatrices: [&[u64]; SOBOL_MATRIX_SIZE] = [ 
    &[ // m = 1
     0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64,
     0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64,
     0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64,
     0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64,
     0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64,
     0x1u64, 0x1u64, 0x1u64, 0x1u64, 0x1u64 ],
    &[ // m = 2
     0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64,
     0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64,
     0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64,
     0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64,
     0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64, 0x3u64, 0x2u64,
     0x3u64, 0x2u64, 0x3u64 ],
    &[ // m = 3
     0x5u64, 0x7u64, 0x4u64, 0x6u64, 0x5u64, 0x7u64, 0x4u64, 0x6u64, 0x5u64,
     0x7u64, 0x4u64, 0x6u64, 0x5u64, 0x7u64, 0x4u64, 0x6u64, 0x5u64, 0x7u64,
     0x4u64, 0x6u64, 0x5u64, 0x7u64, 0x4u64, 0x6u64, 0x5u64, 0x7u64, 0x4u64,
     0x6u64, 0x5u64, 0x7u64, 0x4u64, 0x6u64, 0x5u64, 0x7u64, 0x4u64, 0x6u64,
     0x5u64, 0x7u64, 0x4u64, 0x6u64, 0x5u64, 0x7u64, 0x4u64, 0x6u64, 0x5u64,
     0x7u64 ],
    &[ // m = 4
     0x8u64, 0xcu64, 0xau64, 0xfu64, 0x8u64, 0xcu64, 0xau64, 0xfu64, 0x8u64,
     0xcu64, 0xau64, 0xfu64, 0x8u64, 0xcu64, 0xau64, 0xfu64, 0x8u64, 0xcu64,
     0xau64, 0xfu64, 0x8u64, 0xcu64, 0xau64, 0xfu64, 0x8u64, 0xcu64, 0xau64,
     0xfu64, 0x8u64, 0xcu64, 0xau64, 0xfu64, 0x8u64, 0xcu64, 0xau64, 0xfu64,
     0x8u64, 0xcu64, 0xau64, 0xfu64, 0x8u64, 0xcu64, 0xau64, 0xfu64 ],
    &[ // m = 5
     0x14u64, 0x1eu64, 0x11u64, 0x19u64, 0x15u64, 0x1fu64, 0x10u64, 0x18u64,
     0x14u64, 0x1eu64, 0x11u64, 0x19u64, 0x15u64, 0x1fu64, 0x10u64, 0x18u64,
     0x14u64, 0x1eu64, 0x11u64, 0x19u64, 0x15u64, 0x1fu64, 0x10u64, 0x18u64,
     0x14u64, 0x1eu64, 0x11u64, 0x19u64, 0x15u64, 0x1fu64, 0x10u64, 0x18u64,
     0x14u64, 0x1eu64, 0x11u64, 0x19u64, 0x15u64, 0x1fu64, 0x10u64, 0x18u64,
     0x14u64, 0x1eu64 ],
    &[ // m = 6
     0x22u64, 0x33u64, 0x2au64, 0x3fu64, 0x20u64, 0x30u64, 0x28u64, 0x3cu64,
     0x22u64, 0x33u64, 0x2au64, 0x3fu64, 0x20u64, 0x30u64, 0x28u64, 0x3cu64,
     0x22u64, 0x33u64, 0x2au64, 0x3fu64, 0x20u64, 0x30u64, 0x28u64, 0x3cu64,
     0x22u64, 0x33u64, 0x2au64, 0x3fu64, 0x20u64, 0x30u64, 0x28u64, 0x3cu64,
     0x22u64, 0x33u64, 0x2au64, 0x3fu64, 0x20u64, 0x30u64, 0x28u64, 0x3cu64 ],
    &[ // m = 7
     0x55u64, 0x7fu64, 0x40u64, 0x60u64, 0x50u64, 0x78u64, 0x44u64, 0x66u64,
     0x55u64, 0x7fu64, 0x40u64, 0x60u64, 0x50u64, 0x78u64, 0x44u64, 0x66u64,
     0x55u64, 0x7fu64, 0x40u64, 0x60u64, 0x50u64, 0x78u64, 0x44u64, 0x66u64,
     0x55u64, 0x7fu64, 0x40u64, 0x60u64, 0x50u64, 0x78u64, 0x44u64, 0x66u64,
     0x55u64, 0x7fu64, 0x40u64, 0x60u64, 0x50u64, 0x78u64 ],
    &[ // m = 8
     0x80u64, 0xc0u64, 0xa0u64, 0xf0u64, 0x88u64, 0xccu64, 0xaau64, 0xffu64,
     0x80u64, 0xc0u64, 0xa0u64, 0xf0u64, 0x88u64, 0xccu64, 0xaau64, 0xffu64,
     0x80u64, 0xc0u64, 0xa0u64, 0xf0u64, 0x88u64, 0xccu64, 0xaau64, 0xffu64,
     0x80u64, 0xc0u64, 0xa0u64, 0xf0u64, 0x88u64, 0xccu64, 0xaau64, 0xffu64,
     0x80u64, 0xc0u64, 0xa0u64, 0xf0u64 ],
    &[ // m = 9
     0x140u64, 0x1e0u64, 0x110u64, 0x198u64, 0x154u64, 0x1feu64, 0x101u64,
     0x181u64, 0x141u64, 0x1e1u64, 0x111u64, 0x199u64, 0x155u64, 0x1ffu64,
     0x100u64, 0x180u64, 0x140u64, 0x1e0u64, 0x110u64, 0x198u64, 0x154u64,
     0x1feu64, 0x101u64, 0x181u64, 0x141u64, 0x1e1u64, 0x111u64, 0x199u64,
     0x155u64, 0x1ffu64, 0x100u64, 0x180u64, 0x140u64, 0x1e0u64 ],
    &[ // m = 10
     0x220u64, 0x330u64, 0x2a8u64, 0x3fcu64, 0x202u64, 0x303u64, 0x282u64,
     0x3c3u64, 0x222u64, 0x333u64, 0x2aau64, 0x3ffu64, 0x200u64, 0x300u64,
     0x280u64, 0x3c0u64, 0x220u64, 0x330u64, 0x2a8u64, 0x3fcu64, 0x202u64,
     0x303u64, 0x282u64, 0x3c3u64, 0x222u64, 0x333u64, 0x2aau64, 0x3ffu64,
     0x200u64, 0x300u64, 0x280u64, 0x3c0u64 ],
    &[ // m = 11
     0x550u64, 0x7f8u64, 0x404u64, 0x606u64, 0x505u64, 0x787u64, 0x444u64,
     0x666u64, 0x555u64, 0x7ffu64, 0x400u64, 0x600u64, 0x500u64, 0x780u64,
     0x440u64, 0x660u64, 0x550u64, 0x7f8u64, 0x404u64, 0x606u64, 0x505u64,
     0x787u64, 0x444u64, 0x666u64, 0x555u64, 0x7ffu64, 0x400u64, 0x600u64,
     0x500u64, 0x780u64 ],
    &[ // m = 12
     0x808u64, 0xc0cu64, 0xa0au64, 0xf0fu64, 0x888u64, 0xcccu64, 0xaaau64,
     0xfffu64, 0x800u64, 0xc00u64, 0xa00u64, 0xf00u64, 0x880u64, 0xcc0u64,
     0xaa0u64, 0xff0u64, 0x808u64, 0xc0cu64, 0xa0au64, 0xf0fu64, 0x888u64,
     0xcccu64, 0xaaau64, 0xfffu64, 0x800u64, 0xc00u64, 0xa00u64, 0xf00u64 ],
    &[ // m = 13
     0x1414u64, 0x1e1eu64, 0x1111u64, 0x1999u64, 0x1555u64, 0x1fffu64,
     0x1000u64, 0x1800u64, 0x1400u64, 0x1e00u64, 0x1100u64, 0x1980u64,
     0x1540u64, 0x1fe0u64, 0x1010u64, 0x1818u64, 0x1414u64, 0x1e1eu64,
     0x1111u64, 0x1999u64, 0x1555u64, 0x1fffu64, 0x1000u64, 0x1800u64,
     0x1400u64, 0x1e00u64 ],
    &[ // m = 14
     0x2222u64, 0x3333u64, 0x2aaau64, 0x3fffu64, 0x2000u64, 0x3000u64,
     0x2800u64, 0x3c00u64, 0x2200u64, 0x3300u64, 0x2a80u64, 0x3fc0u64,
     0x2020u64, 0x3030u64, 0x2828u64, 0x3c3cu64, 0x2222u64, 0x3333u64,
     0x2aaau64, 0x3fffu64, 0x2000u64, 0x3000u64, 0x2800u64, 0x3c00u64 ],
    &[ // m = 15
     0x5555u64, 0x7fffu64, 0x4000u64, 0x6000u64, 0x5000u64, 0x7800u64,
     0x4400u64, 0x6600u64, 0x5500u64, 0x7f80u64, 0x4040u64, 0x6060u64,
     0x5050u64, 0x7878u64, 0x4444u64, 0x6666u64, 0x5555u64, 0x7fffu64,
     0x4000u64, 0x6000u64, 0x5000u64, 0x7800u64 ],
    &[ // m = 16
     0x8000u64, 0xc000u64, 0xa000u64, 0xf000u64, 0x8800u64, 0xcc00u64,
     0xaa00u64, 0xff00u64, 0x8080u64, 0xc0c0u64, 0xa0a0u64, 0xf0f0u64,
     0x8888u64, 0xccccu64, 0xaaaau64, 0xffffu64, 0x8000u64, 0xc000u64,
     0xa000u64, 0xf000u64 ],
    &[ // m = 17
     0x14000u64, 0x1e000u64, 0x11000u64, 0x19800u64, 0x15400u64, 0x1fe00u64,
     0x10100u64, 0x18180u64, 0x14140u64, 0x1e1e0u64, 0x11110u64, 0x19998u64,
     0x15554u64, 0x1fffeu64, 0x10001u64, 0x18001u64, 0x14001u64, 0x1e001u64 ],
    &[ // m = 18
     0x22000u64, 0x33000u64, 0x2a800u64, 0x3fc00u64, 0x20200u64, 0x30300u64,
     0x28280u64, 0x3c3c0u64, 0x22220u64, 0x33330u64, 0x2aaa8u64, 0x3fffcu64,
     0x20002u64, 0x30003u64, 0x28002u64, 0x3c003u64 ],
    &[ // m = 19
     0x55000u64, 0x7f800u64, 0x40400u64, 0x60600u64, 0x50500u64, 0x78780u64,
     0x44440u64, 0x66660u64, 0x55550u64, 0x7fff8u64, 0x40004u64, 0x60006u64,
     0x50005u64, 0x78007u64 ],
    &[ // m = 20
     0x80800u64, 0xc0c00u64, 0xa0a00u64, 0xf0f00u64, 0x88880u64, 0xcccc0u64,
     0xaaaa0u64, 0xffff0u64, 0x80008u64, 0xc000cu64, 0xa000au64, 0xf000fu64 ],
    &[ // m = 21
     0x141400u64, 0x1e1e00u64, 0x111100u64, 0x199980u64, 0x155540u64,
     0x1fffe0u64, 0x100010u64, 0x180018u64, 0x140014u64, 0x1e001eu64 ],
    &[ // m = 22
     0x222200u64, 0x333300u64, 0x2aaa80u64, 0x3fffc0u64, 0x200020u64,
     0x300030u64, 0x280028u64, 0x3c003cu64 ],
    &[ // m = 23
     0x555500u64, 0x7fff80u64, 0x400040u64, 0x600060u64, 0x500050u64,
     0x780078u64 ],
    &[ // m = 24
     0x800080u64, 0xc000c0u64, 0xa000a0u64, 0xf000f0u64 ],
    &[ // m = 25
     0x1400140u64, 0x1e001e0u64 ] ];

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const VdCSobolMatricesInv: [&[u64]; SOBOL_MATRIX_SIZE] = [ 
    &[ // m = 1
     0x2u64, 0x3u64 ],
    &[ // m = 2
     0xcu64, 0x4u64, 0xau64, 0x5u64 ],
    &[ // m = 3
     0x28u64, 0x30u64, 0x10u64, 0x3cu64, 0x22u64, 0x11u64 ],
    &[ // m = 4
     0xf0u64, 0x50u64, 0x30u64, 0x10u64, 0x88u64, 0x44u64, 0x22u64, 0x11u64 ],
    &[ // m = 5
     0x220u64, 0x3c0u64, 0x360u64, 0x300u64, 0x100u64, 0x330u64, 0x2a8u64,
     0x264u64, 0x202u64, 0x101u64 ],
    &[ // m = 6
     0xcc0u64, 0x440u64, 0xf00u64, 0x500u64, 0x300u64, 0x100u64, 0xaa0u64,
     0x550u64, 0x808u64, 0x404u64, 0x202u64, 0x101u64 ],
    &[ // m = 7
     0x2a80u64, 0x3300u64, 0x1100u64, 0xf00u64, 0x500u64, 0x300u64, 0x100u64,
     0x3fc0u64, 0x2020u64, 0x1010u64, 0x808u64, 0x404u64, 0x202u64, 0x101u64 ],
    &[ // m = 8
     0xff00u64, 0x5500u64, 0x3300u64, 0x1100u64, 0xf00u64, 0x500u64, 0x300u64,
     0x100u64, 0x8080u64, 0x4040u64, 0x2020u64, 0x1010u64, 0x808u64, 0x404u64,
     0x202u64, 0x101u64 ],
    &[ // m = 9
     0x20200u64, 0x3fc00u64, 0x35600u64, 0x33000u64, 0x31200u64, 0x30c00u64,
     0x30600u64, 0x30000u64, 0x10000u64, 0x30300u64, 0x28280u64, 0x24240u64,
     0x22220u64, 0x21210u64, 0x20a08u64, 0x20604u64, 0x20002u64, 0x10001u64 ],
    &[ // m = 10
     0xc0c00u64, 0x40400u64, 0xff000u64, 0x55000u64, 0xf3c00u64, 0x51400u64,
     0xf0000u64, 0x50000u64, 0x30000u64, 0x10000u64, 0xa0a00u64, 0x50500u64,
     0x88880u64, 0x44440u64, 0x82820u64, 0x41410u64, 0x80008u64, 0x40004u64,
     0x20002u64, 0x10001u64 ],
    &[ // m = 11
     0x282800u64, 0x303000u64, 0x101000u64, 0xff000u64, 0x2d7800u64,
     0x330000u64, 0x110000u64, 0xf0000u64, 0x50000u64, 0x30000u64, 0x10000u64,
     0x3c3c00u64, 0x222200u64, 0x111100u64, 0x88880u64, 0x387840u64,
     0x200020u64, 0x100010u64, 0x80008u64, 0x40004u64, 0x20002u64, 0x10001u64 ],
    &[ // m = 12
     0xf0f000u64, 0x505000u64, 0x303000u64, 0x101000u64, 0xff0000u64,
     0x550000u64, 0x330000u64, 0x110000u64, 0xf0000u64, 0x50000u64, 0x30000u64,
     0x10000u64, 0x888800u64, 0x444400u64, 0x222200u64, 0x111100u64,
     0x800080u64, 0x400040u64, 0x200020u64, 0x100010u64, 0x80008u64, 0x40004u64,
     0x20002u64, 0x10001u64 ],
    &[ // m = 13
     0x2222000u64, 0x3c3c000u64, 0x3636000u64, 0x3030000u64, 0x1010000u64,
     0xff0000u64, 0x550000u64, 0x330000u64, 0x110000u64, 0xf0000u64, 0x50000u64,
     0x30000u64, 0x10000u64, 0x3333000u64, 0x2aaa800u64, 0x2666400u64,
     0x2000200u64, 0x1000100u64, 0x800080u64, 0x400040u64, 0x200020u64,
     0x100010u64, 0x80008u64, 0x40004u64, 0x20002u64, 0x10001u64 ],
    &[ // m = 14
     0xcccc000u64, 0x4444000u64, 0xf0f0000u64, 0x5050000u64, 0x3030000u64,
     0x1010000u64, 0xff0000u64, 0x550000u64, 0x330000u64, 0x110000u64,
     0xf0000u64, 0x50000u64, 0x30000u64, 0x10000u64, 0xaaaa000u64, 0x5555000u64,
     0x8000800u64, 0x4000400u64, 0x2000200u64, 0x1000100u64, 0x800080u64,
     0x400040u64, 0x200020u64, 0x100010u64, 0x80008u64, 0x40004u64, 0x20002u64,
     0x10001u64 ],
    &[ // m = 15
     0x2aaa8000u64, 0x33330000u64, 0x11110000u64, 0xf0f0000u64, 0x5050000u64,
     0x3030000u64, 0x1010000u64, 0xff0000u64, 0x550000u64, 0x330000u64,
     0x110000u64, 0xf0000u64, 0x50000u64, 0x30000u64, 0x10000u64, 0x3fffc000u64,
     0x20002000u64, 0x10001000u64, 0x8000800u64, 0x4000400u64, 0x2000200u64,
     0x1000100u64, 0x800080u64, 0x400040u64, 0x200020u64, 0x100010u64,
     0x80008u64, 0x40004u64, 0x20002u64, 0x10001u64 ],
    &[ // m = 16
     0xffff0000u64, 0x55550000u64, 0x33330000u64, 0x11110000u64, 0xf0f0000u64,
     0x5050000u64, 0x3030000u64, 0x1010000u64, 0xff0000u64, 0x550000u64,
     0x330000u64, 0x110000u64, 0xf0000u64, 0x50000u64, 0x30000u64, 0x10000u64,
     0x80008000u64, 0x40004000u64, 0x20002000u64, 0x10001000u64, 0x8000800u64,
     0x4000400u64, 0x2000200u64, 0x1000100u64, 0x800080u64, 0x400040u64,
     0x200020u64, 0x100010u64, 0x80008u64, 0x40004u64, 0x20002u64, 0x10001u64 ],
    &[ // m = 17
     0x200020000u64, 0x3fffc0000u64, 0x355560000u64, 0x333300000u64,
     0x311120000u64, 0x30f0c0000u64, 0x305060000u64, 0x303000000u64,
     0x301020000u64, 0x300fc0000u64, 0x300560000u64, 0x300300000u64,
     0x300120000u64, 0x3000c0000u64, 0x300060000u64, 0x300000000u64,
     0x100000000u64, 0x300030000u64, 0x280028000u64, 0x240024000u64,
     0x220022000u64, 0x210021000u64, 0x208020800u64, 0x204020400u64,
     0x202020200u64, 0x201020100u64, 0x200820080u64, 0x200420040u64,
     0x200220020u64, 0x200120010u64, 0x2000a0008u64, 0x200060004u64,
     0x200000002u64, 0x100000001u64 ],
    &[ // m = 18
     0xc000c0000u64, 0x400040000u64, 0xffff00000u64, 0x555500000u64,
     0xf333c0000u64, 0x511140000u64, 0xf0f000000u64, 0x505000000u64,
     0xf030c0000u64, 0x501040000u64, 0xf00f00000u64, 0x500500000u64,
     0xf003c0000u64, 0x500140000u64, 0xf00000000u64, 0x500000000u64,
     0x300000000u64, 0x100000000u64, 0xa000a0000u64, 0x500050000u64,
     0x880088000u64, 0x440044000u64, 0x820082000u64, 0x410041000u64,
     0x808080800u64, 0x404040400u64, 0x802080200u64, 0x401040100u64,
     0x800880080u64, 0x400440040u64, 0x800280020u64, 0x400140010u64,
     0x800000008u64, 0x400000004u64, 0x200000002u64, 0x100000001u64 ],
    &[ // m = 19
     0x2800280000u64, 0x3000300000u64, 0x1000100000u64, 0xffff00000u64,
     0x2d55780000u64, 0x3333000000u64, 0x1111000000u64, 0xf0f000000u64,
     0x2d05280000u64, 0x3303300000u64, 0x1101100000u64, 0xf00f00000u64,
     0x2d00780000u64, 0x3300000000u64, 0x1100000000u64, 0xf00000000u64,
     0x500000000u64, 0x300000000u64, 0x100000000u64, 0x3c003c0000u64,
     0x2200220000u64, 0x1100110000u64, 0x880088000u64, 0x3840384000u64,
     0x2020202000u64, 0x1010101000u64, 0x808080800u64, 0x3804380400u64,
     0x2002200200u64, 0x1001100100u64, 0x800880080u64, 0x3800780040u64,
     0x2000000020u64, 0x1000000010u64, 0x800000008u64, 0x400000004u64,
     0x200000002u64, 0x100000001u64 ],
    &[ // m = 20
     0xf000f00000u64, 0x5000500000u64, 0x3000300000u64, 0x1000100000u64,
     0xffff000000u64, 0x5555000000u64, 0x3333000000u64, 0x1111000000u64,
     0xff0ff00000u64, 0x5505500000u64, 0x3303300000u64, 0x1101100000u64,
     0xff00000000u64, 0x5500000000u64, 0x3300000000u64, 0x1100000000u64,
     0xf00000000u64, 0x500000000u64, 0x300000000u64, 0x100000000u64,
     0x8800880000u64, 0x4400440000u64, 0x2200220000u64, 0x1100110000u64,
     0x8080808000u64, 0x4040404000u64, 0x2020202000u64, 0x1010101000u64,
     0x8008800800u64, 0x4004400400u64, 0x2002200200u64, 0x1001100100u64,
     0x8000000080u64, 0x4000000040u64, 0x2000000020u64, 0x1000000010u64,
     0x800000008u64, 0x400000004u64, 0x200000002u64, 0x100000001u64 ],
    &[ // m = 21
     0x22002200000u64, 0x3c003c00000u64, 0x36003600000u64, 0x30003000000u64,
     0x10001000000u64, 0xffff000000u64, 0x5555000000u64, 0x3333000000u64,
     0x23113200000u64, 0x3cf0cc00000u64, 0x36506600000u64, 0x30300000000u64,
     0x10100000000u64, 0xff00000000u64, 0x5500000000u64, 0x3300000000u64,
     0x1100000000u64, 0xf00000000u64, 0x500000000u64, 0x300000000u64,
     0x100000000u64, 0x33003300000u64, 0x2a802a80000u64, 0x26402640000u64,
     0x20202020000u64, 0x10101010000u64, 0x8080808000u64, 0x4040404000u64,
     0x2020202000u64, 0x32013201000u64, 0x2a00aa00800u64, 0x26006600400u64,
     0x20000000200u64, 0x10000000100u64, 0x8000000080u64, 0x4000000040u64,
     0x2000000020u64, 0x1000000010u64, 0x800000008u64, 0x400000004u64,
     0x200000002u64, 0x100000001u64 ],
    &[ // m = 22
     0xcc00cc00000u64, 0x44004400000u64, 0xf000f000000u64, 0x50005000000u64,
     0x30003000000u64, 0x10001000000u64, 0xffff000000u64, 0x5555000000u64,
     0xcf33fc00000u64, 0x45115400000u64, 0xf0f00000000u64, 0x50500000000u64,
     0x30300000000u64, 0x10100000000u64, 0xff00000000u64, 0x5500000000u64,
     0x3300000000u64, 0x1100000000u64, 0xf00000000u64, 0x500000000u64,
     0x300000000u64, 0x100000000u64, 0xaa00aa00000u64, 0x55005500000u64,
     0x80808080000u64, 0x40404040000u64, 0x20202020000u64, 0x10101010000u64,
     0x8080808000u64, 0x4040404000u64, 0xa802a802000u64, 0x54015401000u64,
     0x80000000800u64, 0x40000000400u64, 0x20000000200u64, 0x10000000100u64,
     0x8000000080u64, 0x4000000040u64, 0x2000000020u64, 0x1000000010u64,
     0x800000008u64, 0x400000004u64, 0x200000002u64, 0x100000001u64 ],
    &[ // m = 23
     0x2a802a800000u64, 0x330033000000u64, 0x110011000000u64, 0xf000f000000u64,
     0x50005000000u64, 0x30003000000u64, 0x10001000000u64, 0xffff000000u64,
     0x2ad57f800000u64, 0x333300000000u64, 0x111100000000u64, 0xf0f00000000u64,
     0x50500000000u64, 0x30300000000u64, 0x10100000000u64, 0xff00000000u64,
     0x5500000000u64, 0x3300000000u64, 0x1100000000u64, 0xf00000000u64,
     0x500000000u64, 0x300000000u64, 0x100000000u64, 0x3fc03fc00000u64,
     0x202020200000u64, 0x101010100000u64, 0x80808080000u64, 0x40404040000u64,
     0x20202020000u64, 0x10101010000u64, 0x8080808000u64, 0x3f807f804000u64,
     0x200000002000u64, 0x100000001000u64, 0x80000000800u64, 0x40000000400u64,
     0x20000000200u64, 0x10000000100u64, 0x8000000080u64, 0x4000000040u64,
     0x2000000020u64, 0x1000000010u64, 0x800000008u64, 0x400000004u64,
     0x200000002u64, 0x100000001u64 ],
    &[ // m = 24
     0xff00ff000000u64, 0x550055000000u64, 0x330033000000u64, 0x110011000000u64,
     0xf000f000000u64, 0x50005000000u64, 0x30003000000u64, 0x10001000000u64,
     0xffff00000000u64, 0x555500000000u64, 0x333300000000u64, 0x111100000000u64,
     0xf0f00000000u64, 0x50500000000u64, 0x30300000000u64, 0x10100000000u64,
     0xff00000000u64, 0x5500000000u64, 0x3300000000u64, 0x1100000000u64,
     0xf00000000u64, 0x500000000u64, 0x300000000u64, 0x100000000u64,
     0x808080800000u64, 0x404040400000u64, 0x202020200000u64, 0x101010100000u64,
     0x80808080000u64, 0x40404040000u64, 0x20202020000u64, 0x10101010000u64,
     0x800000008000u64, 0x400000004000u64, 0x200000002000u64, 0x100000001000u64,
     0x80000000800u64, 0x40000000400u64, 0x20000000200u64, 0x10000000100u64,
     0x8000000080u64, 0x4000000040u64, 0x2000000020u64, 0x1000000010u64,
     0x800000008u64, 0x400000004u64, 0x200000002u64, 0x100000001u64 ],
    &[ // m = 25
     0x2020202000000u64, 0x3fc03fc000000u64, 0x3560356000000u64,
     0x3300330000000u64, 0x3120312000000u64, 0x30c030c000000u64,
     0x3060306000000u64, 0x3000300000000u64, 0x1000100000000u64,
     0xffff00000000u64, 0x555500000000u64, 0x333300000000u64, 0x111100000000u64,
     0xf0f00000000u64, 0x50500000000u64, 0x30300000000u64, 0x10100000000u64,
     0xff00000000u64, 0x5500000000u64, 0x3300000000u64, 0x1100000000u64,
     0xf00000000u64, 0x500000000u64, 0x300000000u64, 0x100000000u64,
     0x3030303000000u64, 0x2828282800000u64, 0x2424242400000u64,
     0x2222222200000u64, 0x2121212100000u64, 0x20a0a0a080000u64,
     0x2060606040000u64, 0x2000000020000u64, 0x1000000010000u64,
     0x800000008000u64, 0x400000004000u64, 0x200000002000u64, 0x100000001000u64,
     0x80000000800u64, 0x40000000400u64, 0x20000000200u64, 0x10000000100u64,
     0x8000000080u64, 0x4000000040u64, 0x2000000020u64, 0x1000000010u64,
     0x800000008u64, 0x400000004u64, 0x200000002u64, 0x100000001u64 ],
    &[ // m = 26
     0xc0c0c0c000000u64, 0x4040404000000u64, 0xff00ff0000000u64,
     0x5500550000000u64, 0xf3c0f3c000000u64, 0x5140514000000u64,
     0xf000f00000000u64, 0x5000500000000u64, 0x3000300000000u64,
     0x1000100000000u64, 0xffff00000000u64, 0x555500000000u64,
     0x333300000000u64, 0x111100000000u64, 0xf0f00000000u64, 0x50500000000u64,
     0x30300000000u64, 0x10100000000u64, 0xff00000000u64, 0x5500000000u64,
     0x3300000000u64, 0x1100000000u64, 0xf00000000u64, 0x500000000u64,
     0x300000000u64, 0x100000000u64, 0xa0a0a0a000000u64, 0x5050505000000u64,
     0x8888888800000u64, 0x4444444400000u64, 0x8282828200000u64,
     0x4141414100000u64, 0x8000000080000u64, 0x4000000040000u64,
     0x2000000020000u64, 0x1000000010000u64, 0x800000008000u64,
     0x400000004000u64, 0x200000002000u64, 0x100000001000u64, 0x80000000800u64,
     0x40000000400u64, 0x20000000200u64, 0x10000000100u64, 0x8000000080u64,
     0x4000000040u64, 0x2000000020u64, 0x1000000010u64, 0x800000008u64,
     0x400000004u64, 0x200000002u64, 0x100000001u64 ] ];