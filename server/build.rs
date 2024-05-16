fn main () -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/offer.proto")?;
    tonic_build::compile_protos("../protos/product.proto")?;
    tonic_build::compile_protos("../protos/subscribe.proto")?;
    Ok(())
  }