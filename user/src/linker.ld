
BASE_ADDRESS = 0x80500000;
SECTIONS{
    . = BASE_ADDRESS;
    . = ALIGN(4K);
    .text : {
    }

    . = ALIGN(4K);
    .rodata : {
    }

    . = ALIGN(4K);

    .data : {
    }

    . = ALIGN(4K);
    .bss : {
    }

    /* 结束地址 */
    . = ALIGN(4K);

}
