use common::network::{tcp_connect, Role, DEFAULT_LOCAL};
use common::circuits_patch;
use mpz_circuits::circuits::AES128;
use mpz_common::Context;
use common::garble_setup::setup_evaluator;
use mpz_memory_core::{binary::U8, Array, MemoryExt, ViewExt};
use mpz_vm_core::{Call, CallableExt, Execute};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    let tcp = tcp_connect(Role::Bob, DEFAULT_LOCAL).await?;
    let mut context = Context::new_single_threaded(tcp);

    // Instantiate a vm for garbled circuits.
    let mut evaluator = setup_evaluator().await?;

    // Define input types.
    let key: Array<U8, 16> = evaluator.alloc()?;
    let msg: Array<U8, 16> = evaluator.alloc()?;
    let alice_shar: Array<U8, 16> = evaluator.alloc()?;

    // Define input visibility.
    evaluator.mark_blind(key)?;
    evaluator.mark_private(msg)?;
    evaluator.mark_blind(alice_shar)?;

    // Define output.
    let bob_shar: Array<U8, 16> =
        evaluator.call(Call::builder(circuits_patch::add_delivery_layer(AES128.clone())).arg(key).arg(msg).arg(alice_shar).build()?)?;

    let mut bob_shar = evaluator.decode(bob_shar)?;

    // Assign the message.
    evaluator.assign(
        msg,
        [
            0x6b_u8, 0xc1, 0xbe, 0xe2, 0x2e, 0x40, 0x9f, 0x96, 0xe9, 0x3d, 0x7e, 0x11, 0x73, 0x93,
            0x17, 0x2a,
        ],
    )?;

    // Commit the values
    evaluator.commit(key)?;
    evaluator.commit(msg)?;
    evaluator.commit(alice_shar)?;

    // Execute the circuit.
    evaluator.execute_all(&mut context).await?;

    let output = bob_shar.try_recv()?.unwrap();
    println!("bob_shar: {:x?}", output);

    Ok(())
}
