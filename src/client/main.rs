use clap::{Parser, Subcommand};
use std::env;

use authentication::auth_client::AuthClient;
use authentication::{SignInRequest, SignOutRequest, SignUpRequest};
use tonic::transport::Channel;
use tonic::{Request, Response};

use crate::authentication::{SignInResponse, SignOutResponse, SignUpResponse};

pub mod authentication {
    tonic::include_proto!("authentication");
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    SignIn {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignUp {
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: String,
    },
    SignOut {
        #[arg(short, long)]
        session_token: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // AUTH_SERVICE_IP can be set to your droplet's ip address once your app is deployed
    let auth_ip = env::var("AUTH_SERVICE_IP").unwrap_or("[::0]".to_owned());
    let mut client: AuthClient<Channel> =
        AuthClient::connect(format!("http://{}:50051", auth_ip)).await?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::SignIn { username, password }) => {
            let request: Request<SignInRequest> = Request::new(SignInRequest {
                username: username.clone(),
                password: password.clone(),
            }); // Create a new `SignInRequest`.

            // Make a sign in request. Propagate any errors. Convert Response<SignInResponse> into SignInResponse.
            let response: SignInResponse = client.sign_in(request).await?.into_inner();

            println!("{:?}", response);
        }
        Some(Commands::SignUp { username, password }) => {
            let request: Request<SignUpRequest> = Request::new(SignUpRequest {
                username: username.clone(),
                password: password.clone(),
            }); // Create a new `SignUpRequest`.

            let response: Response<SignUpResponse> = client.sign_up(request).await?; // Make a sign-up request. Propagate any errors.

            println!("{:?}", response.into_inner());
        }
        Some(Commands::SignOut { session_token }) => {
            let request: Request<SignOutRequest> = Request::new(SignOutRequest {
                session_token: session_token.clone(),
            }); // Create a new `SignOutRequest`.

            let response: Response<SignOutResponse> = client.sign_out(request).await?; // Make a sign-out request. Propagate any errors.

            println!("{:?}", response.into_inner());
        }
        None => {}
    }

    Ok(())
}
