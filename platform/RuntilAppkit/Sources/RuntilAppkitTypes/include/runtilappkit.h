#ifndef _RUNTIL_RUNTILAPPKIT_H_
#define _RUNTIL_RUNTILAPPKIT_H_

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    void (*on_launch)(void*);
    void (*will_terminate)(void*);
} AppCbs;

#ifdef __cplusplus
}
#endif

#endif
