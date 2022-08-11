#include <stdio.h>

static char STRING[] = "Hello world on static";
const char on_rodata[] = "constant string";

static char UNINITIALIED_BUFFER[0x10];

int main() {
  printf("msg = %s\n", STRING);
  printf("const = %s", on_rodata);
}
