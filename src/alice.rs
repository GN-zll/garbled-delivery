use common::network::{tcp_connect, Role, DEFAULT_LOCAL};
use common::circuits_patch;
use mpz_circuits::circuits::AES128;
use mpz_common::Context;
use common::garble_setup::setup_garbler;
use mpz_memory_core::{binary::U8, Array, MemoryExt, ViewExt};
use mpz_vm_core::{Call, CallableExt, Execute};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Open a connection.
    // Open a connection.
    let tcp = tcp_connect(Role::Alice, DEFAULT_LOCAL).await?;
    let mut context = Context::new_single_threaded(tcp);

    // Instantiate a vm for garbled circuits.
    let mut garble_vm = setup_garbler().await?;

    // Define input types.
    let key: Array<U8, 16> = garble_vm.alloc()?;
    let message: Array<U8, 16> = garble_vm.alloc()?;
    let alice_shar: Array<U8, 16> = garble_vm.alloc()?;

    // Define input visibility.
    garble_vm.mark_private(key)?;
    garble_vm.mark_blind(message)?;
    garble_vm.mark_private(alice_shar)?;

    // Define output
    let bob_shar: Array<U8, 16> = garble_vm.call(
        Call::builder(circuits_patch::add_delivery_layer(AES128.clone()))
            .arg(key)
            .arg(message)
            .arg(alice_shar)
            .build()?,
    )?;

    let mut bob_shar = garble_vm.decode(bob_shar)?;

    // Assign random values to alice_shar
    let random_alice: [u8; 16] = std::array::from_fn(|_| rand::random::<u8>());
    garble_vm.assign(alice_shar, random_alice.clone())?;

    // Assign the key.
    garble_vm.assign(
        key,
        [
            0x2b_u8, 0x7e, 0x15, 0x16, 0x28, 0xae, 0xd2, 0xa6, 0xab, 0xf7, 0x15, 0x88, 0x09, 0xcf,
            0x4f, 0x3c,
        ],
    )?;

    // Commit the values
    garble_vm.commit(key)?;
    garble_vm.commit(message)?;
    garble_vm.commit(alice_shar)?;

    // Execute the circuit.
    garble_vm.execute_all(&mut context).await?;

    let alice_shar_clear = garble_vm.decode(alice_shar)?.try_recv()?.unwrap();
    let bob_shar_clear = bob_shar.try_recv()?.unwrap();


    let xor: Vec<u8> = alice_shar_clear
        .iter()
        .zip(bob_shar_clear.iter())
        .map(|(a, b)| a ^ b)
        .collect();


    println!("bob_shar: {:x?}", bob_shar_clear);
    println!("alice_shar: {:x?}", alice_shar_clear);
    println!("xor: {:x?}", xor);

    Ok(())
}