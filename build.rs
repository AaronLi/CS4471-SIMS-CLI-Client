
fn main() -> Result<(), Box<dyn std::error::Error>>{
    tonic_build::configure()
        .build_server(false)
        .compile(
            &["proto/frontend_proto/frontend.proto",
                "proto/frontend_proto/frontend_messages.proto"
                    ],
                &["proto"]
        )?;
    Ok(())
}
