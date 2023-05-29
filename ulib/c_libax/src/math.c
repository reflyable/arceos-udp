#include <math.h>
#include <stdint.h>
#if defined(AX_CONFIG_FP_SIMD)
#define FORCE_EVAL(x)                             \
    do {                                          \
        if (sizeof(x) == sizeof(float)) {         \
            fp_force_evalf(x);                    \
        } else if (sizeof(x) == sizeof(double)) { \
            fp_force_eval(x);                     \
        }                                         \
    } while (0)

int __fpclassify(double x)
{
    union {
        double f;
        uint64_t i;
    } u = {x};
    int e = u.i >> 52 & 0x7ff;
    if (!e)
        return u.i << 1 ? FP_SUBNORMAL : FP_ZERO;
    if (e == 0x7ff)
        return u.i << 12 ? FP_NAN : FP_INFINITE;
    return FP_NORMAL;
}

int __fpclassifyf(float x)
{
    union {
        float f;
        uint32_t i;
    } u = {x};
    int e = u.i >> 23 & 0xff;
    if (!e)
        return u.i << 1 ? FP_SUBNORMAL : FP_ZERO;
    if (e == 0xff)
        return u.i << 9 ? FP_NAN : FP_INFINITE;
    return FP_NORMAL;
}

double fabs(double x)
{
    union {
        double f;
        uint64_t i;
    } u = {x};
    u.i &= -1ULL / 2;
    return u.f;
}

static const double toint = 1 / 2.2204460492503131e-16;

double floor(double x)
{
    union {
        double f;
        uint64_t i;
    } u = {x};
    int e = u.i >> 52 & 0x7ff;
    double y;

    if (e >= 0x3ff + 52 || x == 0)
        return x;
    /* y = int(x) - x, where int(x) is an integer neighbor of x */
    if (u.i >> 63)
        y = x - toint + toint - x;
    else
        y = x + toint - toint - x;
    /* special case because of non-nearest rounding modes */
    if (e <= 0x3ff - 1) {
        FORCE_EVAL(y);
        return u.i >> 63 ? -1 : 0;
    }
    if (y > 0)
        return x + y - 1;
    return x + y;
}
#endif
