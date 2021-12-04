const E:f32 = 2.71828;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
//use crossbeam_channel::Receiver;

pub struct Incoming{
   pub weight:f32,
   pub spike:f32,
}
impl Incoming{
    pub fn spike(&mut self)->f32{
        self.weight * self.spike
    }
}

#[derive(Copy)]
#[derive(Clone)]
pub struct Neuron {
    pub s: f32,
    pub eta_reset: f32,
    pub tm: f32,
    pub tc: f32,
    pub threshold:f32,
    pub membrane_potential:f32,
}

impl Neuron{
    pub fn eta(&self)-> f32{
        let thresh = -self.s/self.tm;
        self.eta_reset * E.powf(thresh)
    }

    pub fn eps(&self)-> f32{
        let t1 = -self.s/self.tm;
        let t2 = -self.s/self.tc;
        (1.0 / (1.0 - (self.tc/self.tm))) * (E.powf(t1) - E.powf(t2))
    }

    pub fn synapse(&mut self, signal: f32){

      self.membrane_potential += signal * self.eps();

      if self.membrane_potential > self.threshold{
          println!("Membrane potential {}", self.membrane_potential);
          println!("True");
          println!("Eta {}",self.eta());
          self.membrane_potential -= self.eta();
          self.s = 0.0;
      }else{                
          println!("Membrane potential {}", self.membrane_potential);
          println!("False");
      }

    }
}

