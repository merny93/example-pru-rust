extern crate prusst;

use prusst::{util::VolatileCell, Evtout, IntcConfig, Pruss, Sysevt};
use std::fs::File;
use std::sync::mpsc::channel;
use std::{thread, time};

const BUFFER_LENGTH: usize = 2048;

// try https://robamu.github.io/post/cross-compile-rust-rpi/ for debugging across ssh

fn main() {
    // First run the Multiply Accumulate example showing that a control algorithm can be run on the PRU
    //
    // The PRU can output straight to GPIO pins without any overhead
    // It is comparable to a midrange MCU with slightly less rich peripherals
    //
    // There are 2 PRUs each with 2 cores.
    // Some of these cores can be used for control and others to decode and encode communications
    {
        println!("Running Multiply Accumulate example...");
        // Configure and get a view of the PRU subsystem.
        let mut pruss = Pruss::new(&IntcConfig::new_populated()).unwrap();
        clear_dram(&mut pruss);

        // dram2 is shared dram
        let communication = pruss.dram2.alloc(Communication {
            result: VolatileCell::new(0),
            sample_length: [1; BUFFER_LENGTH],
        });
        // Get a handle to an event out before it is triggered.
        let irq = pruss.intc.register_irq(Evtout::E0);

        // Open, load and run a PRU binary.
        let mut file = File::open("mac.bin").unwrap();
        unsafe {
            pruss.pru0.load_code(&mut file).unwrap().run();
        }

        let start = time::Instant::now();

        println!("PRU code loaded and running...");

        // Wait for the PRU code from hello.bin to trigger an event out.
        irq.wait();

        let end = time::Instant::now();

        println!(
            "PRU is running at {:.1} million multiply accumulates per second",
            communication.result.get() as f32 / (end - start).as_secs_f32() / 1000000.0
        );
        // This prints roughly 13.5 million multiply accumulates per second
        // A multiply accumulate takes 4 cycles along with a few reads and writes
        // at 200Mhz this seems reasonable

        // Clear the triggering interrupt.
        pruss.intc.clear_sysevt(Sysevt::S19);
    }
    // Now run a second example showing that the PRU can do its own things while the main CPU is doing other things
    // The PRU is not at the mercy of the linux scheduler so it has !no! jitter
    //
    // Here I am computing the fibonacci sequence while the main CPU is doing other things
    // In reality the PRU would be doing something like generating a pulse sequence for a motor
    // or decoding some kind of specialty SPI
    //
    // The PRU replaces what most people would use a dedicated MCU for
    {
        println!("Running Fibonacci example...");
        let mut pruss = Pruss::new(&IntcConfig::new_populated()).unwrap();
        clear_dram(&mut pruss);

        let fib_number = 40;

        let communication = pruss.dram2.alloc(VolatileCell::new(fib_number));
        let irq = pruss.intc.register_irq(Evtout::E0);
        let mut file = File::open("fib.bin").unwrap();
        unsafe {
            pruss.pru0.load_code(&mut file).unwrap().run();
        }
        println!("PRU code loaded and running...");

        // spawn a thread that will signal the main thread when the PRU is done
        // linux is preemptive so the main thread does not need to "yield" explicitly as is done here
        let (tx, rx) = channel();
        thread::spawn(move || {
            irq.wait();
            tx.send(())
        });

        while let Err(_e) = rx.try_recv() {
            //doing work!
            thread::sleep(time::Duration::from_micros(1));
            println!("waiting for result... true bare-metal async :)");
        }
        let result = communication.get();

        println!("The {}th fibonacci number is: {}", fib_number, result);
        pruss.intc.clear_sysevt(Sysevt::S19);
        // Do nothing: the `pruss` destructor will stop any running code and release resources.
    }
    println!("We are done...");
}


///
/// Function clears the DRAM of the PRU as it does not get cleared by linux or the PRU
///
/// Results in some really strange errors...
fn clear_dram(pruss: &mut Pruss) {
    pruss.dram0.alloc([0u8; 1024 * 8]);
    pruss.dram1.alloc([0u8; 1024 * 8]);
    pruss.dram2.alloc([0u8; 1024 * 12]);
}


#[repr(C)]
#[derive(Copy, Clone)]
struct Communication {
    result: VolatileCell<u32>,
    sample_length: [u32; BUFFER_LENGTH],
}