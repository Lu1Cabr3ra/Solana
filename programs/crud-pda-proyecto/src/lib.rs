use anchor_lang::prelude::*;

declare_id!("6WiRUomE9Pdf8PbU4byzwnbsziNgmg8fwiAtginJee9x"); 

#[program]
pub mod crud_pda_proyecto {
    use super::*;

    pub fn crear_nota(ctx: Context<CrearNota>, id: u64, contenido: String) -> Result<()> {
        let nota = &mut ctx.accounts.nota;
        nota.autor = *ctx.accounts.usuario.key;
        nota.contenido = contenido;
        nota.id = id;
        Ok(())
    }

    pub fn actualizar_nota(ctx: Context<ActualizarNota>, nuevo_contenido: String) -> Result<()> {
        let nota = &mut ctx.accounts.nota;
        nota.contenido = nuevo_contenido;
        Ok(())
    }

    pub fn borrar_nota(_ctx: Context<BorrarNota>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CrearNota<'info> {
    // Aquí es donde sucede la  PDA usando semillas (seeds)
    #[account(
        init, 
        payer = usuario, 
        space = 8 + 32 + 8 + 200,
        seeds = [b"nota", usuario.key().as_ref(), id.to_le_bytes().as_ref()],
        bump
    )]
    pub nota: Account<'info, Nota>,
    #[account(mut)]
    pub usuario: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ActualizarNota<'info> {
    #[account(mut, has_one = autor)]
    pub nota: Account<'info, Nota>,
    pub autor: Signer<'info>,
}

#[derive(Accounts)]
pub struct BorrarNota<'info> {
    #[account(mut, has_one = autor, close = autor)]
    pub nota: Account<'info, Nota>,
    pub autor: Signer<'info>,
}

#[account]
pub struct Nota {
    pub autor: Pubkey,
    pub id: u64,
    pub contenido: String,
}