#include <errno.h>
#include <libax.h>
#include <stdint.h>
#include <stdio.h>
#include <sys/select.h>
#include <sys/time.h>

#define IS32BIT(x) !((x) + 0x80000000ULL >> 32)
#define CLAMP(x)   (int)(IS32BIT(x) ? (x) : 0x7fffffffU + ((0ULL + (x)) >> 63))

int select(int n, fd_set *__restrict rfds, fd_set *__restrict wfds, fd_set *__restrict efds,
           struct timeval *__restrict tv)
{
    time_t s = tv ? tv->tv_sec : 0;
    long us = tv ? tv->tv_usec : 0;
    // long ns;
    const time_t max_time = (1ULL << (8 * sizeof(time_t) - 1)) - 1;

    if (s < 0 || us < 0) {
        errno = EINVAL;
        return -1;
    }
    if (us / 1000000 > max_time - s) {
        s = max_time;
        us = 999999;
        // ns = 999999999;
    } else {
        s += us / 1000000;
        us %= 1000000;
        // ns = us * 1000;
    }

    return ax_select(n, rfds, wfds, efds, tv ? ((struct timeval *)(long[]){s, us}) : NULL);
}

int pselect(int, fd_set *__restrict, fd_set *__restrict, fd_set *__restrict,
            const struct timespec *__restrict, const sigset_t *__restrict)
{
    unimplemented();
    return 0;
}
