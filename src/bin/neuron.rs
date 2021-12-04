use std::thread;
use srm::Neuron;
use srm::Incoming;
use std::time::Duration;
use crossbeam_channel::unbounded;
use crossbeam_channel::Sender;

fn main(){
    let (tran, recv) = unbounded();
    
    let  r1 = recv.clone();

    //let raw_tran = &tran as *const Sender<f32>;

    let handle1 = thread::spawn(move || {
        let mut neuron = Neuron{
            s:0.0,
            eta_reset:5.0,
            tm:20.0,
            tc:0.3,
            threshold:1.0,
            membrane_potential:0.0,
        };
        
        loop {
            match r1.try_recv(){
                Ok(signal) => neuron.synapse(signal),
                Err(e) => panic!("Error {}", e),
            };
        neuron.s += 1.0;
        thread::sleep(Duration::from_secs(1));
        }
    });

    let mut neuron1 = Incoming{
            weight:1.0,
            spike:1.0,
    };

    let mut neuron2 = Incoming{
            weight:1.0,
            spike:1.0,
    };

    //let raw_tran  = 0x7ffcceb3fe90 as *const Sender<f32>;
    
    //let points_at_tran = unsafe { &*raw_tran };
    
    //let t1 = points_at_tran.clone();

    //println!("points_raw {:?}", points_at_tran);

    for _ in 1..20{ 
    tran.send(neuron1.spike()).unwrap();
    println!("Neuron1 Spike!");

    tran.send(neuron2.spike()).unwrap();
    println!("Neuron2 Spike!");
    }
    handle1.join().unwrap();
}
