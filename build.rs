fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/greeting.proto")?;
    tonic_build::compile_protos("proto/employee.proto")?;
    Ok(())
}
