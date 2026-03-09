import BN from "bn.js";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import type { CrudPdaProyecto } from "../target/types/crud_pda_proyecto";

// Configure the client to use the local cluster
anchor.setProvider(anchor.AnchorProvider.env());

const program = anchor.workspace.CrudPdaProyecto as anchor.Program<CrudPdaProyecto>;

const id = new anchor.BN(1);
const [notaPDA] = anchor.web3.PublicKey.findProgramAddressSync(
  [Buffer.from("nota"), program.provider.publicKey.toBuffer(), id.toArrayLike(Buffer, "le", 8)],
  program.programId
);

console.log("La dirección de tu nota es:", notaPDA.toBase58());

// Ejecuta la función crear
await program.methods
  .crearNota(id, "Mi primera nota con PDA")
  .accounts({
    nota: notaPDA,
    usuario: program.provider.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  })
  .rpc();

console.log("¡Nota creada exitosamente!");