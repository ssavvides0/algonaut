use algonaut::algod::v2::Algod;
use algonaut::core::MicroAlgos;
use algonaut::transaction::{account::Account, TxnBuilder};
use algonaut_transaction::account::Pay;
use dotenv::dotenv;
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load variables in .env
    dotenv().ok();

    let algod = Algod::new(&env::var("ALGOD_URL")?, &env::var("ALGOD_TOKEN")?)?;
    let alice = Account::from_mnemonic("fire enlist diesel stamp nuclear chunk student stumble call snow flock brush example slab guide choice option recall south kangaroo hundred matrix school above zero")?;
    let bob = Account::from_mnemonic("since during average anxiety protect cherry club long lawsuit loan expand embark forum theory winter park twenty ball kangaroo cram burst board host ability left")?;

    let payment = alice.pay(&bob, MicroAlgos(123_456));
    let params = algod.suggested_transaction_params().await?;

    let txn = TxnBuilder::with(&params, payment).build()?;

    let sign_response = alice.sign_transaction(txn)?;

    let send_response = algod.broadcast_signed_transaction(&sign_response).await;
    println!("response: {:?}", send_response);

    Ok(())
}
