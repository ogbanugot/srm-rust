use std::thread;
use srm::Neuron;
use srm::Incoming;
use std::sync::mpsc;

fn main(){
    let (tran, recv) = mpsc::channel();

    let mut neuron = Neuron{
            s:0.0,
            eta_reset:5.0,
            tm:20.0,
            tc:0.3,
            threshold:1.0,
            tx:tran,
            rx:recv,
            membrane_potential:0.0,
        };

    let a = [1.0, 2.0,3.0,4.0,5.0,6.0,7.0,8.0,9.0,10.0];
    for time in a.iter(){

    let ntx1 = mpsc::Sender::clone(&neuron.tx);
    let handle1 = thread::spawn(move || {
            let mut neuron1 = Incoming{
                weight:1.0,
                spike:1.0,
            };
            println!("Neuron1 Spike!");
            ntx1.send(neuron1.spike()).unwrap();
    });

    let ntx2 = mpsc::Sender::clone(&neuron.tx);

    let handle2 = thread::spawn(move || {
            let mut neuron2 = Incoming{
                weight:1.0,
                spike:1.0,
            };
            println!("Neuron2 Spike!");
            ntx2.send(neuron2.spike()).unwrap();
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    neuron.s = *time;
    neuron.check_spike();
    }
}
