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
    let mut dispatcher = Dispatcher::builder()
        .add_system(a.system())
        .add_system(b.system())
        .add_system(c.system())
        .build();

    let mut world = World::default();

    if let Err(run_error) = dispatcher.run_seq(&mut world) {
        println!("[{} errors occurred]", run_error.errors().len());
        for error in run_error.errors() {
            println!("{}", error);
        }
    }
}
