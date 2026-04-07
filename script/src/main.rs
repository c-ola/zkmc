use clap::{Parser, Subcommand};
use sp1_sdk::{Elf, ProveRequest, Prover, ProverClient, ProvingKey, SP1ProofWithPublicValues, SP1Stdin};
use std::{error::Error, path::PathBuf, time::Instant};

const GUEST_ELF: &[u8] = include_bytes!(env!("SP1_ELF_mc-program"));

#[derive(Parser)]
#[command(name = "zkmc-sh-prover", about = "Prove Knowledge of Minecraft strongholds with a ZKVM")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Prove {
        #[arg(short, long, help = "Minecraft World Seed")]
        seed: i64,
        #[arg(short, long, help = "Block X Position")]
        x: i32,
        #[arg(short, long, help = "Block Z Position")]
        z: i32,
        #[arg(short, long, default_value = "proof.bin")]
        output: PathBuf,
    },
    Verify {
        #[arg(short, long, help = "Minecraft World Seed")]
        seed: i64,
        #[arg(short, long, default_value = "proof.bin")]
        input: PathBuf,
    },

    Execute {
        #[arg(short, long, help = "Minecraft World Seed")]
        seed: i64,
        #[arg(short, long, help = "Block X Position")]
        x: i32,
        #[arg(short, long, help = "Block Z Position")]
        z: i32,
    },
    Average {
        #[arg(short, long, help = "Minecraft World Seed")]
        seed: i64,
    },
}

pub fn execute_all_seeds() {
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    sp1_sdk::utils::setup_logger();
    let args = Cli::parse();
    let client = ProverClient::from_env().await;
    let guest_elf = Elf::Static(GUEST_ELF);

    match args.command {
        Commands::Average { seed } => {

            let locations = mc_lib::generate_strongholds(seed);
            let mut cycle_counts = Vec::new();
            for loc in &locations {
                let x = (loc.x * 16) + 4;
                let z = (loc.y * 16) + 4;
                let mut stdin = SP1Stdin::new();
                stdin.write(&seed);
                stdin.write(&x);
                stdin.write(&z);
                let (mut _public_values, report) = client.execute(guest_elf.clone(), stdin).await?;
                //println!("Total Count = {}", report.total_instruction_count());
                cycle_counts.push(report.total_instruction_count())
            }
            println!("{cycle_counts:?}");
            let len = cycle_counts.len();
            let sum: u64 = cycle_counts.iter().sum();
            println!("avg={}", sum as f64 / len as f64);
        }
        Commands::Execute { seed, x, z } => {
            let mut stdin = SP1Stdin::new();
            stdin.write(&seed);
            stdin.write(&x);
            stdin.write(&z);
            let (mut _public_values, report) = client.execute(guest_elf, stdin).await?;
            println!("Total Count = {}", report.total_instruction_count())
        }
        Commands::Prove { seed, x, z, output } => {
            let mut stdin = SP1Stdin::new();
            stdin.write(&seed);
            stdin.write(&x);
            stdin.write(&z);

            let pk = client.setup(guest_elf).await?;
            println!("Generating proof for seed {} at ({}, {})...", seed, x, z);
            let start = Instant::now();
            let proof = client.prove(&pk, stdin).await?;
            println!("Generated proof in {}s", start.elapsed().as_secs_f32());

            proof.save(&output).expect("Failed to save proof");
            println!("Proof saved to {:?}", output);
        }

        Commands::Verify { seed, input } => {
            let pk = client.setup(guest_elf).await?;
            let mut proof = SP1ProofWithPublicValues::load(&input).expect("Failed to load proof");

            println!("Verifying proof from {:?}...", input);
            let start = Instant::now();
            client.verify(&proof, &pk.verifying_key(), None).expect("Verification Failed");
            println!("Verified in {}s", start.elapsed().as_secs_f32());

            let proven_seed = proof.public_values.read::<i64>();

            if proven_seed != seed {
                println!("Proof is for a different seed!");
            } else {
                println!("Valid Proof for seed {proven_seed}");
            }
        }
    }
    Ok(())
}
