#include <stdint.h>

volatile register uint32_t __R30; /* output register for PRU */
volatile register uint32_t __R31; /* configuration register for PRU */

#define PRU_SHAREDMEM 0x00010000

volatile uint32_t *result;

uint32_t fib(uint32_t n)
{
    uint32_t prev = 0;
    uint32_t curr = 1;
    uint32_t next;
    uint32_t i;
    // uint32_t j;

    if (n <= 1)
    {
        return n;
    }

    for (i = 2; i <= n; i++)
    {
        next = prev + curr;
        prev = curr;
        curr = next;
    }

    return curr;
}

void main()
{

    // watchout big unsafe. make sure no one is writing to PRU_SHAREDMEM or else bad things will happen...
    result = (uint32_t *)(PRU_SHAREDMEM);

    uint32_t n;
    n = *result;
    *result = fib(n);
    /*
    The PRU has 64 system events for all the different things that can trigger interrupts, such as timers or I/O (see TRM 4.4.2.2).
    When generating an event with R31, bit 5 triggers an event, while the lower 4 bits select the event number (see TRM 4.4.1.2).
    PRU system event numbers are offset by 16 from the internal event numbers, so internal event 3 is system event 19.
    See PRU interrupts for the full list of events.
    Not confusing at all!
    */
    __R31 = 32 | (19 - 16);
    __halt();
}
