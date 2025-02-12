#include <stdint.h>

// the other way to control shared memory. maybe better?
// https://markayoder.github.io/PRUCookbook/05blocks/blocks.html


volatile register uint32_t __R30; /* output register for PRU */
volatile register uint32_t __R31; /* configuration register for PRU */

#define BUFFER_LENGTH 2048

struct test_struct
{
    volatile uint32_t result;
    volatile uint32_t buffer[BUFFER_LENGTH];
};

#define PRU_SHAREDMEM 0x00010000
#define STRUCT_OFFSET 0x00


volatile struct test_struct *test;


void main()
{
    // watchout big unsafe. make sure no one is writing to PRU_SHAREDMEM or else bad things will happen...
    test = (struct test_struct *)(PRU_SHAREDMEM + STRUCT_OFFSET);

    uint32_t i;
    uint32_t j;
    for (j = 0; j < 100; j++) {
        for (i = 0; i < BUFFER_LENGTH; i += 2)
        {
            test->result += (test->buffer[i] * test->buffer[i + 1]);
        }
    }
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
