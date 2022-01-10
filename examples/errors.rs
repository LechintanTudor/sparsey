use sparsey::prelude::*;

fn a() -> SystemResult {
    let _ = "a".parse::<i32>()?;
    Ok(())
}

fn b() -> SystemResult {
    let _ = "b".parse::<f32>()?;
    Ok(())
}

fn c() -> SystemResult {
    let _ = "c".parse::<bool>()?;
    Ok(())
}

fn main() {
    let mut dispatcher = Dispatcher::builder().add_system(a).add_system(b).add_system(c).build();

    let mut world = World::default();

    if let Err(run_error) = dispatcher.run_seq(&mut world) {
        println!("[{} errors occurred]", run_error.errors().len());
        for error in run_error.errors() {
            println!("{}", error);
        }
    }
}
