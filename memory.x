MEMORY
{
    FLASH   : org = 0x08000000, len = 64k
    RAM     : org = 0x20000000, len = 8k
}

/* no heap */
__heap_base__ = _sheap;
__heap_end__ = _sheap;
