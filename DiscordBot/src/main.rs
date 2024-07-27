use std::env;
use std::collections::HashSet;

use serenity::{
    model::{
        channel::Ready,
        prelude::Activity,
        gateway::Ready,
        event::ResumedEvent
    },
    http::Http,
    framework::standard::{
        macros::{
            group,
            hook,
        },
        StandardFramework,
    },
    async_trait,
    prelude::*,
};
use tracing::{debug, error, info, instrument};


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!(
            "Connected as --> {} [id: {}]",
            ready.user.name, ready.user.id
        );
        let status =
            env::var("DISCORD_STATUS").expect("Set your DISCORD_STATUS environment variable!");
        ctx.set_activity(Activity::playing(status)).await;
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file.");

    let token = env::var("DISCORD_TOKEN").expect("Set your DISCORD_TOKEN environment variable!");
    let prefix = env::var("PREFIX").expect("Set your PREFIX environment variable!");

    let http = Http::new(&token);

    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Initialise error tracing
    tracing_subscriber::fmt::init();

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(prefix))
        .before(before)
        .group(&GENERAL_GROUP);

    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_VOICE_STATES;

    let mut client = Client::builder(&token, intents)
        .framework(framework)
        .register_songbird()
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<ShardManagerContainer>(client.shard_manager.clone());
    }

    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}