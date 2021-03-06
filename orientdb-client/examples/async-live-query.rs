use async_std::task::{self, block_on};
use futures::StreamExt;
use orientdb_client::aio::OrientDB;
use orientdb_client::OrientResult;

fn main() -> OrientResult<()> {
    block_on(async {
        let client = OrientDB::connect(("localhost", 2424)).await?;
        let session = client.session("demodb", "admin", "admin").await?;

        let (unsubscriber, mut stream) = session.live_query("select from V").run().await?;

        task::spawn(async move {
            let _ = session
                .command("insert into V set id = 1")
                .run()
                .await
                .expect("Failed to run the query");
            unsubscriber
                .unsubscribe()
                .await
                .expect("Failed to unsbubscrbe the live query");
        });

        while let Some(item) = stream.next().await {
            println!("Event {:?}", item?);
        }

        Ok(())
    })
}
