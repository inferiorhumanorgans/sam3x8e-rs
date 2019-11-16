/* Linker script for a Macchina M2 / SAM3x8e */

MEMORY
{
  FLASH : ORIGIN = 0x00080000, LENGTH = 0x00080000 /* Flash, 512K */
  RAM : ORIGIN = 0x20000000, LENGTH = 0x00010000 /* 64K */
}