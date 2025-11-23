use anyhow::Error as Anyhow;
use mpz_core::Block;
use mpz_garble::protocol::semihonest::{Evaluator, Garbler};
use mpz_memory_core::correlated::Delta;
use mpz_ot::{
    chou_orlandi::{Receiver as BaseReceiver, Sender as BaseSender},
    cot::{DerandCOTReceiver, DerandCOTSender},
    kos::{Receiver, ReceiverConfig, Sender, SenderConfig},
};
use rand::{rngs::StdRng, SeedableRng};

pub async fn setup_garbler() -> Result<Garbler<DerandCOTSender<Sender<BaseReceiver>>>, Anyhow> {
    let base_receiver = BaseReceiver::new();

    let mut rng = StdRng::seed_from_u64(0);
    let delta = Block::random(&mut rng);

    let sender = Sender::new(SenderConfig::default(), delta, base_receiver);
    let sender = DerandCOTSender::new(sender);

    let garbler = Garbler::new(sender, [0u8; 16], Delta::new(delta));

    Ok(garbler)
}

pub async fn setup_evaluator() -> Result<Evaluator<DerandCOTReceiver<Receiver<BaseSender>>>, Anyhow>
{
    let base_sender = BaseSender::new();

    let receiver = Receiver::new(ReceiverConfig::default(), base_sender);
    let receiver = DerandCOTReceiver::new(receiver);

    let evaluator = Evaluator::new(receiver);

    Ok(evaluator)
}
