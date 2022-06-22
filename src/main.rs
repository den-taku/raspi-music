use alto::{Alto, AltoResult};

fn main() -> AltoResult<()> {
    let alto = Alto::load_default()?;

    for s in alto.enumerate_outputs() {
        println!("Found device: {}", s.to_str().unwrap());
    }

    let device = alto.open(None)?;
    let context = device.new_context(None)?;

    context.set_position([1.0, 4.0, 5.0])?;
    context.set_velocity([2.5, 0.0, 0.0])?;
    context.set_orientation(([0.0, 0.0, 1.0], [0.0, 1.0, 0.0]))?;

    let _source = context.new_static_source()?;

    Ok(())
}
