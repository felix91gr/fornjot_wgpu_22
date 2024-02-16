use fj::handle_model;

fn main() -> fj::Result {
    let mut fj = fj::Instance::new();
    let model = star::model(5, 1., 2., 1., &mut fj.core);
    handle_model(&model, &mut fj.core)?;
    Ok(())
}
