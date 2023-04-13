use rapier_testbed3d::{Testbed, TestbedApp};

mod bounceball;
mod bounceballzerog;
mod recoil;
mod recoilgradual;

fn main() {
    let builders: Vec<(_, fn(&mut Testbed))> = vec![
        ("Bounce ball", bounceball::init_world),
        ("Bounce ball zero-G", bounceballzerog::init_world),
        ("Recoil", recoil::init_world),
        ("Recoil gradual", recoilgradual::init_world),
    ];

    let testbed = TestbedApp::from_builders(0, builders);
    testbed.run()
}
