use crate::websocket::subscriptions::subscribe_message;
use futures_util::SinkExt;
use tokio_tungstenite::tungstenite::protocol::Message;

pub async fn subscribe_to_streams<S>(
    write: &mut S,
    streams: &[String],
    base_id: u64,
) -> Result<(), Box<dyn std::error::Error>>
where
    S: SinkExt<Message> + Unpin,
    <S as futures_util::Sink<Message>>::Error: std::error::Error + 'static,
{
    let subscribe_msg = subscribe_message(streams.to_vec(), base_id);
    println!("Subscribe Payload: {}", subscribe_msg);
    write.send(Message::Text(subscribe_msg)).await?;
    Ok(())
}
