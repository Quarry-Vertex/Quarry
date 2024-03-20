// move to it's own binary this should be in a main function
// sends 'amount' btc to wallet 'address'
// amount will be passed into as SAT
pub async fn peg_out (address: &str, amount: f64) {
   let event_stream = pool.event::<()>(&"PegOutQSAT")
        .filter(filter)
        .stream(Duration::from_secs(1));

    // Process incoming events
    event_stream.for_each(|event| {
        // Handle the event here
        println!("Received event: {:?}", event);
        Ok(())
    }).await?;
}