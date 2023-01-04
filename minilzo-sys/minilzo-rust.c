#include <minilzo.h>

LZO_EXTERN(int) lzo_init_func(void) {
    return lzo_init();
}