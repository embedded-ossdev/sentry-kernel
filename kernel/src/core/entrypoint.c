#include <inttypes.h>
#include <stdbool.h>

/* kernel includes */
#include <sentry/arch/asm-generic/platform.h>
#include <sentry/managers/io.h>
#include <sentry/managers/debug.h>
#include <sentry/managers/clock.h>
#include <sentry/managers/interrupt.h>
#include <sentry/managers/security.h>
#include <sentry/managers/task.h>
#include <sentry/managers/memory.h>
#include <sentry/managers/time.h>
#include <sentry/thread.h>

/* used for debug printing only */
extern uint32_t _bootupstack;

/*
 * address if the PSP idle stack, as defined in the layout (see m7fw.ld)
 */

#if __GNUC__
#if __clang__
# pragma clang optimize off
#else
__attribute__((optimize("-fno-stack-protector")))
#endif
#endif
__attribute__((noreturn)) void _entrypoint(void)
{
    /* early init phase */
    mgr_interrupt_early_init();
    /* platform init phase */
    mgr_clock_init(); /* init bus clocking, needed for debug */
    mgr_io_init();  /* I/O probing and init, needed for debug */
    #ifndef CONFIG_BUILD_TARGET_RELEASE
    mgr_debug_init();
    #endif
    /* debug primitive can be used from now own transparently */
#ifdef CONFIG_BUILD_TARGET_AUTOTEST
    pr_autotest("WARN: starting autotest mode");
    pr_autotest("INFO: no task discover in this mode");
#endif
    pr_info("Starting Sentry kernel release %s", "v0.1");
    pr_info("kernel bootup stack at %p, current frame: %p", &_bootupstack, __platform_get_current_sp());
    pr_info("booting on SoC %s", CONFIG_ARCH_SOCNAME);
    pr_info("configured dts file: %s", CONFIG_DTS_FILE);
    /* initialize security manager */
    mgr_security_init();
    /* initialize memory manager */
    mgr_mm_init();
    /* user interrupts manager init */
    mgr_interrupt_init();
    /* delays and scheduler init */
    mgr_time_init();
    /* tasks initialization (probing) */
    mgr_task_init();
    /* device autogenerated listing update with associated owner for each (need task probing first) */
    mgr_device_init();
    /* finishing platform init. platform init flag is set */
    pr_info("Platform initialization done, continuing with upper layers");
    platform_init();
    /* enable interrupts */
    interrupt_enable();
#if CONFIG_BUILD_TARGET_AUTOTEST
    pr_autotest("INFO: init finished");
#endif
    pr_debug("starting userspace");
    mgr_task_start();
    __builtin_unreachable();
    /* This part of the function is never reached */
}
