//This example shows a simulation for a conveyor belt from where raw PCBs are
//processed in stages (each stage adding a particular type of component, and the
//conveyor belt stopping while an item is being processed). After each step, if
//the item hasn't reached its final stage, it rejoins the back of the queue
//waiting to be processed again.
//
//Sometimes, similar situations appear in software when processing elements
//using event loops.
//
//This example is also meant to show how you can develop an application-specific
//API so that the code written inside the process generator is simple and can
//be followed naturally using domain-specific notions.
//
#![feature(generators, generator_trait)]
use desim::{Effect, EndCondition, ResourceId, SimGen, SimState, Simulation};
use rand::rngs::SmallRng as Rng;
use rand::{RngCore as RngT, SeedableRng};

// PCBStage describes the stages through which a PCB passes as it is processed
// in the system.
#[derive(Copy, Clone, Debug)]
enum PCBStage {
    Init,
    RequestPip,
    RequestEt,
    InWork,
    SurfaceMountPlaced,
    ThruHolePlaced,
    ElectricalTestPerformed,
    Done,
}

// PCBState is the type yielded by proceses in this simulation. Besides the
// intended effect on the simulation, it stores two pieces of
// application-specific data: a PCB id and the processing stage. PCBs will be
// uniquely identifiable by the (process_id, pcb_id) pair.
#[derive(Clone, Debug)]
struct PCBState {
    pcb_id: usize,
    effect: Effect,
    log: bool,
    stage: PCBStage,
}

impl SimState for PCBState {
    fn get_effect(&self) -> Effect {
        self.effect
    }
    fn set_effect(&mut self, e: Effect) {
        self.effect = e;
    }
    fn should_log(&self) -> bool {
        self.log
    }
}

// The following two structures, Resources and PCBStateCtx, are defined in order
// to simplify the simulation code written in each process. For a simple example
// such as this, it might add unnecessary complexity, but if one needs to write
// multiple simulations in a given domain the extra abstraction can pay off.

// The resources used by this simulation, a pick-and-place machine and an
// electrical tester
#[derive(Copy, Clone)]
struct Resources {
    pip: ResourceId, // pick-and-place machine
    et: ResourceId,  // electrical tester
}

// We'll initialize one PCBStateCtx in each process of the simulation. It will
// be the data structure responsible for holding the state needed in the process,
// and the functions in its implementation will hand out PCBState objects to be
// yielded at the apropriate times in the simulation.
struct PCBStateCtx {
    rng: Rng,
    res: Resources,
    state: PCBState,
}

impl PCBStateCtx {
    fn new(r: Resources) -> PCBStateCtx {
        PCBStateCtx {
            rng: Rng::from_entropy(),
            res: r,
            state: PCBState {
                pcb_id: 0,
                effect: Effect::Wait,
                log: true,
                stage: PCBStage::Init,
            },
        }
    }

    fn set_new_id(&mut self, pcb_id: usize) {
        self.state.pcb_id = pcb_id;
    }

    #[inline]
    fn res(&mut self, r_id: ResourceId, should_log: bool, stage: PCBStage, need: bool) -> PCBState {
        let mut r_state = self.state.clone();
        r_state.stage = stage.clone();
        r_state.log = should_log;
        if need {
            r_state.effect = Effect::Request(r_id);
        } else {
            r_state.effect = Effect::Release(r_id);
            self.state.stage = stage;
        }
        r_state
    }
    #[inline]
    fn work(&self, should_log: bool, service_time: f64, stage: PCBStage) -> PCBState {
        let mut r_state = self.state.clone();
        r_state.stage = stage;
        r_state.log = should_log;
        r_state.effect = Effect::TimeOut(service_time);
        r_state
    }

    fn need_pip(&mut self) -> PCBState {
        self.res(self.res.pip, true, PCBStage::RequestPip, true)
    }
    fn need_et(&mut self) -> PCBState {
        self.res(self.res.et, true, PCBStage::RequestEt, true)
    }
    fn free_pip(&mut self, stage_end: PCBStage) -> PCBState {
        self.res(self.res.pip, true, stage_end, false)
    }
    fn free_et(&mut self, stage_end: PCBStage) -> PCBState {
        self.res(self.res.et, true, stage_end, false)
    }
    fn pip_work(&mut self) -> PCBState {
        let st = match self.state.stage {
            PCBStage::Init => (15 + self.rng.next_u32() % 20) as f64,
            PCBStage::SurfaceMountPlaced => (30 + self.rng.next_u32() % 20) as f64,
            _ => 0.0 as f64,
        };
        self.work(true, st, PCBStage::InWork)
    }
    fn et_work(&mut self) -> PCBState {
        let st = match self.state.stage {
            PCBStage::ThruHolePlaced => (5 + self.rng.next_u32() % 5) as f64,
            _ => 0.0 as f64,
        };
        self.work(true, st, PCBStage::InWork)
    }

    fn mark(&mut self, stage: PCBStage) -> PCBState {
        self.state.stage = PCBStage::Init;
        let mut r_state = self.state.clone();
        r_state.stage = stage;
        r_state.log = true;
        r_state.effect = Effect::Trace;
        r_state
    }
}

fn process_code(r: Resources) -> Box<SimGen<PCBState>> {
    Box::new(move |_| {
        let mut current_pcb_id = 0;
        let mut ctx = PCBStateCtx::new(r);
        loop {
            // PCB first processing stage
            yield ctx.need_pip();
            yield ctx.pip_work();
            yield ctx.free_pip(PCBStage::SurfaceMountPlaced);
            // requeue PCB for second processing stage
            yield ctx.need_pip();
            yield ctx.pip_work();
            yield ctx.free_pip(PCBStage::ThruHolePlaced);
            // queue for electrical testing
            yield ctx.need_et();
            yield ctx.et_work();
            yield ctx.free_et(PCBStage::ElectricalTestPerformed);
            yield ctx.mark(PCBStage::Done);

            current_pcb_id += 1;
            ctx.set_new_id(current_pcb_id);
        }
    })
}

fn main() {
    let mut s = Simulation::new();
    let pip = s.create_resource(1);
    let et = s.create_resource(1);
    let res = Resources { pip, et };
    for _ in 1..5 {
        let p = s.create_process(process_code(res));
        s.schedule_event(
            0.0,
            p,
            PCBState {
                pcb_id: 0,
                effect: Effect::Event {
                    time: 0.,
                    process: p,
                },
                log: true,
                stage: PCBStage::Init,
            },
        );
    }
    s = s.run(EndCondition::Time(500.0));
    let evts = s.processed_events();
    println!("time: (pid, pcb_id) action stage");
    for (ev, state) in evts {
        println!(
            "{:?}: id({},{}) {:?}, {:?}",
            ev.time(),
            ev.process(),
            state.pcb_id,
            state.effect,
            state.stage
        );
    }
}
