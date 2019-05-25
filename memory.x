/*
The H743zi has much more flash and ram than this,
but I didn't spend time figuring out how to get it
working. Pull requests welcome.
*/

MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}
